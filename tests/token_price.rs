use bigdecimal::BigDecimal;
use eth_liquadation::data::erc20::{Convert, UNIQUE_TOKEN_DATA};
use ethers::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

#[tokio::test]
async fn test_token_price_uniswap_versus_oracle() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the Ethereum client connection and wallet
    const WS_URL: &str = "ws://localhost:8546";
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);

    println!(" number of test to run {} ", 3 * UNIQUE_TOKEN_DATA.len());
    for token in UNIQUE_TOKEN_DATA.values() {
        // uniswap price
        println!("get token uniswap price for {}", token.symbol);
        let token_price_uniswap = token.get_token_price_in_("USDC", &client).await?;

        // price from chainlink oracle price
        println!("get token oracle price for {}", token.symbol);
        let token_price_oracle = token.get_token_oracle_price(&client).await?;

        let lower_bound = BigDecimal::from_str("0.96")? * &token_price_oracle;
        let upper_bound = BigDecimal::from_str("1.04")? * &token_price_oracle;

        if token_price_uniswap < lower_bound || token_price_uniswap > upper_bound {
            println!("checking {} ", token.name);
            println!("symbol {} ", token.symbol);

            println!("uniswap price {}", token_price_uniswap);
            println!("aave oracle price {}", token_price_oracle);
        }

        assert!(
            token_price_uniswap > lower_bound && token_price_uniswap < upper_bound,
            "price out of bound"
        );
    }
    Ok(())
}
