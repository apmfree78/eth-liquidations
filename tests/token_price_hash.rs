use anyhow::Result;
use bigdecimal::BigDecimal;
use eth_liquadation::data::erc20::Convert;
use eth_liquadation::data::token_data_hash::{
    get_token_data, get_unique_token_data, save_erc20_tokens_from_static_data,
};
use eth_liquadation::data::token_price_hash::{
    generate_token_price_hash, get_saved_token_price, print_saved_token_prices,
    set_saved_token_price,
};
use ethers::prelude::*;
use std::sync::Arc;

#[tokio::test]
async fn test_setting_price_with_token_price_hash() -> Result<()> {
    // Set up the Ethereum client connection and wallet
    const WS_URL: &str = "ws://localhost:8546";
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;
    let token_data = get_token_data().await?;

    generate_token_price_hash(&client).await?;

    // get address for WETH
    let weth_token = token_data.get("WETH").unwrap();
    let weth_price = weth_token.get_token_oracle_price(&client).await?;
    let new_price = 2.0 * &weth_price;

    set_saved_token_price(weth_token.address.as_str(), new_price.clone()).await?;

    let updated_price = get_saved_token_price(weth_token.address.to_lowercase()).await?;

    assert_eq!(new_price, updated_price);

    set_saved_token_price(weth_token.address.as_str(), weth_price.clone()).await?;

    Ok(())
}

#[tokio::test]
async fn test_token_price_token_hash_versus_oracle() -> Result<()> {
    // Set up the Ethereum client connection and wallet
    const WS_URL: &str = "ws://localhost:8546";
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    let unique_token_data = get_unique_token_data().await?;

    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;

    generate_token_price_hash(&client).await?;

    print_saved_token_prices().await?;
    println!(" number of test to run {} ", unique_token_data.len());
    for token in unique_token_data.values() {
        // if token.name == "Maker" {
        //     continue;
        // }

        // uniswap price
        let token_price_saved = token.get_saved_price_from_token_price_hash().await?;

        // price from chainlink oracle price
        let token_price_oracle = token.get_token_oracle_price(&client).await?;

        assert_eq!(token_price_saved, token_price_oracle);
    }
    Ok(())
}

// #[tokio::test]
// async fn test_token_hash_price_update() -> Result<()> {
//     // Set up the Ethereum client connection and wallet
//     const WS_URL: &str = "ws://localhost:8546";
//     let provider = Provider::<Ws>::connect(WS_URL).await?;
//     let client = Arc::new(provider);
//     let unique_token_data = get_unique_token_data().await?;
//
//     // populate token state
//     save_erc20_tokens_from_static_data(&client).await?;
//
//     generate_token_price_hash(&client).await?;
//
//     print_saved_token_prices().await?;
//
//     println!(" number of test to run {} ", unique_token_data.len());
//     for token in unique_token_data.values() {
//         // below will get token price from uniswap place in saved token hash
//         update_token_price_for_(token, &client).await?;
//
//         // uniswap price
//         let token_price_saved = token.get_saved_price_from_token_price_hash().await?;
//
//         // price from chainlink oracle price
//         let token_price_uniswap = token.get_token_price_in_("USDC", &client).await?;
//
//         println!("token price check for {}", token.symbol);
//
//         assert_eq!(token_price_saved, token_price_uniswap);
//     }
//     Ok(())
// }
