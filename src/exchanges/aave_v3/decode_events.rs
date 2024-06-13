use crate::exchanges::aave_v3::events::ReserveUsedAsCollateralEnabledEvent;

use super::events::{
    AaveEventType, AaveUserEvent, BorrowEvent, RepayEvent, ReserveCollateralEvent,
    ReserveUsedAsCollateralDisabledEvent, SupplyEvent, WithdrawEvent,
};
use ethers::abi::Address;
use ethers::core::abi::RawLog;
use ethers::core::types::{Log, U256};

pub fn create_aave_event_from_log(event_type: AaveUserEvent, log: &Log) -> AaveEventType {
    match event_type {
        AaveUserEvent::WithDraw => {
            let withdraw_event = decode_withdraw_event(log).unwrap();
            AaveEventType::WithdrawEvent(withdraw_event)
        }
        AaveUserEvent::Borrow => {
            let borrow_event = decode_borrow_event(log).unwrap();
            AaveEventType::BorrowEvent(borrow_event)
        }
        AaveUserEvent::Repay => {
            let repay_event = decode_repay_event(log).unwrap();
            AaveEventType::RepayEvent(repay_event)
        }
        AaveUserEvent::Supply => {
            let supply_event = decode_supply_event(log).unwrap();
            AaveEventType::SupplyEvent(supply_event)
        }
        AaveUserEvent::ReserveUsedAsCollateralDisabled => {
            let reserve_disable_event = decode_reserve_used_as_colladeral_event::<
                ReserveUsedAsCollateralDisabledEvent,
            >(log)
            .unwrap();
            AaveEventType::ReserveUsedAsCollateralDisabled(reserve_disable_event)
        }
        AaveUserEvent::ReserveUsedAsCollateralEnabled => {
            let reserve_enable_event =
                decode_reserve_used_as_colladeral_event::<ReserveUsedAsCollateralEnabledEvent>(log)
                    .unwrap();
            AaveEventType::ReserveUsedAsCollateralEnabled(reserve_enable_event)
        }
        _ => AaveEventType::Unknown,
    }
}

pub fn decode_borrow_event(log: &Log) -> Result<BorrowEvent, Box<dyn std::error::Error>> {
    if log.topics.len() < 4 {
        // Check the number of indexed parameters
        return Err("Incorrect number of topics for Borrow event".into());
    }

    let reserve: Address = log.topics[1].into();
    let on_behalf_of: Address = log.topics[2].into();
    let referral_code = U256::from_big_endian(log.topics[3].as_bytes());
    let referral_code: u16 = referral_code
        .low_u64()
        .try_into()
        .map_err(|_| "Referral code is too large for u16")?;

    // Assuming the data contains the rest in order: user, amount, interestRateMode, borrowRate
    // Proceed with decoding data which is just raw binary (not RLP encoded)
    let raw_log: RawLog = RawLog::from(log.clone());
    let data_slice = raw_log.data;
    if data_slice.len() < 128 {
        return Err("Data field too short to decode all fields".into());
    }
    // Extract the Address directly from data slice assuming first 20 bytes are the address
    let user = Address::from_slice(&data_slice[12..32]);
    let amount = U256::from_big_endian(&data_slice[32..64]);
    let interest_rate_mode = data_slice[95]; // assuming 1 byte for interest rate mode
    let borrow_rate = U256::from_big_endian(&data_slice[96..128]);

    let borrow_event = BorrowEvent {
        reserve,
        user,
        on_behalf_of,
        amount,
        interest_rate_mode,
        borrow_rate,
        referral_code,
    };

    Ok(borrow_event)
}

pub fn decode_repay_event(log: &Log) -> Result<RepayEvent, Box<dyn std::error::Error>> {
    if log.topics.len() < 3 {
        return Err("Must have 3 topics for Repay Event".into());
    }

    let reserve: Address = log.topics[1].into();
    let user: Address = log.topics[2].into();
    let repayer: Address = log.topics[3].into();

    if log.data.len() < 64 {
        // Check sufficient data length for amount (32 bytes) + bool (1 byte)
        return Err("Data slice too short".into());
    }

    let data_slice = log.data.as_ref();
    let amount = U256::from_big_endian(&data_slice[0..32]);
    let use_a_tokens: bool = data_slice[63] != 0;

    let repay_event = RepayEvent {
        reserve,
        user,
        repayer,
        amount,
        use_a_tokens,
    };

    Ok(repay_event)
}

pub fn decode_withdraw_event(log: &Log) -> Result<WithdrawEvent, Box<dyn std::error::Error>> {
    if log.topics.len() < 3 {
        return Err("Must have 3 topics for WithDraw Event".into());
    }

    let reserve: Address = log.topics[1].into();
    let user: Address = log.topics[2].into();
    let to: Address = log.topics[3].into();

    if log.data.len() < 32 {
        // Check sufficient data length for amount (32 bytes) + bool (1 byte)
        return Err("Data slice too short".into());
    }

    let data_slice = log.data.as_ref();
    let amount = U256::from_big_endian(&data_slice[0..32]);

    let withdraw_event = WithdrawEvent {
        reserve,
        user,
        to,
        amount,
    };

    Ok(withdraw_event)
}

pub fn decode_supply_event(log: &Log) -> Result<SupplyEvent, Box<dyn std::error::Error>> {
    if log.topics.len() < 3 {
        // Check the number of indexed parameters
        return Err("Incorrect number of topics for Borrow event".into());
    }

    let reserve: Address = log.topics[1].into();
    let on_behalf_of: Address = log.topics[2].into();
    let referral_code = U256::from_big_endian(log.topics[3].as_bytes());
    let referral_code: u16 = referral_code
        .low_u64()
        .try_into()
        .map_err(|_| "Referral code is too large for u16")?;

    let data_slice = log.data.as_ref();
    if data_slice.len() < 64 {
        return Err("Data slice too short for extracting u16".into());
    }

    // Assuming the data contains the rest in order: user, amount, interestRateMode, borrowRate
    let user = Address::from_slice(&data_slice[12..32]);
    let amount = U256::from_big_endian(&data_slice[44..64]);

    Ok(SupplyEvent {
        reserve,
        user,
        on_behalf_of,
        amount,
        referral_code,
    })
}

pub fn decode_reserve_used_as_colladeral_event<T: ReserveCollateralEvent>(
    log: &Log,
) -> Result<T, Box<dyn std::error::Error>> {
    if log.topics.len() < 3 {
        return Err("Must have 3 topics for Repay Event".into());
    }

    let reserve: Address = log.topics[1].into();
    let user: Address = log.topics[2].into();

    let event = T::new(reserve, user);

    Ok(event)
}
