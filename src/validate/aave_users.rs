use crate::{
    abi::{aave_v3_data_provider::AAVE_V3_DATA_PROVIDER, aave_v3_pool::AAVE_V3_POOL},
    data::{
        address::CONTRACT,
        erc20::u256_to_big_decimal,
        token_data_hash::{get_token_data, get_unique_token_data},
        token_price_hash::get_saved_token_price,
        users_to_track::{get_tracked_users, reset_tracked_users},
    },
    exchanges::aave_v3::user_structs::{
        AaveTokenU256, LiquidationArgs, LiquidationCandidate, LiquidationCloseFactor, BPS_FACTOR,
        CLOSE_FACTOR_HF_THRESHOLD, LIQUIDATION_THRESHOLD,
    },
    utils::type_conversion::{address_to_string, f64_to_u256, u256_to_f64},
};

use anyhow::{anyhow, Result};
use bigdecimal::{BigDecimal, FromPrimitive};
use colored::*;
use ethers::{
    providers::{Middleware, Provider, Ws},
    types::{Address, U256},
};
use log::{debug, info};
use num_traits::ToPrimitive;
use std::sync::Arc;

pub async fn validate_liquidation_candidates(client: &Arc<Provider<Ws>>) -> Result<()> {
    let mut validation_count: u16 = 0;
    let aave_v3_pool_address: Address = CONTRACT.get_address().aave_v3_pool.parse()?;
    let aave_v3_pool = AAVE_V3_POOL::new(aave_v3_pool_address, client.clone());
    let standard_scale = &BigDecimal::from_u64(10_u64.pow(18)).unwrap();
    let token_data = get_token_data().await?;

    let eth_token = token_data
        .get("WETH")
        .unwrap_or_else(|| panic!("could not find WETH token"));
    let eth_price_usd = get_saved_token_price(eth_token.address.to_lowercase()).await?;
    let eth_price_usd = &BigDecimal::from_f64(eth_price_usd).unwrap();

    let user_liquidation_candidates = get_tracked_users().await?;
    if user_liquidation_candidates.is_empty() {
        return Ok(());
    }

    // RESETs
    reset_tracked_users().await?; // clear all tracked users

    for user in &user_liquidation_candidates {
        let LiquidationCandidate {
            user_id,
            estimated_profit,
            debt_token,
            collateral_token,
        } = user.clone();

        let health_factor = match aave_v3_pool.get_user_account_data(user_id).call().await {
            Ok((_, _, _, _, _, health_factor)) => health_factor,
            Err(err) => {
                debug!("error fetching usr health factor => {}", err);
                U256::zero()
            }
        };

        let health_factor = u256_to_big_decimal(&health_factor) / standard_scale;
        let health_factor = health_factor.to_f64().unwrap();

        if health_factor < LIQUIDATION_THRESHOLD {
            validation_count += 1;

            let (liquidation_args, profit_scaled) =
                calculate_user_liquidation_usd_profit(&user_id, health_factor, client).await?;

            let user_id_string = address_to_string(liquidation_args.user);
            let debt = address_to_string(liquidation_args.debt);
            let collateral = address_to_string(liquidation_args.collateral);
            let profit = u256_to_big_decimal(&profit_scaled) / standard_scale;
            let profit = profit.to_f64().unwrap();

            info!(
                "user {} liquidation ready /w health score of {}",
                user_id_string.green().bold(),
                format!("{:2}", health_factor).red()
            );
            info!(
                "liquidation profit is {}, with debt {} and collateral {}",
                format!("{:2}", profit).green().bold(),
                debt.yellow().bold(),
                collateral.black().bold(),
            );

            info!(
                "estimated profit for this was {}",
                format!("{:2}", estimated_profit).green().bold(),
            );

            // validate that same collateral and debt tokens as
            // found in prevous block
            if liquidation_args.debt != debt_token {
                debug!("debt tokens do not match, expected => {}", debt_token);
            }

            if liquidation_args.collateral != collateral_token {
                debug!(
                    "collateral token do not match, expected => {}",
                    collateral_token
                );
            }

            if profit > 0.0 {
                let gas_cost = calculate_gas_cost(client).await?;
                let gas_cost = u256_to_big_decimal(&gas_cost);

                let gas_cost_usd = gas_cost * eth_price_usd / standard_scale;

                info!(
                    "gas cost $ is {}",
                    format!("{:2}", gas_cost_usd).red().bold(),
                );
            }
        } else {
            info!(
                "user {} is health score is too high => {}",
                user_id, health_factor
            );
        }
    }

    info!(
        "Out of {} user, {} are ready for liquidation",
        user_liquidation_candidates.len(),
        validation_count
    );

    Ok(())
}

