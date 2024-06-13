use eth_liquadation::{
    data::token_price_hash::{generate_token_price_hash, print_saved_token_prices},
    events::aave_events::update_users_with_event_from_log,
    exchanges::aave_v3::{
        events::{
            BORROW_SIGNATURE, REPAY_SIGNATURE, RESERVE_USED_AS_COLLATERAL_DISABLED_SIGNATURE,
            RESERVE_USED_AS_COLLATERAL_ENABLED_SIGNATURE, SUPPLY_SIGNATURE, WITHDRAW_SIGNATURE,
        },
        implementations::aave_user_data::GenerateUsers,
        user_structs::{AaveUserData, SampleSize},
    },
    mempool::detect_price_update::detect_price_update_and_find_users_to_liquidate,
    utils::logging::setup_logger,
};
use ethers::{
    abi::Address,
    core::types::{Filter, Log, TxHash},
    providers::Middleware,
    providers::{Provider, Ws},
};
use futures::{lock::Mutex, stream, StreamExt};
use log::{error, info};
use std::sync::Arc;

const WS_URL: &str = "ws://localhost:8546";
const AAVE_V3_POOL_ADDRESS: &str = "0x87870Bca3F3fD6335C3F4ce8392D69350B4fA4E2";
enum Event {
    // Block(ethers::types::Block<H256>),
    AaveV3Log(Log),
    PendingTransactions(TxHash),
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initiate logger
    setup_logger().expect("Failed to initialize logger.");

    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);

    let user_hash = AaveUserData::get_users(&client, SampleSize::SmallBatch).await?;

    // Initialize TOKEN_PRICE_HASH global hashmap of token prices
    if let Err(e) = generate_token_price_hash(&client).await {
        error!("Failed to initialize token prices: {}", e);
    }

    print_saved_token_prices().await?;

    let user_data = Arc::new(Mutex::new(user_hash));

    let filter = Filter::new()
        .address(AAVE_V3_POOL_ADDRESS.parse::<Address>()?)
        .events(
            [
                WITHDRAW_SIGNATURE,
                BORROW_SIGNATURE,
                REPAY_SIGNATURE,
                SUPPLY_SIGNATURE,
                RESERVE_USED_AS_COLLATERAL_ENABLED_SIGNATURE,
                RESERVE_USED_AS_COLLATERAL_DISABLED_SIGNATURE,
            ]
            .to_vec(),
        );

    // Create multiple subscription streams.
    let log_stream: stream::BoxStream<'_, Result<Event, Box<dyn std::error::Error + Send + Sync>>> =
        client
            .subscribe_logs(&filter)
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

    info!("Subscribed to pending transactions");

    // Merge the streams into a single stream.
    let combined_stream = stream::select_all(vec![log_stream, tx_stream]);

    info!("Combined streams");

    combined_stream
        .for_each(|event| async {
            let client = Arc::clone(&client);
            let user_data = Arc::clone(&user_data);
            match event {
                Ok(Event::AaveV3Log(log)) => {
                    let mut users = user_data.lock().await;
                    if let Err(error) =
                        update_users_with_event_from_log(log, &mut users, &client).await
                    {
                        error!("could not update users => {}", error);
                    }
                }
                Ok(Event::PendingTransactions(tx)) => {
                    if let Err(error) =
                        detect_price_update_and_find_users_to_liquidate(&user_data, tx, &client)
                            .await
                    {
                        error!("problem with price update detection => {}", error);
                    }
                }
                Err(e) => error!("Error: {:?}", e),
            }
        })
        .await;

    Ok(())
}
