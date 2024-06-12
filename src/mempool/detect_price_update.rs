use crate::exchanges::aave_v3::user_structs::AaveUsersHash;
use crate::mempool::update_token_price::update_token_price_for_;
use crate::utils::type_conversion::address_to_string;
use crate::{data::erc20::TOKEN_DATA, mempool::liquidations::find_users_and_liquidate};
use ethers::{
    core::types::TxHash,
    providers::{Provider, Ws},
};
use ethers::{prelude::*, utils::keccak256};
use eyre::Result;
use futures::lock::Mutex;
use std::sync::Arc;

pub async fn detect_price_update_and_find_users_to_liquidate(
    user_data: &Arc<Mutex<AaveUsersHash>>,
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
                        let to_address = address_to_string(to).to_lowercase();
                        if let Some(token) = TOKEN_DATA.get(&*to_address) {
                            println!("TRANSMIT FOUND!!!");
                            println!("Transaction from address: {:?}", to);
                            let contract = AGGREGATOR::new(to, client.clone());

                            // below if statement is for testing purposes only
                            if let Ok(description) = contract.description().call().await {
                                println!("aggregator for {}", description);
                            }

                            // TODO - handle case where ETH price is update -> will update all ETH and ETH related tokens
                            // update price of token
                            update_token_price_for_(token, client).await?;

                            find_users_and_liquidate(user_data, token, client).await?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
