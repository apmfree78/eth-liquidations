use eth_liquadation::events::aave_events::update_users_with_event_from_log;
use eth_liquadation::exchanges::aave_v3::events::{
    BORROW_SIGNATURE, REPAY_SIGNATURE, RESERVE_USED_AS_COLLATERAL_DISABLED_SIGNATURE,
    RESERVE_USED_AS_COLLATERAL_ENABLED_SIGNATURE, SUPPLY_SIGNATURE, WITHDRAW_SIGNATURE,
};
use eth_liquadation::mempool::detect_price_update::detect_price_update;
use eth_liquadation::{
    events::aave_events::scan_and_update_aave_events,
    exchanges::aave_v3::user_data::{AaveUserData, Generate},
};
use ethers::abi::Address;
use ethers::providers::Middleware;
use ethers::types::H256;
use ethers::{
    core::types::{Filter, Log, TxHash},
    providers::{Provider, Ws},
};
use futures::{stream, StreamExt};
use std::sync::Arc;

const WS_URL: &'static str = "ws://localhost:8546";
const AAVE_V3_POOL_ADDRESS: &'static str = "0x87870Bca3F3fD6335C3F4ce8392D69350B4fA4E2";
enum Event {
    // Block(ethers::types::Block<H256>),
    Log(Log),
    PendingTransactions(TxHash),
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);

    let mut aave_user_list = AaveUserData::get_users(&client).await?;

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
    // detect_price_update(&client).await?;
    // Create multiple subscription streams.
    let log_stream: stream::BoxStream<'_, Result<Event, Box<dyn std::error::Error + Send + Sync>>> =
        client
            .subscribe_logs(&filter)
            .await?
            .map(|log| Ok(Event::Log(log)))
            .boxed();

    println!("Subscribed to logs");

    let tx_stream: stream::BoxStream<'_, Result<Event, Box<dyn std::error::Error + Send + Sync>>> =
        client
            .subscribe_pending_txs()
            .await?
            .map(|tx| Ok(Event::PendingTransactions(tx)))
            .boxed();

    println!("Subscribed to pending transactions");

    // Merge the streams into a single stream.
    let combined_stream = stream::select_all(vec![log_stream, tx_stream]);

    println!("Combined streams");

    combined_stream
        .for_each(|event| async {
            match event {
                Ok(Event::Log(log)) => {
                    println!("new log found! ==> {:#?}", log);
                    // TODO - set this up with Mutex
                    // update_users_with_event_from_log(log, &mut aave_user_list)
                }
                Ok(Event::PendingTransactions(tx)) => {
                    // println!("New pending transaction: {:?}", tx);
                    let _ = detect_price_update(tx, &client).await;
                }
                Err(e) => eprintln!("Error: {:?}", e),
            }
        })
        .await;

    // Your code to handle the events goes here.
    // println!(" user list {:#?} ", aave_user_list);

    // scan_and_update_aave_events(&mut aave_user_list, &client).await?;
    // for user in &aave_user_list {
    //     let calculated_health_factor = user
    //         .get_health_factor_from_(PricingSource::UniswapV3, &client)
    //         .await?;

    //     // TODO - 10% of graphl data is corrupted and unrealiable for user token amount ( and maybe
    //     // the tokens themselves) please clean out this corrupted data
    //     println!(" user data {:#?}", user);
    //     println!("----------------------------------------");
    //     println!(" user heath factor {}", user.health_factor);
    //     println!(
    //         " user heath factor oracle {}",
    //         user.get_health_factor_from_(PricingSource::AaveOracle, &client)
    //             .await?
    //     );
    //     println!(" user heath factor ft {}", calculated_health_factor);
    //     if calculated_health_factor > BigDecimal::from(10) * &user.health_factor {
    //         println!(" user data {:#?}", user);
    //     }
    // }
    Ok(())
}
