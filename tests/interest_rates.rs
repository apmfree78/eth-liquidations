use anyhow::Result;
use eth_liquadation::data::token_data_hash::{
    get_unique_token_data, save_erc20_tokens_from_static_data,
};
use eth_liquadation::interest::calculate_interest::{get_rate_for_each_token, SECONDS_PER_YEAR};
use ethers::providers::{Provider, Ws};
use std::sync::Arc;

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

fn round_f64(value: f64, decimal_places: u32) -> f64 {
    let factor = 10f64.powi(decimal_places as i32);
    (value * factor).round() / factor
}
