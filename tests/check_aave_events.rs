use eth_liquadation::exchanges::aave_v3::decode_events::{
    decode_borrow_event, decode_repay_event, decode_reserve_used_as_colladeral_event,
    decode_supply_event, decode_withdraw_event,
};
use eth_liquadation::exchanges::aave_v3::events::{
    BorrowEvent, RepayEvent, ReserveCollateralEvent, ReserveUsedAsCollateralDisabledEvent,
    ReserveUsedAsCollateralEnabledEvent, SupplyEvent, WithdrawEvent,
};
use ethers::abi::Address;
use ethers::core::types::{Bytes, Log, U256};
use ethers::prelude::*;

#[tokio::test]
async fn test_repay_event_decoding() -> Result<(), Box<dyn std::error::Error>> {
    let repay_event = RepayEvent {
        reserve: "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
            .parse()
            .unwrap(),
        user: "0x5ed5dd65ab0dc1bccc44eedaa40680c231faaa9f"
            .parse()
            .unwrap(),
        repayer: "0x5ed5dd65ab0dc1bccc44eedaa40680c231faaa9f"
            .parse()
            .unwrap(),
        amount: U256::from(400000000),
        use_a_tokens: true,
    };

    let topics = generate_repay_event_topics_field(&repay_event);
    let data = generate_repay_event_log_data_field(&repay_event);

    // create mock log
    let log = Log {
        address: "0x87870bca3f3fd6335c3f4ce8392d69350b4fa4e2"
            .parse()
            .unwrap(),
        topics,
        data,
        ..Default::default()
    };

    // decode log
    let decoded_repay_event = decode_repay_event(&log).unwrap();

    // test originala event matches decoded
    assert_eq!(repay_event.reserve, decoded_repay_event.reserve);
    assert_eq!(repay_event.user, decoded_repay_event.user);
    assert_eq!(repay_event.repayer, decoded_repay_event.repayer);
    assert_eq!(repay_event.amount, decoded_repay_event.amount);
    assert_eq!(repay_event.use_a_tokens, decoded_repay_event.use_a_tokens);

    Ok(())
}

#[tokio::test]
async fn test_borrow_event_decoding() -> Result<(), Box<dyn std::error::Error>> {
    let borrow_event = BorrowEvent {
        reserve: "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
            .parse()
            .unwrap(),
        user: "0x893411580e590d62ddbca8a703d61cc4a8c7b2b9"
            .parse()
            .unwrap(),
        on_behalf_of: "0x54be3a794282c030b15e43ae2bb182e14c409c5e"
            .parse()
            .unwrap(),
        amount: "25000000000000000000".into(),
        interest_rate_mode: 2,
        borrow_rate: "2000000".into(),
        referral_code: 123,
    };

    let topics = generate_borrow_event_topics_field(&borrow_event);
    let data = generate_borrow_event_log_data_field(&borrow_event);

    // create mock log
    let log = Log {
        address: "0x87870bca3f3fd6335c3f4ce8392d69350b4fa4e2"
            .parse()
            .unwrap(),
        topics,
        data,
        ..Default::default()
    };

    // decode log
    let decoded_borrow_event = decode_borrow_event(&log).unwrap();

    // test originala event matches decoded
    assert_eq!(borrow_event.reserve, decoded_borrow_event.reserve);
    assert_eq!(borrow_event.user, decoded_borrow_event.user);
    assert_eq!(borrow_event.on_behalf_of, decoded_borrow_event.on_behalf_of);
    assert_eq!(
        borrow_event.referral_code,
        decoded_borrow_event.referral_code
    );
    assert_eq!(borrow_event.amount, decoded_borrow_event.amount);
    assert_eq!(
        borrow_event.interest_rate_mode,
        decoded_borrow_event.interest_rate_mode
    );
    assert_eq!(borrow_event.borrow_rate, decoded_borrow_event.borrow_rate);

    Ok(())
}

