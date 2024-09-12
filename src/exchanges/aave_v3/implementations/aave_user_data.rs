use super::super::get_user_api::{get_aave_v3_users, get_all_aave_v3_users, UserAccountData};
use super::super::get_user_from_contract::get_aave_v3_user_from_data_provider;
use super::super::user_structs::{AaveUserData, AaveUsersHash, PricingSource, SampleSize};
use crate::abi::aave_v3_pool::AAVE_V3_POOL;
use crate::data::address::CONTRACT;
use crate::data::erc20::{u256_to_big_decimal, Convert};
use crate::data::token_data_hash::{get_token_data, TOKENS_WITH_NO_AGGREGATOR};
use crate::data::token_price_hash::{generate_token_price_hash, get_saved_token_price};
use crate::exchanges::aave_v3::implementations::aave_users_hash::UpdateUsers;
use crate::exchanges::aave_v3::user_structs::{
    LiquidationCloseFactor, CLOSE_FACTOR_HF_THRESHOLD, HEALTH_FACTOR_THRESHOLD,
};
use anyhow::Result;
use async_trait::async_trait;
use bigdecimal::ToPrimitive;
use bigdecimal::{BigDecimal, FromPrimitive};
use ethers::{
    providers::{Provider, Ws},
    types::Address,
};
use log::{debug, error, info};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[async_trait]
pub trait GenerateUsers {
    async fn get_users(
        client: &Arc<Provider<Ws>>,
        sample_size: SampleSize,
    ) -> Result<AaveUsersHash>;
}

#[async_trait]
pub trait GetUserData {
    async fn get_collateral_times_liquidation_factor_and_total_debt(
        &self,
        source_for_pricing: PricingSource,
        client: &Arc<Provider<Ws>>,
    ) -> Result<(f64, f64)>;
    async fn get_user_liquidation_usd_profit(
        &self,
        health_factor: f64,
    ) -> Result<(f64, Address, Address)>;
}

#[async_trait]
pub trait UpdateUserData {
    async fn update_meta_data(
        &mut self,
        source_for_pricing: PricingSource,
        client: &Arc<Provider<Ws>>,
    ) -> Result<()>;
}

#[async_trait]
pub trait HealthFactor {
    async fn get_health_factor_from_(
        &self,
        source_for_pricing: PricingSource,
        client: &Arc<Provider<Ws>>,
    ) -> Result<f64>;

    async fn is_user_valid_when_checking_against_official_health_factor(
        &mut self,
        client: &Arc<Provider<Ws>>,
    ) -> Result<bool>;
}

#[async_trait]
impl GenerateUsers for AaveUserData {
    async fn get_users(
        client: &Arc<Provider<Ws>>,
        sample_size: SampleSize,
    ) -> Result<AaveUsersHash> {
        let aave_v3_pool_address: Address = CONTRACT.get_address().aave_v3_pool.parse()?;
        let aave_v3_pool = AAVE_V3_POOL::new(aave_v3_pool_address, client.clone());

        // Initialize TOKEN_PRICE_HASH global hashmap of token prices
        if let Err(e) = generate_token_price_hash(client).await {
            error!("Failed to initialize token prices: {}", e);
        }

        // store all data that we need for user
        let mut user_data: Vec<AaveUserData> = Vec::new();

        // let aave_users = get_aave_v3_users().await?;
        let aave_users = match sample_size {
            SampleSize::All => get_all_aave_v3_users().await?,
            SampleSize::SmallBatch => get_aave_v3_users().await?,
        };

        info!("got aave_v3 users");
        let standard_scale = BigDecimal::from_u64(10_u64.pow(18)).unwrap();
        if aave_users.is_empty() {
            panic!("zero users found from graphql");
        } else {
            info!("found { } users from aave v3 graphql", aave_users.len());
        }

        let mut valid_users_from_graphql: u16 = 0;
        let mut valid_users_from_contract: u16 = 0;
        for user in &aave_users {
            let user_id: Address = user.id.parse()?;
            let (_, _, _, _, _, real_health_factor) =
                aave_v3_pool.get_user_account_data(user_id).call().await?;

            let real_health_factor = u256_to_big_decimal(&real_health_factor) / &standard_scale;
            let real_health_factor = real_health_factor.to_f64().unwrap();

            // this is list of tokens that user is either using as colladeral or borrowing
            let (user_tokens, total_debt, collateral_times_liquidation_factor, health_factor) =
                user.get_list_of_user_tokens(client).await?;

            let has_forbidden_token = user_tokens
                .iter()
                .any(|token| TOKENS_WITH_NO_AGGREGATOR.contains(&token.token.symbol.as_str()));

            // check that user does not have token we cannot track
            if has_forbidden_token {
                debug!("excluded use with forbidden token!!");
                continue;
            }

            let mut aave_user = AaveUserData {
                id: user_id,
                total_debt: total_debt.clone(),
                collateral_times_liquidation_factor,
                health_factor: health_factor.clone(),
                tokens: user_tokens,
            };

            // debug!("***************************");

            // this step is needed to make sure collateral and debt values are scaled properly
            aave_user
                .update_meta_data(PricingSource::SavedTokenPrice, client)
                .await?;

            // validate user data
            let graphql_health_factor = aave_user.health_factor;

            let lower_bound = 0.995 * real_health_factor;
            let upper_bound = 1.005 * real_health_factor;

            // debug!("real health factor => {}", real_health_factor);
            // debug!("graphql health factor => {}", graphql_health_factor);
            // debug!("***************************");

            // for estimating user profit
            // make sure health factor is with 0.5% of actual otherwise pull user data directly from pool contract
            if graphql_health_factor > lower_bound && graphql_health_factor < upper_bound {
                // save data to AvveUserData
                valid_users_from_graphql += 1;

                user_data.push(aave_user);
            } else {
                // get user data from pool contract
                let aave_user_data_result =
                    get_aave_v3_user_from_data_provider(aave_user.id, client).await;

                match aave_user_data_result {
                    Ok(user_from_aave_contract) => {
                        valid_users_from_contract += 1;

                        user_data.push(user_from_aave_contract);
                    }
                    Err(_) => {
                        // error!("user did not fit criteria => {}", error);
                    }
                };
            }
        }

        info!("{} users saved", user_data.len());
        info!(
            "{} valid users saved from graphQL",
            valid_users_from_graphql
        );
        info!(
            "{} valid users saved from data provider contract",
            valid_users_from_contract
        );

        let mut user_data_hash = HashMap::new();

        for user in &user_data {
            user_data_hash.insert(user.id, user.clone());
        }

        let mut user_hash = AaveUsersHash {
            user_data: user_data_hash,
            standard_user_ids_by_token: HashMap::<Address, HashSet<Address>>::new(),
            whale_user_ids_by_token: HashMap::<Address, HashSet<Address>>::new(),
        };

        user_hash.intialize_token_user_mapping().await?;

        Ok(user_hash)
    }
}

