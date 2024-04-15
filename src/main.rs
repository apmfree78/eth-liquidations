use eth_liquadation::abi::aave_oracle::AAVE_ORACLE;
use eth_liquadation::abi::aave_v3_pool::AAVE_V3_POOL;
use eth_liquadation::abi::erc20::ERC20;
use eth_liquadation::crypto_data::{generate_token, TOKEN_DATA};
use eth_liquadation::get_aave_users::get_aave_v3_users;
use ethers::abi::Address;
use ethers::core::k256::U256;
use ethers::providers::{Provider, Ws};
use std::sync::Arc;
use uniswap_sdk_core::{constants::Rounding, prelude::*};
use uniswap_v3_sdk::{
    constants::{FeeAmount, FACTORY_ADDRESS},
    extensions::get_pool,
};

const WS_URL: &str = "ws://localhost:8546";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);

    let token_1 = generate_token(
        1,
        18,
        "WETH",
        "Wrapped Ether",
        "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
    )
    .await?;

    let weth_address: Address = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse()?;
    let aave_oracle_address: Address = "0x54586bE62E3c3580375aE3723C145253060Ca0C2".parse()?;
    let aave_v3_pool_address: Address = "0x87870Bca3F3fD6335C3F4ce8392D69350B4fA4E2".parse()?;

    let weth_contract = ERC20::new(weth_address, client.clone());
    let aave_oracle = AAVE_ORACLE::new(aave_oracle_address, client.clone());
    let aave_v3_pool = AAVE_V3_POOL::new(aave_v3_pool_address, client.clone());

    let weth_USD_price = aave_oracle.get_asset_price(weth_address).call().await?;

    let weth_token = TOKEN_DATA
        .get("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2")
        .unwrap_or_else(|| panic!("Token not found"));
    println!("name from Token => {} ", weth_token.name);
    println!(
        "name from Contract => {} ",
        weth_contract.name().call().await?
    );
    println!("WETH price in USD from aave oracle {} ", weth_USD_price);

    let token_2 = generate_token(
        1,
        16,
        "USDC",
        "USD Coin",
        "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    )
    .await?;
    // println!("hashmap {:?}", TOKEN_DATA);

    let pool = get_pool(
        1,
        FACTORY_ADDRESS,
        token_1.meta.address,
        token_2.meta.address,
        FeeAmount::MEDIUM,
        client.clone(),
        None,
    )
    .await?;

    let price = pool.token0_price();

    let significant_digits: u8 = 6;
    let rounding_strategy = Rounding::RoundHalfUp; // Choose based on your requirement

    let formatted_price = price
        .to_significant(significant_digits, rounding_strategy)
        .expect("Failed to format the price");
    println!("WETH price: {:?}", formatted_price);

    let aave_users = get_aave_v3_users().await?;
    for user in &aave_users {
        println!("User details: {:#?}", user); // Assuming AaveUser implements Debug
        let user_id: Address = user.id.parse()?;
        let user_account = aave_v3_pool.get_user_account_data(user_id).call().await?;
        println!("user account details {:#?} ", user_account);
    }
    Ok(())
}
