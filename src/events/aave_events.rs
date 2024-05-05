use crate::exchanges::aave_v3::data::AaveUserData;
use crate::exchanges::aave_v3::events::{
    create_aave_event_from_log, get_user_action_from_event, AaveEvent, AaveEventType,
    AaveUserEvent, Update,
};
use ethers::{prelude::*, utils::keccak256};
use std::collections::HashMap;
use std::sync::Arc;
const AAVE_V3_POOL_ADDRESS: &str = "0x87870Bca3F3fD6335C3F4ce8392D69350B4fA4E2";
use eyre::Result;

const WITHDRAW_SIGNATURE: &str = "Withdraw(address,address,address,uint256)";
const BORROW_SIGNATURE: &str = "Borrow(address,address,address,uint256,uint8,uint256)";
const REPAY_SIGNATURE: &str = "Repay(address,address,address,uint256,bool)";
const SUPPLY_SIGNATURE: &str = "Supply(address,address,address,uint256,uint16)";

pub async fn scan_and_update_aave_events(
    users: &mut Vec<AaveUserData>,
    client: &Arc<Provider<Ws>>,
) -> Result<()> {
    let aave_event_map = setup_event_map();

    // Compute the Keccak-256 hashes of the event signatures
    let current_block = client.get_block_number().await?;
    let lookback_blocks = U64::from(1000);
    let from_block_number: U64 = if current_block > lookback_blocks {
        current_block - lookback_blocks
    } else {
        U64::from(0)
    };

    // Explicitly convert U64 to BlockNumber for the filter
    let from_block: BlockNumber = BlockNumber::from(from_block_number);

    let filter = Filter::new()
        .address(AAVE_V3_POOL_ADDRESS.parse::<Address>()?)
        .events(
            [
                WITHDRAW_SIGNATURE,
                BORROW_SIGNATURE,
                REPAY_SIGNATURE,
                SUPPLY_SIGNATURE,
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
        if !log.topics.is_empty() {
            //determine which aave event was found
            if let Some(aave_event_enum) = aave_event_map.get(&log.topics[0]) {
                println!("{:?} event: {:?}", aave_event_enum, log);

                // extract event data from log
                let aave_event_type_with_data = create_aave_event_from_log(*aave_event_enum, &log);

                // extract struct data from event enum
                let event = extract_aave_event_data(&aave_event_type_with_data).unwrap();

                // update aave user
                update_aave_user(users, event).unwrap();
            }
        }
    }

    Ok(())
}

fn setup_event_map() -> HashMap<H256, AaveUserEvent> {
    let mut event_map = HashMap::new();
    let borrow_hash: H256 = H256::from(keccak256(BORROW_SIGNATURE.as_bytes()));
    let withdraw_hash: H256 = H256::from(keccak256(WITHDRAW_SIGNATURE.as_bytes()));
    let repay_hash: H256 = H256::from(keccak256(REPAY_SIGNATURE.as_bytes()));
    let supply_hash: H256 = H256::from(keccak256(SUPPLY_SIGNATURE.as_bytes()));

    event_map.insert(withdraw_hash, AaveUserEvent::WithDraw);
    event_map.insert(borrow_hash, AaveUserEvent::Borrow);
    event_map.insert(repay_hash, AaveUserEvent::Repay);
    event_map.insert(supply_hash, AaveUserEvent::Supply);
    event_map
}

pub fn extract_aave_event_data(
    event_enum: &AaveEventType,
) -> Result<Box<dyn AaveEvent>, Box<dyn std::error::Error>> {
    match event_enum {
        AaveEventType::WithdrawEvent(event) => Ok(Box::new(*event)),
        AaveEventType::BorrowEvent(event) => Ok(Box::new(*event)),
        AaveEventType::RepayEvent(event) => Ok(Box::new(*event)),
        AaveEventType::SupplyEvent(event) => Ok(Box::new(*event)),
        _ => Err("Unhandled event type".into()),
    }
}

// TODO - refactor to make users a hashmap
pub fn update_aave_user(
    users: &mut Vec<AaveUserData>,
    event: Box<dyn AaveEvent>,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_address = event.get_user();
    let user_action = get_user_action_from_event(event);
    println!("user action {:#?}", user_action);
    for user in users.iter_mut() {
        if user.id == user_address {
            println!("updating user {:#?}", user);
            if let Err(e) = user.update(&user_action) {
                return Err(e);
            } else {
                println!("user updated! => {:#?}", user);
                return Ok(());
            }
        }
    }
    Ok(())
}