pub async fn calculate_user_liquidation_usd_profit(
    user_id: &Address,
    health_factor: f64,
    client: &Arc<Provider<Ws>>,
) -> Result<(LiquidationArgs, U256)> {
    let standard_scale = U256::exp10(18);
    let aave_v3_data_provider_address: Address =
        CONTRACT.get_address().aave_v3_data_provider.parse()?;
    let unique_token_data = get_unique_token_data().await?;
    let bps_factor = U256::from(BPS_FACTOR as u64);

    let mut liquidation_args = LiquidationArgs {
        collateral: Address::zero(),
        debt: Address::zero(),
        user: *user_id,
        debt_to_cover: U256::from(0),
        _receive_a_token: false,
    };

    if health_factor >= LIQUIDATION_THRESHOLD {
        return Ok((liquidation_args, U256::from(0)));
    }

    let liquidation_factor = if health_factor < CLOSE_FACTOR_HF_THRESHOLD {
        LiquidationCloseFactor::Half
    } else {
        LiquidationCloseFactor::Full
    };

    let aave_v3_data_pool =
        AAVE_V3_DATA_PROVIDER::new(aave_v3_data_provider_address, client.clone());

    let liquidation_close_factor_scaled = match liquidation_factor {
        LiquidationCloseFactor::Full => standard_scale,
        LiquidationCloseFactor::Half => U256::from(5) * U256::exp10(17), // 0.5 scaled
    };

    let mut tokens = Vec::new();
    let mut highest_token_debt = U256::from(0);
    let mut highest_token_decimal_factor = U256::from(0);
    let mut highest_token_price = U256::from(0);
    let mut maximum_profit = U256::from(0);

    for token in unique_token_data.values() {
        let token_address = token.address.parse()?;
        let decimal_factor = U256::exp10(token.decimals.into());

        let (a_token_balance, stable_debt, variable_debt, _, _, _, _, _, use_as_collateral) =
            match aave_v3_data_pool
                .get_user_reserve_data(token_address, *user_id)
                .call()
                .await
            {
                Ok(result) => result,
                Err(_) => {
                    debug!("could not retrieve user data using aave v3 pool");
                    return Ok((liquidation_args, U256::from(0)));
                }
            };

        let total_debt = stable_debt + variable_debt;

        if total_debt > U256::from(0) || a_token_balance > U256::from(0) {
            tokens.push(AaveTokenU256 {
                token: token.clone(),
                current_total_debt: total_debt,
                usage_as_collateral_enabled: use_as_collateral,
                current_atoken_balance: a_token_balance,
                reserve_liquidation_bonus: U256::from(token.liquidation_bonus),
                reserve_liquidation_threshold: U256::from(token.liquidation_threshold),
            })
        }

        if total_debt > highest_token_debt {
            highest_token_debt = total_debt;

            let token_price = get_saved_token_price(token.address.to_lowercase()).await?;
            // let token_price = big_decimal_to_u256_scaled(&token_price).unwrap(); // price times 10^18
            let token_price = f64_to_u256(token_price).unwrap();
            highest_token_price = token_price;
            highest_token_decimal_factor = decimal_factor;

            // let debt_to_cover =
            //     highest_token_debt / decimal_factor * liquidation_close_factor_scaled * token_price
            //         / standard_scale;

            let debt_to_cover = highest_token_debt
                .checked_mul(liquidation_close_factor_scaled)
                .ok_or_else(|| anyhow!("overflow!"))?;

            let debt_to_cover = debt_to_cover
                .checked_div(standard_scale)
                .ok_or_else(|| anyhow!("overflow or div by zero!"))?;

            // let debt_to_cover = debt_to_cover
            //     .checked_mul(token_price)
            //     .ok_or("looks like overflow")?;
            //
            // let debt_to_cover = debt_to_cover
            //     .checked_div(standard_scale)
            //     .ok_or("could be overflow or div by zero")?;

            liquidation_args = LiquidationArgs {
                debt: token_address,
                debt_to_cover,
                ..liquidation_args
            }
        }
    }

    // now loop through to get find optimal liquidation combo
    for token in tokens {
        let liquidation_bonus = token.reserve_liquidation_bonus;
        let debt_to_cover_in_usd_scaled = liquidation_args.debt_to_cover;
        // let a_token_balance = token.current_atoken_balance;
        // let decimal_factor = U256::exp10(token.token.decimals.into());

        // calculate profit
        // profit = debtToCover$ * liquidaitonBonus * (liquidationBonus - 1)
        // let profit_usd_scaled = debt_to_cover_in_usd_scaled * liquidation_bonus / bps_factor
        //     * (liquidation_bonus - bps_factor)
        //     / bps_factor
        //     * a_token_balance
        //     / decimal_factor;

        if liquidation_bonus > U256::from(0) && token.usage_as_collateral_enabled {
            let profit_usd_scaled = debt_to_cover_in_usd_scaled
                .checked_mul(highest_token_price)
                .ok_or_else(|| anyhow!("profit calc overflow"))?;

            let profit_usd_scaled = profit_usd_scaled
                .checked_div(highest_token_decimal_factor)
                .ok_or_else(|| anyhow!("profit overflow div by zero"))?;

            let profit_usd_scaled = profit_usd_scaled
                .checked_mul(liquidation_bonus)
                .ok_or_else(|| anyhow!("profit overflow liquidation bonus"))?;

            let bonus_minus_one = liquidation_bonus
                .checked_sub(bps_factor)
                .ok_or_else(|| anyhow!("additionl error"))?;

            let profit_usd_scaled = profit_usd_scaled
                .checked_mul(bonus_minus_one)
                .ok_or_else(|| anyhow!("profit overflow liquidation bonus minus one"))?;

            let profit_usd_scaled = profit_usd_scaled
                .checked_div(bps_factor)
                .ok_or_else(|| anyhow!("profit overflow div bps factor 1"))?;

            let profit_usd_scaled = profit_usd_scaled
                .checked_div(bps_factor)
                .ok_or_else(|| anyhow!("profit overflow div bps factor 2"))?;

            if profit_usd_scaled > maximum_profit {
                maximum_profit = profit_usd_scaled;

                let token_address = token.token.address.parse()?;

                liquidation_args = LiquidationArgs {
                    collateral: token_address,
                    ..liquidation_args
                }
            }
        }
    }

    Ok((liquidation_args, maximum_profit))
}

