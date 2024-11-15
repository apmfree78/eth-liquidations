use anyhow::Result;
use dotenv::dotenv;
use eth_liquadation::{
    data::{
        token_data_hash::{save_btc_as_token, save_erc20_tokens_from_static_data},
        token_price_hash::{generate_token_price_hash, print_saved_token_prices},
    },
    events::aave_events::{set_aave_event_signature_filter, update_users_with_event_from_log},
    exchanges::aave_v3::{
        implementations::aave_user_data::GenerateUsers,
        user_structs::{AaveUserData, AaveUsersHash, SampleSize},
    },
    interest::calculate_interest::update_interest_for_all_whale_users_tokens,
    mempool::detect_price_update::detect_price_update_and_find_users_to_liquidate,
    utils::logging::setup_logger,
    validate::aave_users,
};
use ethers::{
    core::types::{Log, TxHash},
    providers::{Middleware, Provider, Ws},
    types::{Address, BlockNumber},
};
use futures::{lock::Mutex, stream, StreamExt};
use log::{debug, error, info};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

// SET ws url and CHAIN we are using
const WS_URL: &str = "ws://localhost:8546";

enum Event {
    Block(ethers::types::Block<TxHash>),
    AaveV3Log(Log),
    PendingTransactions(TxHash),
}

#[tokio::main]
async fn main() -> Result<()> {
    // initiate logger and environment variables
    dotenv().ok();
    setup_logger().expect("Failed to initialize logger.");

    // setup provider

    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);

    let initial_block = client.get_block(BlockNumber::Latest).await?.unwrap();
    let last_block_timestamp = initial_block.timestamp.as_u32();
    info!("initial block timestamp => {}", last_block_timestamp);
    let last_block_timestamp = Arc::new(Mutex::new(last_block_timestamp));

    // need this otherwise cannot reconstruct user data from sratch
    save_erc20_tokens_from_static_data(&client).await?;

    let aave_users = Arc::new(Mutex::new(AaveUsersHash {
        user_data: HashMap::<Address, AaveUserData>::new(),
        standard_user_ids_by_token: HashMap::<Address, HashSet<Address>>::new(),
        whale_user_ids_by_token: HashMap::<Address, HashSet<Address>>::new(),
    }));

    AaveUserData::get_users(&aave_users, SampleSize::All).await?;

    // Initialize TOKEN_PRICE_HASH global hashmap of token prices and save mock BTC TOKEN
    save_btc_as_token(&client).await?;
    if let Err(e) = generate_token_price_hash(&client).await {
        error!("Failed to initialize token prices: {}", e);
    }

    print_saved_token_prices().await?;

    let aave_event_filter = set_aave_event_signature_filter()?;
    // Create multiple subscription streams.
    let aave_log_stream: stream::BoxStream<'_, Result<Event>> = client
        .subscribe_logs(&aave_event_filter)
        .await?
        .map(|log| Ok(Event::AaveV3Log(log)))
        .boxed();

    info!("Subscribed to aave v3 logs");

    let tx_stream: stream::BoxStream<'_, Result<Event>> = client
        .subscribe_pending_txs()
        .await?
        .map(|tx| Ok(Event::PendingTransactions(tx)))
        .boxed();

    let block_stream: stream::BoxStream<'_, Result<Event>> = client
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
            let aave_users_data = Arc::clone(&aave_users);
            let last_timestamp = Arc::clone(&last_block_timestamp);

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
                    let mut last_time = last_timestamp.lock().await;
                    let current_block_timestamp = block.timestamp.as_u32();

                    if let Err(error) = update_interest_for_all_whale_users_tokens(
                        *last_time,
                        current_block_timestamp,
                        &aave_users_data,
                    )
                    .await
                    {
                        error!("problem with interest rate update => {}", error);
                    };

                    *last_time = current_block_timestamp;

                    // FOR TESTING ONLY
                    // debug!("using these prices to find health factor in new block");
                    // let _ = generate_token_price_hash(&client).await;
                    // if let Err(_) = print_saved_token_prices().await {
                    //     error!("could not print out prices");
                    // };

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
