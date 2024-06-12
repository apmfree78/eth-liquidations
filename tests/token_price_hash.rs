use bigdecimal::BigDecimal;
use eth_liquadation::data::erc20::{Convert, TOKEN_DATA, UNIQUE_TOKEN_DATA};
use eth_liquadation::data::token_price_hash::{
    generate_token_price_hash, get_saved_token_price, print_saved_token_prices,
    set_saved_token_price,
};
use ethers::prelude::*;
use std::sync::Arc;

#[tokio::test]
async fn test_setting_price_with_token_price_hash() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the Ethereum client connection and wallet
    const WS_URL: &str = "ws://localhost:8546";
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);

    generate_token_price_hash(&client).await?;

    // get address for WETH
    let weth_token = TOKEN_DATA.get("WETH").unwrap();
    let weth_price = weth_token.get_token_oracle_price(&client).await?;
    let new_price = BigDecimal::from(2) * &weth_price;

    set_saved_token_price(weth_token.address, new_price.clone()).await?;

    let updated_price = get_saved_token_price(weth_token.address.to_lowercase()).await?;

    assert_eq!(new_price, updated_price);

    set_saved_token_price(weth_token.address, weth_price.clone()).await?;

    Ok(())
}

#[tokio::test]
async fn test_token_price_token_hash_versus_oracle() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the Ethereum client connection and wallet
    const WS_URL: &str = "ws://localhost:8546";
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);

    generate_token_price_hash(&client).await?;

    print_saved_token_prices().await?;
    println!(" number of test to run {} ", UNIQUE_TOKEN_DATA.len());
    for token in UNIQUE_TOKEN_DATA.values() {
        if token.name == "Maker" {
            continue;
        }

        // uniswap price
        let token_price_saved = token.get_saved_price_from_token_price_hash().await?;

        // price from chainlink oracle price
        let token_price_oracle = token.get_token_oracle_price(&client).await?;

        assert_eq!(token_price_saved, token_price_oracle);
    }
    Ok(())
}