#[tokio::test]
async fn test_supply_event_decoding() -> Result<(), Box<dyn std::error::Error>> {
    let supply_event = SupplyEvent {
        reserve: "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
            .parse()
            .unwrap(),
        user: "0x893411580e590d62ddbca8a703d61cc4a8c7b2b9"
            .parse()
            .unwrap(),
        on_behalf_of: "0x54be3a794282c030b15e43ae2bb182e14c409c5e"
            .parse()
            .unwrap(),
        amount: "25000000000000000000".into(),
        referral_code: 123,
    };

    let topics = generate_supply_event_topics_field(&supply_event);
    let data = generate_supply_event_log_data_field(&supply_event);

    // create mock log
    let log = Log {
        address: "0x87870bca3f3fd6335c3f4ce8392d69350b4fa4e2"
            .parse()
            .unwrap(),
        topics,
        data,
        ..Default::default()
    };

    // decode log
    let decoded_supply_event = decode_supply_event(&log).unwrap();

    // test originala event matches decoded
    assert_eq!(supply_event.reserve, decoded_supply_event.reserve);
    assert_eq!(supply_event.user, decoded_supply_event.user);
    assert_eq!(supply_event.on_behalf_of, decoded_supply_event.on_behalf_of);
    assert_eq!(
        supply_event.referral_code,
        decoded_supply_event.referral_code
    );
    assert_eq!(supply_event.amount, decoded_supply_event.amount);

    Ok(())
}

#[tokio::test]
async fn test_withdraw_event_decoding() -> Result<(), Box<dyn std::error::Error>> {
    let withdraw_event = WithdrawEvent {
        reserve: "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
            .parse()
            .unwrap(),
        user: "0x5ed5dd65ab0dc1bccc44eedaa40680c231faaa9f"
            .parse()
            .unwrap(),
        to: "0x5ed5dd65ab0dc1bccc44eedaa40680c231faaa9f"
            .parse()
            .unwrap(),
        amount: U256::from(400000000),
    };

    let topics = generate_withdraw_event_topics_field(&withdraw_event);
    let data = generate_withdraw_event_log_data_field(&withdraw_event);

    // create mock log
    let log = Log {
        address: "0x87870bca3f3fd6335c3f4ce8392d69350b4fa4e2"
            .parse()
            .unwrap(),
        topics,
        data,
        ..Default::default()
    };

    // decode log
    let decoded_withdraw_event = decode_withdraw_event(&log).unwrap();

    // test originala event matches decoded
    assert_eq!(withdraw_event.reserve, decoded_withdraw_event.reserve);
    assert_eq!(withdraw_event.user, decoded_withdraw_event.user);
    assert_eq!(withdraw_event.to, decoded_withdraw_event.to);
    assert_eq!(withdraw_event.amount, decoded_withdraw_event.amount);

    Ok(())
}

#[tokio::test]
async fn test_collateral_enable_event_decoding() -> Result<(), Box<dyn std::error::Error>> {
    let event = ReserveUsedAsCollateralEnabledEvent {
        reserve: "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
            .parse()
            .unwrap(),
        user: "0x5ed5dd65ab0dc1bccc44eedaa40680c231faaa9f"
            .parse()
            .unwrap(),
    };

    let topics = generate_collateral_event_topics_field(event.clone());

    // create mock log
    let log = Log {
        address: "0x87870bca3f3fd6335c3f4ce8392d69350b4fa4e2"
            .parse()
            .unwrap(),
        topics,
        ..Default::default()
    };

    // decode log
    let decoded_event =
        decode_reserve_used_as_colladeral_event::<ReserveUsedAsCollateralEnabledEvent>(&log)
            .unwrap();

    // test originala event matches decoded
    assert_eq!(event.reserve, decoded_event.reserve);
    assert_eq!(event.user, decoded_event.user);

    Ok(())
}

