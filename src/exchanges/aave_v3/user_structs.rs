use super::get_user_api::{get_aave_v3_users, get_all_aave_v3_users, UserAccountData};
use super::get_user_from_contract::get_aave_v3_user_from_data_provider;
use crate::abi::aave_v3_pool::AAVE_V3_POOL;
use crate::data::address::AAVE_V3_POOL_ADDRESS;
use crate::data::erc20::{u256_to_big_decimal, Convert, Erc20Token, TOKEN_DATA};
use async_trait::async_trait;
use bigdecimal::{BigDecimal, FromPrimitive, Zero};
use ethers::abi::Address;
use ethers::providers::{Provider, Ws};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

const HEALTH_FACTOR_THRESHOLD: f32 = 1.1;

#[derive(Clone, Copy, Debug)]
pub enum PricingSource {
    AaveOracle,
    UniswapV3,
    SavedTokenPrice,
}

pub enum SampleSize {
    SmallBatch, // up to 100 users
    All,        // all users
}

#[derive(Clone, Debug)]
pub struct AaveToken {
    pub token: Erc20Token,
    pub current_total_debt: BigDecimal,
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

/*
This below struct is single source of truth for all Aave v3 user data,
this is the main data source that will be kept up to date based on user
events found in logs and changes in token prices
 */
#[derive(Clone, Debug)]
pub struct AaveUsersHash {
    pub user_data: HashMap<Address, AaveUserData>,
    pub standard_user_ids_by_token: HashMap<Address, Vec<Address>>,
    pub low_health_user_ids_by_token: HashMap<Address, Vec<Address>>,
}

pub enum UsersToLiquidate {
    Users(Vec<Address>),
    None,
}

pub enum UserType {
    Standard,
    LowHealth,
}

#[async_trait]
pub trait UpdateUsers {
    // TODO - create method to scan all users that hold specific token, find and update health factor
    // then return user id with health factors below 1
    async fn update_users_health_factor_by_token_and_return_liquidation_candidates(
        &mut self,
        token_address: Address,
        user_type: UserType,
        client: &Arc<Provider<Ws>>,
    ) -> Result<UsersToLiquidate, Box<dyn std::error::Error>>;
    async fn add_new_user(
        &mut self,
        user_to_add: Address,
        client: &Arc<Provider<Ws>>,
    ) -> Result<(), Box<dyn std::error::Error>>;
    // check user health factor and if its in the right token => user id mapping, move if necessary
    fn update_token_to_user_mapping_for_(
        &mut self,
        user_id: Address,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn remove_user_from_token_user_mapping(
        &mut self,
        user_id: Address,
        token: Address,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn move_user_from_standard_to_low_health_token_user_mapping(
        &mut self,
        user_id: Address,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn move_user_from_low_health_to_standard_token_user_mapping(
        &mut self,
        user_id: Address,
    ) -> Result<(), Box<dyn std::error::Error>>;
    // async fn add_user_for_token_user_mapping(
    //     &mut self,
    //     user_id: Address,
    //     token_address: Address,
    //     client: &Arc<Provider<Ws>>,
    // ) -> Result<(), Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait GenerateUsers {
    async fn get_users(
        client: &Arc<Provider<Ws>>,
        sample_size: SampleSize,
    ) -> Result<AaveUsersHash, Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait GetUserData {
    async fn get_collateral_times_liquidation_factor_and_total_debt(
        &self,
        source_for_pricing: PricingSource,
        client: &Arc<Provider<Ws>>,
    ) -> Result<(BigDecimal, BigDecimal), Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait UpdateUserData {
    async fn update_meta_data(
        &mut self,
        source_for_pricing: PricingSource,
        client: &Arc<Provider<Ws>>,
    ) -> Result<(), Box<dyn std::error::Error>>;
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
    async fn is_user_valid_when_checking_against_official_health_factor(
        &mut self,
        client: &Arc<Provider<Ws>>,
    ) -> Result<bool, Box<dyn std::error::Error>>;
}

#[async_trait]
impl UpdateUsers for AaveUsersHash {
    async fn add_new_user(
        &mut self,
        user_to_add: Address,
        client: &Arc<Provider<Ws>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match get_aave_v3_user_from_data_provider(user_to_add, client).await {
            Ok(user) => {
                if user.health_factor
                    > BigDecimal::from_f32(HEALTH_FACTOR_THRESHOLD).expect("invalid f32")
                {
                    for token in &user.tokens {
                        let token_address: Address = token.token.address.parse()?;

                        let mut users = self
                            .standard_user_ids_by_token
                            .entry(token_address)
                            .or_insert_with(Vec::new)
                            .clone();

                        users.push(user_to_add);

                        self.standard_user_ids_by_token.insert(token_address, users);
                    }
                } else {
                    // LOW HEALTH SCORE USERS
                    for token in &user.tokens {
                        let token_address: Address = token.token.address.parse()?;

                        let mut users = self
                            .low_health_user_ids_by_token
                            .entry(token_address)
                            .or_insert_with(Vec::new)
                            .clone();

                        users.push(user_to_add);

                        self.low_health_user_ids_by_token
                            .insert(token_address, users);
                    }
                }
                self.user_data.insert(user.id, user);
                // println!(
                //     "new user successfully added {:?}",
                //     self.user_data.get(&user_id).unwrap()
                // )
            }
            Err(error) => println!("user did not fit criteria ==> {}", error),
        };
        Ok(())
    }

    fn update_token_to_user_mapping_for_(
        &mut self,
        user_id: Address,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // get user health factor

        // if health factor is less than HEALTH_FACTOR_THRESHOLD then
        // then user belongs in  low health factor hash map
        let user = self.user_data.get(&user_id).expect("Invalid user id");
        let health_factor = &user.health_factor;

        let low_health_factor =
            health_factor <= &BigDecimal::from_f32(HEALTH_FACTOR_THRESHOLD).expect("invalid f32");

        if low_health_factor {
            self.move_user_from_standard_to_low_health_token_user_mapping(user_id)
                .unwrap_or_else(|err| println!("could not move user to new mapping => {}", err));
        } else {
            self.move_user_from_low_health_to_standard_token_user_mapping(user_id)
                .unwrap_or_else(|err| println!("could not move user to new mapping => {}", err));
        }

        Ok(())
    }

    // standard mapping ===> low health factor mapping
    fn move_user_from_standard_to_low_health_token_user_mapping(
        &mut self,
        user_id: Address,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let user = self.user_data.get(&user_id).expect("Invalid user id");

        for token in &user.tokens {
            let token_address: Address = token.token.address.parse()?;

            if self.standard_user_ids_by_token.contains_key(&token_address) {
                let mut standard_users = self
                    .standard_user_ids_by_token
                    .entry(token_address)
                    .or_insert_with(Vec::new)
                    .clone();

                if let Some(index) = standard_users.iter().position(|&id| id == user_id) {
                    standard_users.remove(index);

                    // update mapping with user id removed
                    self.standard_user_ids_by_token
                        .insert(token_address, standard_users);
                };

                // move OR add user id to low health factor mapping
                let mut low_health_users = self
                    .low_health_user_ids_by_token
                    .entry(token_address)
                    .or_insert_with(Vec::new)
                    .clone();

                low_health_users.push(user_id);

                self.low_health_user_ids_by_token
                    .insert(token_address, low_health_users);
            }
        }
        Ok(())
    }

    // low health mapping ===> standard mapping
    fn move_user_from_low_health_to_standard_token_user_mapping(
        &mut self,
        user_id: Address,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let user = self.user_data.get(&user_id).expect("Invalid user id");

        for token in &user.tokens {
            let token_address: Address = token.token.address.parse()?;

            if self
                .low_health_user_ids_by_token
                .contains_key(&token_address)
            {
                let mut low_health_users = self
                    .low_health_user_ids_by_token
                    .entry(token_address)
                    .or_insert_with(Vec::new)
                    .clone();

                if let Some(index) = low_health_users.iter().position(|&id| id == user_id) {
                    low_health_users.remove(index);

                    // update mapping with user id removed
                    self.low_health_user_ids_by_token
                        .insert(token_address, low_health_users);
                };

                // move OR add user id to standard mapping
                let mut standard_users = self
                    .standard_user_ids_by_token
                    .entry(token_address)
                    .or_insert_with(Vec::new)
                    .clone();

                standard_users.push(user_id);

                self.standard_user_ids_by_token
                    .insert(token_address, standard_users);
            }
        }
        Ok(())
    }

    fn remove_user_from_token_user_mapping(
        &mut self,
        user_id: Address,
        token_address: Address,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // CHECK STANDARD MAPPING
        if self.standard_user_ids_by_token.contains_key(&token_address) {
            let mut users = self
                .standard_user_ids_by_token
                .entry(token_address)
                .or_insert_with(Vec::new)
                .clone();

            if let Some(index) = users.iter().position(|&id| id == user_id) {
                users.remove(index);

                self.standard_user_ids_by_token.insert(token_address, users);

                return Ok(());
            };
        } else {
            println!("token is not present, no need to remove it");
        }

        // CHECK LOW HEALTH MAPPING
        if self
            .low_health_user_ids_by_token
            .contains_key(&token_address)
        {
            let mut users = self
                .low_health_user_ids_by_token
                .entry(token_address)
                .or_insert_with(Vec::new)
                .clone();

            if let Some(index) = users.iter().position(|&id| id == user_id) {
                users.remove(index);

                self.low_health_user_ids_by_token
                    .insert(token_address, users);
            };
        }

        Ok(())
    }

    // async fn add_user_for_token_user_mapping(
    //     &mut self,
    //     user_id: Address,
    //     token_address: Address,
    //     client: &Arc<Provider<Ws>>,
    // ) -> Result<(), Box<dyn std::error::Error>> {
    //     // get user health factor
    //
    //     let user = self.user_data.get(&user_id).expect("Invalid user id");
    //     let health_factor = user
    //         .get_health_factor_from_(PricingSource::UniswapV3, client)
    //         .await?;
    //
    //     let low_health_factor =
    //         health_factor <= BigDecimal::from_f32(HEALTH_FACTOR_THRESHOLD).expect("invalid f32");
    //
    //     if low_health_factor {
    //         let mut users = self
    //             .low_health_user_ids_by_token
    //             .entry(token_address)
    //             .or_insert_with(Vec::new)
    //             .clone();
    //
    //         users.push(user_id);
    //
    //         self.low_health_user_ids_by_token
    //             .insert(token_address, users);
    //     } else {
    //         let mut users = self
    //             .standard_user_ids_by_token
    //             .entry(token_address)
    //             .or_insert_with(Vec::new)
    //             .clone();
    //
    //         users.push(user_id);
    //
    //         self.standard_user_ids_by_token.insert(token_address, users);
    //     }
    //
    //     Ok(())
    // }
    async fn update_users_health_factor_by_token_and_return_liquidation_candidates(
        &mut self,
        token_address: Address,
        user_type: UserType,
        client: &Arc<Provider<Ws>>,
    ) -> Result<UsersToLiquidate, Box<dyn std::error::Error>> {
        match user_type {
            UserType::LowHealth => {
                let low_health_users = self
                    .low_health_user_ids_by_token
                    .entry(token_address)
                    .or_insert_with(Vec::new);

                for user_id in low_health_users {
                    let user = self
                        .user_data
                        .get_mut(user_id)
                        .unwrap_or_else(|| panic!("could not find user with user id"));

                    user.update_meta_data(PricingSource::UniswapV3, client)
                        .await?;

                    // TODO - find if user has health factor < 1 and add to arry if true
                }
            }
            UserType::Standard => {}
        }
        Ok(UsersToLiquidate::None)
    }
}

#[async_trait]
impl GenerateUsers for AaveUserData {
    async fn get_users(
        client: &Arc<Provider<Ws>>,
        sample_size: SampleSize,
    ) -> Result<AaveUsersHash, Box<dyn std::error::Error>> {
        println!("connecting to aave_v3_pool");
        let aave_v3_pool = AAVE_V3_POOL::new(*AAVE_V3_POOL_ADDRESS, client.clone());

        // store all data that we need for user
        let mut aave_user_data: Vec<AaveUserData> = Vec::new();

        // let aave_users = get_aave_v3_users().await?;
        let aave_users = match sample_size {
            SampleSize::All => get_all_aave_v3_users().await?,
            SampleSize::SmallBatch => get_aave_v3_users().await?,
        };

        println!("got aave_v3 users");
        let bps_factor = BigDecimal::from_u64(10_u64.pow(4)).unwrap();
        let standard_scale = BigDecimal::from_u64(10_u64.pow(18)).unwrap();
        println!("found { } users from aave v3 graphql", aave_users.len());
        let mut valid_users_from_graphql: u16 = 0;
        let mut valid_users_from_contract: u16 = 0;
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
            let user_tokens = user.get_list_of_user_tokens().await?;

            let aave_user = AaveUserData {
                id: user_id,
                total_debt: total_debt.clone(),
                colladeral_times_liquidation_factor,
                health_factor: health_factor.clone(),
                tokens: user_tokens,
            };

            // validate user data - at least 10% of graphql data for aave users is not accurate
            let aave_user_health_factor = aave_user.health_factor.clone();
            let aave_user_calculated_health_factor = aave_user
                .get_health_factor_from_(PricingSource::AaveOracle, client)
                .await?;
            let lower_bound = BigDecimal::from_str("0.95")? * &aave_user_health_factor;
            let upper_bound = BigDecimal::from_str("1.05")? * &aave_user_health_factor;

            if aave_user_calculated_health_factor > lower_bound
                && aave_user_calculated_health_factor < upper_bound
            {
                // save data to AvveUserData
                valid_users_from_graphql += 1;
                aave_user_data.push(aave_user);
            } else {
                // get user data from pool contract
                let aave_user_data_result =
                    get_aave_v3_user_from_data_provider(aave_user.id, client).await;

                match aave_user_data_result {
                    Ok(aave_user) => {
                        valid_users_from_contract += 1;
                        aave_user_data.push(aave_user);
                    }
                    Err(error) => {
                        println!("user did not fit criteria => {}", error);
                    }
                };
            }
        }

        println!("{} valid users saved", aave_user_data.len());
        println!(
            "{} valid users saved from graphQL",
            valid_users_from_graphql
        );
        println!(
            "{} valid users saved from data provider contract",
            valid_users_from_contract
        );

        let mut user_data_hash = HashMap::new();
        // token address => user ids that own (or borrow) that token
        let mut token_owned_by_user_hash = HashMap::<Address, Vec<Address>>::new();
        // token address => user ids that own (or borrow) that token AND have low health factor
        let mut token_owned_by_user_hash_with_low_health_score =
            HashMap::<Address, Vec<Address>>::new();

        for user in &aave_user_data {
            user_data_hash.insert(user.id, user.clone());

            let has_low_health_factor = user.health_factor
                == BigDecimal::from_f32(HEALTH_FACTOR_THRESHOLD).expect("invalid f32");

            for token in &user.tokens {
                let token_address: Address = token.token.address.parse()?;

                let mut user_ids = token_owned_by_user_hash
                    .entry(token_address)
                    .or_insert_with(Vec::new)
                    .clone();

                let mut low_health_user_ids = token_owned_by_user_hash_with_low_health_score
                    .entry(token_address)
                    .or_insert_with(Vec::new)
                    .clone();

                if has_low_health_factor {
                    low_health_user_ids.push(user.id);
                    token_owned_by_user_hash_with_low_health_score
                        .insert(token_address, low_health_user_ids);
                } else {
                    user_ids.push(user.id);
                    token_owned_by_user_hash.insert(token_address, user_ids);
                }
            }
        }

        Ok(AaveUsersHash {
            user_data: user_data_hash,
            standard_user_ids_by_token: token_owned_by_user_hash,
            low_health_user_ids_by_token: token_owned_by_user_hash_with_low_health_score,
        })
    }
}

#[async_trait]
impl GetUserData for AaveUserData {
    async fn get_collateral_times_liquidation_factor_and_total_debt(
        &self,
        source_for_pricing: PricingSource,
        client: &Arc<Provider<Ws>>,
    ) -> Result<(BigDecimal, BigDecimal), Box<dyn std::error::Error>> {
        let bps_factor = BigDecimal::from_u64(10_u64.pow(4)).unwrap();

        let mut total_debt_usd = BigDecimal::zero();
        let mut liquidation_threshold_collateral_sum = BigDecimal::zero();

        for r in &self.tokens {
            let token = TOKEN_DATA.get(&*r.token.symbol).unwrap();

            // 1. get token price USD
            let token_price_usd = match source_for_pricing {
                PricingSource::UniswapV3 => token.get_token_price_in_("USDC", client).await?,
                PricingSource::AaveOracle => token.get_token_oracle_price(client).await?,
                PricingSource::SavedTokenPrice => {
                    token.get_saved_price_from_token_price_hash().await?
                }
            };

            // 2. get get current total debt in USD
            // *************************************
            let current_total_debt = r.current_total_debt.clone();
            // *************************************

            if current_total_debt > BigDecimal::zero() {
                let current_total_debt_usd = &current_total_debt * &token_price_usd;

                // 3. add current total debt to total debt
                total_debt_usd += &current_total_debt_usd;
            }

            if r.usage_as_collateral_enabled {
                // 4. get atoken balance in USD
                // *************************************
                let current_atoken_balance = r.current_atoken_balance.clone();
                // *************************************

                if current_atoken_balance > BigDecimal::zero() {
                    let current_atoken_usd = &current_atoken_balance * &token_price_usd;

                    // 5. update liquidity threshold colleral sum
                    let liquidation_threshold = r.reserve_liquidation_threshold.clone();
                    // *************************************

                    liquidation_threshold_collateral_sum +=
                        current_atoken_usd * &liquidation_threshold / &bps_factor;
                }
            }
        }

        Ok((liquidation_threshold_collateral_sum, total_debt_usd))
    }
}

#[async_trait]
impl UpdateUserData for AaveUserData {
    async fn update_meta_data(
        &mut self,
        source_for_pricing: PricingSource,
        client: &Arc<Provider<Ws>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (colladeral_times_liquidation_factor, total_debt) = self
            .get_collateral_times_liquidation_factor_and_total_debt(source_for_pricing, client)
            .await?;

        self.colladeral_times_liquidation_factor = colladeral_times_liquidation_factor;
        self.total_debt = total_debt;

        // now with updated  debt and collateral values ,  we can find health factor
        let health_factor = self
            .get_health_factor_from_(source_for_pricing, client)
            .await?;

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
    ) -> Result<BigDecimal, Box<dyn std::error::Error>> {
        let (liquidation_threshold_collateral_sum, current_total_debt) = self
            .get_collateral_times_liquidation_factor_and_total_debt(source_for_pricing, &client)
            .await?;

        let health_factor = if current_total_debt > BigDecimal::zero() {
            liquidation_threshold_collateral_sum / current_total_debt
        } else {
            println!("No valid health factor");
            BigDecimal::from(0)
        };
        Ok(health_factor)
    }

    async fn get_and_update_health_factor_with_(
        &mut self,
        source_for_pricing: PricingSource,
        client: &Arc<Provider<Ws>>,
    ) -> Result<BigDecimal, Box<dyn std::error::Error>> {
        // obtain latest health factor
        let health_factor = self
            .get_health_factor_from_(source_for_pricing, client)
            .await?;

        self.health_factor = health_factor.clone();

        Ok(health_factor)
    }

    async fn is_user_valid_when_checking_against_official_health_factor(
        &mut self,
        client: &Arc<Provider<Ws>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let aave_v3_pool = AAVE_V3_POOL::new(*AAVE_V3_POOL_ADDRESS, client.clone());

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
        // println!("health factor {}", health_factor);

        let aave_user_calculated_health_factor = self
            .get_health_factor_from_(PricingSource::AaveOracle, &client)
            .await?;

        // CHECK that health factor calculated from user data is
        // within + or - 5% of official health factor we get from
        // getUserAccount contract call
        let lower_bound = BigDecimal::from_str("0.95")? * &health_factor;
        let upper_bound = BigDecimal::from_str("1.05")? * &health_factor;

        Ok(aave_user_calculated_health_factor > lower_bound
            && aave_user_calculated_health_factor < upper_bound)
    }
}
