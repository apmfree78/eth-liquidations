use abi::Address;
use eth_liquadation::abi::aave_v3_data_provider::AAVE_V3_DATA_PROVIDER;
use eth_liquadation::abi::chainlink_registry::CHAINLINK_REGISTRY;
use eth_liquadation::abi::erc20::ERC20;
use eth_liquadation::data::address::{CONTRACT, ETH, USD};
use eth_liquadation::data::token_data_hash::{
    get_token_data, get_unique_token_data, save_erc20_tokens_from_static_data,
};
use ethers::prelude::*;
use std::sync::Arc;

// pub static SKIP_AGGREGATOR_CHECK: &[&str] = &["KNC"];

#[tokio::test]
async fn test_token_data_matches_token_contract(
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Set up the Ethereum client connection and wallet
    const WS_URL: &str = "ws://localhost:8546";
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);

    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;

    // grab tokens from token state
    let token_data = get_token_data().await?;
    let unique_token_data = get_unique_token_data().await?;

    println!(" number of test to run {} ", 4 * token_data.len());
    for token in unique_token_data.values() {
        if token.name == "Maker" {
            continue;
        }

        let address: Address = token.address.parse()?;
        let data_provider_address: Address =
            CONTRACT.get_address().aave_v3_data_provider.parse()?;

        let address_str = format!("{:?}", address);

        println!("checking {} ", token.name);
        let token_contract = ERC20::new(address, client.clone());
        let aave_v3_data_provider =
            AAVE_V3_DATA_PROVIDER::new(data_provider_address, client.clone());

        // Retrieve the token name from the contract
        let contract_name = token_contract.name().call().await?;
        let contract_symbol = token_contract.symbol().call().await?;
        let contract_decimals = token_contract.decimals().call().await?;

        let (_, _, liquidation_threshold, liquidation_bonus, _, _, _, _, _, _) =
            aave_v3_data_provider
                .get_reserve_configuration_data(address)
                .call()
                .await?;

        // let aggregator = match get_aggregator(address, &client).await? {
        //     Some(value) => value,
        //     None => Address::zero(),
        // };

        println!("normalized address {}", address_str);
        println!("normalized address length {}", address_str.len());
        println!("original token address {:?}", token.address);
        // let _token = token_data
        //     .get(&token.address.to_string().to_lowercase())
        //     .unwrap();
        let _token = token_data.get(&address_str).unwrap();

        assert_eq!(
            contract_name, _token.name,
            "Contract name does not match expected token name"
        );

        assert_eq!(
            contract_symbol, token.symbol,
            "Contract name does not match expected token symbol"
        );

        assert_eq!(
            contract_decimals, token.decimals,
            "Contract decimals does not match expected token decimals"
        );

        assert_eq!(
            liquidation_bonus,
            token.liquidation_bonus.into(),
            "Contract liquidation bonus does not match expected token liquidation bonus"
        );

        assert_eq!(
            liquidation_threshold,
            token.liquidation_threshold.into(),
            "Contract liquidation threshold does not match expected token liquidation threshold"
        );

        assert_eq!(
            liquidation_threshold,
            token.liquidation_threshold.into(),
            "Contract liquidation threshold does not match expected token liquidation threshold"
        );

        // let token_aggregator: Address = token.chainlink_aggregator.parse()?;
        //
        // if !SKIP_AGGREGATOR_CHECK.contains(&token.symbol) {
        //     assert_eq!(
        //         aggregator, token_aggregator,
        //         "Chainlink aggregator does not match expected token aggregator"
        //     );
        // }
    }

    Ok(())
}

async fn get_aggregator(
    token: Address,
    client: &Arc<Provider<Ws>>,
) -> Result<Option<Address>, Box<dyn std::error::Error + Send + Sync>> {
    let chainlink_registry_address: Address =
        CONTRACT.get_address().chain_link_feed_registry.parse()?;
    let usd_address: Address = USD.parse()?;
    let eth_address: Address = ETH.parse()?;

    let chainlink_registry = CHAINLINK_REGISTRY::new(chainlink_registry_address, client.clone());

    // First attempt to get the feed
    println!("first atteempt to get feed");
    let first_try = chainlink_registry.get_feed(token, usd_address).call().await;

    match first_try {
        Ok(aggregator) => Ok(Some(aggregator)),
        Err(_) => {
            // If the first attempt fails, make a second attempt
            println!("second atteempt to get feed");
            let second_try = chainlink_registry.get_feed(token, eth_address).call().await;
            match second_try {
                Ok(aggregator) => Ok(Some(aggregator)),
                Err(_) => {
                    println!("both atteempt failed, retuning none");
                    Ok(None)
                } // Return None if the second attempt also fails
            }
        }
    }
}
