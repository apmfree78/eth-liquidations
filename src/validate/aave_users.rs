use crate::{
    abi::{aave_v3_data_provider::AAVE_V3_DATA_PROVIDER, aave_v3_pool::AAVE_V3_POOL},
    data::{
        address::{AAVE_V3_DATA_PROVIDER_ADDRESS, AAVE_V3_POOL_ADDRESS},
        erc20::{u256_to_big_decimal, TOKEN_DATA, UNIQUE_TOKEN_DATA},
        token_price_hash::{generate_token_price_hash, get_saved_token_price},
        users_to_track::{get_tracked_users, reset_tracked_users},
    },
    exchanges::aave_v3::user_structs::{
        AaveToken, BPS_FACTOR, CLOSE_FACTOR_HF_THRESHOLD, LIQUIDATION_THRESHOLD,
    },
    utils::type_conversion::address_to_string,
};
use bigdecimal::{BigDecimal, FromPrimitive};
use colored::*;
use ethers::{
    providers::{Middleware, Provider, Ws},
    types::{Address, U256},
    utils::format_units,
};
use log::{debug, info};
use num_bigint::BigInt;
use num_traits::Zero;
use std::sync::Arc;

#[derive(PartialEq)]
pub enum LiquidationCloseFactor {
    Full,
    Half,
}

#[derive(Debug)]
pub struct LiquidationArgs {
    collateral: Address,
    debt: Address,
    user: Address,
    debt_to_cover: BigDecimal,
    receive_a_token: bool,
}

