use crate::data::erc20::Erc20Token;
use bigdecimal::BigDecimal;
use ethers::{abi::Address, types::U256};
use std::collections::{HashMap, HashSet};

pub const HEALTH_FACTOR_THRESHOLD: f32 = 1.1;
pub const DEFAULT_LIQUIDATION_CLOSE_FACTOR: f32 = 0.5;
pub const PROFIT_THRESHOLD_MAINNET: f32 = 1.0; // raise to $10 for prod
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
pub const LIQUIDATION_THRESHOLD: f32 = 1.00;
pub const BPS_FACTOR: u64 = 10_u64.pow(4);

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

#[derive(PartialEq)]
pub enum LiquidationCloseFactor {
    Full,
    Half,
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
pub struct AaveTokenU256 {
    pub token: Erc20Token,
    pub current_total_debt: U256,
    pub usage_as_collateral_enabled: bool,
    pub current_atoken_balance: U256,
    pub reserve_liquidation_threshold: U256,
    pub reserve_liquidation_bonus: U256,
}

#[derive(Clone, Debug)]
pub struct AaveUserData {
    pub id: Address,
    pub total_debt: BigDecimal,
    pub collateral_times_liquidation_factor: BigDecimal,
    pub tokens: Vec<AaveToken>,
    pub health_factor: BigDecimal,
}

#[derive(Debug)]
pub struct LiquidationArgs {
    pub collateral: Address,
    pub debt: Address,
    pub user: Address,
    pub debt_to_cover: U256,
    pub _receive_a_token: bool,
}

#[derive(Debug, Clone)]
pub struct LiquidationCandidate {
    pub user: Address,
    pub estimated_profit: BigDecimal,
    pub debt_token: Address,       // highest debt token
    pub collateral_token: Address, // biggest collateral token
}

/*
This below struct is single source of truth for all Aave v3 user data,
this is the main data source that will be kept up to date based on user
events found in logs and changes in token prices
 */
#[derive(Clone, Debug)]
pub struct AaveUsersHash {
    pub user_data: HashMap<Address, AaveUserData>,
    pub standard_user_ids_by_token: HashMap<Address, HashSet<Address>>,
    pub low_health_user_ids_by_token: HashMap<Address, HashSet<Address>>,
}

pub enum UsersToLiquidate {
    Users(Vec<LiquidationCandidate>),
    None,
}

#[derive(Clone, Copy, Debug)]
pub enum UserType {
    Standard,
    LowHealth,
}
