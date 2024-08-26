use crate::abi::chainlink_aggregator::CHAINLINK_AGGREGATOR;
use crate::data::chainlink_feed_map::get_chainlink_aggregator_map;
use crate::exchanges::aave_v3::user_structs::AaveUsersHash;
use crate::mempool::decode_new_price::decode_transmit_report;
use crate::mempool::liquidations::find_users_and_liquidate;
use crate::mempool::update_token_price::update_token_price_for_;
use crate::utils::type_conversion::address_to_string;
use ethers::abi::{Abi, Token};
use ethers::{
    core::types::TxHash,
    providers::{Provider, Ws},
};
use ethers::{prelude::*, utils::keccak256};
use eyre::Result;
use futures::lock::Mutex;
use log::debug;
use std::sync::Arc;

pub async fn detect_price_update_and_find_users_to_liquidate(
    user_data: &Arc<Mutex<AaveUsersHash>>,
    pending_tx: TxHash,
    client: &Arc<Provider<Ws>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let transmit_signature = "transmit(bytes,bytes32[],bytes32[],bytes32)";

    let chain_aggregator_map = get_chainlink_aggregator_map().await?;

    abigen!(
        AGGREGATOR,
        r#"[function description() external view returns (string)]"#
    );

    let transmit_abi = r#"
    [
        {
            "constant": false,
            "inputs": [
                {"name": "report", "type": "bytes"},
                {"name": "rs", "type": "bytes32[]"},
                {"name": "ss", "type": "bytes32[]"},
                {"name": "rawVs", "type": "bytes32"}
            ],
            "name": "transmit",
            "outputs": [],
            "payable": false,
            "stateMutability": "nonpayable",
            "type": "function"
        }
    ]
    "#;

    // Compute the Keccak-256 hashes of the event signatures
    // let transmit_hash = H256::from(keccak256(transmit_signature.as_bytes()));
    let transmit_hash = keccak256(transmit_signature.as_bytes())[0..4].to_vec();

    // Print out each new transaction hash
    if let Ok(Some(tx)) = client.get_transaction(pending_tx).await {
        // If the transaction involves a contract interaction, `to` will be Some(address)
        if let Some(to) = tx.to {
            // The `data` field contains the input data for contract interactions
            if !tx.input.0.is_empty() && tx.input.0.len() >= 4 {
                let data = tx.input.0.clone();

                if data.starts_with(&transmit_hash) {
                    let to_address = address_to_string(to).to_lowercase();
                    if let Some(token) = chain_aggregator_map.get(&*to_address) {
                        debug!("TRANSMIT FOUND!!!");
                        // debug!("tx.input => {:#?}", tx.input);

                        // TEST DECODING
                        // let abi_json = r#"[{"inputs":[{"internalType":"bytes","name":"report","type":"bytes"},{"internalType":"bytes32[]","name":"rs","type":"bytes32[]"},{"internalType":"bytes32[]","name":"ss","type":"bytes32[]"},{"internalType":"bytes32","name":"rawVs","type":"bytes32"}],"name":"transmit","type":"function"}]"#;
                        let abi = Abi::load(transmit_abi.as_bytes())?;
                        if let Ok(function) = abi.function("transmit") {
                            if let Ok(decoded) = function.decode_input(&data[4..]) {
                                // Skip selector if included
                                match decoded.as_slice() {
                                    [Token::Bytes(report), Token::Array(_), Token::Array(_), Token::FixedBytes(_)] =>
                                    {
                                        let observations = decode_transmit_report(report.clone())?;
                                    }
                                    _ => {
                                        return Err(
                                            "Failed to match decoded arguments with expected types"
                                                .into(),
                                        )
                                    }
                                }
                            } else {
                                return Err("Failed to decode input data for `transmit`".into());
                            }
                        } else {
                            return Err("Transmit function not found in ABI".into());
                        }

                        if tx.transaction_type == Some(2.into()) {
                            // Confirming it's an EIP-1559 transaction
                            if let Some(max_priority_fee_per_gas) = tx.max_priority_fee_per_gas {
                                debug!("Max Priority Fee per Gas: {}", max_priority_fee_per_gas);
                            }
                            if let Some(max_fee_per_gas) = tx.max_fee_per_gas {
                                debug!("Max Fee per Gas: {}", max_fee_per_gas);
                            }
                        } else {
                            debug!("NOT EIP-1559 transaction");
                        }

                        debug!("Transaction from address: {:?}", to);
                        let contract = AGGREGATOR::new(to, client.clone());

                        // below if statement is for testing purposes only
                        if let Ok(description) = contract.description().call().await {
                            debug!("aggregator for {}", description);
                        }
                        // ***************************************************************************
                        // ***************************************************************************

                        // update price of token
                        update_token_price_for_(token, client).await?;

                        find_users_and_liquidate(user_data, token, tx, client).await?;
                    }
                }
            }
        }
    }
    Ok(())
}
