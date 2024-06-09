use crate::data::erc20::Erc20Token;
use bigdecimal::BigDecimal;
use ethers::abi::Address;
use std::collections::HashMap;

pub const HEALTH_FACTOR_THRESHOLD: f32 = 1.1;
pub const DEFAULT_LIQUIDATION_CLOSE_FACTOR: f32 = 0.5;
/**
 * @dev Maximum percentage of borrower's debt to be repaid in a liquidation
 * @dev Percentage applied when the users health factor is below `CLOSE_FACTOR_HF_THRESHOLD`
 * Expressed in bps, a value of 1e4 results in 100.00%
 */
pub const MAX_LIQUIDATION_CLOSE_FACTOR: f32 = 1.0;

/**
 * @dev This constant represents below which health factor value it is possible to liquidate
 * an amount of debt corresponding to `MAX_LIQUIDATION_CLOSE_FACTOR`.
 * A value of 0.95e18 results in 0.95
 */
pub const CLOSE_FACTOR_HF_THRESHOLD: f32 = 0.95;

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
