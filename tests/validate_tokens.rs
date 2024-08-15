use abi::Address;
use eth_liquadation::abi::aave_v3_data_provider::{self, AAVE_V3_DATA_PROVIDER};
use eth_liquadation::abi::erc20::ERC20;
use eth_liquadation::data::address::CONTRACT;
use eth_liquadation::data::erc20::{TOKEN_DATA, UNIQUE_TOKEN_DATA};
use ethers::prelude::*;
use std::sync::Arc;

#[tokio::test]
async fn test_token_data_matches_token_contract() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the Ethereum client connection and wallet
    const WS_URL: &str = "ws://localhost:8546";
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);

    println!(" number of test to run {} ", 4 * TOKEN_DATA.len());
    for token in UNIQUE_TOKEN_DATA.values() {
        if token.name == "Maker" {
            continue;
        }

        let address: Address = token.address.parse()?;
        // let address_str = address.to_string().to_lowercase();
        let address_str = format!("{:?}", address);

        println!("checking {} ", token.name);
        let token_contract = ERC20::new(address, client.clone());
        let data_provider_address: Address =
            CONTRACT.get_address().aave_v3_data_provider.parse()?;
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

        println!("normalized address {}", address_str);
        println!("normalized address length {}", address_str.len());
        println!("original token address {:?}", token.address);
        // let _token = TOKEN_DATA
        //     .get(&token.address.to_string().to_lowercase())
        //     .unwrap();
        let _token = TOKEN_DATA.get(&address_str).unwrap();

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
    }

    Ok(())
}
