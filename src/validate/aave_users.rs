use crate::{
    abi::{aave_v3_data_provider::AAVE_V3_DATA_PROVIDER, aave_v3_pool::AAVE_V3_POOL},
    data::{
        address::{AAVE_V3_DATA_PROVIDER_ADDRESS, AAVE_V3_POOL_ADDRESS},
        erc20::{u256_to_big_decimal, TOKEN_DATA, UNIQUE_TOKEN_DATA},
        token_price_hash::{generate_token_price_hash, get_saved_token_price},
        users_to_track::{get_tracked_users, reset_tracked_users},
    },
    exchanges::aave_v3::user_structs::{
        AaveTokenU256, LiquidationArgs, LiquidationCloseFactor, BPS_FACTOR,
        CLOSE_FACTOR_HF_THRESHOLD, LIQUIDATION_THRESHOLD,
    },
    utils::type_conversion::address_to_string,
};

use bigdecimal::{BigDecimal, FromPrimitive};
use colored::*;
// use ethers::{core::k256::ecdsa::SigningKey, middleware::SignerMiddleware, signers::Wallet};
use ethers::{
    providers::{Middleware, Provider, Ws},
    types::{Address, U256},
};
use log::info;
use num_traits::{ToPrimitive, Zero};
use std::sync::Arc;

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

            let (liquidation_args, profit_scaled) =
                calculate_user_liquidation_usd_profit(user_id, &health_factor, client).await?;

            let user_id_string = address_to_string(liquidation_args.user);
            let debt = address_to_string(liquidation_args.debt);
            let collateral = address_to_string(liquidation_args.collateral);
            let profit = &u256_to_big_decimal(&profit_scaled) / &standard_scale;

            info!(
                "user {} liquidation ready /w health score of {}",
                user_id_string.green().bold(),
                format!("{:2}", health_factor.with_scale(4)).red()
            );
            info!(
                "liquidation profit is {}, with debt {} and collateral {}",
                format!("{:2}", profit.with_scale(2)).green().bold(),
                debt.yellow().bold(),
                collateral.black().bold(),
            );

            // only check gas cost of profit if greater than zero

            if profit > BigDecimal::zero() {
                let gas_cost = calculate_gas_cost(client).await?;

                // TODO - calculated NET profit
                let gas_cost_usd =
                    &u256_to_big_decimal(&gas_cost) * &eth_price_usd / &standard_scale;

                info!(
                    "gas cost $ is {}",
                    format!("{:2}", gas_cost_usd.with_scale(2)).red().bold(),
                );
            }
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
) -> Result<(LiquidationArgs, U256), Box<dyn std::error::Error>> {
    let bps_factor = U256::from(BPS_FACTOR);
    let standard_scale = U256::exp10(18);

    // update token hash prices to aave oracle values
    generate_token_price_hash(client).await?;

    let mut liquidation_args = LiquidationArgs {
        collateral: Address::zero(),
        debt: Address::zero(),
        user: *user_id,
        debt_to_cover: U256::from(0),
        _receive_a_token: false,
    };

    if health_factor >= &BigDecimal::from_f32(LIQUIDATION_THRESHOLD).unwrap() {
        return Ok((liquidation_args, U256::from(0)));
    }

    let liquidation_factor =
        if health_factor < &BigDecimal::from_f32(CLOSE_FACTOR_HF_THRESHOLD).unwrap() {
            LiquidationCloseFactor::Half
        } else {
            LiquidationCloseFactor::Full
        };

    let aave_v3_data_pool =
        AAVE_V3_DATA_PROVIDER::new(*AAVE_V3_DATA_PROVIDER_ADDRESS, client.clone());

    let liquidation_close_factor_scaled = match liquidation_factor {
        LiquidationCloseFactor::Full => standard_scale,
        LiquidationCloseFactor::Half => U256::from(5) * U256::exp10(17), // 0.5 scaled
    };

    let mut tokens = Vec::new();
    let mut highest_token_debt = U256::from(0);
    let mut maximum_profit = U256::from(0);

    for token in UNIQUE_TOKEN_DATA.values() {
        let token_address = token.address.parse()?;
        let decimal_factor = U256::exp10(token.decimals.into());

        let (a_token_balance, stable_debt, variable_debt, _, _, _, _, _, use_as_collateral) =
            aave_v3_data_pool
                .get_user_reserve_data(token_address, *user_id)
                .call()
                .await?;

        let total_debt = stable_debt + variable_debt;

        if total_debt > U256::from(0) || a_token_balance > U256::from(0) {
            tokens.push(AaveTokenU256 {
                token: *token,
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
            let token_price = big_decimal_to_u256_scaled(&token_price).unwrap(); // price times 10^18

            // let debt_to_cover =
            //     highest_token_debt / decimal_factor * liquidation_close_factor_scaled * token_price
            //         / standard_scale;

            let debt_to_cover = highest_token_debt
                .checked_mul(liquidation_close_factor_scaled)
                .ok_or("overflow!")?;

            let debt_to_cover = debt_to_cover
                .checked_div(decimal_factor)
                .ok_or("overflow or div by zero!")?;

            let debt_to_cover = debt_to_cover
                .checked_mul(token_price)
                .ok_or("looks like overflow")?;

            let debt_to_cover = debt_to_cover
                .checked_div(standard_scale)
                .ok_or("could be overflow or div by zero")?;

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
        let a_token_balance = token.current_atoken_balance;
        let decimal_factor = U256::exp10(token.token.decimals.into());

        // calculate profit
        // profit = debtToCover$ * liquidaitonBonus * (liquidationBonus - 1) * aTokenBalance
        // let profit_usd_scaled = debt_to_cover_in_usd_scaled * liquidation_bonus / bps_factor
        //     * (liquidation_bonus - bps_factor)
        //     / bps_factor
        //     * a_token_balance
        //     / decimal_factor;

        if liquidation_bonus > U256::from(0) && token.usage_as_collateral_enabled {
            let profit_usd_scaled = debt_to_cover_in_usd_scaled
                .checked_mul(a_token_balance)
                .ok_or("profit calc overflow")?;

            let profit_usd_scaled = profit_usd_scaled
                .checked_div(decimal_factor)
                .ok_or("profit overflow div by zero")?;

            let profit_usd_scaled = profit_usd_scaled
                .checked_mul(liquidation_bonus)
                .ok_or("profit overflow liquidation bonus")?;

            let bonus_minus_one = liquidation_bonus
                .checked_sub(bps_factor)
                .ok_or("additionl error")?;

            let profit_usd_scaled = profit_usd_scaled
                .checked_mul(bonus_minus_one)
                .ok_or("profit overflow liquidation bonus minus one")?;

            let profit_usd_scaled = profit_usd_scaled
                .checked_div(bps_factor)
                .ok_or("profit overflow div bps factor 1")?;

            let profit_usd_scaled = profit_usd_scaled
                .checked_div(bps_factor)
                .ok_or("profit overflow div bps factor 2")?;

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

pub async fn calculate_gas_cost(
    client: &Arc<Provider<Ws>>,
) -> Result<U256, Box<dyn std::error::Error>> {
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
        .ok_or("overflow calculating total cost")?;

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
