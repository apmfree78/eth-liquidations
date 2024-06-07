use super::super::get_user_from_contract::get_aave_v3_user_from_data_provider;
use super::super::user_structs::{
    AaveUsersHash, PricingSource, UserType, UsersToLiquidate, HEALTH_FACTOR_THRESHOLD,
};
use super::aave_user_data::UpdateUserData;
use async_trait::async_trait;
use bigdecimal::{BigDecimal, FromPrimitive};
use ethers::abi::Address;
use ethers::providers::{Provider, Ws};
// use std::collections::HashMap;
// use std::str::FromStr;
use std::sync::Arc;

#[async_trait]
pub trait UpdateUsers {
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
    fn update_token_to_user_mapping_for_all_users_with_token_(
        &mut self,
        token_address: Address,
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
                            .or_default()
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

    fn update_token_to_user_mapping_for_all_users_with_token_(
        &mut self,
        token_address: Address,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let low_health_users = self
            .low_health_user_ids_by_token
            .entry(token_address)
            .or_default()
            .clone();

        for user in low_health_users {
            self.update_token_to_user_mapping_for_(user)?;
        }

        let standard_users = self
            .standard_user_ids_by_token
            .entry(token_address)
            .or_default()
            .clone();

        for user in standard_users {
            self.update_token_to_user_mapping_for_(user)?;
        }

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
                    .or_default()
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
                    .or_default()
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
                    .or_default()
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
                    .or_default()
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
                .or_default()
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
                .or_default()
                .clone();

            if let Some(index) = users.iter().position(|&id| id == user_id) {
                users.remove(index);

                self.low_health_user_ids_by_token
                    .insert(token_address, users);
            };
        }

        Ok(())
    }

    // TODO --> REFACTOR THIS , will only update token price of token that was updated by Transmit
    async fn update_users_health_factor_by_token_and_return_liquidation_candidates(
        &mut self,
        token_address: Address,
        user_type: UserType,
        client: &Arc<Provider<Ws>>,
    ) -> Result<UsersToLiquidate, Box<dyn std::error::Error>> {
        // track already updated users and liquidation candidates
        let mut liquidation_candidates = Vec::<Address>::new();

        match user_type {
            UserType::LowHealth => {
                let low_health_users = self
                    .low_health_user_ids_by_token
                    .entry(token_address)
                    .or_default();

                for user_id in low_health_users {
                    let user = self
                        .user_data
                        .get_mut(user_id)
                        .unwrap_or_else(|| panic!("could not find user with user id"));

                    user.update_meta_data(PricingSource::SavedTokenPrice, client)
                        .await?;

                    // TODO - find if user has health factor < 1 and add to arry if true
                    if user.health_factor < BigDecimal::from_f32(HEALTH_FACTOR_THRESHOLD).unwrap() {
                        liquidation_candidates.push(user.id);
                    }
                }
            }
            UserType::Standard => {
                let standard_users = self
                    .standard_user_ids_by_token
                    .entry(token_address)
                    .or_default();

                for user_id in standard_users {
                    let user = self
                        .user_data
                        .get_mut(user_id)
                        .unwrap_or_else(|| panic!("could not find user with user id"));

                    user.update_meta_data(PricingSource::SavedTokenPrice, client)
                        .await?;

                    // TODO - find if user has health factor < 1 and add to arry if true
                    if user.health_factor < BigDecimal::from_f32(HEALTH_FACTOR_THRESHOLD).unwrap() {
                        liquidation_candidates.push(user.id);
                    }
                }
            }
        }

        if liquidation_candidates.is_empty() {
            Ok(UsersToLiquidate::None)
        } else {
            Ok(UsersToLiquidate::Users(liquidation_candidates))
        }
    }
}
