use crate::data::address::CONTRACT;
use crate::data::erc20::u256_to_big_decimal;
use crate::data::token_data_hash::set_token_interest_rates;
use crate::exchanges::aave_v3::implementations::aave_user_data::UpdateUserData;
use crate::exchanges::aave_v3::{
    decode_events::create_aave_event_from_log,
    events::{AaveEvent, AaveEventType, AaveUserEvent},
    implementations::aave_users_hash::UpdateUsers,
    update_user::{get_user_action_from_event, TokenToRemove, Update},
    user_structs::{AaveUsersHash, PricingSource},
};
use crate::interest::calculate_interest::TokenRates;
use anyhow::Result;
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use ethers::{
    abi::Address,
    core::types::{Filter, Log},
    providers::Ws,
};
use ethers::{prelude::*, utils::keccak256};
use log::{debug, error};
use num_bigint::BigInt;
use std::collections::HashMap;
use std::sync::Arc;

use crate::exchanges::aave_v3::events::{
    LiquidationEvent, ReserveDataUpdatedEvent, BORROW_SIGNATURE, LIQUIDATION_SIGNATURE,
    REPAY_SIGNATURE, RESERVE_DATA_SIGNATURE, RESERVE_USED_AS_COLLATERAL_DISABLED_SIGNATURE,
    RESERVE_USED_AS_COLLATERAL_ENABLED_SIGNATURE, SUPPLY_SIGNATURE, WITHDRAW_SIGNATURE,
};

pub async fn update_users_with_event_from_log(
    log: Log,
    users: &mut AaveUsersHash,
    client: &Arc<Provider<Ws>>,
) -> Result<()> {
    let aave_event_map = setup_event_map();

    if !log.topics.is_empty() {
        //determine which aave event was found
        if let Some(aave_event_enum) = aave_event_map.get(&log.topics[0]) {
            // debug!("{:?} event", aave_event_enum);

            // extract event data from log
            let aave_event_type_with_data = create_aave_event_from_log(*aave_event_enum, &log);
            // debug!("event data => {:?}", aave_event_type_with_data);

            match *aave_event_enum {
                AaveUserEvent::Liquidation => {
                    if let AaveEventType::LiquidationEvent(event) = aave_event_type_with_data {
                        update_aave_liquidated_user(users, event, client).await?;
                    }
                }
                AaveUserEvent::ReserveDataUpdated => {
                    if let AaveEventType::ReserveDataUpdated(event) = aave_event_type_with_data {
                        update_aave_token_interest_rates(event).await?
                    }
                }
                _ => {
                    // extract struct data from event enum
                    let event =
                        extract_aave_event_data(&aave_event_type_with_data).unwrap_or_else(|err| {
                            panic!(
                                "count not extract data from event {:#?} with error {}",
                                aave_event_type_with_data, err
                            );
                        });
                    // update aave user
                    update_aave_user(users, event, client).await?;
                }
            }

            // if event is LIQUATION handle separately (see else)
            // if *aave_event_enum != AaveUserEvent::Liquidation
            //     && **aave_event_enum != AaveUserEvent::ReserveDataUpdated
            // {
            //     // extract struct data from event enum
            //     let event =
            //         extract_aave_event_data(&aave_event_type_with_data).unwrap_or_else(|err| {
            //             panic!(
            //                 "count not extract data from event {:#?} with error {}",
            //                 aave_event_type_with_data, err
            //             );
            //         });
            //
            //     // update aave user
            //     update_aave_user(users, event, client).await?;
            // } else if let AaveEventType::LiquidationEvent(event) = aave_event_type_with_data {
            //     update_aave_liquidated_user(users, event, client).await?;
            // }
        }
    }
    Ok(())
}

pub fn set_aave_event_signature_filter() -> Result<Filter> {
    let aave_v3_pool_address = CONTRACT.get_address().aave_v3_pool.clone();

    let filter = Filter::new()
        .address(aave_v3_pool_address.parse::<Address>()?)
        .events(
            [
                WITHDRAW_SIGNATURE,
                RESERVE_DATA_SIGNATURE,
                BORROW_SIGNATURE,
                REPAY_SIGNATURE,
                SUPPLY_SIGNATURE,
                RESERVE_USED_AS_COLLATERAL_ENABLED_SIGNATURE,
                RESERVE_USED_AS_COLLATERAL_DISABLED_SIGNATURE,
                LIQUIDATION_SIGNATURE,
            ]
            .to_vec(),
        );
    Ok(filter)
}

fn setup_event_map() -> HashMap<H256, AaveUserEvent> {
    let mut event_map = HashMap::new();
    let borrow_hash: H256 = H256::from(keccak256(BORROW_SIGNATURE.as_bytes()));
    let reserve_data_hash: H256 = H256::from(keccak256(RESERVE_DATA_SIGNATURE.as_bytes()));
    let withdraw_hash: H256 = H256::from(keccak256(WITHDRAW_SIGNATURE.as_bytes()));
    let repay_hash: H256 = H256::from(keccak256(REPAY_SIGNATURE.as_bytes()));
    let supply_hash: H256 = H256::from(keccak256(SUPPLY_SIGNATURE.as_bytes()));
    let reserve_enable_hash: H256 =
        keccak256(RESERVE_USED_AS_COLLATERAL_ENABLED_SIGNATURE.as_bytes()).into();
    let reserve_disabled_hash: H256 =
        keccak256(RESERVE_USED_AS_COLLATERAL_DISABLED_SIGNATURE.as_bytes()).into();
    let liquidation_hash: H256 = keccak256(LIQUIDATION_SIGNATURE.as_bytes()).into();

    // TODO - add reserve_data_hash
    event_map.insert(withdraw_hash, AaveUserEvent::WithDraw);
    event_map.insert(liquidation_hash, AaveUserEvent::Liquidation);
    event_map.insert(borrow_hash, AaveUserEvent::Borrow);
    event_map.insert(repay_hash, AaveUserEvent::Repay);
    event_map.insert(supply_hash, AaveUserEvent::Supply);
    event_map.insert(reserve_data_hash, AaveUserEvent::ReserveDataUpdated);
    event_map.insert(
        reserve_enable_hash,
        AaveUserEvent::ReserveUsedAsCollateralEnabled,
    );
    event_map.insert(
        reserve_disabled_hash,
        AaveUserEvent::ReserveUsedAsCollateralDisabled,
    );
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
        AaveEventType::ReserveUsedAsCollateralDisabled(event) => Ok(Box::new(*event)),
        AaveEventType::ReserveUsedAsCollateralEnabled(event) => Ok(Box::new(*event)),
        _ => Err("Unhandled event type".into()),
    }
}

