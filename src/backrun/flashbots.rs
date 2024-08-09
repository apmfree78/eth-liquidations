use ethers::core::rand::thread_rng;
use ethers::types::Transaction;
use ethers::{
    core::types::{transaction::eip2718::TypedTransaction, Chain, TransactionRequest},
    middleware::SignerMiddleware,
    providers::{Middleware, Provider, Ws},
    signers::{LocalWallet, Signer, Wallet},
    types::Address,
};
use ethers_flashbots::{BroadcasterMiddleware, BundleRequest, PendingBundleError, SimulatedBundle};
use log::error;
use std::str::FromStr;
use std::{env, sync::Arc};
use url::Url;

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

// TODO - have chatgpt review

pub async fn submit_to_flashbots(
    client: &Arc<Provider<Ws>>,
    backrun_tx: TypedTransaction, // the transaction that will backrup mempool_tx
    mempool_tx: Transaction,
) -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the network

    let client = Arc::clone(client);

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

    // get last block number
    let block_number = client.get_block_number().await?;

    let signature = client.signer().sign_transaction(&backrun_tx).await?;
    let bundle = BundleRequest::new()
        .push_transaction(backrun_tx.rlp_signed(&signature))
        .push_transaction(mempool_tx)
        .set_block(block_number + 1)
        .set_simulation_block(block_number)
        .set_simulation_timestamp(0);

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