#[async_trait]
impl GetUserData for AaveUserData {
    async fn get_collateral_times_liquidation_factor_and_total_debt(
        &self,
        source_for_pricing: PricingSource,
        client: &Arc<Provider<Ws>>,
    ) -> Result<(f64, f64)> {
        let token_data = get_token_data().await?;

        let mut total_debt_usd = 0.0;
        let mut liquidation_threshold_collateral_sum = 0.0;

        for r in &self.tokens {
            let token = token_data.get(&r.token.symbol).unwrap();

            // 1. get token price USD
            let token_price_usd = match source_for_pricing {
                PricingSource::AaveOracle => token.get_token_oracle_price(client).await?,
                PricingSource::SavedTokenPrice => {
                    token.get_saved_price_from_token_price_hash().await?
                }
            };

            // 2. get get current total debt in USD
            // *************************************
            let current_total_debt = r.current_total_debt;
            // *************************************

            if current_total_debt > 0.0 {
                let current_total_debt_usd = current_total_debt * token_price_usd;

                // 3. add current total debt to total debt
                total_debt_usd += current_total_debt_usd;
            }

            if r.usage_as_collateral_enabled {
                // 4. get atoken balance in USD
                // *************************************
                let current_atoken_balance = r.current_atoken_balance;
                // *************************************

                if current_atoken_balance > 0.0 {
                    let current_atoken_usd = current_atoken_balance * token_price_usd;

                    // 5. update liquidity threshold colleral sum
                    let liquidation_threshold = r.reserve_liquidation_threshold;
                    // *************************************

                    liquidation_threshold_collateral_sum +=
                        current_atoken_usd * liquidation_threshold;
                }
            }
        }

        Ok((liquidation_threshold_collateral_sum, total_debt_usd))
    }

