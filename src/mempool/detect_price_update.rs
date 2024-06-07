use crate::{data::{
    erc20::{Convert, TOKEN_DATA},
    token_price_hash::set_saved_token_price,
}, exchanges::aave_v3::user_structs::{UserType, UsersToLiquidate}};
use crate::exchanges::aave_v3::{
    implementations::aave_users_hash::UpdateUsers, user_structs::AaveUsersHash,
};
use crate::utils::type_conversion::address_to_string;
use abi::Address;
use ethers::{prelude::*, utils::keccak256};
use eyre::Result;
use std::sync::Arc;

use ethers::{
    core::types::TxHash,
    providers::{Provider, Ws},
};
use futures::lock::Mutex;

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
                            // println!("data => {:?}", data);
                            if let Ok(description) = contract.description().call().await {
                                println!("aggregator for {}", description);
                            }

                            println!("price updated for {} => {}", token.name, token.symbol);

                            // TODO - handle case where ETH price is update -> will update all ETH and ETH related tokens
                            // update price of token
                            let token_price = token.get_token_price_in_("USDC", client).await?;
                            let token_address: Address = token.address.parse()?;
                            set_saved_token_price(token.address, token_price).await?;

                            let mut users = user_data.lock().await;
                            // 1. update low health user health factor (that own or borrow token)
                            // 2. check if liquidation candidates found
                            if let Ok(liquidations) = users.update_users_health_factor_by_token_and_return_liquidation_candidates(token_address,UserType::LowHealth,client).await {
                                match liquidations {
                                    UsersToLiquidate::Users(users_to_liquidate) => println!("FOUND USERS TO LIQUIDATE {:#?}", users_to_liquidate),
                                    UsersToLiquidate::None => {}
                                }
                                
                            }
                            // 3.  update standard user health factor (that own or borrow token)
                            // 4. repeat step 2 above
                            if let Ok(liquidations) = users.update_users_health_factor_by_token_and_return_liquidation_candidates(token_address,UserType::Standard,client).await {
                                match liquidations {
                                    UsersToLiquidate::Users(users_to_liquidate) => println!("FOUND USERS TO LIQUIDATE {:#?}", users_to_liquidate),
                                    UsersToLiquidate::None => {}
                                }
                                
                            }
                            // 5. clean up - update token ==> user mapping for all users with updated health factors
                            // use => update_token_to_user_mapping_for_all_users_with_token_(...)
                            users.update_token_to_user_mapping_for_all_users_with_token_(token_address)?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
