use bigdecimal::BigDecimal;
use eth_liquadation::exchanges::aave_v3::data::{
    AaveUserData, Generate, HealthFactor, PricingSource,
};
use ethers::providers::{Provider, Ws};
use std::str::FromStr;
use std::sync::Arc;

#[tokio::test]
async fn test_that_health_factor_is_self_consistent_in_user_data(
) -> Result<(), Box<dyn std::error::Error>> {
    // Set up the Ethereum client connection and wallet
    const WS_URL: &str = "ws://localhost:8546";
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);

    // Round both to four decimal places
    let scale = 3;

    let aave_user_list: Vec<AaveUserData> = AaveUserData::get_users(&client).await?;

    for user in &aave_user_list {
        let given_health_factor = &user.health_factor.with_scale(scale);
        if user.total_debt > BigDecimal::from(0) {
            let calculated_health_factor =
                &user.colladeral_times_liquidation_factor / &user.total_debt;
            let calculated_health_factor = &calculated_health_factor.with_scale(scale);

            assert_eq!(given_health_factor, calculated_health_factor)
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_that_calculated_health_factor_roughly_matches_given_one(
) -> Result<(), Box<dyn std::error::Error>> {
    // Set up the Ethereum client connection and wallet
    const WS_URL: &str = "ws://localhost:8546";
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);

    // Round both to four decimal places

    let aave_user_list: Vec<AaveUserData> = AaveUserData::get_users(&client).await?;

    for user in &aave_user_list {
        let given_health_factor = user.health_factor.clone();
        if user.total_debt > BigDecimal::from(0) {
            let health_factor_uniswap_v3 = user
                .get_health_factor_from_(PricingSource::UniswapV3, &client)
                .await?;

            let health_factor_oracle = user
                .get_health_factor_from_(PricingSource::AaveOracle, &client)
                .await?;

            let lower_bound = BigDecimal::from_str("0.95")? * &given_health_factor;
            let upper_bound = BigDecimal::from_str("1.05")? * &given_health_factor;

            // println!("testing {:#?}", user.tokens);
            println!("health_factor_uniswap_v3: {}", health_factor_uniswap_v3);
            println!("health_factor_oracle: {}", health_factor_oracle);
            println!("given health_factor: {}", given_health_factor);

            assert!(
                health_factor_oracle > lower_bound && health_factor_oracle < upper_bound,
                "aave oracle health factor out of bound"
            );

            assert!(
                health_factor_uniswap_v3 > lower_bound && health_factor_uniswap_v3 < upper_bound,
                "uniswap health factor out of bound"
            );
        }
    }

    Ok(())
}
