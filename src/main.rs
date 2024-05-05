use bigdecimal::BigDecimal;
use eth_liquadation::{
    events::aave_events::scan_and_update_aave_events,
    exchanges::aave_v3::data::{AaveUserData, Generate, HealthFactor, PricingSource},
};
use ethers::providers::{Provider, Ws};
use std::sync::Arc;

const WS_URL: &str = "ws://localhost:8546";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);

    let mut aave_user_list = AaveUserData::get_users(&client).await?;
    // println!(" user list {:#?} ", aave_user_list);

    scan_and_update_aave_events(&mut aave_user_list, &client).await?;
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

    // scan published blocks for aave user events where account data is updated
    // if event found with matching user then update data for that user,
    // this includes adding in new tokens
    // TODO - create method(s) to take event info and update relevant user in aave_user_list

    // also scan for mempool events for oracle updates, if found then:
    //1. find all users that are borrowing or using token as colladeral
    // 2. re-calculate their health factor with updated token price (from uniswap v3)
    // TODO - create method to update relevant user health factor
}
