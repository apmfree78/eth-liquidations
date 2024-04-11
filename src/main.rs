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

    // let pool_address = get_pool_address(&token_1, &token_2).await?;
    // println!("Calculated Pool Address: {:?}", pool_address);

    // let pool_contract = get_pool_contract(
    //     FACTORY_ADDRESS,
    //     token_1.meta.address,
    //     token_2.meta.address,
    //     FeeAmount::MEDIUM,
    //     client.clone(),
    // );

    // println!("Pool Contract: {:?}", pool_contract);

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

async fn get_pool_address(
    token_a: &Token,
    token_b: &Token,
) -> Result<Address, Box<dyn std::error::Error>> {
    // Fee Tier
    let fee = FeeAmount::MEDIUM; // Assuming this matches the Uniswap SDK's representation

    // Calculate Pool Address
    let pool_address = pool::get_address(
        token_a, token_b, fee, None, // init_code_hash_manual_override
        None, // factory_address_override
    );

    Ok(pool_address)
}

async fn generate_token(
    chain_id: u64,
    decimals: u8,
    symbol: &str,
    name: &str,
    address: &str,
) -> Result<Token, Box<dyn std::error::Error>> {
    return Ok(Token {
        chain_id,
        decimals,
        symbol: Some(symbol.into()),
        name: Some(name.into()),
        meta: TokenMeta {
            address: address.parse()?,
            buy_fee_bps: None,
            sell_fee_bps: None,
        },
    });
}
