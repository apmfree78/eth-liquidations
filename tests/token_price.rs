use bigdecimal::{BigDecimal, FromPrimitive, Zero};
use eth_liquadation::abi::aave_oracle::AAVE_ORACLE;
use eth_liquadation::abi::erc20::ERC20;
use eth_liquadation::crypto_data::{
    generate_token, u256_to_big_decimal, AaveToken, Convert, Erc20Token, AAVE_ORACLE_ADDRESS,
    AAVE_V3_POOL_ADDRESS, TOKEN_DATA, WETH_ADDRESS,
};
use ethers::prelude::*;
use std::sync::Arc;

#[tokio::test]
async fn test_token_price_uniswap_versus_oracle() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the Ethereum client connection and wallet
    const WS_URL: &str = "ws://localhost:8546";
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    let aave_oracle = AAVE_ORACLE::new(*AAVE_ORACLE_ADDRESS, client.clone());

    println!(" number of test to run {} ", 3 * TOKEN_DATA.len());
    for token in TOKEN_DATA.values() {
        if token.name == "Maker" {
            continue;
        }

        let address: Address = token.address.parse()?;
        // TODO - fix scaling
        let oracle_decimal_factor = BigDecimal::from_u64(10_u64.pow(8)).unwrap();

        let token_price_uniswap = token.get_token_price_in_("USDC", &client).await?;
        let token_price_oracle = aave_oracle.get_asset_price(address).call().await?;
        let token_price_oracle = u256_to_big_decimal(&token_price_oracle);

        println!("checking {} ", token.name);

        println!("uniswap price {}", token_price_uniswap);
        println!(
            "aave oracle price {}",
            token_price_oracle / oracle_decimal_factor
        );
    }
    Ok(())
}
