use crate::abi::liquidate_user::{User, LIQUIDATE_USER};
use crate::data::address::CONTRACT;
use crate::data::erc20::u256_to_big_decimal;
use crate::exchanges::aave_v3::user_structs::LiquidationCandidate;
use anyhow::Result;
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use ethers::core::rand::thread_rng;
use ethers::types::{Address, BlockNumber, Bytes, Transaction, U256};
use ethers::utils::{parse_units, ParseUnits};
use ethers::{
    core::types::{transaction::eip2718::TypedTransaction, Chain},
    middleware::SignerMiddleware,
    providers::{Middleware, Provider, Ws},
    signers::{LocalWallet, Signer},
    types::{Eip1559TransactionRequest, NameOrAddress},
};
use ethers_flashbots::{BroadcasterMiddleware, BundleRequest, PendingBundleError, SimulatedBundle};
use log::{debug, error};
use std::cmp::min;
use std::str::FromStr;
use std::{env, sync::Arc};
use url::Url;

const TRANSACTION_GAS_USED: u64 = 300_000;
const TRANSACTION_MAX_COST: f64 = 0.017;

// See https://www.mev.to/builders for up to date builder URLs
static BUILDER_URLS: &[&str] = &[
    "https://builder0x69.io",
    "https://rpc.beaverbuild.org",
    "https://relay.flashbots.net",
    "https://rsync-builder.xyz",
    "https://rpc.titanbuilder.xyz",
    "https://api.blocknative.com/v1/auction",
    "https://mev.api.blxrbdn.com",
    "https://eth-builder.com",
    "https://builder.gmbit.co/rpc",
    "https://buildai.net",
    "https://rpc.payload.de",
    "https://rpc.lightspeedbuilder.info",
    "https://rpc.nfactorial.xyz",
    "https://rpc.lokibuilder.xyz",
];

pub async fn submit_to_flashbots(
    user: &[LiquidationCandidate],
    mempool_tx: Transaction,
    client: &Arc<Provider<Ws>>,
) -> Result<()> {
    let minor_tip = U256::from(parse_units("0.003", 18)?);

    // check that tranascation cost is not too high
    let transaction_cost =
        get_transaction_cost_in_eth(&mempool_tx, TRANSACTION_GAS_USED, client).await?;

    if transaction_cost > TRANSACTION_MAX_COST {
        debug!("Cost of Transaction is too high: {}", transaction_cost);
        return Ok(());
    } else {
        debug!("Cost of this transaction in ETH is: {}", transaction_cost);
    }

    let liquidate_user_address: Address = CONTRACT.get_address().liquidate_user.parse()?;

    // *******************************************************
    // CREATE BACKRUN Transaction
    let calldata = get_liquidate_user_calldata(client, user)?;

    let backrun_tx = TypedTransaction::Eip1559(Eip1559TransactionRequest {
        chain_id: Some(Chain::Mainnet.into()), // Mainnet
        max_priority_fee_per_gas: mempool_tx.max_priority_fee_per_gas,
        max_fee_per_gas: mempool_tx.max_fee_per_gas,
        to: Some(NameOrAddress::Address(liquidate_user_address)),
        data: Some(calldata), // Encoded data for the transaction
        value: Some(minor_tip),
        ..Default::default()
    });
    // *******************************************************
    // CREATE SIGNED CLIENT WITH FLASHBOT MIDDLEWARE SET TO BROADCAST TO FLASHBOT AND BUILDER RELAYS
    let client = Arc::clone(client); // need this to avoid lifetime not long enough error

    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not found in .env file");
    // let private_key_searcher = env::var("PRIVATE_KEY_SEARCHER").expect("PRIVATE_KEY_SEARCHER not found in .env file");

    // create wallet from private key
    let wallet = LocalWallet::from_str(&private_key)?.with_chain_id(Chain::Mainnet);

    // This is your searcher identity
    let bundle_signer = LocalWallet::new(&mut thread_rng());
    // for productio deployment
    // let bundle_signer: LocalWallet = private_key_searcher.parse()?;

    // Add signer and Flashbots middleware
    let client = SignerMiddleware::new(
        BroadcasterMiddleware::new(
            client,
            BUILDER_URLS
                .iter()
                .map(|url| Url::parse(url).unwrap())
                .collect(),
            Url::parse("https://relay.flashbots.net")?,
            bundle_signer,
        ),
        wallet,
    );

    // *******************************************************
    // GENERATE Transaction BUNDLE FOR BACKRUN
    // get last block number
    let block_number = client.get_block_number().await?;

    let signature = client.signer().sign_transaction(&backrun_tx).await?;
    let bundle = BundleRequest::new()
        .push_transaction(backrun_tx.rlp_signed(&signature))
        .push_transaction(mempool_tx)
        .set_block(block_number + 1)
        .set_simulation_block(block_number)
        .set_simulation_timestamp(0);

    // *******************************************************
    // SIMULATE SENDING BUNDLE AND LISTEN FOR RESPONSE
    // Simulate it
    let simulated_bundle: SimulatedBundle = client.inner().simulate_bundle(&bundle).await?;
    println!("Simulated bundle: {:?}", simulated_bundle);

    if is_simulation_success(&simulated_bundle) {
        println!("Flashbot bundle submission simulated successfully")
    } else {
        error!("error submitting flashbots bundle");
    }

    // FOR PRODUCTION
    // Send it
    // let results = client.inner().send_bundle(&bundle).await?;
    //
    // // You can also optionally wait to see if the bundle was included
    // for result in results {
    //     match result {
    //         Ok(pending_bundle) => match pending_bundle.await {
    //             Ok(bundle_hash) => println!(
    //                 "Bundle with hash {:?} was included in target block",
    //                 bundle_hash
    //             ),
    //             Err(PendingBundleError::BundleNotIncluded) => {
    //                 println!("Bundle was not included in target block.")
    //             }
    //             Err(e) => println!("An error occured: {}", e),
    //         },
    //         Err(e) => println!("An error occured: {}", e),
    //     }
    // }

    Ok(())
}

