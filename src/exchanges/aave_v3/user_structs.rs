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

pub const HEALTH_FACTOR_THRESHOLD: f32 = 1.1;

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
