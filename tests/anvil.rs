use anyhow::Result;
use ethers::contract::abigen;
use ethers::types::{Address, Eip1559TransactionRequest, NameOrAddress, I256, U256};

use eth_liquadation::backrun::anvil_simulator::AnvilSimulator; // from ethers-rs
                                                               // Use the `abigen!` macro to generate type-safe bindings for the Chainlink price feed contract
abigen!(
    AggregatorV3Interface,
    r#"[
        function latestAnswer() external view returns (int256)
    ]"#,
);

#[tokio::test]
async fn test_simulate_anvil_transaction_test() -> anyhow::Result<()> {
    // Provide the RPC URL, e.g., from an environment variable or hardcoded for testing
    let rpc_url = std::env::var("RPC_URL").unwrap_or_else(|_| "ws://localhost:8546".to_string());

    // Create an instance of AnvilSimulator
    let anvil_simulator = AnvilSimulator::new(&rpc_url).await?;

    // Chainlink BTC/USD price feed contract address
    let price_feed_address: Address = "0xF4030086522a5bEEa4988F8cA5B36dbC97BeE88c".parse()?;

    // Create a contract instance to generate the calldata
    let contract = AggregatorV3Interface::new(price_feed_address, anvil_simulator.client.clone());

    // Get the calldata for 'latestAnswer' function
    let calldata = contract
        .method::<_, I256>("latestAnswer", ())?
        .calldata()
        .unwrap();

    // Create the Eip1559TransactionRequest
    let backrun_tx = Eip1559TransactionRequest {
        from: None, // 'from' will be set inside the simulate_anvil_transaction_test method
        to: Some(NameOrAddress::Address(price_feed_address)),
        data: Some(calldata),
        max_priority_fee_per_gas: Some(U256::from(0)),
        max_fee_per_gas: Some(U256::from(1_500_000_000u64)),
        value: None,
        chain_id: Some(1u64.into()),
        ..Default::default()
    };

    // Run the simulation
    let result = anvil_simulator
        .simulate_anvil_transaction_test(&backrun_tx)
        .await?;

    // Print the result
    println!("Simulation Result: {:?}", result);

    // Optionally, assert expected outcomes
    // assert!(result.is_some());

    Ok(())
}
