use bigdecimal::BigDecimal;
use eth_liquadation::crypto_data::{AaveUserData, Generate};
use ethers::providers::{Provider, Ws};
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
