use crate::exchanges::aave_v3::user_structs::AaveUsersHash;
use crate::mempool::decode_new_price::decode_forward_fn;
use crate::mempool::liquidations::find_users_and_liquidate;
use crate::mempool::update_token_price::update_token_price_for_;
use crate::utils::type_conversion::address_to_string;
use crate::{
    data::chainlink_feed_map::get_chainlink_aggregator_map,
    mempool::decode_new_price::get_chainlink_price_from_transmit_tx,
};
use anyhow::Result;
use ethers::{
    core::types::TxHash,
    providers::{Provider, Ws},
};
use ethers::{prelude::*, utils::keccak256};
use futures::lock::Mutex;
use log::debug;
use std::sync::Arc;

pub async fn detect_price_update_and_find_users_to_liquidate(
    user_data: &Arc<Mutex<AaveUsersHash>>,
    pending_tx: TxHash,
    client: &Arc<Provider<Ws>>,
) -> Result<()> {
    let forward_signature = "forward(address,bytes)";
    let chain_aggregator_map = get_chainlink_aggregator_map().await?;

    // abigen!(
    //     AGGREGATOR,
    //     r#"[function description() external view returns (string)]"#
    // );

    // Compute the Keccak-256 hashes of the event signatures
    let forward_hash = keccak256(forward_signature.as_bytes())[0..4].to_vec();

    // Print out each new transaction hash
    if let Ok(Some(tx)) = client.get_transaction(pending_tx).await {
        // If the transaction involves a contract interaction, `to` will be Some(address)
        if let Some(_) = tx.to {
            // The `data` field contains the input data for contract interactions
            if !tx.input.0.is_empty() && tx.input.0.len() >= 4 {
                let data = tx.input.0.clone();

                if data.starts_with(&forward_hash) {
                    // extract address from data ==> forward(address,bytes)
                    let (aggregator, transmit_data) = decode_forward_fn(&data.into())?;
                    let aggregator_address = address_to_string(aggregator).to_lowercase();
                    if let Some(token) = chain_aggregator_map.get(&*aggregator_address) {
                        debug!("TRANSMIT FOUND!!!");
                        let new_token_price =
                            get_chainlink_price_from_transmit_tx(&transmit_data.into(), token)
                                .await?;
                        debug!("new price of {} is {}", token.symbol, new_token_price);

                        // if tx.transaction_type == Some(2.into()) {
                        //     // Confirming it's an EIP-1559 transaction
                        //     if let Some(max_priority_fee_per_gas) = tx.max_priority_fee_per_gas {
                        //         debug!("Max Priority Fee per Gas: {}", max_priority_fee_per_gas);
                        //     }
                        //     if let Some(max_fee_per_gas) = tx.max_fee_per_gas {
                        //         debug!("Max Fee per Gas: {}", max_fee_per_gas);
                        //     }
                        // } else {
                        //     debug!("NOT EIP-1559 transaction");
                        // }

                        // debug!("Transaction from address: {:?}", to);
                        // let contract = AGGREGATOR::new(to, client.clone());
                        //
                        // // below if statement is for testing purposes only
                        // if let Ok(description) = contract.description().call().await {
                        //     debug!("aggregator for {}", description);
                        // }
                        // ***************************************************************************
                        // ***************************************************************************

                        // update price of token
                        update_token_price_for_(token, new_token_price, client).await?;

                        find_users_and_liquidate(user_data, token, tx, client).await?;
                    }
                }
            }
        }
    }
    Ok(())
}
