use crate::abi::liquidate_user::{User, LIQUIDATE_USER};
use crate::backrun::simulation::convert_transaction_to_typed_transaction;
use crate::data::address::CONTRACT;
use crate::data::erc20::u256_to_big_decimal;
use crate::exchanges::aave_v3::user_structs::{LiquidationCandidate, PROFIT_THRESHOLD_MAINNET};
use crate::utils::type_conversion::{f64_to_u256, usd_to_eth};
use anyhow::{anyhow, Result};
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use ethers::core::rand::thread_rng;
use ethers::types::{Address, Block, BlockNumber, Bytes, TraceType, Transaction, H256, U256};
use ethers::{
    core::types::{transaction::eip2718::TypedTransaction, Chain},
    middleware::SignerMiddleware,
    providers::{Middleware, Provider, Ws},
    signers::{LocalWallet, Signer},
    types::{Eip1559TransactionRequest, NameOrAddress},
};
use ethers_flashbots::{BroadcasterMiddleware, BundleRequest, PendingBundleError, SimulatedBundle};
use log::{debug, error, info};
use rand::Rng;
use std::cmp::min;
use std::str::FromStr;
use std::{env, sync::Arc};
use url::Url;

const BRIBE_PERCENTAGE: f64 = 0.15;

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
    let liquidate_user_address: Address = CONTRACT.get_address().liquidate_user.parse()?;
    // get last block number
    let block = client.get_block(BlockNumber::Latest).await?.unwrap();

    let next_base_fee = calculate_next_block_base_fee(&block)?;
    let buffer = next_base_fee / 20; // Add 5% buffer
    let adjusted_max_fee = next_base_fee + buffer;

    // *******************************************************
    // CREATE BACKRUN Transaction
    let calldata = get_liquidate_user_calldata(user, client)?;

    let backrun_tx = Eip1559TransactionRequest {
        chain_id: Some(Chain::Mainnet.into()), // Mainnet
        max_priority_fee_per_gas: Some(U256::from(0)),
        max_fee_per_gas: Some(adjusted_max_fee),
        to: Some(NameOrAddress::Address(liquidate_user_address)),
        data: Some(calldata), // Encoded data for the transaction
        value: None,
        ..Default::default()
    };

    // *******************************************************
    // CREATE SIGNED CLIENT WITH FLASHBOT MIDDLEWARE SET TO BROADCAST TO FLASHBOT AND BUILDER RELAYS
    let client_signed = Arc::clone(client); // need this to avoid lifetime not long enough error

    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not found in .env file");
    // let private_key_searcher = env::var("PRIVATE_KEY_SEARCHER").expect("PRIVATE_KEY_SEARCHER not found in .env file");

    // create wallet from private key
    let wallet = LocalWallet::from_str(&private_key)?.with_chain_id(Chain::Mainnet);

    // This is your searcher identity
    let bundle_signer = LocalWallet::new(&mut thread_rng()); // for productio deployment
                                                             // let bundle_signer: LocalWallet = private_key_searcher.parse()?;

    // Add signer and Flashbots middleware
    let client_signed = SignerMiddleware::new(
        BroadcasterMiddleware::new(
            client_signed,
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

    // Specify block number
    let next_block_number = BlockNumber::Pending.as_number().unwrap();

    let signature = client_signed
        .signer()
        .sign_transaction(&TypedTransaction::Eip1559(backrun_tx.clone()))
        .await?;

    let bundle = BundleRequest::new()
        .push_transaction(mempool_tx.clone())
        .push_transaction(TypedTransaction::Eip1559(backrun_tx.clone()).rlp_signed(&signature))
        .set_block(next_block_number)
        .set_simulation_block(next_block_number)
        .set_simulation_timestamp(0);

    // *******************************************************
    // SIMULATE SENDING BUNDLE AND LISTEN FOR RESPONSE
    // Simulate it
    let simulated_bundle: SimulatedBundle = client_signed.inner().simulate_bundle(&bundle).await?;
    println!("Simulated bundle: {:?}", simulated_bundle);

    if is_simulation_success(&simulated_bundle) {
        info!("Flashbot bundle submission simulated successfully")
    } else {
        error!("error submitting flashbots bundle");
        return Ok(());
    }

    // *******************************************************
    // FROM SIMULATION GET GAS COST, NET PROFIT, AND MINER BRIBE NEEDED FOR PRODUCTION
    //  get gas used
    let gas_used = simulated_bundle.transactions[1].gas_used;
    let gas_used = gas_used.low_u64();

    // check that tranascation cost is not too high
    let transaction_cost = get_transaction_cost_in_eth(&backrun_tx, gas_used, adjusted_max_fee)?;
    let profit_in_eth = get_estimated_transaction_profit_in_eth(user).await?;

    if 2.0 * transaction_cost > profit_in_eth {
        debug!("Cost of Transaction is too high: {}", transaction_cost);
        return Ok(());
    } else {
        debug!("Estimated profit: {}", profit_in_eth);
        debug!("Cost of this transaction in ETH is: {}", transaction_cost);
    }

    let miner_bribe = BRIBE_PERCENTAGE * (profit_in_eth - transaction_cost);
    let miner_bribe = f64_to_u256(miner_bribe)?;
    let backrun_tx = Eip1559TransactionRequest {
        value: Some(miner_bribe),
        ..backrun_tx
    };

    // resign transaction
    let signature = client_signed
        .signer()
        .sign_transaction(&TypedTransaction::Eip1559(backrun_tx.clone()))
        .await?;

    // *******************************************************
    let production_bundle = BundleRequest::new()
        .push_transaction(mempool_tx)
        .push_transaction(TypedTransaction::Eip1559(backrun_tx).rlp_signed(&signature))
        .set_block(next_block_number)
        .set_simulation_block(next_block_number)
        .set_simulation_timestamp(0);

    // FOR PRODUCTION
    // Send it
    let results = client_signed
        .inner()
        .send_bundle(&production_bundle)
        .await?;

    // You can also optionally wait to see if the bundle was included
    for result in results {
        match result {
            Ok(pending_bundle) => match pending_bundle.await {
                Ok(bundle_hash) => println!(
                    "Bundle with hash {:?} was included in target block",
                    bundle_hash
                ),
                Err(PendingBundleError::BundleNotIncluded) => {
                    println!("Bundle was not included in target block.")
                }
                Err(e) => println!("An error occured: {}", e),
            },
            Err(e) => println!("An error occured: {}", e),
        }
    }

    Ok(())
}

fn get_liquidate_user_calldata(
    liquidation_users: &[LiquidationCandidate],
    client: &Arc<Provider<Ws>>,
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

async fn get_estimated_transaction_profit_in_eth(
    liquidation_users: &[LiquidationCandidate],
) -> Result<f64> {
    let mut total_profit = 0_f64;

    for user in liquidation_users {
        if user.estimated_profit > PROFIT_THRESHOLD_MAINNET {
            total_profit += user.estimated_profit;
        }
    }

    // convert to eth
    let profit_in_eth = usd_to_eth(total_profit).await?;

    Ok(profit_in_eth)
}

fn is_simulation_success(bundle: &SimulatedBundle) -> bool {
    for (index, transaction) in bundle.transactions.iter().enumerate() {
        if let Some(err) = &transaction.error {
            info!("Transaction {} failed with error: {}", index, err);
            return false;
        }
        if let Some(revert) = &transaction.revert {
            info!("Transaction {} reverted with message: {}", index, revert);
            return false;
        }
    }
    info!("All transactions in the bundle simulated successfully.");
    true
}

fn get_transaction_cost_in_eth(
    tx: &Eip1559TransactionRequest,
    gas_cost: u64,
    next_base_fee: U256,
) -> Result<f64> {
    let gas_price = min(tx.max_fee_per_gas.unwrap(), next_base_fee);

    // convert to U256
    let gas_cost = U256::from(gas_cost);

    let transaction_cost = gas_cost * gas_price;
    let transaction_cost_wei = u256_to_big_decimal(&transaction_cost);

    let wei_to_eth = BigDecimal::from_u64(10_u64.pow(18)).unwrap();
    let transaction_cost_eth = &transaction_cost_wei / &wei_to_eth;
    let transaction_cost_eth = transaction_cost_eth.to_f64().unwrap();
    Ok(transaction_cost_eth)
}

/// Calculate the next block base fee
pub fn calculate_next_block_base_fee(block: &Block<H256>) -> Result<U256> {
    // Get the block base fee per gas
    let base_fee = block
        .base_fee_per_gas
        .ok_or(anyhow!("Block missing base fee per gas"))?;

    // Get the mount of gas used in the block
    let gas_used = block.gas_used;

    // Get the target gas used
    let mut target_gas_used = block.gas_limit / 2;
    target_gas_used = if target_gas_used == U256::zero() {
        U256::one()
    } else {
        target_gas_used
    };

    // Calculate the new base fee
    let new_base_fee = {
        if gas_used > target_gas_used {
            base_fee
                + ((base_fee * (gas_used - target_gas_used)) / target_gas_used) / U256::from(8u64)
        } else {
            base_fee
                - ((base_fee * (target_gas_used - gas_used)) / target_gas_used) / U256::from(8u64)
        }
    };

    // Add a random seed so it hashes differently
    let seed = rand::thread_rng().gen_range(0..9);
    Ok(new_base_fee + seed)
}
