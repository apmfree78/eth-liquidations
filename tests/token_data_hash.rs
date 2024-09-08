use eth_liquadation::data::token_data_hash::{
    get_and_save_erc20_by_token_address, get_tokens_priced_in_eth,
    save_erc20_tokens_from_static_data,
};
use eth_liquadation::data::{erc20::Erc20Token, token_data_hash::get_token_data};
use ethers::providers::{Provider, Ws};
use std::sync::Arc;

#[tokio::test]
async fn get_and_save_weth_token_is_valid() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    const WS_URL: &str = "ws://localhost:8546";
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    // populate token state

    // TEST FIRST TOKEN
    let control_token = Erc20Token {
        name: "Wrapped Ether".to_string(),
        symbol: "WETH".to_string(),
        decimals: 18,
        address: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".to_string(),
        liquidation_bonus: 10500,
        liquidation_threshold: 8300,
        chain_link_price_feed: "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419".to_string(),
        chainlink_aggregator: "0xE62B71cf983019BFf55bC83B48601ce8419650CC".to_string(),
    };

    let test_token = get_and_save_erc20_by_token_address(&control_token.address, &client).await?;

    assert_eq!(
        control_token.name, test_token.name,
        "Contract name does not match expected token name"
    );

    assert_eq!(
        control_token.symbol, test_token.symbol,
        "Contract name does not match expected token symbol"
    );

    assert_eq!(
        control_token.decimals, test_token.decimals,
        "Contract decimals does not match expected token decimals"
    );

    assert_eq!(
        control_token.liquidation_bonus, test_token.liquidation_bonus,
        "Contract liquidation bonus does not match expected token liquidation bonus"
    );

    assert_eq!(
        control_token.liquidation_threshold, test_token.liquidation_threshold,
        "Contract liquidation threshold does not match expected token liquidation threshold"
    );

    assert_eq!(
        control_token.chain_link_price_feed.to_lowercase(),
        test_token.chain_link_price_feed.to_lowercase(),
        "Contract chainlink price feed does not match expected token chainlink price feed"
    );

    assert_eq!(
        control_token.chainlink_aggregator.to_lowercase(),
        test_token.chainlink_aggregator.to_lowercase(),
        "Contract chainlink aggregator does not match expected token aggregator"
    );

    // TODO - now test that token that is retreived from global state matches too
    let token_data = get_token_data().await?;
    let test_token = token_data.get(&control_token.address).unwrap();

    assert_eq!(
        control_token.name, test_token.name,
        "Contract name does not match expected token name"
    );

    assert_eq!(
        control_token.symbol, test_token.symbol,
        "Contract name does not match expected token symbol"
    );

    assert_eq!(
        control_token.decimals, test_token.decimals,
        "Contract decimals does not match expected token decimals"
    );

    assert_eq!(
        control_token.liquidation_bonus, test_token.liquidation_bonus,
        "Contract liquidation bonus does not match expected token liquidation bonus"
    );

    assert_eq!(
        control_token.liquidation_threshold, test_token.liquidation_threshold,
        "Contract liquidation threshold does not match expected token liquidation threshold"
    );

    assert_eq!(
        control_token.chain_link_price_feed.to_lowercase(),
        test_token.chain_link_price_feed.to_lowercase(),
        "Contract chainlink price feed does not match expected token chainlink price feed"
    );

    assert_eq!(
        control_token.chainlink_aggregator.to_lowercase(),
        test_token.chainlink_aggregator.to_lowercase(),
        "Contract chainlink aggregator does not match expected token aggregator"
    );

    Ok(())
}

#[tokio::test]
async fn get_and_save_link_token_is_valid() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    const WS_URL: &str = "ws://localhost:8546";
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    // populate token state

    // TEST FIRST TOKEN
    let control_token = Erc20Token {
        name: "ChainLink Token".to_string(),
        symbol: "LINK".to_string(),
        decimals: 18,
        address: "0x514910771AF9Ca656af840dff83E8264EcF986CA".to_string(),
        liquidation_bonus: 10700,
        liquidation_threshold: 6800,
        chain_link_price_feed: "0x2c1d072e956AFFC0D435Cb7AC38EF18d24d9127c".to_string(),
        chainlink_aggregator: "0x20807cf61ad17c31837776fa39847a2fa1839e81".to_string(),
    };

    let test_token = get_and_save_erc20_by_token_address(&control_token.address, &client).await?;

    assert_eq!(
        control_token.name, test_token.name,
        "Contract name does not match expected token name"
    );

    assert_eq!(
        control_token.symbol, test_token.symbol,
        "Contract name does not match expected token symbol"
    );

    assert_eq!(
        control_token.decimals, test_token.decimals,
        "Contract decimals does not match expected token decimals"
    );

    assert_eq!(
        control_token.liquidation_bonus, test_token.liquidation_bonus,
        "Contract liquidation bonus does not match expected token liquidation bonus"
    );

    assert_eq!(
        control_token.liquidation_threshold, test_token.liquidation_threshold,
        "Contract liquidation threshold does not match expected token liquidation threshold"
    );

    assert_eq!(
        control_token.chain_link_price_feed.to_lowercase(),
        test_token.chain_link_price_feed.to_lowercase(),
        "Contract chainlink price feed does not match expected token chainlink price feed"
    );

    assert_eq!(
        control_token.chainlink_aggregator.to_lowercase(),
        test_token.chainlink_aggregator.to_lowercase(),
        "Contract chainlink aggregator does not match expected token aggregator"
    );

    // TODO - now test that token that is retreived from global state matches too
    let token_data = get_token_data().await?;
    let test_token = token_data.get(&control_token.address).unwrap();

    assert_eq!(
        control_token.name, test_token.name,
        "Contract name does not match expected token name"
    );

    assert_eq!(
        control_token.symbol, test_token.symbol,
        "Contract name does not match expected token symbol"
    );

    assert_eq!(
        control_token.decimals, test_token.decimals,
        "Contract decimals does not match expected token decimals"
    );

    assert_eq!(
        control_token.liquidation_bonus, test_token.liquidation_bonus,
        "Contract liquidation bonus does not match expected token liquidation bonus"
    );

    assert_eq!(
        control_token.liquidation_threshold, test_token.liquidation_threshold,
        "Contract liquidation threshold does not match expected token liquidation threshold"
    );

    assert_eq!(
        control_token.chain_link_price_feed.to_lowercase(),
        test_token.chain_link_price_feed.to_lowercase(),
        "Contract chainlink price feed does not match expected token chainlink price feed"
    );

    assert_eq!(
        control_token.chainlink_aggregator.to_lowercase(),
        test_token.chainlink_aggregator.to_lowercase(),
        "Contract chainlink aggregator does not match expected token aggregator"
    );

    Ok(())
}

#[tokio::test]
async fn test_that_connect_eth_tokens_are_valid(
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Set up the Ethereum client connection and wallet
    const WS_URL: &str = "ws://localhost:8546";
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;
    let token_map = get_tokens_priced_in_eth().await?;

    let token_symbols_priced_in_eth =
        vec!["WETH", "wstETH", "rETH", "cbETH", "LDO", "weETH", "ETHx"];

    // for token in token_map.keys() {
    //     println!("contains token {}", token);
    // }

    for token in token_symbols_priced_in_eth {
        println!("checking if contains token {}", token);
        assert!(token_map.contains_key(token));
    }

    Ok(())
}
