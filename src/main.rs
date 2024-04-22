// use bigdecimal::{BigDecimal, FromPrimitive, Zero};
// use eth_liquadation::abi::aave_oracle::AAVE_ORACLE;
use eth_liquadation::abi::aave_v3_pool::AAVE_V3_POOL;
// use eth_liquadation::abi::erc20::ERC20;
use eth_liquadation::crypto_data::{
    generate_token, Generate, AAVE_ORACLE_ADDRESS, AAVE_V3_POOL_ADDRESS, TOKEN_DATA, WETH_ADDRESS,
};
use eth_liquadation::crypto_data::{u256_to_big_decimal, AaveUserData};
use eth_liquadation::get_aave_users::{get_aave_v3_users, UserAccountData};
use ethers::abi::Address;
use ethers::providers::{Provider, Ws};
use std::sync::Arc;
use uniswap_sdk_core::{constants::Rounding, prelude::*};
// use uniswap_v3_sdk::{
//     constants::{FeeAmount, FACTORY_ADDRESS},
//     extensions::get_pool,
// };

const WS_URL: &str = "ws://localhost:8546";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);

    let aave_user_list = AaveUserData::get_users(&client).await?;
    println!(" user list {:#?} ", aave_user_list);

    // for user in &aave_user_list {
    //     // let calculated_health_factor = user.get_health_factor_with_uniswap_v3(&client).await?;

    //     println!(" user data {:#?}", user);
    //     println!(" user heath factor {}", user.health_factor);
    //     // println!(" user calculated heath factor {}", calculated_health_factor);
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