#[tokio::test]
async fn test_collateral_disable_event_decoding() -> Result<(), Box<dyn std::error::Error>> {
    let event = ReserveUsedAsCollateralDisabledEvent {
        reserve: "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
            .parse()
            .unwrap(),
        user: "0x5ed5dd65ab0dc1bccc44eedaa40680c231faaa9f"
            .parse()
            .unwrap(),
    };

    let topics = generate_collateral_event_topics_field(event.clone());

    // create mock log
    let log = Log {
        address: "0x87870bca3f3fd6335c3f4ce8392d69350b4fa4e2"
            .parse()
            .unwrap(),
        topics,
        ..Default::default()
    };

    // decode log
    let decoded_event =
        decode_reserve_used_as_colladeral_event::<ReserveUsedAsCollateralDisabledEvent>(&log)
            .unwrap();

    // test originala event matches decoded
    assert_eq!(event.reserve, decoded_event.reserve);
    assert_eq!(event.user, decoded_event.user);

    Ok(())
}

fn generate_borrow_event_topics_field(&event: &BorrowEvent) -> Vec<H256> {
    let topics = vec![
        H256::from_slice(&[0u8; 32]),
        event.reserve.into(),
        event.on_behalf_of.into(),
        H256::from(u16_to_bytes_array(event.referral_code)),
    ];
    topics
}

fn generate_repay_event_topics_field(&event: &RepayEvent) -> Vec<H256> {
    let topics = vec![
        H256::from_slice(&[0u8; 32]),
        event.reserve.into(),
        event.user.into(),
        event.repayer.into(),
    ];
    topics
}

fn generate_collateral_event_topics_field<T: ReserveCollateralEvent>(event: T) -> Vec<H256> {
    let topics = vec![
        H256::from_slice(&[0u8; 32]),
        event.get_reserve().into(),
        event.get_user().into(),
    ];
    topics
}

fn generate_withdraw_event_topics_field(&event: &WithdrawEvent) -> Vec<H256> {
    let topics = vec![
        H256::from_slice(&[0u8; 32]),
        event.reserve.into(),
        event.user.into(),
        event.to.into(),
    ];
    topics
}

fn generate_supply_event_topics_field(&event: &SupplyEvent) -> Vec<H256> {
    let topics = vec![
        H256::from_slice(&[0u8; 32]),
        event.reserve.into(),
        event.on_behalf_of.into(),
        H256::from(u16_to_bytes_array(event.referral_code)),
    ];
    topics
}

fn generate_borrow_event_log_data_field(&event: &BorrowEvent) -> Bytes {
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

fn generate_repay_event_log_data_field(&event: &RepayEvent) -> Bytes {
    let amount_bit_array = u256_to_bytes_array(event.amount);
    let use_a_tokens_bit_array = boolean_to_bytes_array(event.use_a_tokens);

    let mut data = Vec::new();

    data.extend_from_slice(&amount_bit_array);
    data.extend_from_slice(&use_a_tokens_bit_array);
    data.into()
}

fn generate_supply_event_log_data_field(&event: &SupplyEvent) -> Bytes {
    let user_bit_array = address_to_bytes_array(event.user);
    let amount_bit_array = u256_to_bytes_array(event.amount);

    let mut data = Vec::new();
    data.extend_from_slice(&user_bit_array);
    data.extend_from_slice(&amount_bit_array);
    data.into()
}

fn generate_withdraw_event_log_data_field(&event: &WithdrawEvent) -> Bytes {
    let amount_bit_array = u256_to_bytes_array(event.amount);

    let mut data = Vec::new();
    data.extend_from_slice(&amount_bit_array);
    data.into()
}

fn u256_to_bytes_array(number: U256) -> [u8; 32] {
    let mut number_bytes = [0u8; 32];
    number.to_big_endian(&mut number_bytes);
    number_bytes
}

fn boolean_to_bytes_array(boolean: bool) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes[31] = if boolean { 1 } else { 0 };
    bytes
}

fn u8_to_bytes_array(value: u8) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes[31] = value;
    bytes
}

fn u16_to_bytes_array(value: u16) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes[30..32].copy_from_slice(&value.to_be_bytes());
    bytes
}

fn address_to_bytes_array(address: Address) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes[12..32].copy_from_slice(&address.as_bytes());
    bytes
}
