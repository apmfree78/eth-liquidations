#[path = "./mocks/generate_logs.rs"]
mod generate_logs;

use eth_liquadation::exchanges::aave_v3::decode_events::{
    decode_borrow_event, decode_repay_event, decode_reserve_used_as_colladeral_event,
    decode_supply_event, decode_withdraw_event,
};
use eth_liquadation::exchanges::aave_v3::events::{
    BorrowEvent, RepayEvent, ReserveUsedAsCollateralDisabledEvent,
    ReserveUsedAsCollateralEnabledEvent, SupplyEvent, WithdrawEvent,
};
use ethers::core::types::U256;

use crate::generate_logs::{
    create_log_for_borrow_event, create_log_for_collateral_disable_event,
    create_log_for_collateral_enable_event, create_log_for_repay_event,
    create_log_for_supply_event, create_log_for_withdraw_event,
};

const AAVE_V3_POOL: &str = "0x87870Bca3F3fD6335C3F4ce8392D69350B4fA4E2";

#[test]
fn test_repay_event_decoding() -> Result<(), Box<dyn std::error::Error>> {
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

    let log = create_log_for_repay_event(&repay_event, AAVE_V3_POOL);

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

#[test]
fn test_borrow_event_decoding() -> Result<(), Box<dyn std::error::Error>> {
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

    // create mock log
    let log = create_log_for_borrow_event(&borrow_event, AAVE_V3_POOL);

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

#[test]
fn test_supply_event_decoding() -> Result<(), Box<dyn std::error::Error>> {
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

    let log = create_log_for_supply_event(&supply_event, AAVE_V3_POOL);

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

#[test]
fn test_withdraw_event_decoding() -> Result<(), Box<dyn std::error::Error>> {
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

    let log = create_log_for_withdraw_event(&withdraw_event, AAVE_V3_POOL);

    // decode log
    let decoded_withdraw_event = decode_withdraw_event(&log).unwrap();

    // test originala event matches decoded
    assert_eq!(withdraw_event.reserve, decoded_withdraw_event.reserve);
    assert_eq!(withdraw_event.user, decoded_withdraw_event.user);
    assert_eq!(withdraw_event.to, decoded_withdraw_event.to);
    assert_eq!(withdraw_event.amount, decoded_withdraw_event.amount);

    Ok(())
}

#[test]
fn test_collateral_enable_event_decoding() -> Result<(), Box<dyn std::error::Error>> {
    let event = ReserveUsedAsCollateralEnabledEvent {
        reserve: "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
            .parse()
            .unwrap(),
        user: "0x5ed5dd65ab0dc1bccc44eedaa40680c231faaa9f"
            .parse()
            .unwrap(),
    };

    let log = create_log_for_collateral_enable_event(event.clone(), AAVE_V3_POOL);

    // decode log
    let decoded_event =
        decode_reserve_used_as_colladeral_event::<ReserveUsedAsCollateralEnabledEvent>(&log)
            .unwrap();

    // test originala event matches decoded
    assert_eq!(event.reserve, decoded_event.reserve);
    assert_eq!(event.user, decoded_event.user);

    Ok(())
}

#[test]
fn test_collateral_disable_event_decoding() -> Result<(), Box<dyn std::error::Error>> {
    let event = ReserveUsedAsCollateralDisabledEvent {
        reserve: "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
            .parse()
            .unwrap(),
        user: "0x5ed5dd65ab0dc1bccc44eedaa40680c231faaa9f"
            .parse()
            .unwrap(),
    };

    let log = create_log_for_collateral_disable_event(event.clone(), AAVE_V3_POOL);

    // decode log
    let decoded_event =
        decode_reserve_used_as_colladeral_event::<ReserveUsedAsCollateralDisabledEvent>(&log)
            .unwrap();

    // test originala event matches decoded
    assert_eq!(event.reserve, decoded_event.reserve);
    assert_eq!(event.user, decoded_event.user);

    Ok(())
}
