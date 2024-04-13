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

    // Define the address and create the contract instance
    let address: Address = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse()?;
    let weth_contract = ERC20::new(address, client.clone());

    // Retrieve the token name from the contract
    let contract_name = weth_contract.name().call().await?;

    // Retrieve the expected name from the local data
    let expected_name = TOKEN_DATA
        .get("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2")
        .expect("Token not found")
        .name;

    // Assert that the names match
    assert_eq!(
        contract_name, expected_name,
        "Contract name does not match expected token name"
    );

    Ok(())
}
