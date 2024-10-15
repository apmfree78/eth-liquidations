use ethers::{
    core::types::{transaction::eip2718::TypedTransaction, Chain},
    providers::{Middleware, Provider, Ws},
    types::{
        AccountDiff, Address, BigEndianHash, BlockNumber, BlockTrace, Diff,
        Eip1559TransactionRequest, NameOrAddress, StateDiff, StorageDiff, TraceType, Transaction,
        TransactionRequest, H256, U256,
    },
};
use std::collections::BTreeMap;
use std::{env, sync::Arc};

use crate::utils::type_conversion::{h256_to_address, h256_to_u256};

pub async fn simulate_transaction_bundle(
    mempool_tx: Transaction,
    backrun_tx: Eip1559TransactionRequest,
    client: &Arc<Provider<Ws>>,
) -> anyhow::Result<Vec<BlockTrace>> {
    // Define trace types
    let trace_types = vec![TraceType::Trace, TraceType::StateDiff];

    let mempool_tx_typed = convert_transaction_to_typed_transaction(&mempool_tx);
    // Create request vector
    let simulation_request = vec![
        (mempool_tx_typed, trace_types.clone()),
        (TypedTransaction::Eip1559(backrun_tx), trace_types),
    ];

    // Call trace_call_many
    let traces = client
        .trace_call_many(simulation_request, Some(BlockNumber::Latest))
        .await?;

    Ok(traces)
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

pub async fn process_state_diffs(traces: Vec<BlockTrace>) -> anyhow::Result<()> {
    for (idx, trace) in traces.iter().enumerate() {
        println!("Processing transaction {}:", idx + 1);

        if let Some(state_diff) = &trace.state_diff {
            // state_diff is a BTreeMap<Address, AccountDiff>
            for (account_address, account_diff) in &state_diff.0 {
                println!("Contract Address with state changes: {:?}", account_address);

                // storage_diff is a BTreeMap<H256, StorageDiff>
                // Replace `target_contract_address` with your contract's address
                // TODO - add contract address
                let target_contract_address: Address = "0xYourContractAddress".parse().unwrap();
                if *account_address == target_contract_address {
                    // Parse the storage changes for `topProfitAccount`
                    parse_top_profit_account(&account_diff.storage);
                }
            }
        } else {
            println!("No stateDiff available for this transaction.");
        }
    }

    Ok(())
}
fn parse_top_profit_account(storage_diff: &BTreeMap<H256, Diff<H256>>) {
    // Define the storage slots for the struct fields
    let base_slot = U256::zero(); // Adjust based on your contract's storage layout

    // For structs, each member occupies a slot in sequence
    let slots = [
        base_slot,                 // userId
        base_slot + U256::from(1), // debtToken
        base_slot + U256::from(2), // collateralToken
        base_slot + U256::from(3), // profit
    ];

    let mut values = vec![];

    for slot in &slots {
        // Convert slot to H256
        let slot_h256 = H256::from_uint(slot);
        if let Some(diff_entry) = storage_diff.get(&slot_h256) {
            // Handle the Diff<H256> variants
            match diff_entry {
                Diff::Same => {
                    // Value hasn't changed; you may need to retrieve the current value from storage
                    // For simulation purposes, you might use a default value or skip it
                    values.push(H256::zero()); // Placeholder
                }
                Diff::Born(value) => {
                    // The slot was created with this value
                    values.push(value.clone());
                }
                Diff::Died(_) => {
                    // The slot was destroyed; you may choose to handle this accordingly
                    values.push(H256::zero()); // Placeholder
                }
                Diff::Changed(changed) => {
                    // The slot value changed from 'changed.from' to 'changed.to'
                    values.push(changed.to.clone());
                }
            }
        } else {
            // Slot not present in storage_diff; it hasn't changed
            // You may need to retrieve the current value from storage or assume a default
            values.push(H256::zero()); // Placeholder
        }
    }

    if values.len() == 4 {
        let user_id = h256_to_address(&values[0]);
        let debt_token = h256_to_address(&values[1]);
        let collateral_token = h256_to_address(&values[2]);
        let profit = h256_to_u256(&values[3]);

        println!("TopProfitUserAccount:");
        println!("  userId: {:?}", user_id);
        println!("  debtToken: {:?}", debt_token);
        println!("  collateralToken: {:?}", collateral_token);
        println!("  profit: {:?}", profit);
    }
}
