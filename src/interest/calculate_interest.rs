use crate::data::token_data_hash::get_unique_token_data;
use crate::exchanges::aave_v3::implementations::aave_users_hash::UpdateUsers;
use crate::exchanges::aave_v3::user_structs::AaveUsersHash;
use anyhow::Result;
use futures::lock::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

pub const SECONDS_PER_YEAR: f64 = 3600.0 * 24.0 * 365.0;

pub struct TokenRates {
    pub variable_borrow_rate: f64,
    pub stable_borrow_rate: f64,
    pub liquidity_rate: f64,
}

pub async fn update_interest_for_all_whale_users_tokens(
    last_timestamp: u32,
    current_timestamp: u32,
    user_data: &Arc<Mutex<AaveUsersHash>>,
) -> Result<()> {
    let mut users = user_data.lock().await;
    let whales = users.get_hashset_of_whales();

    let token_interest_rate_map =
        get_rate_for_each_token(last_timestamp, current_timestamp).await?;

    for whale in whales {
        let whale_user = users.user_data.get_mut(&whale).unwrap();

        for token in whale_user.tokens.iter_mut() {
            let interest_rates = token_interest_rate_map.get(&token.token.address).unwrap();

            if token.token.stable_borrow_rate > 0.0 && token.current_stable_debt > 0.0 {
                token.current_stable_debt *= interest_rates.stable_borrow_rate;
            }

            if token.current_variable_debt > 0.0 && token.token.variable_borrow_rate > 0.0 {
                token.current_variable_debt *= interest_rates.variable_borrow_rate;
            }

            if token.current_atoken_balance > 0.0 && token.token.liquidity_rate > 0.0 {
                token.current_atoken_balance *= interest_rates.liquidity_rate;
            }
        }
    }

    Ok(())
}

pub async fn get_rate_for_each_token(
    last_timestamp: u32,
    current_timestamp: u32,
) -> Result<HashMap<String, TokenRates>> {
    let tokens = get_unique_token_data().await?;

    let mut borrow_rate_token_map = HashMap::<String, TokenRates>::new();

    for token in tokens.values() {
        let token_rates = TokenRates {
            variable_borrow_rate: calculate_compound_interest(
                token.variable_borrow_rate,
                last_timestamp,
                current_timestamp,
            ),
            stable_borrow_rate: calculate_compound_interest(
                token.stable_borrow_rate,
                last_timestamp,
                current_timestamp,
            ),
            liquidity_rate: calculate_linear_interest(
                token.liquidity_rate,
                last_timestamp,
                current_timestamp,
            ),
        };

        borrow_rate_token_map.insert(token.address.to_lowercase(), token_rates);
    }

    Ok(borrow_rate_token_map)
}

fn calculate_compound_interest(rate: f64, last_timestamp: u32, current_timestamp: u32) -> f64 {
    if rate == 0.0 {
        return 1.0;
    };

    let time = current_timestamp - last_timestamp;
    let time = time as f64;

    let exp_minus_one = time - 1.0;

    let exp_minus_two = if time > 2.0 { time as f64 - 2.0 } else { 0.0 };

    let base_power_two = rate * rate / (SECONDS_PER_YEAR * SECONDS_PER_YEAR);
    let base_power_three = base_power_two * rate / SECONDS_PER_YEAR;

    let second_term = time * exp_minus_one * base_power_two / 2.0;
    let third_term = time * exp_minus_one * exp_minus_two * base_power_three / 2.0;

    return 1.0 + (rate * time) / SECONDS_PER_YEAR + second_term + third_term;
}

fn calculate_linear_interest(rate: f64, last_timestamp: u32, current_timestamp: u32) -> f64 {
    if rate == 0.0 {
        return 1.0;
    };

    let time = current_timestamp - last_timestamp;
    let time = time as f64;

    return 1.0 + (rate * time) / SECONDS_PER_YEAR;
}