pub async fn update_aave_user(
    users: &mut AaveUsersHash,
    event: Box<dyn AaveEvent>,
    client: &Arc<Provider<Ws>>,
) -> Result<()> {
    let user_address = event.get_user();
    let user_action = get_user_action_from_event(event, client).await?;

    if users.user_data.contains_key(&user_address) {
        let user = users.user_data.get_mut(&user_address).unwrap();
        let user_id = user.id;

        debug!("updating user {}", user_id.to_string());
        // debug!("user debt ...{:?}", user.total_debt.with_scale(SCALE));
        // debug!(
        //     "user a scaled collateral...{:?}",
        //     user.collateral_times_liquidation_factor.with_scale(SCALE)
        // );
        // debug!(
        //     "user health factor...{:?}",
        //     user.health_factor.with_scale(SCALE)
        // );

        let token_to_remove = match user.update(&user_action, client).await {
            Ok(remove_token) => match remove_token {
                TokenToRemove::TokenToRemove(token_address) => {
                    // there is a token to remove
                    let token_address: Address = token_address.parse()?;
                    Some(token_address)
                }
                TokenToRemove::None => None, // no token to remove
            },
            Err(err) => return Err(err),
        };

        debug!("user updated!");

        user.update_meta_data(PricingSource::SavedTokenPrice, client)
            .await?;

        // debug!(
        //     "updated user debt ...{:?}",
        //     user.total_debt.with_scale(SCALE)
        // );
        // debug!(
        //     "updated user scaled collateral...{:?}",
        //     user.collateral_times_liquidation_factor.with_scale(SCALE)
        // );
        // debug!(
        //     "updated user health factor...{:?}",
        //     user.health_factor.with_scale(SCALE)
        // );

        // update token => user mappings , includes adding new tokens
        users
            .update_token_user_mapping_for_(user_id, client)
            .await?;

        // check if there is token to remove, if so removie it
        if let Some(token_address) = token_to_remove {
            users
                .remove_user_from_token_user_mapping(user_id, token_address)
                .unwrap_or_else(|err| {
                    error!("could not remove user from token mapping => {}", err)
                });
        };
        return Ok(());
    } else {
        // add new user @ user_address since not in our database
        users.add_new_user(user_address, client).await?;
    }

    Ok(())
}

pub async fn update_aave_liquidated_user(
    users: &mut AaveUsersHash,
    event: LiquidationEvent,
    client: &Arc<Provider<Ws>>,
) -> Result<()> {
    let user_address = event.get_user();

    if users.user_data.contains_key(&user_address) {
        let user = users.user_data.get_mut(&user_address).unwrap();
        let user_id = user.id;

        debug!("updating user {} post liquidation", user_id.to_string());
        debug!("user debt ...{:?}", user.total_debt);
        debug!(
            "user a scaled collateral...{:?}",
            user.collateral_times_liquidation_factor
        );
        debug!("user health factor...{:?}", user.health_factor);

        user.liquidate(event)?;

        debug!("user liquidated!");

        user.update_meta_data(PricingSource::SavedTokenPrice, client)
            .await?;
        debug!("updated user debt ...{:?}", user.total_debt);
        debug!(
            "updated user scaled collateral...{:?}",
            user.collateral_times_liquidation_factor
        );
        debug!("updated user health factor...{:?}", user.health_factor);

        // update token => user mappings , includes adding new tokens
        users
            .update_token_user_mapping_for_(user_id, client)
            .await?;

        return Ok(());
    } else {
        // add new user @ user_address since not in our database
        users.add_new_user(user_address, client).await?;
    }

    Ok(())
}

pub async fn update_aave_token_interest_rates(event: ReserveDataUpdatedEvent) -> Result<()> {
    let token_address = event.get_reserve();
    let rate_scale = BigDecimal::new(BigInt::from(1), -27);

    let variable_borrow_rate = &u256_to_big_decimal(&event.variable_borrow_rate) / &rate_scale;
    let stable_borrow_rate = &u256_to_big_decimal(&event.stable_borrow_rate) / &rate_scale;
    let liquidity_rate = &u256_to_big_decimal(&event.liquidity_rate) / &rate_scale;

    let updated_interest_rates = TokenRates {
        variable_borrow_rate: variable_borrow_rate.to_f64().unwrap(),
        stable_borrow_rate: stable_borrow_rate.to_f64().unwrap(),
        liquidity_rate: liquidity_rate.to_f64().unwrap(),
    };

    set_token_interest_rates(token_address, updated_interest_rates).await?;

    Ok(())
}
