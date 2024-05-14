use eth_liquadation::{
    exchanges::aave_v3::events::{
        BorrowEvent, RepayEvent, ReserveCollateralEvent, ReserveUsedAsCollateralDisabledEvent,
        ReserveUsedAsCollateralEnabledEvent, SupplyEvent, WithdrawEvent,
    },
    utils::type_conversion::str_to_h256_hash,
};
use ethers::core::types::{Bytes, Log, H256};

use eth_liquadation::utils::type_conversion::{
    address_to_bytes_array, boolean_to_bytes_array, u16_to_bytes_array, u256_to_bytes_array,
    u8_to_bytes_array,
};

const WITHDRAW_SIGNATURE: &str = "Withdraw(address,address,address,uint256)";
const RESERVE_USED_AS_COLLATERAL_ENABLED_SIGNATURE: &str =
    "ReserveUsedAsCollateralEnabled(address,address)";
const RESERVE_USED_AS_COLLATERAL_DISABLED_SIGNATURE: &str =
    "ReserveUsedAsCollateralDisabled(address,address)";
const BORROW_SIGNATURE: &str = "Borrow(address,address,address,uint256,uint8,uint256,uint16)";
const REPAY_SIGNATURE: &str = "Repay(address,address,address,uint256,bool)";
const SUPPLY_SIGNATURE: &str = "Supply(address,address,address,uint256,uint16)";

pub fn create_log_for_collateral_enable_event(
    event: ReserveUsedAsCollateralEnabledEvent,
    address: &str,
) -> Log {
    let topics = generate_collateral_event_enable_topics_field(event);
    // create mock log
    Log {
        address: address.parse().unwrap(),
        topics,
        ..Default::default()
    }
}

pub fn create_log_for_collateral_disable_event(
    event: ReserveUsedAsCollateralDisabledEvent,
    address: &str,
) -> Log {
    let topics = generate_collateral_event_disable_topics_field(event);
    // create mock log
    Log {
        address: address.parse().unwrap(),
        topics,
        ..Default::default()
    }
}

pub fn create_log_for_withdraw_event(&event: &WithdrawEvent, address: &str) -> Log {
    let topics = generate_withdraw_event_topics_field(&event);
    let data = generate_withdraw_event_log_data_field(&event);
    // create mock log
    Log {
        address: address.parse().unwrap(),
        topics,
        data,
        ..Default::default()
    }
}

pub fn create_log_for_supply_event(&event: &SupplyEvent, address: &str) -> Log {
    let topics = generate_supply_event_topics_field(&event);
    let data = generate_supply_event_log_data_field(&event);
    // create mock log
    Log {
        address: address.parse().unwrap(),
        topics,
        data,
        ..Default::default()
    }
}

pub fn create_log_for_repay_event(&event: &RepayEvent, address: &str) -> Log {
    let topics = generate_repay_event_topics_field(&event);
    let data = generate_repay_event_log_data_field(&event);

    // create mock log
    Log {
        address: address.parse().unwrap(),
        topics,
        data,
        ..Default::default()
    }
}

pub fn create_log_for_borrow_event(&event: &BorrowEvent, address: &str) -> Log {
    let topics = generate_borrow_event_topics_field(&event);
    let data = generate_borrow_event_log_data_field(&event);

    // create mock log
    Log {
        address: address.parse().unwrap(),
        topics,
        data,
        ..Default::default()
    }
}

pub fn generate_borrow_event_topics_field(&event: &BorrowEvent) -> Vec<H256> {
    let topics = vec![
        str_to_h256_hash(BORROW_SIGNATURE),
        event.reserve.into(),
        event.on_behalf_of.into(),
        H256::from(u16_to_bytes_array(event.referral_code)),
    ];
    topics
}

pub fn generate_repay_event_topics_field(&event: &RepayEvent) -> Vec<H256> {
    let topics = vec![
        str_to_h256_hash(REPAY_SIGNATURE),
        event.reserve.into(),
        event.user.into(),
        event.repayer.into(),
    ];
    topics
}

pub fn generate_collateral_event_enable_topics_field(
    event: ReserveUsedAsCollateralEnabledEvent,
) -> Vec<H256> {
    let topics = vec![
        str_to_h256_hash(RESERVE_USED_AS_COLLATERAL_ENABLED_SIGNATURE),
        event.get_reserve().into(),
        event.get_user().into(),
    ];
    topics
}

pub fn generate_collateral_event_disable_topics_field(
    event: ReserveUsedAsCollateralDisabledEvent,
) -> Vec<H256> {
    let topics = vec![
        str_to_h256_hash(RESERVE_USED_AS_COLLATERAL_DISABLED_SIGNATURE),
        event.get_reserve().into(),
        event.get_user().into(),
    ];
    topics
}

pub fn generate_withdraw_event_topics_field(&event: &WithdrawEvent) -> Vec<H256> {
    let topics = vec![
        str_to_h256_hash(WITHDRAW_SIGNATURE),
        event.reserve.into(),
        event.user.into(),
        event.to.into(),
    ];
    topics
}

pub fn generate_supply_event_topics_field(&event: &SupplyEvent) -> Vec<H256> {
    let topics = vec![
        str_to_h256_hash(SUPPLY_SIGNATURE),
        event.reserve.into(),
        event.on_behalf_of.into(),
        H256::from(u16_to_bytes_array(event.referral_code)),
    ];
    topics
}

pub fn generate_borrow_event_log_data_field(&event: &BorrowEvent) -> Bytes {
    let user_bit_array = address_to_bytes_array(event.user);
    let amount_bit_array = u256_to_bytes_array(event.amount);
    let interest_rate_bit_array = u8_to_bytes_array(event.interest_rate_mode);
    let borrow_rate_bit_array = u256_to_bytes_array(event.borrow_rate);

    let mut data = Vec::new();
    data.extend_from_slice(&user_bit_array);
    data.extend_from_slice(&amount_bit_array);
    data.extend_from_slice(&interest_rate_bit_array);
    data.extend_from_slice(&borrow_rate_bit_array);
    data.into()
}

pub fn generate_repay_event_log_data_field(&event: &RepayEvent) -> Bytes {
    let amount_bit_array = u256_to_bytes_array(event.amount);
    let use_a_tokens_bit_array = boolean_to_bytes_array(event.use_a_tokens);

    let mut data = Vec::new();

    data.extend_from_slice(&amount_bit_array);
    data.extend_from_slice(&use_a_tokens_bit_array);
    data.into()
}

pub fn generate_supply_event_log_data_field(&event: &SupplyEvent) -> Bytes {
    let user_bit_array = address_to_bytes_array(event.user);
    let amount_bit_array = u256_to_bytes_array(event.amount);

    let mut data = Vec::new();
    data.extend_from_slice(&user_bit_array);
    data.extend_from_slice(&amount_bit_array);
    data.into()
}

pub fn generate_withdraw_event_log_data_field(&event: &WithdrawEvent) -> Bytes {
    let amount_bit_array = u256_to_bytes_array(event.amount);

    let mut data = Vec::new();
    data.extend_from_slice(&amount_bit_array);
    data.into()
}
