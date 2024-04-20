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

    let mut aave_user_list = AaveUserData::get_users(&client).await?;

    println!(" user data {:#?}", aave_user_list);
    Ok(())

    // scan published blocks for aave user events where account data is updated
    // if event found with matching user then update data for that user
    //
    // also scan for mempool events for oracle updates, if found then:
    //1. find all users that are borrowing or using token as colladeral
    // 2. re-calculate their health factor ???
}

// HOW TO GET PRICE FROM UNISWAP V3

// let token_1 = generate_token(
//     1,
//     18,
//     "WETH",
//     "Wrapped Ether",
//     "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
// )
// .await?;

// let weth_contract = ERC20::new(*WETH_ADDRESS, client.clone());
// let aave_oracle = AAVE_ORACLE::new(*AAVE_ORACLE_ADDRESS, client.clone());
// let aave_v3_pool = AAVE_V3_POOL::new(*AAVE_V3_POOL_ADDRESS, client.clone());

// let weth_USD_price = aave_oracle.get_asset_price(*WETH_ADDRESS).call().await?;

// let weth_token = TOKEN_DATA
//     .get("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2")
//     .unwrap_or_else(|| panic!("Token not found"));
// println!("name from Token => {} ", weth_token.name);
// println!(
//     "name from Contract => {} ",
//     weth_contract.name().call().await?
// );
// println!("WETH price in USD from aave oracle {} ", weth_USD_price);

// let token_2 = generate_token(
//     1,
//     16,
//     "USDC",
//     "USD Coin",
//     "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
// )
// .await?;
// // println!("hashmap {:?}", TOKEN_DATA);

// let pool = get_pool(
//     1,
//     FACTORY_ADDRESS,
//     token_1.meta.address,
//     token_2.meta.address,
//     FeeAmount::MEDIUM,
//     client.clone(),
//     None,
// )
// .await?;

// let price = pool.token0_price();

// let significant_digits: u8 = 6;
// let rounding_strategy = Rounding::RoundHalfUp; // Choose based on your requirement
// let scalingFactor = BigDecimal::from_u64(10_u64.pow(18)).unwrap();

// let formatted_price = price
//     .to_significant(significant_digits, rounding_strategy)
//     .expect("Failed to format the price");
// println!("WETH price: {:?}", formatted_price);
