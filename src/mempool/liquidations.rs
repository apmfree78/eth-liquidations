use crate::backrun::flashbots::submit_to_flashbots;
use crate::data::erc20::Erc20Token;
use crate::data::token_data_hash::get_token_data;
use crate::data::users_to_track::add_tracked_users;
use crate::exchanges::aave_v3::user_structs::{LiquidationCandidate, UserType, UsersToLiquidate};
use crate::exchanges::aave_v3::{
    implementations::aave_users_hash::UpdateUsers, user_structs::AaveUsersHash,
};
use ethers::types::Transaction;
use log::info;
use std::sync::Arc;

use anyhow::Result;
use ethers::providers::{Provider, Ws};
use futures::lock::Mutex;

pub async fn find_users_and_liquidate(
    user_data: &Arc<Mutex<AaveUsersHash>>,
    token: &Erc20Token,
    mempool_tx: Transaction,
    client: &Arc<Provider<Ws>>,
) -> Result<()> {
    // edge CASE
    let token_data = get_token_data().await?;
    let wsteth_token = token_data.get("wstETH").unwrap();
    // 1. update low health user health factor (that own or borrow token)
    // 2. check if liquidation candidates found

    let mut user_accounts_to_liquidate =
        update_and_get_accounts_to_liquidate(user_data, token, UserType::LowHealth, client).await?;

    //EDGE CASE
    if token.symbol == "stETH" {
        let wsteth_user_accounts_low_health = update_and_get_accounts_to_liquidate(
            user_data,
            wsteth_token,
            UserType::LowHealth,
            client,
        )
        .await?;

        if !wsteth_user_accounts_low_health.is_empty() {
            user_accounts_to_liquidate.extend(wsteth_user_accounts_low_health);
        }
    }

    // TODO - UNCOMMENT WHEN SEPOLIA IS READY
    // submit_to_flashbots(&users_to_liquidate, mempool_tx, client);

    // 3.  update standard user health factor (that own or borrow token)
    update_and_get_accounts_to_liquidate(user_data, token, UserType::Standard, client).await?;

    if token.symbol == "stETH" {
        update_and_get_accounts_to_liquidate(user_data, wsteth_token, UserType::Standard, client)
            .await?;
    }
    // 4. repeat step 2 above
    // 5. clean up - update token ==> user mapping for all users with updated health factors
    let mut users = user_data.lock().await;
    users
        .update_token_to_user_mapping_for_all_users_with_token_(token, client)
        .await?;
    if token.symbol == "stETH" {
        users
            .update_token_to_user_mapping_for_all_users_with_token_(wsteth_token, client)
            .await?;
    }
    Ok(())
}

async fn update_and_get_accounts_to_liquidate(
    user_data: &Arc<Mutex<AaveUsersHash>>,
    token: &Erc20Token,
    user_type: UserType,
    client: &Arc<Provider<Ws>>,
) -> Result<Vec<LiquidationCandidate>> {
    let mut users = user_data.lock().await;
    let empty_vec_to_return_if_no_qualifed_accounts = Vec::<LiquidationCandidate>::new();
    // 1. update low health user health factor (that own or borrow token)
    // 2. check if liquidation candidates found
    if let Ok(liquidations) = users
        .update_users_health_factor_by_token_and_return_liquidation_candidates(
            token, user_type, client,
        )
        .await
    {
        match liquidations {
            UsersToLiquidate::Users(users_to_liquidate) => {
                info!(
                    "FOUND {} USERS TO LIQUIDATE AMONG {:?} USERS",
                    users_to_liquidate.len(),
                    user_type
                );
                add_tracked_users(users_to_liquidate.clone()).await?;

                return Ok(users_to_liquidate);
            }
            UsersToLiquidate::None => {
                info!("NO USER TO LIQUIDATE AMONG {:?} USERS", user_type);
                return Ok(empty_vec_to_return_if_no_qualifed_accounts);
            }
        }
    };
    Ok(empty_vec_to_return_if_no_qualifed_accounts)
}