    async fn get_user_liquidation_usd_profit(
        &self,
        health_factor: f64,
    ) -> Result<(f64, Address, Address)> {
        // should be health factor threshold and not liquidation threshold because
        // looking at profit POTENTIAL
        if health_factor >= HEALTH_FACTOR_THRESHOLD {
            return Ok((0.0, Address::zero(), Address::zero()));
        }

        let liquidation_factor = if health_factor < CLOSE_FACTOR_HF_THRESHOLD {
            LiquidationCloseFactor::Half
        } else {
            LiquidationCloseFactor::Full
        };

        let liquidation_close_factor = match liquidation_factor {
            LiquidationCloseFactor::Full => 1.0,
            LiquidationCloseFactor::Half => 0.5,
        };

        let mut highest_token_debt = 0.0;
        let mut token_highest_debt: &str = "";
        let mut token_highest_collateral = Address::zero();
        let mut highest_debt_to_cover = 0.0;
        // let mut highest_decimal_factor = 0.0;
        let mut maximum_profit = 0.0;

        for token in &self.tokens {
            // let decimal_factor = 10_u64.pow(token.token.decimals.into()) as f64;

            if token.current_total_debt > highest_token_debt {
                highest_token_debt = token.current_total_debt;
                token_highest_debt = &token.token.address;
                // highest_decimal_factor = decimal_factor;

                // let debt_to_cover =
                //     highest_token_debt / decimal_factor * liquidation_close_factor_scaled * token_price

                highest_debt_to_cover = highest_token_debt * liquidation_close_factor;
            }
        }

        // if user has no debt then profit is zero
        if highest_token_debt == 0.0 {
            return Ok((0.0, Address::zero(), Address::zero()));
        }

        // now loop through to get find optimal liquidation combo
        for token in &self.tokens {
            let liquidation_bonus = token.reserve_liquidation_bonus;

            // calculate profit
            // profit = debtToCover$ * liquidaitonBonus * (liquidationBonus - 1)
            // to unscale divide by bps_factor twice and by decimal_factor once

            if liquidation_bonus > 0.0 && token.usage_as_collateral_enabled {
                let debt_token_price =
                    get_saved_token_price(token_highest_debt.to_lowercase()).await?;
                let profit_usd = (highest_debt_to_cover * debt_token_price)
                    // / highest_decimal_factor
                    * liquidation_bonus
                    // / BPS_FACTOR
                    * (liquidation_bonus - 1.0);
                // / BPS_FACTOR;

                if profit_usd > maximum_profit {
                    maximum_profit = profit_usd;
                    token_highest_collateral = token.token.address.parse()?;
                }
            }
        }

        let debt_token: Address = token_highest_debt.parse()?;
        Ok((maximum_profit, debt_token, token_highest_collateral))
    }
}

#[async_trait]
impl UpdateUserData for AaveUserData {
    async fn update_meta_data(
        &mut self,
        source_for_pricing: PricingSource,
        client: &Arc<Provider<Ws>>,
    ) -> Result<()> {
        let (collateral_times_liquidation_factor, total_debt) = self
            .get_collateral_times_liquidation_factor_and_total_debt(source_for_pricing, client)
            .await?;

        self.collateral_times_liquidation_factor = collateral_times_liquidation_factor;
        self.total_debt = total_debt;

        // now with updated  debt and collateral values ,  we can find health factor

        let health_factor = if self.total_debt > 0.0 {
            self.collateral_times_liquidation_factor / self.total_debt
        } else {
            // warn!("no health factor because user has no debt");
            0.0
        };

        self.health_factor = health_factor;
        Ok(())
    }
}

#[async_trait]
impl HealthFactor for AaveUserData {
    async fn get_health_factor_from_(
        &self,
        source_for_pricing: PricingSource,
        client: &Arc<Provider<Ws>>,
    ) -> Result<f64> {
        let (liquidation_threshold_collateral_sum, current_total_debt) = self
            .get_collateral_times_liquidation_factor_and_total_debt(source_for_pricing, client)
            .await?;

        let health_factor = if current_total_debt > 0.0 {
            liquidation_threshold_collateral_sum / current_total_debt
        } else {
            // warn!("no health factor because user has no debt");
            0.0
        };
        Ok(health_factor)
    }

    async fn is_user_valid_when_checking_against_official_health_factor(
        &mut self,
        client: &Arc<Provider<Ws>>,
    ) -> Result<bool> {
        let aave_v3_pool_address: Address = CONTRACT.get_address().aave_v3_pool.parse()?;
        let aave_v3_pool = AAVE_V3_POOL::new(aave_v3_pool_address, client.clone());

        let standard_scale = BigDecimal::from_u64(10_u64.pow(18)).unwrap();
        let (
            _total_collateral_base,
            _total_debt_base,
            _available_borrows_base,
            _current_liquidation_threshold,
            _ltv,
            health_factor,
        ) = aave_v3_pool.get_user_account_data(self.id).call().await?;

        let health_factor = u256_to_big_decimal(&health_factor) / &standard_scale;
        let health_factor = health_factor.to_f64().unwrap();

        let aave_user_calculated_health_factor = self
            .get_health_factor_from_(PricingSource::AaveOracle, client)
            .await?;

        // CHECK that health factor calculated from user data is
        // within + or - 1% of official health factor we get from
        // getUserAccount contract call
        let lower_bound = 0.99 * health_factor;
        let upper_bound = 1.01 * health_factor;

        Ok(aave_user_calculated_health_factor > lower_bound
            && aave_user_calculated_health_factor < upper_bound)
    }
}
