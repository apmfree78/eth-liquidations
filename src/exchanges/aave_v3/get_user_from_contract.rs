use super::implementations::aave_user_data::{HealthFactor, UpdateUserData};
use super::user_structs::{AaveToken, AaveUserData, PricingSource, BPS_FACTOR};
use crate::abi::aave_v3_data_provider::AAVE_V3_DATA_PROVIDER;
use crate::data::address::CONTRACT;
use crate::data::erc20::u256_to_big_decimal;
use crate::data::token_data_hash::get_unique_token_data;
use anyhow::{anyhow, Result};
use bigdecimal::{BigDecimal, ToPrimitive};
use ethers::providers::{Provider, Ws};
use ethers::types::Address;
use std::sync::Arc;

pub async fn get_aave_v3_user_from_data_provider(
    user_address: Address,
    client: &Arc<Provider<Ws>>,
) -> Result<AaveUserData> {
    let aave_v3_data_pool_address: Address =
        CONTRACT.get_address().aave_v3_data_provider.parse()?;
    let aave_v3_data_pool = AAVE_V3_DATA_PROVIDER::new(aave_v3_data_pool_address, client.clone());
    let unique_token_data = get_unique_token_data().await?;

    let mut tokens = Vec::new();

    for token in unique_token_data.values() {
        let token_address = token.address.parse()?;
        let decimal_factor = BigDecimal::from(10_u64.pow(token.decimals.into()));

        let (a_token_balance, stable_debt, variable_debt, _, _, _, _, _, use_as_collateral) =
            aave_v3_data_pool
                .get_user_reserve_data(token_address, user_address)
                .call()
                .await?;

        let stable_debt = u256_to_big_decimal(&stable_debt) / &decimal_factor;
        let variable_debt = u256_to_big_decimal(&variable_debt) / &decimal_factor;
        let a_token_balance = u256_to_big_decimal(&a_token_balance) / &decimal_factor;

        if &stable_debt + &variable_debt > BigDecimal::from(0)
            || a_token_balance > BigDecimal::from(0)
        {
            tokens.push(AaveToken {
                token: token.clone(),
                current_variable_debt: variable_debt.to_f64().unwrap(),
                current_stable_debt: stable_debt.to_f64().unwrap(),
                usage_as_collateral_enabled: use_as_collateral,
                current_atoken_balance: a_token_balance.to_f64().unwrap(),
                reserve_liquidation_bonus: token.liquidation_bonus as f64 / BPS_FACTOR as f64,
                reserve_liquidation_threshold: token.liquidation_threshold as f64
                    / BPS_FACTOR as f64,
            })
        }
    }

    let mut user_data = AaveUserData {
        id: user_address,
        total_debt: 0_f64, //placeholder value , will calculate below
        collateral_times_liquidation_factor: 0_f64, //placeholder value , will calculate below
        tokens,
        health_factor: 0_f64, //placeholder value , will calculate below
    };

    user_data
        .update_meta_data(PricingSource::AaveOracle, client)
        .await?;

    if user_data.total_debt == 0.0 {
        return Err(anyhow!("user has no debt"));
    }

    if user_data
        .is_user_valid_when_checking_against_official_health_factor(client)
        .await?
    {
        Ok(user_data)
    } else {
        Err(anyhow!(
            "user calculated health factor does not match official health factor"
        ))
    }
}
