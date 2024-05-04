use crate::abi::aave_v3_pool::AAVE_V3_POOL;
use crate::exchanges::aave_v3::data::AaveUserData;
use crate::exchanges::aave_v3::events::{
    create_aave_event_from_log, get_user_action_from_event, AaveEvent, AaveEventType,
    AaveUserEvent, Update,
};
use ethers::contract::Event;
use ethers::{prelude::*, utils::keccak256};
use std::sync::Arc;
const AAVE_V3_POOL_ADDRESS: &str = "0x87870Bca3F3fD6335C3F4ce8392D69350B4fA4E2";
use eyre::Result;

pub async fn scan_and_update_aave_events(
    users: &mut Vec<AaveUserData>,
    client: &Arc<Provider<Ws>>,
) -> Result<()> {
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

    // TODO - refactor users into a hashmap
    for log in logs.iter() {
        if log.topics.len() > 0 {
            match log.topics[0] {
                topic if topic == withdraw_hash => {
                    println!("Withdraw event: {:?}", log);

                    if let AaveEventType::WithdrawEvent(event) =
                        create_aave_event_from_log(AaveUserEvent::WithDraw, &log)
                    {
                        update_aave_user(users, event).unwrap();
                    }
                }
                topic if topic == borrow_hash => {
                    println!("Borrow event: {:?}", log);
                    if let AaveEventType::BorrowEvent(event) =
                        create_aave_event_from_log(AaveUserEvent::Borrow, &log)
                    {
                        update_aave_user(users, event).unwrap();
                    }
                }
                topic if topic == repay_hash => {
                    println!("Repay event: {:?}", log);
                    if let AaveEventType::RepayEvent(event) =
                        create_aave_event_from_log(AaveUserEvent::Repay, &log)
                    {
                        update_aave_user(users, event).unwrap();
                    }
                }
                topic if topic == supply_hash => {
                    println!("Supply event: {:?}", log);
                    if let AaveEventType::SupplyEvent(event) =
                        create_aave_event_from_log(AaveUserEvent::Supply, &log)
                    {
                        update_aave_user(users, event).unwrap();
                    }
                }
                _ => {
                    println!("Unknown event: {:?}", log);
                }
            }
        }
    }

    Ok(())
}

// TODO - refactor to make users a hashmap
pub fn update_aave_user<T: AaveEvent>(
    users: &mut Vec<AaveUserData>,
    event: T,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_action = get_user_action_from_event(&event);
    for user in users.iter_mut() {
        if user.id == event.get_from() {
            user.update(&user_action).unwrap();
            return Ok(());
        }
    }
    Ok(())
}
