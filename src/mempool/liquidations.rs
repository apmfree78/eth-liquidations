use crate::backrun::flashbots::submit_to_flashbots;
use crate::data::erc20::Erc20Token;
use crate::data::users_to_track::add_tracked_users;
use crate::exchanges::aave_v3::user_structs::{UserType, UsersToLiquidate};
use crate::exchanges::aave_v3::{
    implementations::aave_users_hash::UpdateUsers, user_structs::AaveUsersHash,
};
use ethers::types::Transaction;
use eyre::Result;
use log::info;
use std::sync::Arc;

use ethers::providers::{Provider, Ws};
use futures::lock::Mutex;

pub async fn find_users_and_liquidate(
    user_data: &Arc<Mutex<AaveUsersHash>>,
    token: &Erc20Token,
    mempool_tx: Transaction,
    client: &Arc<Provider<Ws>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut users = user_data.lock().await;
    // 1. update low health user health factor (that own or borrow token)
    // 2. check if liquidation candidates found
    if let Ok(liquidations) = users
        .update_users_health_factor_by_token_and_return_liquidation_candidates(
            token,
            UserType::LowHealth,
            client,
        )
        .await
    {
        match liquidations {
            UsersToLiquidate::Users(users_to_liquidate) => {
                info!("FOUND {} USERS TO LIQUIDATE", users_to_liquidate.len());
                add_tracked_users(users_to_liquidate.clone()).await?;

                // TODO - UNCOMMENT WHEN SEPOLIA IS READY
                // submit_to_flashbots(&users_to_liquidate, mempool_tx, client);
            }
            UsersToLiquidate::None => {}
        }
    }
    // 3.  update standard user health factor (that own or borrow token)
    // 4. repeat step 2 above
    if let Ok(liquidations) = users
        .update_users_health_factor_by_token_and_return_liquidation_candidates(
            token,
            UserType::Standard,
            client,
        )
        .await
    {
        match liquidations {
            UsersToLiquidate::Users(users_to_liquidate) => {
                info!("FOUND {} USERS TO LIQUIDATE", users_to_liquidate.len());
                add_tracked_users(users_to_liquidate).await?;
            }
            UsersToLiquidate::None => {}
        }
    }
    // 5. clean up - update token ==> user mapping for all users with updated health factors
    users
        .update_token_to_user_mapping_for_all_users_with_token_(token, client)
        .await?;

    Ok(())
}
