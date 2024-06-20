use crate::{
    abi::aave_v3_pool::AAVE_V3_POOL,
    data::{
        address::AAVE_V3_POOL_ADDRESS,
        erc20::u256_to_big_decimal,
        users_to_track::{get_tracked_users, reset_tracked_users},
    },
    exchanges::aave_v3::user_structs::LIQUIDATION_THRESHOLD,
    utils::type_conversion::address_to_string,
};
use bigdecimal::{BigDecimal, FromPrimitive};
use colored::*;
use ethers::providers::{Provider, Ws};
use log::debug;
use std::sync::Arc;

pub async fn validate_liquidation_candidates(
    client: &Arc<Provider<Ws>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut validation_count: u16 = 0;
    let aave_v3_pool = AAVE_V3_POOL::new(*AAVE_V3_POOL_ADDRESS, client.clone());
    let standard_scale = BigDecimal::from_u64(10_u64.pow(18)).unwrap();

    let user_liquidation_candidates = get_tracked_users().await?;
    if user_liquidation_candidates.is_empty() {
        return Ok(());
    }
    reset_tracked_users().await?; // clear all tracked users

    for user_id in &user_liquidation_candidates {
        let (_, _, _, _, _, health_factor) =
            aave_v3_pool.get_user_account_data(*user_id).call().await?;

        let health_factor = u256_to_big_decimal(&health_factor) / &standard_scale;

        if health_factor < BigDecimal::from_f32(LIQUIDATION_THRESHOLD).unwrap() {
            validation_count += 1;

            let user_id_string = address_to_string(*user_id);
            debug!(
                "user {} is ready for liquidation with health score of {}",
                user_id_string.green().bold(),
                format!("{:2}", health_factor.with_scale(2)).red()
            );
        } else {
            debug!(
                "user {} is health score is too high => {}",
                user_id,
                health_factor.with_scale(2)
            );
        }
    }

    debug!(
        "Out of {} user, {} are ready for liquidation",
        user_liquidation_candidates.len(),
        validation_count
    );

    Ok(())
}
