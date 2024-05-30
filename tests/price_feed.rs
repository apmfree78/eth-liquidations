// create function to get price feed data  using aave_oracle getSourceOfAsset method

use eth_liquadation::abi::aave_oracle::AAVE_ORACLE;
use eth_liquadation::data::erc20::{address_to_string, TOKEN_DATA};
use ethers::{
    providers::{Provider, Ws},
    types::Address,
};
use std::str::FromStr;
use std::{ops::Deref, sync::Arc};

#[tokio::test]
async fn verify_price_oarcles_are_valid() -> Result<(), Box<dyn std::error::Error>> {
    const WS_URL: &str = "ws://localhost:8546";
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    let aave_oracle_address: Address = "0x54586bE62E3c3580375aE3723C145253060Ca0C2"
        .parse()
        .expect("Invalid address");

    let aave_oracle = AAVE_ORACLE::new(aave_oracle_address, client.clone());

    for token in TOKEN_DATA.deref().values() {
        let token_address: Address = Address::from_str(token.address)?;
        let token_price_feed = aave_oracle
            .get_source_of_asset(token_address)
            .call()
            .await?;

        let token_price_feed = address_to_string(token_price_feed);
        println!("**********************************************************************");
        println!("for token => {}", token.name);
        println!(" symbol => {}", token.symbol);
        println!("internal token price feed {}", token.chain_link_price_feed);
        println!("aave contract token price feed {}", token_price_feed);
        println!("**********************************************************************");
        assert_eq!(
            token.chain_link_price_feed.to_lowercase(),
            token_price_feed.to_lowercase()
        );
    }

    Ok(())
}
