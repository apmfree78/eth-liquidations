use ethers::{prelude::*, utils::keccak256};
use std::sync::Arc;
const AAVE_V3_POOL_ADDRESS: &str = "0x87870Bca3F3fD6335C3F4ce8392D69350B4fA4E2";
use eyre::Result;

pub async fn scan_for_aave_events(client: &Arc<Provider<Ws>>) -> Result<()> {
    // Define the event signatures
    let withdraw_signature = "Withdraw(address,address,address,uint256)";
    let borrow_signature = "Borrow(address,address,address,uint256,uint8,uint256)";
    let repay_signature = "Repay(address,address,address,uint256,bool)";
    let supply_signature = "Supply(address,address,address,uint256,uint16)";

    // Compute the Keccak-256 hashes of the event signatures
    let withdraw_hash = H256::from(keccak256(withdraw_signature.as_bytes()));
    let borrow_hash = H256::from(keccak256(borrow_signature.as_bytes()));
    let repay_hash = H256::from(keccak256(repay_signature.as_bytes()));
    let supply_hash = H256::from(keccak256(supply_signature.as_bytes()));
    let current_block = client.get_block_number().await?;
    let lookback_blocks = U64::from(1000);
    let from_block_number: U64 = if current_block > lookback_blocks {
        current_block - lookback_blocks
    } else {
        U64::from(0)
    };

    // Explicitly convert U64 to BlockNumber for the filter
    let from_block: BlockNumber = BlockNumber::from(from_block_number);

    // let contract_address: Address = AAVE_V3_POOL_ADDRESS.parse()?;
    // let contract = AAVE_V3_POOL::new(contract_address, Arc::clone(&client));

    let filter = Filter::new()
        .address(AAVE_V3_POOL_ADDRESS.parse::<Address>()?)
        .events(
            [
                withdraw_signature,
                borrow_signature,
                repay_signature,
                supply_signature,
            ]
            .to_vec(),
        )
        // .topic1(token_topics.to_vec())
        // .topic2(token_topics.to_vec())
        // .from_block(0);
        .from_block(from_block)
        .to_block(BlockNumber::Latest);
    let logs = client.get_logs(&filter).await?;
    println!("{} aave events found!", logs.iter().len());

    for log in logs.iter() {
        if log.topics.len() > 0 {
            match log.topics[0] {
                topic if topic == withdraw_hash => {
                    println!("Withdraw event: {:?}", log);
                }
                topic if topic == borrow_hash => {
                    println!("Borrow event: {:?}", log);
                }
                topic if topic == repay_hash => {
                    println!("Repay event: {:?}", log);
                }
                topic if topic == supply_hash => {
                    println!("Supply event: {:?}", log);
                }
                _ => {
                    println!("Unknown event: {:?}", log);
                }
            }
        }
    }

    Ok(())
}
