// create function to get price feed data  using aave_oracle getSourceOfAsset method

use anyhow::Result;
use eth_liquadation::data::chainlink_data::get_chainlink_price_feeds_by_chain;
use eth_liquadation::data::token_data_hash::{get_token_data, save_erc20_tokens_from_static_data};
use eth_liquadation::utils::type_conversion::address_to_string;
use ethers::abi::Address;
use ethers::contract::abigen;
use ethers::providers::{Provider, Ws};
use std::sync::Arc;

// #[tokio::test]
// async fn verify_price_oarcles_are_valid() -> Result<()> {
//     const WS_URL: &str = "ws://localhost:8546";
//     let provider = Provider::<Ws>::connect(WS_URL).await?;
//     let client = Arc::new(provider);
//     let aave_oracle_address: Address = "0x54586bE62E3c3580375aE3723C145253060Ca0C2"
//         .parse()
//         .expect("Invalid address");
//
//     let aave_oracle = AAVE_ORACLE::new(aave_oracle_address, client.clone());
//
//     for token in TOKEN_DATA.deref().values() {
//         let token_address: Address = Address::from_str(token.address)?;
//         let token_price_feed = aave_oracle
//             .get_source_of_asset(token_address)
//             .call()
//             .await?;
//
//         let token_price_feed = address_to_string(token_price_feed);
//         println!("**********************************************************************");
//         println!("for token => {}", token.name);
//         println!(" symbol => {}", token.symbol);
//         println!("internal token price feed {}", token.chain_link_price_feed);
//         println!("aave contract token price feed {}", token_price_feed);
//         println!("**********************************************************************");
//         assert_eq!(
//             token.chain_link_price_feed.to_lowercase(),
//             token_price_feed.to_lowercase()
//         );
//     }
//
//     Ok(())
// }

#[tokio::test]
async fn check_chainlink_data_is_valid() -> Result<()> {
    const WS_URL: &str = "ws://localhost:8546";
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;

    let price_feeds = get_chainlink_price_feeds_by_chain();

    abigen!(
        DESCRIPTION,
        r#"[function description() external view returns (string)]"#
    );

    for price_feed in price_feeds {
        if price_feed.base_currency.is_empty() {
            continue;
        }

        let price_feed_address: Address = price_feed.address.parse()?;
        let price_feed_contract = DESCRIPTION::new(price_feed_address, client.clone());

        let description = price_feed_contract.description().call().await?;
        let expected_description =
            format!("{} / {}", price_feed.token_symbol, price_feed.base_currency);

        println!("**********************************************************************");
        println!("for token {}", price_feed.token_symbol);
        println!("for base currency {}", price_feed.base_currency);
        println!("description ==> {}", description);
        println!("**********************************************************************");
        assert_eq!(description, expected_description);
    }

    Ok(())
}
#[tokio::test]
async fn find_price_aggregators_are_valid() -> Result<()> {
    const WS_URL: &str = "ws://localhost:8546";
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;
    let token_data = get_token_data().await?;

    abigen!(
        AGGREGATOR,
        r#"[function aggregator() external view returns (address)]"#
    );

    for token in token_data.values() {
        if token.chain_link_price_feed.is_empty() {
            println!("no price feed found for {}", token.symbol);
            continue;
        }
        let price_feed_address: Address = token.chain_link_price_feed.parse().unwrap();

        let contract = AGGREGATOR::new(price_feed_address, client.clone());

        if let Ok(aggregator_address) = contract.aggregator().call().await {
            let aggregator_address = address_to_string(aggregator_address);
            println!("**********************************************************************");
            println!("for token => {}", token.name);
            println!(" symbol => {}", token.symbol);
            println!(" token aggregator {}", token.chainlink_aggregator);
            println!("token aggregator address {}", aggregator_address);
            println!("**********************************************************************");
            assert_eq!(
                token.chainlink_aggregator.to_lowercase(),
                aggregator_address.to_lowercase()
            );
        } else {
            println!("no valid aggregator found for...");
            println!("for token => {}", token.name);
            println!(" symbol => {}", token.symbol);
            println!(
                " token chain_link_price_feed {}",
                token.chain_link_price_feed
            );
        }
    }

    Ok(())
}
