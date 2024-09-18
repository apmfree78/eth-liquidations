use anyhow::Result;
use dotenv::dotenv;
use eth_liquadation::{
    data::{
        token_data_hash::save_erc20_tokens_from_static_data,
        token_price_hash::generate_token_price_hash,
    },
    exchanges::aave_v3::{
        implementations::aave_user_data::{GenerateUsers, HealthFactor},
        user_structs::{AaveUserData, AaveUsersHash, PricingSource, SampleSize},
    },
};
use ethers::providers::{Provider, Ws};
use std::sync::Arc;

#[tokio::test]
async fn test_that_health_factor_is_self_consistent_in_user_data() -> Result<()> {
    dotenv().ok();

    // Set up the Ethereum client connection and wallet
    const WS_URL: &str = "ws://localhost:8546";
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);

    save_erc20_tokens_from_static_data(&client).await?;
    generate_token_price_hash(&client).await?;

    let aave_users_hash: AaveUsersHash =
        AaveUserData::get_users(&client, SampleSize::SmallBatch).await?;

    for user in aave_users_hash.user_data.values() {
        let given_health_factor = user.health_factor;
        if user.total_debt > 0.0 {
            let calculated_health_factor =
                &user.collateral_times_liquidation_factor / &user.total_debt;
            let calculated_health_factor = calculated_health_factor;

            assert_eq!(given_health_factor, calculated_health_factor)
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_that_calculated_health_factor_roughly_matches_given_one() -> Result<()> {
    dotenv().ok();
    // Set up the Ethereum client connection and wallet
    const WS_URL: &str = "ws://localhost:8546";
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);

    save_erc20_tokens_from_static_data(&client).await?;
    generate_token_price_hash(&client).await?;

    let aave_users_hash: AaveUsersHash =
        AaveUserData::get_users(&client, SampleSize::SmallBatch).await?;

    for user in aave_users_hash.user_data.values() {
        let given_health_factor = user.health_factor.clone();
        if user.total_debt > 0.0 {
            println!("testing {:#?}", user.tokens);

            let health_factor_oracle = user
                .get_health_factor_from_(PricingSource::SavedTokenPrice, &client)
                .await?;

            let lower_bound = 0.99 * given_health_factor;
            let upper_bound = 1.01 * given_health_factor;

            println!("health_factor_oracle: {}", health_factor_oracle);
            println!("given health_factor: {}", given_health_factor);

            assert!(
                health_factor_oracle > lower_bound && health_factor_oracle < upper_bound,
                "aave oracle health factor out of bound"
            );
        }
    }

    Ok(())
}
