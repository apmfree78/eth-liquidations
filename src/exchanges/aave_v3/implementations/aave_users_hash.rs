use crate::data::erc20::Erc20Token;
use crate::data::token_data_hash::{get_tokens_priced_in_btc, get_tokens_priced_in_eth};
use crate::exchanges::aave_v3::user_structs::{
    LiquidationCandidate, LIQUIDATION_THRESHOLD, LIQUIDATION_THRESHOLD_LOWER_BOUND,
    PROFIT_THRESHOLD_MAINNET,
};

use super::super::get_user_from_contract::get_aave_v3_user_from_data_provider;
use super::super::user_structs::{
    AaveUsersHash, PricingSource, UserType, UsersToLiquidate, HEALTH_FACTOR_THRESHOLD,
};
use super::aave_user_data::{GetUserData, UpdateUserData};
use anyhow::Result;
use async_trait::async_trait;
use bigdecimal::{BigDecimal, FromPrimitive};
use ethers::abi::Address;
use ethers::providers::{Provider, Ws};
use log::{debug, error};
use std::collections::HashSet;
use std::sync::Arc;

pub enum TokenPriceType {
    PricedInETH,
    PricedInBTC,
}

#[async_trait]
pub trait UpdateUsers {
    fn get_hashset_of_whales(&self) -> HashSet<Address>;
    async fn update_users_health_factor_by_token_and_return_liquidation_candidates(
        &mut self,
        token: &Erc20Token,
        user_type: UserType,
        client: &Arc<Provider<Ws>>,
    ) -> Result<UsersToLiquidate>;
    async fn generate_hashset_of_user_by_user_type_for_(
        &mut self,
        token_price_type: TokenPriceType,
        user_type: UserType,
    ) -> Result<HashSet<Address>>;
    async fn generate_hashset_of_user_by_user_type_for_tokens_priced_in_eth(
        &mut self,
        user_type: UserType,
    ) -> Result<HashSet<Address>>;
    fn get_users_owning_token_by_user_type(
        &mut self,
        token: &Erc20Token,
        user_type: UserType,
    ) -> Result<HashSet<Address>>; // get clone of user ids , NOT a mutable reference
    async fn add_new_user(
        &mut self,
        user_to_add: Address,
        client: &Arc<Provider<Ws>>,
    ) -> Result<()>;
    async fn update_token_to_user_mapping_for_all_users_with_token_(
        &mut self,
        token: &Erc20Token,
        client: &Arc<Provider<Ws>>,
    ) -> Result<()>;
    fn intialize_token_user_mapping(&mut self) -> Result<()>;
    // check user health factor and if its in the right token => user id mapping, move if necessary
    async fn update_token_user_mapping_for_(
        &mut self,
        user_id: Address,
        client: &Arc<Provider<Ws>>,
    ) -> Result<()>;
    fn remove_user_from_token_user_mapping(
        &mut self,
        user_id: Address,
        token: Address,
    ) -> Result<()>;
    async fn move_user_from_standard_to_low_health_token_user_mapping(
        &mut self,
        user_id: Address,
        client: &Arc<Provider<Ws>>,
    ) -> Result<()>;
    fn move_user_from_low_health_to_standard_token_user_mapping(
        &mut self,
        user_id: Address,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

#[async_trait]
impl UpdateUsers for AaveUsersHash {
    async fn add_new_user(
        &mut self,
        user_to_add: Address,
        client: &Arc<Provider<Ws>>,
    ) -> Result<()> {
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
                            .or_default()
                            .clone();

                        users.insert(user_to_add);

                        self.standard_user_ids_by_token.insert(token_address, users);
                    }
                } else {
                    // LOW HEALTH SCORE USERS
                    for token in &user.tokens {
                        let token_address: Address = token.token.address.parse()?;

                        let mut users = self
                            .low_health_user_ids_by_token
                            .entry(token_address)
                            .or_default()
                            .clone();

                        users.insert(user_to_add);

                        self.low_health_user_ids_by_token
                            .insert(token_address, users);
                    }
                }
                self.user_data.insert(user.id, user);
                debug!("new user successfully added",)
            }
            Err(_) => {
                // warn!("user did not fit criteria ==> {}", error),
            }
        };
        Ok(())
    }

    fn intialize_token_user_mapping(&mut self) -> Result<()> {
        for user in self.user_data.values() {
            // NOTE =====> THIS ASSUMES user health factor is VALID
            let has_low_health_factor = user.health_factor
                <= BigDecimal::from_f32(HEALTH_FACTOR_THRESHOLD).expect("invalid f32");

            for token in &user.tokens {
                let token_address: Address = token.token.address.parse()?;

                let mut standard_user_ids = self
                    .standard_user_ids_by_token
                    .entry(token_address)
                    .or_default()
                    .clone();

                let mut low_health_user_ids = self
                    .low_health_user_ids_by_token
                    .entry(token_address)
                    .or_default()
                    .clone();

                if has_low_health_factor {
                    low_health_user_ids.insert(user.id);
                    self.low_health_user_ids_by_token
                        .insert(token_address, low_health_user_ids);
                } else {
                    standard_user_ids.insert(user.id);
                    self.standard_user_ids_by_token
                        .insert(token_address, standard_user_ids);
                }
            }
        }

        Ok(())
    }

    async fn update_token_to_user_mapping_for_all_users_with_token_(
        &mut self,
        token: &Erc20Token,
        client: &Arc<Provider<Ws>>,
    ) -> Result<()> {
        if token.symbol == "BTC" {
            return Ok(());
        }
        let low_health_users =
            self.get_users_owning_token_by_user_type(token, UserType::LowHealth)?;

        for user_id in low_health_users {
            self.update_token_user_mapping_for_(user_id, client).await?;
        }

        let standard_users = self.get_users_owning_token_by_user_type(token, UserType::Standard)?;

        for user_id in standard_users {
            self.update_token_user_mapping_for_(user_id, client).await?;
        }

        Ok(())
    }

    async fn update_token_user_mapping_for_(
        &mut self,
        user_id: Address,
        client: &Arc<Provider<Ws>>,
    ) -> Result<()> {
        // get user health factor

        // if health factor is less than HEALTH_FACTOR_THRESHOLD and profit potential is greater than PROFIT_THRESHOLD then
        // then user belongs in  low health factor hash map
        let user = self.user_data.get(&user_id).expect("Invalid user id");
        let health_factor = &user.health_factor;
        let (user_profit_potential, _, _) =
            user.get_user_liquidation_usd_profit(health_factor).await?;

        let low_health_factor =
            health_factor <= &BigDecimal::from_f32(HEALTH_FACTOR_THRESHOLD).expect("invalid f32");
        let profitable = user_profit_potential
            > BigDecimal::from_f32(PROFIT_THRESHOLD_MAINNET).expect("invalid f32");

        if low_health_factor && profitable {
            self.move_user_from_standard_to_low_health_token_user_mapping(user_id, client)
                .await?;
        } else {
            self.move_user_from_low_health_to_standard_token_user_mapping(user_id)
                .unwrap_or_else(|err| error!("could not move user to new mapping => {}", err));
        }

        Ok(())
    }

    // standard mapping ===> low health factor mapping
    async fn move_user_from_standard_to_low_health_token_user_mapping(
        &mut self,
        user_id: Address,
        client: &Arc<Provider<Ws>>,
    ) -> Result<()> {
        let user = self.user_data.get(&user_id).expect("Invalid user id");
        let mut user_transfered_to_low_health = false;

        for token in &user.tokens {
            let token_address: Address = token.token.address.parse()?;

            if self.standard_user_ids_by_token.contains_key(&token_address) {
                let mut standard_users = self
                    .standard_user_ids_by_token
                    .entry(token_address)
                    .or_default()
                    .clone();

                if standard_users.remove(&user_id) {
                    // update mapping with user id removed
                    self.standard_user_ids_by_token
                        .insert(token_address, standard_users);
                }

                // move OR add user id to low health factor mapping if not already present
                let mut low_health_users = self
                    .low_health_user_ids_by_token
                    .entry(token_address)
                    .or_default()
                    .clone();

                if low_health_users.insert(user_id) {
                    self.low_health_user_ids_by_token
                        .insert(token_address, low_health_users);

                    user_transfered_to_low_health = true;
                }
            }
        }

        // if user has been transfered to low health hash then we need to update their data
        if user_transfered_to_low_health {
            match get_aave_v3_user_from_data_provider(user.id, client).await {
                Ok(user) => {
                    self.user_data.insert(user_id, user);
                }
                Err(error) => debug!(
                    "could not get updated user from data provider contract => {}",
                    error
                ),
            };
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
                    .or_default()
                    .clone();

                if low_health_users.remove(&user_id) {
                    // update mapping with user id removed
                    self.low_health_user_ids_by_token
                        .insert(token_address, low_health_users);
                }

                // move OR add user id to standard mapping
                let mut standard_users = self
                    .standard_user_ids_by_token
                    .entry(token_address)
                    .or_default()
                    .clone();

                if standard_users.insert(user_id) {
                    self.standard_user_ids_by_token
                        .insert(token_address, standard_users);
                }
            }
        }
        Ok(())
    }

    fn remove_user_from_token_user_mapping(
        &mut self,
        user_id: Address,
        token_address: Address,
    ) -> Result<()> {
        // CHECK STANDARD MAPPING
        if self.standard_user_ids_by_token.contains_key(&token_address) {
            let mut users = self
                .standard_user_ids_by_token
                .entry(token_address)
                .or_default()
                .clone();

            if users.remove(&user_id) {
                self.standard_user_ids_by_token.insert(token_address, users);

                return Ok(());
            }
        }

        // CHECK LOW HEALTH MAPPING
        if self
            .low_health_user_ids_by_token
            .contains_key(&token_address)
        {
            let mut users = self
                .low_health_user_ids_by_token
                .entry(token_address)
                .or_default()
                .clone();

            if users.remove(&user_id) {
                self.low_health_user_ids_by_token
                    .insert(token_address, users);
            }
        }

        Ok(())
    }

    async fn update_users_health_factor_by_token_and_return_liquidation_candidates(
        &mut self,
        main_token: &Erc20Token,
        user_type: UserType,
        client: &Arc<Provider<Ws>>,
    ) -> Result<UsersToLiquidate> {
        // track already updated users and liquidation candidates
        let mut liquidation_candidates = Vec::<LiquidationCandidate>::new();

        let user_ids_array = match main_token.symbol.as_str() {
            "WETH" => {
                let users = self
                    .generate_hashset_of_user_by_user_type_for_(
                        TokenPriceType::PricedInETH,
                        user_type,
                    )
                    .await?;
                users.into_iter().collect()
            }
            "BTC" => {
                let users = self
                    .generate_hashset_of_user_by_user_type_for_(
                        TokenPriceType::PricedInBTC,
                        user_type,
                    )
                    .await?;
                users.into_iter().collect()
            }
            _ => self.get_users_owning_token_by_user_type(main_token, user_type)?,
        };

        for user_id in user_ids_array {
            let user = self
                .user_data
                .get_mut(&user_id)
                .unwrap_or_else(|| panic!("could not find user with user id"));

            user.update_meta_data(PricingSource::SavedTokenPrice, client)
                .await?;

            if user.health_factor < BigDecimal::from_f32(LIQUIDATION_THRESHOLD).unwrap()
                && user.health_factor
                    > BigDecimal::from_f32(LIQUIDATION_THRESHOLD_LOWER_BOUND).unwrap()
            {
                // now check for user profitability
                let (profitability, debt_token, collateral_token) = user
                    .get_user_liquidation_usd_profit(&user.health_factor)
                    .await?;

                if profitability > BigDecimal::from_f32(PROFIT_THRESHOLD_MAINNET).unwrap() {
                    liquidation_candidates.push(LiquidationCandidate {
                        user_id: user.id,
                        estimated_profit: profitability,
                        debt_token,
                        collateral_token,
                    });
                }
            }
        }

        if liquidation_candidates.is_empty() {
            Ok(UsersToLiquidate::None)
        } else {
            Ok(UsersToLiquidate::Users(liquidation_candidates))
        }
    }

    async fn generate_hashset_of_user_by_user_type_for_(
        &mut self,
        token_price_type: TokenPriceType,
        user_type: UserType,
    ) -> Result<HashSet<Address>> {
        let mut users_that_own_token_price_type_tokens = HashSet::<Address>::new();

        let token_price_type_tokens = match token_price_type {
            TokenPriceType::PricedInETH => get_tokens_priced_in_eth().await?,
            TokenPriceType::PricedInBTC => get_tokens_priced_in_btc().await?,
        };

        for token in token_price_type_tokens.values() {
            match user_type {
                UserType::LowHealth => {
                    let low_health_users =
                        self.get_users_owning_token_by_user_type(token, UserType::LowHealth)?;
                    debug!(
                        "{} user with low health score have {}",
                        low_health_users.len(),
                        token.symbol
                    );

                    if !low_health_users.is_empty() {
                        users_that_own_token_price_type_tokens.extend(low_health_users);
                    }
                }
                UserType::Standard => {
                    let standard_users =
                        self.get_users_owning_token_by_user_type(token, UserType::Standard)?;

                    // debug!(
                    //     "{} standard users have {}",
                    //     standard_users.len(),
                    //     token.symbol
                    // );
                    if !standard_users.is_empty() {
                        users_that_own_token_price_type_tokens.extend(standard_users);
                    }
                }
            }
        }
        // debug!(
        //     "hashset of tokens connected to ETH contain {} unique users",
        //     users_with_tokens_priced_in_eth.len()
        // );
        Ok(users_that_own_token_price_type_tokens)
    }

    async fn generate_hashset_of_user_by_user_type_for_tokens_priced_in_eth(
        &mut self,
        user_type: UserType,
    ) -> Result<HashSet<Address>> {
        let mut users_with_tokens_priced_in_eth = HashSet::<Address>::new();
        let token_price_priced_in_eth = get_tokens_priced_in_eth().await?;

        for token in token_price_priced_in_eth.values() {
            match user_type {
                UserType::LowHealth => {
                    let low_health_users =
                        self.get_users_owning_token_by_user_type(token, UserType::LowHealth)?;
                    debug!(
                        "{} user with low health score have {}",
                        low_health_users.len(),
                        token.symbol
                    );

                    if !low_health_users.is_empty() {
                        users_with_tokens_priced_in_eth.extend(low_health_users);
                    }
                }
                UserType::Standard => {
                    let standard_users =
                        self.get_users_owning_token_by_user_type(token, UserType::Standard)?;

                    // debug!(
                    //     "{} standard users have {}",
                    //     standard_users.len(),
                    //     token.symbol
                    // );
                    if !standard_users.is_empty() {
                        users_with_tokens_priced_in_eth.extend(standard_users);
                    }
                }
            }
        }
        // debug!(
        //     "hashset of tokens connected to ETH contain {} unique users",
        //     users_with_tokens_priced_in_eth.len()
        // );
        Ok(users_with_tokens_priced_in_eth)
    }

    fn get_users_owning_token_by_user_type(
        &mut self,
        token: &Erc20Token,
        user_type: UserType,
    ) -> Result<HashSet<Address>> {
        let token_address: Address = token.address.parse()?;

        match user_type {
            UserType::LowHealth => {
                let low_health_users = self
                    .low_health_user_ids_by_token
                    .get(&token_address)
                    .unwrap_or_else(|| {
                        // TODO should we really panic here?
                        panic!(
                            "invalid low_health_user_ids_by_token {} => {}",
                            token.symbol, token.address
                        )
                    });

                Ok(low_health_users.to_owned())
            }
            UserType::Standard => {
                let standard_users = self
                    .standard_user_ids_by_token
                    .get(&token_address)
                    .unwrap_or_else(|| panic!("invalid standard_user_ids_by_token"));

                Ok(standard_users.to_owned())
            }
        }
    }

    fn get_hashset_of_whales(&self) -> HashSet<Address> {
        let mut whales = HashSet::<Address>::new();

        for user_hashset in self.low_health_user_ids_by_token.values() {
            whales.extend(user_hashset.iter());
        }

        whales
    }
}