pub async fn validate_liquidation_candidates(
    client: &Arc<Provider<Ws>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut validation_count: u16 = 0;
    let aave_v3_pool = AAVE_V3_POOL::new(*AAVE_V3_POOL_ADDRESS, client.clone());
    let standard_scale = BigDecimal::from_u64(10_u64.pow(18)).unwrap();
    let eth_token = TOKEN_DATA
        .get("WETH")
        .unwrap_or_else(|| panic!("could not find WETH token"));
    let eth_price_usd = get_saved_token_price(eth_token.address.to_lowercase()).await?;

    let user_liquidation_candidates = get_tracked_users().await?;
    if user_liquidation_candidates.is_empty() {
        return Ok(());
    }

    // RESETs
    reset_tracked_users().await?; // clear all tracked users

    for user_id in &user_liquidation_candidates {
        let (_, _, _, _, _, health_factor) =
            aave_v3_pool.get_user_account_data(*user_id).call().await?;

        let health_factor = u256_to_big_decimal(&health_factor) / &standard_scale;

        if health_factor < BigDecimal::from_f32(LIQUIDATION_THRESHOLD).unwrap() {
            validation_count += 1;

            let (liquidation_args, profit) =
                calculate_user_liquidation_usd_profit(user_id, &health_factor, client).await?;

            let user_id_string = address_to_string(liquidation_args.user);
            let debt = address_to_string(liquidation_args.debt);
            let collateral = address_to_string(liquidation_args.collateral);

            info!(
                "user {} liquidation ready /w health score of {}",
                user_id_string.green().bold(),
                format!("{:2}", health_factor.with_scale(2)).red()
            );
            info!(
                "liquidation profit is {}, with debt {} and collateral {}",
                format!("{:2}", profit.with_scale(2)).green().bold(),
                debt.yellow().bold(),
                collateral.black().bold(),
            );

            let gas_cost = calculate_gas_cost(&liquidation_args, client).await?;

            // TODO - calculated NET profit
            let gas_cost_usd = &u256_to_big_decimal(&gas_cost) * &eth_price_usd / &standard_scale;

            info!(
                "gas cost $ is {}",
                format!("{:2}", gas_cost_usd.with_scale(2)).red().bold(),
            );
        } else {
            info!(
                "user {} is health score is too high => {}",
                user_id,
                health_factor.with_scale(2)
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
    health_factor: &BigDecimal,
    client: &Arc<Provider<Ws>>,
) -> Result<(LiquidationArgs, BigDecimal), Box<dyn std::error::Error>> {
    // update token hash prices to aave oracle values
    generate_token_price_hash(client).await?;

    let mut liquidation_args = LiquidationArgs {
        collateral: Address::zero(),
        debt: Address::zero(),
        user: *user_id,
        debt_to_cover: BigDecimal::from(0),
        receive_a_token: false,
    };

    if health_factor >= &BigDecimal::from_f32(LIQUIDATION_THRESHOLD).unwrap() {
        return Ok((liquidation_args, BigDecimal::from(0)));
    }

    let liquidation_factor =
        if health_factor < &BigDecimal::from_f32(CLOSE_FACTOR_HF_THRESHOLD).unwrap() {
            LiquidationCloseFactor::Half
        } else {
            LiquidationCloseFactor::Full
        };

    let aave_v3_data_pool =
        AAVE_V3_DATA_PROVIDER::new(*AAVE_V3_DATA_PROVIDER_ADDRESS, client.clone());

    let liquidation_close_factor = match liquidation_factor {
        LiquidationCloseFactor::Full => BigDecimal::from(1),
        LiquidationCloseFactor::Half => BigDecimal::from_f32(0.5).unwrap(),
    };

    let mut tokens = Vec::new();
    let mut highest_token_debt = BigDecimal::from(0);
    let mut maximum_profit = BigDecimal::from(0);

    for token in UNIQUE_TOKEN_DATA.values() {
        let token_address = token.address.parse()?;
        let decimal_factor = BigDecimal::from_u64(10_u64.pow(token.decimals.into())).unwrap();
        let bps_factor = BigDecimal::from_u64(BPS_FACTOR).unwrap();

        let (a_token_balance, stable_debt, variable_debt, _, _, _, _, _, use_as_collateral) =
            aave_v3_data_pool
                .get_user_reserve_data(token_address, *user_id)
                .call()
                .await?;

        let total_debt = stable_debt + variable_debt;
        let total_debt = u256_to_big_decimal(&total_debt) / &decimal_factor;
        let a_token_balance = u256_to_big_decimal(&a_token_balance) / &decimal_factor;

        if total_debt > BigDecimal::from(0) || a_token_balance > BigDecimal::from(0) {
            tokens.push(AaveToken {
                token: *token,
                current_total_debt: total_debt.clone(),
                usage_as_collateral_enabled: use_as_collateral,
                current_atoken_balance: a_token_balance,
                reserve_liquidation_bonus: &BigDecimal::from(token.liquidation_bonus) / &bps_factor,
                reserve_liquidation_threshold: BigDecimal::from(token.liquidation_threshold),
            })
        }

        if total_debt > highest_token_debt {
            highest_token_debt = total_debt;

            let token_price = get_saved_token_price(token.address.to_lowercase()).await?;

            liquidation_args = LiquidationArgs {
                debt: token_address,
                debt_to_cover: &highest_token_debt * &liquidation_close_factor * &token_price,
                ..liquidation_args
            }
        }
    }

    // now loop through to get find optimal liquidation combo
    for token in tokens {
        let liquidation_bonus = &token.reserve_liquidation_bonus;
        let debt_to_cover_in_usd = &liquidation_args.debt_to_cover;
        let a_token_balance = &token.current_atoken_balance;
        let one = &BigDecimal::from(1);

        // debug!(
        //     "liquidaiton bonus {}, debt to cover {}, a token balance {}",
        //     liquidation_bonus,
        //     debt_to_cover_in_usd.with_scale(3),
        //     a_token_balance.with_scale(3)
        // );

        // calculate profit
        // profit = debtToCover$ * liquidaitonBonus * (liquidationBonus - 1) * aTokenBalance
        let profit_usd =
            debt_to_cover_in_usd * liquidation_bonus * (liquidation_bonus - one) * a_token_balance;

        if profit_usd > maximum_profit {
            maximum_profit = profit_usd;
            let token_address = token.token.address.parse()?;

            liquidation_args = LiquidationArgs {
                collateral: token_address,
                ..liquidation_args
            }
        }
    }

    Ok((liquidation_args, maximum_profit))
}

pub async fn calculate_gas_cost(
    liquidation_args: &LiquidationArgs,
    client: &Arc<Provider<Ws>>,
) -> Result<U256, Box<dyn std::error::Error>> {
    let aave_v3_pool = AAVE_V3_POOL::new(*AAVE_V3_POOL_ADDRESS, client.clone());

    debug!("estmating gas cost");
    let estimated_gas = aave_v3_pool
        .liquidation_call(
            liquidation_args.collateral,
            liquidation_args.debt,
            liquidation_args.user,
            big_decimal_to_u256(liquidation_args.debt_to_cover.clone())?,
            liquidation_args.receive_a_token,
        )
        .estimate_gas()
        .await?;

    debug!("estmating gas price");
    let gas_price = client.get_gas_price().await?;

    info!("gas cost => {}, gas price => {}", estimated_gas, gas_price);
    let total_cost = estimated_gas
        .checked_mul(gas_price)
        .ok_or("overflow calculating total cost")?;

    let eth_cost = format_units(total_cost, 18)?;
    info!("cost in eth of liquidation call is {}", eth_cost);

    Ok(total_cost)
}

pub fn big_decimal_to_u256(value: BigDecimal) -> Result<U256, &'static str> {
    // Convert BigDecimal to BigInt
    let (num, scale) = value.into_bigint_and_exponent();
    let big_int = if scale >= 0 {
        num * BigInt::from(10).pow(scale as u32)
    } else {
        // Handle scale if it's negative; reduce the number's precision
        num.checked_div(&BigInt::from(10).pow((-scale) as u32))
            .ok_or("Division error")?
    };

    // Ensure the BigInt is non-negative
    if big_int < Zero::zero() {
        return Err("Negative values cannot be converted to U256");
    }

    // Convert BigInt to U256
    U256::from_dec_str(&big_int.to_string()).map_err(|_| "Overflow or conversion error")
}
