use dotenv::dotenv;
use eth_liquadation::{
    data::token_price_hash::{generate_token_price_hash, print_saved_token_prices},
    events::aave_events::{set_aave_event_signature_filter, update_users_with_event_from_log},
    exchanges::aave_v3::{
        implementations::aave_user_data::GenerateUsers,
        user_structs::{AaveUserData, SampleSize},
    },
    mempool::detect_price_update::detect_price_update_and_find_users_to_liquidate,
    utils::logging::setup_logger,
    validate::aave_users,
};
use ethers::{
    core::types::{Log, TxHash},
    providers::{Middleware, Provider, Ws},
};
// use ethers::{
//     middleware::SignerMiddleware,
//     signers::{LocalWallet, Signer},
// };
use futures::{lock::Mutex, stream, StreamExt};
use log::{error, info, warn};
use std::sync::Arc;

const WS_URL: &str = "ws://localhost:8546";

enum Event {
    Block(ethers::types::Block<TxHash>),
    AaveV3Log(Log),
    PendingTransactions(TxHash),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initiate logger and environment variables
    dotenv().ok();
    setup_logger().expect("Failed to initialize logger.");

    // let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not found in .env file");

    // setup provider
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);

    // create wallet from private key
    // let wallet: LocalWallet = private_key.parse()?;

    // sign the wallet with provider
    // let signed_client = SignerMiddleware::new(client.clone(), wallet.with_chain_id(1u64));
    // let signed_client = Arc::new(signed_client);

    let aave_users = AaveUserData::get_users(&client, SampleSize::All).await?;

    // Initialize TOKEN_PRICE_HASH global hashmap of token prices
    if let Err(e) = generate_token_price_hash(&client).await {
        error!("Failed to initialize token prices: {}", e);
    }

    print_saved_token_prices().await?;

    let aave_users = Arc::new(Mutex::new(aave_users));

    let aave_event_filter = set_aave_event_signature_filter()?;
    // Create multiple subscription streams.
    let aave_log_stream: stream::BoxStream<
        '_,
        Result<Event, Box<dyn std::error::Error + Send + Sync>>,
    > = client
        .subscribe_logs(&aave_event_filter)
        .await?
        .map(|log| Ok(Event::AaveV3Log(log)))
        .boxed();

    info!("Subscribed to aave v3 logs");

    let tx_stream: stream::BoxStream<'_, Result<Event, Box<dyn std::error::Error + Send + Sync>>> =
        client
            .subscribe_pending_txs()
            .await?
            .map(|tx| Ok(Event::PendingTransactions(tx)))
            .boxed();

    let block_stream: stream::BoxStream<
        '_,
        Result<Event, Box<dyn std::error::Error + Send + Sync>>,
    > = client
        .subscribe_blocks()
        .await?
        .map(|block| Ok(Event::Block(block)))
        .boxed();

    info!("Subscribed to pending transactions");

    // Merge the streams into a single stream.
    let combined_stream = stream::select_all(vec![aave_log_stream, tx_stream, block_stream]);

    info!("Combined streams");

    combined_stream
        .for_each(|event| async {
            let client = Arc::clone(&client);
            // let signed_client = Arc::clone(&signed_client);

            let aave_users_data = Arc::clone(&aave_users);
            match event {
                Ok(Event::AaveV3Log(log)) => {
                    let mut users = aave_users_data.lock().await;
                    if let Err(error) =
                        update_users_with_event_from_log(log, &mut users, &client).await
                    {
                        error!("could not update users => {}", error);
                    }
                }
                Ok(Event::PendingTransactions(tx)) => {
                    if let Err(error) = detect_price_update_and_find_users_to_liquidate(
                        &aave_users_data,
                        tx,
                        &client,
                    )
                    .await
                    {
                        error!("problem with price update detection => {}", error);
                    }
                }
                Ok(Event::Block(block)) => {
                    info!("NEW BLOCK ===> {}", block.timestamp);
                    // TODO - pass signed client
                    if let Err(error) = aave_users::validate_liquidation_candidates(&client).await {
                        error!("error looking up liquidation candidates => {}", error);
                    }
                }
                Err(e) => error!("Error: {:?}", e),
            }
        })
        .await;

    Ok(())
}
