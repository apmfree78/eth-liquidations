use crate::abi::liquidate_user::{User, LIQUIDATE_USER};
use crate::data::address::LIQUIDATE_USER_ADDRESS;
use crate::exchanges::aave_v3::user_structs::LiquidationCandidate;
use ethers::{
    core::types::transaction::eip2718::TypedTransaction,
    providers::{Provider, Ws},
};
use std::sync::Arc;

// User is defined as below
// pub struct User {
//     pub id: ::ethers::core::types::Address,
//     pub debt_token: ::ethers::core::types::Address,
//     pub collateral_token: ::ethers::core::types::Address,
// }
pub async fn get_liquidate_user_contract_transaction(
    client: &Arc<Provider<Ws>>,
    liquidation_users: Vec<LiquidationCandidate>,
) -> Result<TypedTransaction, Box<dyn std::error::Error>> {
    // convert user to correct type
    let mut users = Vec::<User>::new();

    for user in liquidation_users {
        users.push(User {
            id: user.user_id,
            debt_token: user.debt_token,
            collateral_token: user.collateral_token,
        })
    }

    let liquidate_user = LIQUIDATE_USER::new(*LIQUIDATE_USER_ADDRESS, client.clone());

    // Encode the function with parameters, and get TypedTransaction
    let transaction = liquidate_user.find_and_liquidate_account(users).tx;

    Ok(transaction)
}