pub async fn calculate_gas_cost(client: &Arc<Provider<Ws>>) -> Result<U256> {
    // ESTIMATING GAS FOR BELOW CALC
    // match aave_v3_pool
    //     .liquidation_call(
    //         liquidation_args.collateral,
    //         liquidation_args.debt,
    //         liquidation_args.user,
    //         liquidation_args.debt_to_cover,
    //         liquidation_args.receive_a_token,
    //     )
    //     .estimate_gas()
    //     .await
    // {
    //     Ok(estimated_gas) => estimated_gas_cost = estimated_gas,
    //     Err(e) => {
    //         error!("Error estimating gas => {:?}", e);
    //     }
    // }

    let estimated_gas_cost = U256::from(1_400_000);

    let gas_price = client.get_gas_price().await?;

    // info!(
    //     "gas cost => {}, gas price => {}",
    //     estimated_gas_cost, gas_price
    // );

    let total_cost = estimated_gas_cost
        .checked_mul(gas_price)
        .ok_or_else(|| anyhow!("overflow calculating total cost"))?;

    // let eth_cost = format_units(total_cost, 18)?;
    // info!("cost in eth of liquidation call is {}", eth_cost);

    Ok(total_cost)
}

pub fn big_decimal_to_u256_scaled(value: &BigDecimal) -> Result<U256, &'static str> {
    // Convert BigDecimal to BigInt
    let value: f64 = value.to_f64().unwrap();
    let scaling_factor = 10u128.pow(18);
    let scaled_value = (value * scaling_factor as f64).round() as u128;
    Ok(U256::from(scaled_value))
}
