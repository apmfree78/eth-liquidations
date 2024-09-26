#[path = "./mocks/generate_logs.rs"]
mod generate_logs;

#[path = "./mocks/generate_mock_users.rs"]
mod generate_mock_users;

use anyhow::Result;
use eth_liquadation::data::token_data_hash::{
    get_unique_token_data, save_erc20_tokens_from_static_data,
};
use eth_liquadation::data::token_price_hash::generate_token_price_hash;
use eth_liquadation::events::aave_events::update_users_with_event_from_log;
use eth_liquadation::interest::calculate_interest::{get_rate_for_each_token, SECONDS_PER_YEAR};
use ethers::core::types::U256;
use ethers::providers::{Provider, Ws};
use ethers::types::Address;
use generate_mock_users::generate_mock_user_hash;
use std::sync::Arc;

use eth_liquadation::exchanges::aave_v3::events::ReserveDataUpdatedEvent;
use generate_logs::create_log_for_reserve_data_updated_event;

const WS_URL: &str = "ws://localhost:8546";
const AAVE_V3_POOL: &str = "0x87870Bca3F3fD6335C3F4ce8392D69350B4fA4E2";

#[tokio::test]
async fn check_token_rates_hashmap_is_valid() -> Result<()> {
    const WS_URL: &str = "ws://localhost:8546";
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;
    let tokens = get_unique_token_data().await?;

    let token_interest_rate_map = get_rate_for_each_token(0, 100).await?;

    for token in tokens.values() {
        let expected_stable_rate = 1.0 + (token.stable_borrow_rate * 100.0) / SECONDS_PER_YEAR;
        let expected_variable_rate = 1.0 + (token.variable_borrow_rate * 100.0) / SECONDS_PER_YEAR;
        let expected_liquidity_rate = 1.0 + (token.liquidity_rate * 100.0) / SECONDS_PER_YEAR;

        let token_rates = token_interest_rate_map.get(&token.address).unwrap();

        println!("checking token {}", token.symbol);
        assert_eq!(expected_stable_rate, token_rates.stable_borrow_rate);
        assert_eq!(
            round_f64(expected_variable_rate, 12),
            round_f64(token_rates.variable_borrow_rate, 12)
        );
        assert_eq!(expected_liquidity_rate, token_rates.liquidity_rate);
    }

    Ok(())
}

#[tokio::test]
async fn test_reserve_data_updated_event_decoding() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);

    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;
    generate_token_price_hash(&client).await?;

    let base_5 = U256::from(5);
    let base_1 = U256::from(1);
    let base_2 = U256::from(2);
    let exponent = U256::exp10(25); // 10^25
    let rate_5 = base_5 * exponent; // 2%
    let rate_1 = base_1 * exponent; // 1%
    let rate_2 = base_2 * exponent; // 5%
    let rate_5_f64 = 0.05;
    let rate_1_f64 = 0.01;
    let rate_2_f64 = 0.02;

    let reserve_data_updated_event = ReserveDataUpdatedEvent {
        reserve: "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
            .parse()
            .unwrap(),
        liquidity_rate: rate_2,
        stable_borrow_rate: rate_1,
        variable_borrow_rate: rate_5,
        liquidity_index: rate_5,
        variable_borrow_index: rate_5,
    };

    let log = create_log_for_reserve_data_updated_event(&reserve_data_updated_event, AAVE_V3_POOL);

    let mut user_hash = generate_mock_user_hash().await?;

    // now lets repay user debt and see if amount is updated
    update_users_with_event_from_log(log, &mut user_hash, &client).await?;

    // grab updated token data
    let token_data = get_unique_token_data().await?;
    let updated_token = token_data
        .get("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2")
        .unwrap();

    let token_address: Address = updated_token.address.parse()?;

    // test original event matches decoded
    assert_eq!(token_address, reserve_data_updated_event.reserve);
    assert_eq!(round_f64(updated_token.liquidity_rate, 10), rate_2_f64);
    assert_eq!(round_f64(updated_token.stable_borrow_rate, 10), rate_1_f64);
    assert_eq!(
        round_f64(updated_token.variable_borrow_rate, 10),
        rate_5_f64
    );

    Ok(())
}

fn round_f64(value: f64, decimal_places: u32) -> f64 {
    let factor = 10_f64.powi(decimal_places as i32);
    (value * factor).round() / factor
}
