use crate::abi::qualifyuser::{TopProfitUserAccount, User};
use ethers::{
    core::types::transaction::eip2718::TypedTransaction,
    types::{Eip1559TransactionRequest, NameOrAddress, Transaction, TransactionRequest},
};
use std::env;

use crate::exchanges::aave_v3::user_structs::LiquidationCandidate;

use super::anvil_simulator::AnvilSimulator;

pub async fn find_top_profit_accounts_with_simulation(
    user: &[LiquidationCandidate],
    mempool_tx: &Transaction,
) -> anyhow::Result<TopProfitUserAccount> {
    // get last block number
    let ws_url = env::var("WS_URL").expect("could not retrieve WS_URL");

    // simulate on anvil with mainnet fork
    let anvil = AnvilSimulator::new(&ws_url).await?;

    // *******************************************************
    let qualify_user_data = get_qualify_user_data(user);

    let top_profit_accounts = anvil
        .simulate_oracle_plus_qualify_user_tx(mempool_tx, qualify_user_data)
        .await?;
    Ok(top_profit_accounts)
}

fn get_qualify_user_data(liquidation_users: &[LiquidationCandidate]) -> Vec<User> {
    // convert user to correct type
    let mut users = Vec::<User>::new();

    for user in liquidation_users {
        users.push(User {
            id: user.user_id,
            debt_token: user.debt_token,
            collateral_token: user.collateral_token,
        })
    }

    users
}

pub fn convert_transaction_to_typed_transaction(tx: &Transaction) -> TypedTransaction {
    let to = tx.to.clone().unwrap();
    let to = NameOrAddress::Address(to);

    if tx.transaction_type == Some(2.into()) {
        // EIP-1559 transaction
        TypedTransaction::Eip1559(Eip1559TransactionRequest {
            from: Some(tx.from),
            to: Some(to),
            value: Some(tx.value),
            data: Some(tx.input.clone()),
            nonce: Some(tx.nonce),
            gas: Some(tx.gas),
            max_fee_per_gas: tx.max_fee_per_gas,
            max_priority_fee_per_gas: tx.max_priority_fee_per_gas,
            access_list: tx.access_list.clone().unwrap_or_default(),
            chain_id: tx.chain_id.map(|id| ethers::types::U64::from(id.as_u64())),
        })
    } else {
        // Legacy transaction
        TypedTransaction::Legacy(TransactionRequest {
            from: Some(tx.from),
            to: Some(to),
            value: Some(tx.value),
            data: Some(tx.input.clone()),
            nonce: Some(tx.nonce),
            gas: Some(tx.gas),
            gas_price: tx.gas_price,
            chain_id: tx.chain_id.map(|id| ethers::types::U64::from(id.as_u64())),
        })
    }
}
// fn get_qualify_user_calldata(
//     liquidation_users: &[LiquidationCandidate],
//     client: &Arc<Provider<Ws>>,
// ) -> anyhow::Result<Bytes> {
//     let qualify_user_address: Address = CONTRACT.get_address().qualify_user.parse()?;
//     // convert user to correct type
//     let mut users = Vec::<User>::new();
//
//     for user in liquidation_users {
//         users.push(User {
//             id: user.user_id,
//             debt_token: user.debt_token,
//             collateral_token: user.collateral_token,
//         })
//     }
//
//     let qualify_user = QUALIFY_USER::new(qualify_user_address, client.clone());
//
//     // Encode the function with parameters, and get TypedTransaction
//     let calldata = qualify_user
//         .check_user_accounts(users)
//         .calldata()
//         .expect("Failed to encode");
//
//     Ok(calldata)
// }
//
// pub fn get_state_diffs_from_qualify_user_trace(
//     traces: &[BlockTrace],
// ) -> Option<TopProfitUserAccount> {
//     for (idx, trace) in traces.iter().enumerate() {
//         println!("Processing transaction {}:", idx + 1);
//
//         let qualify_user_address = CONTRACT.get_address().qualify_user.clone();
//
//         if let Some(state_diff) = &trace.state_diff {
//             // state_diff is a BTreeMap<Address, AccountDiff>
//             for (account_address, account_diff) in &state_diff.0 {
//                 println!("Contract Address with state changes: {:?}", account_address);
//
//                 // storage_diff is a BTreeMap<H256, StorageDiff>
//                 // Replace `target_contract_address` with your contract's address
//                 // TODO - add contract address
//                 let qualify_user_address: Address = qualify_user_address.parse().unwrap();
//                 if *account_address == qualify_user_address {
//                     // Parse the storage changes for `topProfitAccount`
//                     return Some(parse_top_profit_account(&account_diff.storage));
//                 }
//             }
//         } else {
//             println!("No stateDiff available for this transaction.");
//         }
//     }
//
//     None
// }
//
// fn parse_top_profit_account(storage_diff: &BTreeMap<H256, Diff<H256>>) -> TopProfitUserAccount {
//     // Define the storage slots for the struct fields
//     let base_slot = U256::zero(); // Adjust based on your contract's storage layout
//
//     // For structs, each member occupies a slot in sequence
//     let slots = [
//         base_slot,                 // userId
//         base_slot + U256::from(1), // debtToken
//         base_slot + U256::from(2), // collateralToken
//         base_slot + U256::from(3), // debtToCover
//         base_slot + U256::from(4), // profit
//     ];
//
//     let mut values = vec![];
//
//     for slot in &slots {
//         // Convert slot to H256
//         let slot_h256 = H256::from_uint(slot);
//         if let Some(diff_entry) = storage_diff.get(&slot_h256) {
//             // Handle the Diff<H256> variants
//             match diff_entry {
//                 Diff::Same => {
//                     // Value hasn't changed; you may need to retrieve the current value from storage
//                     // For simulation purposes, you might use a default value or skip it
//                     values.push(H256::zero()); // Placeholder
//                 }
//                 Diff::Born(value) => {
//                     // The slot was created with this value
//                     values.push(value.clone());
//                 }
//                 Diff::Died(_) => {
//                     // The slot was destroyed; you may choose to handle this accordingly
//                     values.push(H256::zero()); // Placeholder
//                 }
//                 Diff::Changed(changed) => {
//                     // The slot value changed from 'changed.from' to 'changed.to'
//                     values.push(changed.to.clone());
//                 }
//             }
//         } else {
//             // Slot not present in storage_diff; it hasn't changed
//             // You may need to retrieve the current value from storage or assume a default
//             values.push(H256::zero()); // Placeholder
//         }
//     }
//
//     let user_id = h256_to_address(&values[0]);
//     let debt_token = h256_to_address(&values[1]);
//     let collateral_token = h256_to_address(&values[2]);
//     let debt_to_cover = h256_to_u256(&values[3]);
//     let profit = h256_to_u256(&values[4]);
//
//     println!("TopProfitUserAccount:");
//     println!("  userId: {:?}", user_id);
//     println!("  debtToken: {:?}", debt_token);
//     println!("  collateralToken: {:?}", collateral_token);
//     println!("  debt_to_cover: {:?}", debt_to_cover);
//     println!("  profit: {:?}", profit);
//
//     return TopProfitUserAccount {
//         user_id,
//         debt_token,
//         collateral_token,
//         debt_to_cover,
//         profit,
//     };
// }
//
