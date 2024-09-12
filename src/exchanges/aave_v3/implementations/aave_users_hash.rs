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
    async fn intialize_token_user_mapping(&mut self) -> Result<()>;
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
    async fn move_user_from_standard_to_whale_user_mapping(
        &mut self,
        user_id: Address,
        client: &Arc<Provider<Ws>>,
    ) -> Result<()>;
    fn move_user_from_whale_to_standard_token_user_mapping(
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
                if user.health_factor > HEALTH_FACTOR_THRESHOLD {
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
                    // WHALE USERS
                    for token in &user.tokens {
                        let token_address: Address = token.token.address.parse()?;

                        let mut users = self
                            .whale_user_ids_by_token
                            .entry(token_address)
                            .or_default()
                            .clone();

                        users.insert(user_to_add);

                        self.whale_user_ids_by_token.insert(token_address, users);
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

    async fn intialize_token_user_mapping(&mut self) -> Result<()> {
        for user in self.user_data.values() {
            // NOTE =====> THIS ASSUMES user health factor is VALID
            let (user_profit_potential, _, _) = user
                .get_user_liquidation_usd_profit(user.health_factor)
                .await?;
            let is_whale_user = user.health_factor <= HEALTH_FACTOR_THRESHOLD
                && user_profit_potential >= PROFIT_THRESHOLD_MAINNET;

            for token in &user.tokens {
                let token_address: Address = token.token.address.parse()?;

                let mut standard_user_ids = self
                    .standard_user_ids_by_token
                    .entry(token_address)
                    .or_default()
                    .clone();

                let mut whale_user_ids = self
                    .whale_user_ids_by_token
                    .entry(token_address)
                    .or_default()
                    .clone();

                if is_whale_user {
                    whale_user_ids.insert(user.id);
                    self.whale_user_ids_by_token
                        .insert(token_address, whale_user_ids);
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
        let whale_users = self.get_users_owning_token_by_user_type(token, UserType::Whale)?;

        for user_id in whale_users {
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
        // then user belongs in wahle hash map
        let user = self.user_data.get(&user_id).expect("Invalid user id");
        let health_factor = user.health_factor;
        let (user_profit_potential, _, _) =
            user.get_user_liquidation_usd_profit(health_factor).await?;

        let low_health_factor = health_factor <= HEALTH_FACTOR_THRESHOLD;
        let profitable = user_profit_potential > PROFIT_THRESHOLD_MAINNET;

        if low_health_factor && profitable {
            self.move_user_from_standard_to_whale_user_mapping(user_id, client)
                .await?;
        } else {
            self.move_user_from_whale_to_standard_token_user_mapping(user_id)
                .unwrap_or_else(|err| error!("could not move user to new mapping => {}", err));
        }

        Ok(())
    }

    // standard mapping ===> whale factor mapping
    async fn move_user_from_standard_to_whale_user_mapping(
        &mut self,
        user_id: Address,
        client: &Arc<Provider<Ws>>,
    ) -> Result<()> {
        let user = self.user_data.get(&user_id).expect("Invalid user id");
        let mut user_transfered_to_whale = false;

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

                // move OR add user id to whale mapping if not already present
                let mut whale_users = self
                    .whale_user_ids_by_token
                    .entry(token_address)
                    .or_default()
                    .clone();

                if whale_users.insert(user_id) {
                    self.whale_user_ids_by_token
                        .insert(token_address, whale_users);

                    user_transfered_to_whale = true;
                }
            }
        }

        // if user has been transfered to whale hash then we need to update their data
        if user_transfered_to_whale {
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

    // whale mapping ===> standard mapping
    fn move_user_from_whale_to_standard_token_user_mapping(
        &mut self,
        user_id: Address,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let user = self.user_data.get(&user_id).expect("Invalid user id");

        for token in &user.tokens {
            let token_address: Address = token.token.address.parse()?;

            if self.whale_user_ids_by_token.contains_key(&token_address) {
                let mut whale_users = self
                    .whale_user_ids_by_token
                    .entry(token_address)
                    .or_default()
                    .clone();

                if whale_users.remove(&user_id) {
                    // update mapping with user id removed
                    self.whale_user_ids_by_token
                        .insert(token_address, whale_users);
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

        // CHECK WHALE MAPPING
        if self.whale_user_ids_by_token.contains_key(&token_address) {
            let mut users = self
                .whale_user_ids_by_token
                .entry(token_address)
                .or_default()
                .clone();

            if users.remove(&user_id) {
                self.whale_user_ids_by_token.insert(token_address, users);
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

            if user.health_factor < LIQUIDATION_THRESHOLD
                && user.health_factor > LIQUIDATION_THRESHOLD_LOWER_BOUND
            {
                // now check for user profitability
                let (profitability, debt_token, collateral_token) = user
                    .get_user_liquidation_usd_profit(user.health_factor)
                    .await?;

                if profitability > PROFIT_THRESHOLD_MAINNET {
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
                UserType::Whale => {
                    let whale_users =
                        self.get_users_owning_token_by_user_type(token, UserType::Whale)?;
                    debug!(
                        "{} user with low health score have {}",
                        whale_users.len(),
                        token.symbol
                    );

                    if !whale_users.is_empty() {
                        users_that_own_token_price_type_tokens.extend(whale_users);
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

    fn get_users_owning_token_by_user_type(
        &mut self,
        token: &Erc20Token,
        user_type: UserType,
    ) -> Result<HashSet<Address>> {
        let token_address: Address = token.address.parse()?;

        match user_type {
            UserType::Whale => {
                let whale_users = self
                    .whale_user_ids_by_token
                    .get(&token_address)
                    .unwrap_or_else(|| {
                        // TODO should we really panic here?
                        panic!(
                            "invalid whale_user_ids_by_token {} => {}",
                            token.symbol, token.address
                        )
                    });

                Ok(whale_users.to_owned())
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

        for user_hashset in self.whale_user_ids_by_token.values() {
            whales.extend(user_hashset.iter());
        }

        whales
    }
}