pub fn get_liquidate_user_calldata(
    client: &Arc<Provider<Ws>>,
    liquidation_users: &[LiquidationCandidate],
) -> Result<Bytes> {
    let liquidate_user_address: Address = CONTRACT.get_address().liquidate_user.parse()?;
    // convert user to correct type
    let mut users = Vec::<User>::new();

    for user in liquidation_users {
        users.push(User {
            id: user.user_id,
            debt_token: user.debt_token,
            collateral_token: user.collateral_token,
        })
    }

    let liquidate_user = LIQUIDATE_USER::new(liquidate_user_address, client.clone());

    // Encode the function with parameters, and get TypedTransaction
    let calldata = liquidate_user
        .find_and_liquidate_account(users)
        .calldata()
        .expect("Failed to encode");

    Ok(calldata)
}

fn is_simulation_success(bundle: &SimulatedBundle) -> bool {
    for transaction in &bundle.transactions {
        if transaction.error.is_some() || transaction.revert.is_some() {
            // Log the error for debugging or further analysis
            if let Some(err) = &transaction.error {
                println!("Transaction failed with error: {}", err);
            }
            if let Some(revert) = &transaction.revert {
                println!("Transaction reverted with message: {}", revert);
            }
            return false;
        }
    }
    true // All transactions succeeded without errors or reverts
}

async fn get_transaction_cost_in_eth(
    tx: &Transaction,
    gas_cost: u64,
    client: &Arc<Provider<Ws>>,
) -> Result<f64> {
    let current_block = client.get_block(BlockNumber::Latest).await?.unwrap();
    let base_fee_per_gas = current_block.base_fee_per_gas.unwrap();

    let max_fee_per_gas = tx.max_fee_per_gas.unwrap();
    let max_priority_fee_per_gas = tx.max_priority_fee_per_gas.unwrap();

    let gas_price = min(max_fee_per_gas, max_priority_fee_per_gas + base_fee_per_gas);

    // convert to U256
    let gas_cost = U256::from(gas_cost);

    let transaction_cost = gas_cost * gas_price;
    let transaction_cost_wei = u256_to_big_decimal(&transaction_cost);

    let wei_to_eth = BigDecimal::from_u64(10_u64.pow(18)).unwrap();
    let transaction_cost_eth = &transaction_cost_wei / &wei_to_eth;
    let transaction_cost_eth = transaction_cost_eth.to_f64().unwrap();
    Ok(transaction_cost_eth)
}
