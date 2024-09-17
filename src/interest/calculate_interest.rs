use anyhow::Result;
use std::collections::HashMap;

use crate::data::token_data_hash::get_unique_token_data;

const SECONDS_PER_YEAR: f64 = 3600.0 * 24.0 * 365.0;

pub struct TokenRates {
    pub variable_borrow_rate: f64,
    pub stable_borrow_rate: f64,
    pub liquidity_rate: f64,
}

pub fn calculate_compound_interest(rate: f64, last_timestamp: u32, current_timestamp: u32) -> f64 {
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

pub fn calculate_linear_interest(rate: f64, last_timestamp: u32, current_timestamp: u32) -> f64 {
    let time = current_timestamp - last_timestamp;
    let time = time as f64;

    return 1.0 + (rate * time) / SECONDS_PER_YEAR;
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
