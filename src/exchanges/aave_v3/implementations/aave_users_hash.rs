use crate::data::erc20::{Erc20Token, TOKENS_WITH_PRICE_CONNECTED_TO_ETH};
use crate::exchanges::aave_v3::user_structs::CLOSE_FACTOR_HF_THRESHOLD;

use super::super::get_user_from_contract::get_aave_v3_user_from_data_provider;
use super::super::user_structs::{
    AaveUsersHash, PricingSource, UserType, UsersToLiquidate, HEALTH_FACTOR_THRESHOLD,
};
use super::aave_user_data::UpdateUserData;
use async_trait::async_trait;
use bigdecimal::{BigDecimal, FromPrimitive};
use ethers::abi::Address;
use ethers::providers::{Provider, Ws};
use log::{debug, error, warn};
use std::collections::HashSet;
use std::sync::Arc;

#[async_trait]
pub trait UpdateUsers {
    async fn update_users_health_factor_by_token_and_return_liquidation_candidates(
        &mut self,
        token: &Erc20Token,
        user_type: UserType,
        client: &Arc<Provider<Ws>>,
    ) -> Result<UsersToLiquidate, Box<dyn std::error::Error>>;
    fn generate_hashset_of_user_by_user_type_for_tokens_connected_to_eth(
        &mut self,
        user_type: UserType,
    ) -> Result<HashSet<Address>, Box<dyn std::error::Error>>;
    fn get_users_owning_token_by_user_type(
        &mut self,
        token: &Erc20Token,
        user_type: UserType,
    ) -> Result<HashSet<Address>, Box<dyn std::error::Error>>; // get clone of user ids , NOT a mutable reference
    async fn add_new_user(
        &mut self,
        user_to_add: Address,
        client: &Arc<Provider<Ws>>,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn update_token_to_user_mapping_for_all_users_with_token_(
        &mut self,
        token: &Erc20Token,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn intialize_token_user_mapping(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    // check user health factor and if its in the right token => user id mapping, move if necessary
    fn update_token_user_mapping_for_(
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
            Err(error) => warn!("user did not fit criteria ==> {}", error),
        };
        Ok(())
    }

    fn intialize_token_user_mapping(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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

    fn update_token_to_user_mapping_for_all_users_with_token_(
        &mut self,
        token: &Erc20Token,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let low_health_users =
            self.get_users_owning_token_by_user_type(token, UserType::LowHealth)?;

        for user_id in low_health_users {
            self.update_token_user_mapping_for_(user_id)?;
        }

        let standard_users = self.get_users_owning_token_by_user_type(token, UserType::Standard)?;

        for user_id in standard_users {
            self.update_token_user_mapping_for_(user_id)?;
        }

        Ok(())
    }

    fn update_token_user_mapping_for_(
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
                .unwrap_or_else(|err| error!("could not move user to new mapping => {}", err));
        } else {
            self.move_user_from_low_health_to_standard_token_user_mapping(user_id)
                .unwrap_or_else(|err| error!("could not move user to new mapping => {}", err));
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
                }
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
    ) -> Result<(), Box<dyn std::error::Error>> {
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
    ) -> Result<UsersToLiquidate, Box<dyn std::error::Error>> {
        // track already updated users and liquidation candidates
        let mut liquidation_candidates = Vec::<Address>::new();

        let user_ids_array = if main_token.symbol == "WETH" {
            // must update full list of tokens if WETH
            let users_with_tokens_connected_to_eth =
                self.generate_hashset_of_user_by_user_type_for_tokens_connected_to_eth(user_type)?;
            users_with_tokens_connected_to_eth.into_iter().collect()
        } else {
            self.get_users_owning_token_by_user_type(main_token, user_type)?
        };

        for user_id in user_ids_array {
            let user = self
                .user_data
                .get_mut(&user_id)
                .unwrap_or_else(|| panic!("could not find user with user id"));

            user.update_meta_data(PricingSource::SavedTokenPrice, client)
                .await?;

            if user.health_factor < BigDecimal::from_f32(CLOSE_FACTOR_HF_THRESHOLD).unwrap() {
                liquidation_candidates.push(user.id);
            }
        }

        if liquidation_candidates.is_empty() {
            Ok(UsersToLiquidate::None)
        } else {
            Ok(UsersToLiquidate::Users(liquidation_candidates))
        }
    }

    fn generate_hashset_of_user_by_user_type_for_tokens_connected_to_eth(
        &mut self,
        user_type: UserType,
    ) -> Result<HashSet<Address>, Box<dyn std::error::Error>> {
        let mut users_with_tokens_connected_to_eth = HashSet::<Address>::new();
        for token in TOKENS_WITH_PRICE_CONNECTED_TO_ETH.iter() {
            match user_type {
                UserType::LowHealth => {
                    let low_health_users =
                        self.get_users_owning_token_by_user_type(token, UserType::LowHealth)?;

                    users_with_tokens_connected_to_eth.extend(low_health_users);
                }
                UserType::Standard => {
                    let standard_users =
                        self.get_users_owning_token_by_user_type(token, UserType::Standard)?;

                    users_with_tokens_connected_to_eth.extend(standard_users);
                }
            }
        }
        Ok(users_with_tokens_connected_to_eth)
    }

    fn get_users_owning_token_by_user_type(
        &mut self,
        token: &Erc20Token,
        user_type: UserType,
    ) -> Result<HashSet<Address>, Box<dyn std::error::Error>> {
        let token_address: Address = token.address.parse()?;

        match user_type {
            UserType::LowHealth => {
                let low_health_users = self
                    .low_health_user_ids_by_token
                    .get(&token_address)
                    .unwrap_or_else(|| panic!("invalid low_health_user_ids_by_token"));

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
}
