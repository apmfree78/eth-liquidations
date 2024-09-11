use super::implementations::aave_user_data::{HealthFactor, UpdateUserData};
use super::user_structs::{AaveToken, AaveUserData, PricingSource};
use crate::abi::aave_v3_data_provider::AAVE_V3_DATA_PROVIDER;
use crate::data::address::CONTRACT;
use crate::data::erc20::u256_to_big_decimal;
use crate::data::token_data_hash::get_unique_token_data;
use anyhow::{anyhow, Result};
use bigdecimal::BigDecimal;
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

        let (a_token_balance, stable_debt, variable_debt, _, _, _, _, _, use_as_collateral) =
            aave_v3_data_pool
                .get_user_reserve_data(token_address, user_address)
                .call()
                .await?;

        let total_debt = stable_debt + variable_debt;
        let total_debt = u256_to_big_decimal(&total_debt);
        let a_token_balance = u256_to_big_decimal(&a_token_balance);

        if total_debt > BigDecimal::from(0) || a_token_balance > BigDecimal::from(0) {
            tokens.push(AaveToken {
                token: token.clone(),
                current_total_debt: total_debt,
                usage_as_collateral_enabled: use_as_collateral,
                current_atoken_balance: a_token_balance,
                reserve_liquidation_bonus: BigDecimal::from(token.liquidation_bonus),
                reserve_liquidation_threshold: BigDecimal::from(token.liquidation_threshold),
            })
        }
    }

    let mut user_data = AaveUserData {
        id: user_address,
        total_debt: BigDecimal::from(0), //placeholder value , will calculate below
        collateral_times_liquidation_factor: BigDecimal::from(0), //placeholder value , will calculate below
        tokens,
        health_factor: BigDecimal::from(0), //placeholder value , will calculate below
    };

    user_data
        .update_meta_data(PricingSource::AaveOracle, client)
        .await?;

    if user_data.total_debt == BigDecimal::from(0) {
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
