use super::get_users::{get_aave_v3_users, UserAccountData};
use crate::abi::aave_v3_pool::AAVE_V3_POOL;
use crate::data::address::AAVE_V3_POOL_ADDRESS;
use crate::data::erc20::{u256_to_big_decimal, Convert, Erc20Token, TOKEN_DATA};
use async_trait::async_trait;
use bigdecimal::{BigDecimal, FromPrimitive, Zero};
use ethers::providers::{Provider, Ws};
use ethers::types::Address;
use std::str::FromStr;
use std::sync::Arc;

pub enum PricingSource {
    AaveOracle,
    UniswapV3,
}

#[derive(Clone, Debug)]
pub struct AaveToken {
    pub token: Erc20Token,
    pub current_total_debt: BigDecimal,
    pub token_price_eth: BigDecimal,
    pub usage_as_collateral_enabled: bool,
    pub current_atoken_balance: BigDecimal,
    pub reserve_liquidation_threshold: BigDecimal,
    pub reserve_liquidation_bonus: BigDecimal,
}

#[derive(Clone, Debug)]
pub struct AaveUserData {
    pub id: Address,
    pub total_debt: BigDecimal,
    pub colladeral_times_liquidation_factor: BigDecimal,
    pub tokens: Vec<AaveToken>,
    pub health_factor: BigDecimal,
}

#[async_trait]
pub trait Generate {
    async fn get_users(
        client: &Arc<Provider<Ws>>,
    ) -> Result<Vec<AaveUserData>, Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait HealthFactor {
    async fn get_health_factor_from_(
        &self,
        source_for_pricing: PricingSource,
        client: &Arc<Provider<Ws>>,
    ) -> Result<BigDecimal, Box<dyn std::error::Error>>;

    async fn get_and_update_health_factor_with_(
        &mut self,
        source_for_pricing: PricingSource,
        client: &Arc<Provider<Ws>>,
    ) -> Result<BigDecimal, Box<dyn std::error::Error>>;
}

#[async_trait]
impl Generate for AaveUserData {
    async fn get_users(
        client: &Arc<Provider<Ws>>,
    ) -> Result<Vec<AaveUserData>, Box<dyn std::error::Error>> {
        println!("connecting to aave_v3_pool");
        let aave_v3_pool = AAVE_V3_POOL::new(*AAVE_V3_POOL_ADDRESS, client.clone());

        // store all data that we need for user
        let mut aave_user_data: Vec<AaveUserData> = Vec::new();

        let aave_users = get_aave_v3_users().await?;
        println!("got aave_v3 users");
        let bps_factor = BigDecimal::from_u64(10_u64.pow(4)).unwrap();
        let standard_scale = BigDecimal::from_u64(10_u64.pow(18)).unwrap();
        println!("found { } users from aave v3 graphql", aave_users.len());
        for user in &aave_users {
            let user_id: Address = user.id.parse()?;
            // println!("getting user account data from aave_v3_pool");
            let (
                total_collateral_base,
                total_debt_base,
                _available_borrows_base,
                current_liquidation_threshold,
                _ltv,
                health_factor,
            ) = aave_v3_pool.get_user_account_data(user_id).call().await?;

            let total_debt = u256_to_big_decimal(&total_debt_base);
            let total_collateral = u256_to_big_decimal(&total_collateral_base);
            let liquidation_threshold = u256_to_big_decimal(&current_liquidation_threshold);
            let colladeral_times_liquidation_factor =
                &liquidation_threshold * &total_collateral / &bps_factor;
            let health_factor = u256_to_big_decimal(&health_factor) / &standard_scale;

            // println!("getting list of user tokens");
            // this is list of tokens that user is either using as colladeral or borrowing
            let user_tokens = user.get_list_of_user_tokens(&client).await?;

            let aave_user = AaveUserData {
                id: user_id,
                total_debt: total_debt.clone(),
                colladeral_times_liquidation_factor,
                health_factor: health_factor.clone(),
                tokens: user_tokens,
            };

            // validate user data - 10% of graphql data for aave users is not accurate
            let aave_user_health_factor = aave_user.health_factor.clone();
            let aave_user_calculated_health_factor = aave_user
                .get_health_factor_from_(PricingSource::AaveOracle, &client)
                .await?;
            let lower_bound = BigDecimal::from_str("0.95")? * &aave_user_health_factor;
            let upper_bound = BigDecimal::from_str("1.05")? * &aave_user_health_factor;

            if aave_user_calculated_health_factor > lower_bound
                && aave_user_calculated_health_factor < upper_bound
            {
                // save data to AvveUserData
                aave_user_data.push(aave_user);
            }
        }

        println!("{} valid users saved", aave_user_data.len());
        Ok(aave_user_data)
    }
}

#[async_trait]
impl HealthFactor for AaveUserData {
    async fn get_health_factor_from_(
        &self,
        source_for_pricing: PricingSource,
        client: &Arc<Provider<Ws>>,
    ) -> Result<BigDecimal, Box<dyn std::error::Error>> {
        let bps_factor = BigDecimal::from_u64(10_u64.pow(4)).unwrap();

        let mut total_debt_usd = BigDecimal::zero();
        let mut liquidation_threshold_collateral_sum = BigDecimal::zero();

        for r in &self.tokens {
            let token = TOKEN_DATA.get(&*r.token.symbol).unwrap();

            // 1. get token price USD
            let token_price_usd = match source_for_pricing {
                PricingSource::UniswapV3 => token.get_token_price_in_("USDC", &client).await?,
                PricingSource::AaveOracle => token.get_token_oracle_price(&client).await?,
            };

            let token_decimal_factor =
                BigDecimal::from_u64(10_u64.pow(token.decimals.into())).unwrap();

            // 2. get get current total debt in USD
            // *************************************
            let current_total_debt = r.current_total_debt.clone();
            // *************************************

            if current_total_debt > BigDecimal::zero() {
                let current_total_debt_usd =
                    &current_total_debt * &token_price_usd / &token_decimal_factor;

                // 3. add current total debt to total debt
                total_debt_usd += &current_total_debt_usd;
            }

            if r.usage_as_collateral_enabled {
                // 4. get atoken balance in USD
                // *************************************
                let current_atoken_balance = r.current_atoken_balance.clone();
                // *************************************

                if current_atoken_balance > BigDecimal::zero() {
                    let current_atoken_usd =
                        &current_atoken_balance * &token_price_usd / &token_decimal_factor;

                    // 5. update liquidity threshold colleral sum
                    let liquidation_threshold = r.reserve_liquidation_threshold.clone();
                    // *************************************

                    liquidation_threshold_collateral_sum +=
                        current_atoken_usd * &liquidation_threshold / &bps_factor;
                }
            }
        }

        // println!("total debt {}", total_debt_usd);
        let mut health_factor = BigDecimal::zero();
        if total_debt_usd > BigDecimal::zero() {
            health_factor = liquidation_threshold_collateral_sum / total_debt_usd;
        }
        Ok(health_factor)
    }

    async fn get_and_update_health_factor_with_(
        &mut self,
        source_for_pricing: PricingSource,
        client: &Arc<Provider<Ws>>,
    ) -> Result<BigDecimal, Box<dyn std::error::Error>> {
        // obtain latest health factor
        let health_factor = self
            .get_health_factor_from_(source_for_pricing, &client)
            .await?;

        self.health_factor = health_factor.clone();

        Ok(health_factor)
    }
}
