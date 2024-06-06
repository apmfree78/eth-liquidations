use crate::data::erc20::{Convert, TOKEN_DATA};
use crate::data::token_price_hash::set_saved_token_price;
use crate::utils::type_conversion::address_to_string;
use ethers::{prelude::*, utils::keccak256};
use eyre::Result;
use std::sync::Arc;

pub async fn detect_price_update(
    pending_tx: TxHash,
    client: &Arc<Provider<Ws>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let transmit_signature = "transmit(bytes,bytes32[],bytes32[],bytes32)";

    abigen!(
        AGGREGATOR,
        r#"[function description() external view returns (string)]"#
    );

    // Compute the Keccak-256 hashes of the event signatures
    // let transmit_hash = H256::from(keccak256(transmit_signature.as_bytes()));
    let transmit_hash = keccak256(transmit_signature.as_bytes())[0..4].to_vec();

    // Print out each new transaction hash
    // println!("New pending transaction: {:?}", tx);
    if let Ok(Some(tx)) = client.get_transaction(pending_tx).await {
        // println!("Transaction Details: {:?}", tx);

        // If the transaction involves a contract interaction, `to` will be Some(address)
        if let Some(to) = tx.to {
            // println!("Transaction to address: {:?}", to);

            // The `data` field contains the input data for contract interactions
            if !tx.input.0.is_empty() {
                // println!("Transaction data (hex encoded): {:?}", tx.input);
                // To decode this, you would need the ABI of the contract being interacted with
                // Decode the input data
                if tx.input.0.len() >= 4 {
                    let data = tx.input.0;

                    if data.starts_with(&transmit_hash) {
                        println!("TRANSMIT FOUND!!!");
                        println!("Transaction from address: {:?}", to);
                        let contract = AGGREGATOR::new(to, client.clone());
                        // println!("data => {:?}", data);
                        if let Ok(description) = contract.description().call().await {
                            println!("aggregator for {}", description);
                        }
                        let to = address_to_string(to).to_lowercase();
                        if let Some(token) = TOKEN_DATA.get(&*to) {
                            println!("price updated for {} => {}", token.name, token.symbol);

                            // update price of token
                            let token_price = token.get_token_price_in_("USDC", client).await?;
                            set_saved_token_price(token.address, token_price).await?;

                            // TODO - UPDATE USERS HERE
                        } else {
                            println!("unknown price feed");
                        };
                    }
                }
            }
        }
    }
    Ok(())
}

pub async fn detect_price_update_test(client: &Arc<Provider<Ws>>) -> Result<()> {
    let transmit_signature = "transmit(bytes,bytes32[],bytes32[],bytes32)";

    // Compute the Keccak-256 hashes of the event signatures
    // let transmit_hash = H256::from(keccak256(transmit_signature.as_bytes()));
    let transmit_hash = keccak256(transmit_signature.as_bytes())[0..4].to_vec();
    //
    // Subscribe to new pending transactions
    let mut stream = client.subscribe_pending_txs().await?;

    // Print out each new transaction hash
    while let Some(tx) = stream.next().await {
        println!("New pending transaction: {:?}", tx);
        if let Ok(Some(tx)) = client.get_transaction(tx).await {
            // println!("Transaction Details: {:?}", tx);

            // If the transaction involves a contract interaction, `to` will be Some(address)
            if let Some(to) = tx.to {
                // println!("Transaction to address: {:?}", to);

                // The `data` field contains the input data for contract interactions
                if !tx.input.0.is_empty() {
                    // println!("Transaction data (hex encoded): {:?}", tx.input);
                    // To decode this, you would need the ABI of the contract being interacted with
                    // Decode the input data
                    if tx.input.0.len() >= 4 {
                        let data = &tx.input.0.as_ref();

                        // Method ID and parameters
                        let method_id = &data[0..4];
                        // println!("Transaction to address: {:?}", to);
                        if data.starts_with(&transmit_hash) {
                            println!("TRANSMIT FOUND!!!");
                            println!("Transaction to address: {:?}", to);
                            println!("Transaction data (hex encoded): {:?}", tx.input);
                            println!("Method ID: {:?}", method_id);
                            println!("transmit_hash: {:?}", &transmit_hash);

                            // let function = Function {
                            //     state_mutability: StateMutability::NonPayable,
                            //     name: "transmit".to_owned(),
                            //     inputs: vec![
                            //         ParamType::Bytes,
                            //         ParamType::Array(Box::new(ParamType::FixedBytes(32))),
                            //         ParamType::Array(Box::new(ParamType::FixedBytes(32))),
                            //         ParamType::FixedBytes(32),
                            //     ],
                            //     outputs: vec![],
                            //     constant: Some(false),
                            // };
                        }
                    }
                    // let to_address = &data[8..72];
                    // let value = &data[72..];

                    // // Convert parameters from hex to more readable formats
                    // let to_address_str = hex_encode(&data[8..40]); // Get the next 32 bytes for the address, encode to hex string
                    // let to_address = Address::from_str(&format!("0x{}", to_address_str)).unwrap();
                    // let value_str = hex_encode(&data[36..]); // Adjust the slice range as needed
                    // let value = U256::from_str_radix(&value_str, 16).unwrap();

                    // println!("To Address: {:?}", to_address);
                    // println!("Value: {:?}", value);
                }
            }
            // Other transaction details
            // println!("Value transferred (in Wei): {:?}", tx.value);
            // println!("Gas Price: {:?}", tx.gas_price);
        } else {
            // println!("Transaction not found or pending confirmation");
        }
    }
    Ok(())
}
