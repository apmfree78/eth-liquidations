mod crypto_data;
use crypto_data::{generate_token, Erc20Token, TOKEN_DATA};
use ethers::providers::{Provider, Ws};
use ethers::types::{H160, U256};
use std::sync::Arc;
use uniswap_sdk_core::{
    constants::Rounding,
    entities::token::{Token, TokenMeta},
    prelude::*,
};
use uniswap_v3_sdk::{
    constants::{FeeAmount, FACTORY_ADDRESS},
    entities::pool::{self},
    extensions::{get_pool, get_pool_contract},
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

    let token_2 = generate_token(
        1,
        16,
        "USDC",
        "USD Coin",
        "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    )
    .await?;
    println!("hashmap {:?}", TOKEN_DATA);

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
    println!("WEHT price: {:?}", formatted_price);

    Ok(())
}
