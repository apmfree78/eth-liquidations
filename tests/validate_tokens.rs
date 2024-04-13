use eth_liquadation::abi::erc20::ERC20;
use eth_liquadation::crypto_data::{generate_token, Erc20Token, TOKEN_DATA};
use ethers::prelude::*;
use std::sync::Arc;

#[tokio::test]
async fn test_token_data_matches_token_contract() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the Ethereum client connection and wallet
    const WS_URL: &str = "ws://localhost:8546";
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);

    // // Define the address and create the contract instance
    // let address: Address = "0x9f8F72aA9304c8B593d555F12eF6589cC3A579A2".parse()?;
    // println!("starting contract ...");
    // let weth_contract = ERC20::new(address, client.clone());

    // // Retrieve the token name from the contract
    // println!("getting token name ...");
    // let contract_name = weth_contract.name().call().await?;

    // // Retrieve the expected name from the local data
    // println!("getting token data from TOKEN_DATA ...");
    // let expected_name = TOKEN_DATA
    //     .get("0x9f8F72aA9304c8B593d555F12eF6589cC3A579A2")
    //     .expect("Token not found")
    //     .name;

    // // Assert that the names match
    // assert_eq!(
    //     contract_name, expected_name,
    //     "Contract name does not match expected token name"
    // );

    println!(" number of test to run {} ", 3 * TOKEN_DATA.len());
    for token in TOKEN_DATA.values() {
        if token.name == "Maker" {
            continue;
        }

        let address: Address = token.address.parse()?;

        println!("checking {} ", token.name);
        let token_contract = ERC20::new(address, client.clone());

        // Retrieve the token name from the contract
        let contract_name = token_contract.name().call().await?;
        let contract_symbol = token_contract.symbol().call().await?;
        let contract_decimals = token_contract.decimals().call().await?;

        assert_eq!(
            contract_name, token.name,
            "Contract name does not match expected token name"
        );

        assert_eq!(
            contract_symbol, token.symbol,
            "Contract name does not match expected token name"
        );

        assert_eq!(
            contract_decimals, token.decimals,
            "Contract decimals does not match expected token name"
        );
    }

    Ok(())
}
