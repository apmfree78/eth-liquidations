use ethers::{
    core::types::{transaction::eip2718::TypedTransaction, Chain},
    providers::{Middleware, Provider, Ws},
    types::{
        BlockNumber, BlockTrace, Eip1559TransactionRequest, NameOrAddress, TraceType, Transaction,
        TransactionRequest,
    },
};
use std::{env, sync::Arc};

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
