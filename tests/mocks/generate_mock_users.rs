use bigdecimal::{BigDecimal, FromPrimitive};
use eth_liquadation::data::erc20::Erc20Token;
use eth_liquadation::exchanges::aave_v3::implementations::aave_users_hash::UpdateUsers;
use eth_liquadation::exchanges::aave_v3::user_structs::{AaveToken, AaveUserData, AaveUsersHash};
use ethers::abi::Address;
use std::collections::HashMap;

pub const USDT_USER_DEBT: u64 = 26000000000;
pub const USDT_USER_BALANCE: u64 = 30000000000;
pub const WETH_USER_BALANCE: u128 = 10000000000000000000;

pub fn generate_mock_user_hash() -> Result<AaveUsersHash, Box<dyn std::error::Error + Send + Sync>>
{
    let user_address: Address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;
    let user_data = AaveUserData {
        id: user_address,
        total_debt: BigDecimal::from_u64(2603060364429).unwrap(),
        collateral_times_liquidation_factor: BigDecimal::from_f32(4023256458369.85).unwrap(),
        tokens: vec![
            AaveToken {
                token: Erc20Token {
                    name: "Wrapped Ether".to_string(),
                    symbol: "WETH".to_string(),
                    decimals: 18,
                    address: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".to_string(),
                    liquidation_bonus: 10500,
                    liquidation_threshold: 8300,
                    chain_link_price_feed: "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419".to_string(),
                    chainlink_aggregator: "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419".to_string(),
                },
                current_total_debt: BigDecimal::from(0),
                usage_as_collateral_enabled: true,
                current_atoken_balance: BigDecimal::from_u128(WETH_USER_BALANCE).unwrap(),
                reserve_liquidation_threshold: BigDecimal::from(8300),
                reserve_liquidation_bonus: BigDecimal::from(10500),
            },
            AaveToken {
                token: Erc20Token {
                    name: "Tether USD".to_string(),
                    symbol: "USDT".to_string(),
                    decimals: 6,
                    address: "0xdac17f958d2ee523a2206206994597c13d831ec7".to_string(),
                    liquidation_bonus: 10450,
                    liquidation_threshold: 7800,
                    chain_link_price_feed: "0x3E7d1eAB13ad0104d2750B8863b489D65364e32D".to_string(),
                    chainlink_aggregator: "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419".to_string(),
                },
                current_total_debt: BigDecimal::from_u64(USDT_USER_DEBT).unwrap(),
                usage_as_collateral_enabled: true,
                current_atoken_balance: BigDecimal::from_u64(USDT_USER_BALANCE).unwrap(),
                reserve_liquidation_threshold: BigDecimal::from(7800),
                reserve_liquidation_bonus: BigDecimal::from(10450),
            },
        ],
        health_factor: BigDecimal::from_f32(0.900478).unwrap(),
    };

    // health factor => (30000000000 * 0.78/10^6 + 15000000000000000000 * 0.83/10^18)/(26000000000/10^6)

    let mut user_hash = HashMap::new();
    user_hash.insert(user_address, user_data);

    let mut aave_users_hash = AaveUsersHash {
        user_data: user_hash,
        standard_user_ids_by_token: HashMap::new(),
        low_health_user_ids_by_token: HashMap::new(),
    };

    aave_users_hash.intialize_token_user_mapping()?;

    println!("users hash => {:#?}", aave_users_hash);

    Ok(aave_users_hash)
}

pub fn generate_mock_2_user_hash() -> Result<AaveUsersHash, Box<dyn std::error::Error + Send + Sync>>
{
    let user_address: Address = "0x922389be330d20bfb132faf5c73ee0fd81e9ad21".parse()?;
    let user_data = AaveUserData {
        id: user_address,
        total_debt: BigDecimal::from_u64(2603060364429).unwrap(),
        collateral_times_liquidation_factor: BigDecimal::from_f32(4023256458369.85).unwrap(),
        tokens: vec![
            AaveToken {
                token: Erc20Token {
                    name: "Wrapped Ether".to_string(),
                    symbol: "WETH".to_string(),
                    decimals: 18,
                    address: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".to_string(),
                    liquidation_bonus: 10500,
                    liquidation_threshold: 8300,
                    chain_link_price_feed: "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419".to_string(),
                    chainlink_aggregator: "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419".to_string(),
                },
                current_total_debt: BigDecimal::from(0),
                usage_as_collateral_enabled: true,
                current_atoken_balance: BigDecimal::from_u128(WETH_USER_BALANCE).unwrap(),
                reserve_liquidation_threshold: BigDecimal::from(8300),
                reserve_liquidation_bonus: BigDecimal::from(10500),
            },
            AaveToken {
                token: Erc20Token {
                    name: "Tether USD".to_string(),
                    symbol: "USDT".to_string(),
                    decimals: 6,
                    address: "0xdac17f958d2ee523a2206206994597c13d831ec7".to_string(),
                    liquidation_bonus: 10450,
                    liquidation_threshold: 7800,
                    chain_link_price_feed: "0x3E7d1eAB13ad0104d2750B8863b489D65364e32D".to_string(),
                    chainlink_aggregator: "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419".to_string(),
                },
                current_total_debt: BigDecimal::from_u64(5000000000).unwrap(),
                usage_as_collateral_enabled: true,
                current_atoken_balance: BigDecimal::from(0),
                reserve_liquidation_threshold: BigDecimal::from(7800),
                reserve_liquidation_bonus: BigDecimal::from(10450),
            },
        ],
        health_factor: BigDecimal::from_f32(0.900478).unwrap(),
    };

    // (20000000000000000000/10^18) / (5000000000/10^6)

    let user_address_2: Address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;
    let user_data_2 = AaveUserData {
        id: user_address_2,
        total_debt: BigDecimal::from_u64(2603060364429).unwrap(),
        collateral_times_liquidation_factor: BigDecimal::from_f32(4023256458369.85).unwrap(),
        tokens: vec![
            AaveToken {
                token: Erc20Token {
                    name: "Wrapped Ether".to_string(),
                    symbol: "WETH".to_string(),
                    decimals: 18,
                    address: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".to_string(),
                    liquidation_bonus: 10500,
                    liquidation_threshold: 8300,
                    chain_link_price_feed: "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419".to_string(),
                    chainlink_aggregator: "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419".to_string(),
                },
                current_total_debt: BigDecimal::from(0),
                usage_as_collateral_enabled: true,
                current_atoken_balance: BigDecimal::from_u128(WETH_USER_BALANCE).unwrap(),
                reserve_liquidation_threshold: BigDecimal::from(8300),
                reserve_liquidation_bonus: BigDecimal::from(10500),
            },
            AaveToken {
                token: Erc20Token {
                    name: "Tether USD".to_string(),
                    symbol: "USDT".to_string(),
                    decimals: 6,
                    address: "0xdac17f958d2ee523a2206206994597c13d831ec7".to_string(),
                    liquidation_bonus: 10450,
                    liquidation_threshold: 7800,
                    chain_link_price_feed: "0x3E7d1eAB13ad0104d2750B8863b489D65364e32D".to_string(),
                    chainlink_aggregator: "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419".to_string(),
                },
                current_total_debt: BigDecimal::from_u64(USDT_USER_DEBT).unwrap(),
                usage_as_collateral_enabled: true,
                current_atoken_balance: BigDecimal::from_u64(USDT_USER_BALANCE).unwrap(),
                reserve_liquidation_threshold: BigDecimal::from(7800),
                reserve_liquidation_bonus: BigDecimal::from(10450),
            },
        ],
        health_factor: BigDecimal::from_f32(0.900478).unwrap(),
    };

    let mut user_hash = HashMap::new();
    user_hash.insert(user_address, user_data);
    user_hash.insert(user_address_2, user_data_2);

    let mut aave_users_hash = AaveUsersHash {
        user_data: user_hash,
        standard_user_ids_by_token: HashMap::new(),
        low_health_user_ids_by_token: HashMap::new(),
    };

    aave_users_hash.intialize_token_user_mapping()?;

    println!("users hash => {:#?}", aave_users_hash);

    Ok(aave_users_hash)
}

// first user has liquidation factor of 0.83 while other has 8.3
pub fn generate_mock_2_user_hash_v2(
) -> Result<AaveUsersHash, Box<dyn std::error::Error + Send + Sync>> {
    let user_address: Address = "0x922389be330d20bfb132faf5c73ee0fd81e9ad21".parse()?;
    let user_data = AaveUserData {
        id: user_address,
        total_debt: BigDecimal::from_u64(2603060364429).unwrap(),
        collateral_times_liquidation_factor: BigDecimal::from_f32(4023256458369.85).unwrap(),
        tokens: vec![
            AaveToken {
                token: Erc20Token {
                    name: "USD Coin".to_string(),
                    symbol: "USDC".to_string(),
                    decimals: 6,
                    address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string(),
                    liquidation_bonus: 10450,
                    liquidation_threshold: 7800,
                    chain_link_price_feed: "0x736bF902680e68989886e9807CD7Db4B3E015d3C".to_string(),
                    chainlink_aggregator: "".to_string(),
                },
                current_total_debt: BigDecimal::from(0),
                usage_as_collateral_enabled: true,
                current_atoken_balance: BigDecimal::from_u128(100000000).unwrap(),
                reserve_liquidation_threshold: BigDecimal::from(8300),
                reserve_liquidation_bonus: BigDecimal::from(10500),
            },
            AaveToken {
                token: Erc20Token {
                    name: "Tether USD".to_string(),
                    symbol: "USDT".to_string(),
                    decimals: 6,
                    address: "0xdac17f958d2ee523a2206206994597c13d831ec7".to_string(),
                    liquidation_bonus: 10450,
                    liquidation_threshold: 7800,
                    chain_link_price_feed: "0x3E7d1eAB13ad0104d2750B8863b489D65364e32D".to_string(),
                    chainlink_aggregator: "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419".to_string(),
                },
                current_total_debt: BigDecimal::from_u64(100000000).unwrap(),
                usage_as_collateral_enabled: true,
                current_atoken_balance: BigDecimal::from(0),
                reserve_liquidation_threshold: BigDecimal::from(7800),
                reserve_liquidation_bonus: BigDecimal::from(10450),
            },
        ],
        health_factor: BigDecimal::from_f32(0.83).unwrap(),
    };

    let user_address_2: Address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;
    let user_data_2 = AaveUserData {
        id: user_address_2,
        total_debt: BigDecimal::from_u64(2603060364429).unwrap(),
        collateral_times_liquidation_factor: BigDecimal::from_f32(4023256458369.85).unwrap(),
        tokens: vec![
            AaveToken {
                token: Erc20Token {
                    name: "USD Coin".to_string(),
                    symbol: "USDC".to_string(),
                    decimals: 6,
                    address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string(),
                    liquidation_bonus: 10450,
                    liquidation_threshold: 7800,
                    chain_link_price_feed: "0x736bF902680e68989886e9807CD7Db4B3E015d3C".to_string(),
                    chainlink_aggregator: "".to_string(),
                },
                current_total_debt: BigDecimal::from(0),
                usage_as_collateral_enabled: true,
                current_atoken_balance: BigDecimal::from_u128(10000000).unwrap(),
                reserve_liquidation_threshold: BigDecimal::from(8300),
                reserve_liquidation_bonus: BigDecimal::from(10500),
            },
            AaveToken {
                token: Erc20Token {
                    name: "Tether USD".to_string(),
                    symbol: "USDT".to_string(),
                    decimals: 6,
                    address: "0xdac17f958d2ee523a2206206994597c13d831ec7".to_string(),
                    liquidation_bonus: 10450,
                    liquidation_threshold: 7800,
                    chain_link_price_feed: "0x3E7d1eAB13ad0104d2750B8863b489D65364e32D".to_string(),
                    chainlink_aggregator: "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419".to_string(),
                },
                current_total_debt: BigDecimal::from_u64(1000000).unwrap(),
                usage_as_collateral_enabled: true,
                current_atoken_balance: BigDecimal::from(0),
                reserve_liquidation_threshold: BigDecimal::from(7800),
                reserve_liquidation_bonus: BigDecimal::from(10450),
            },
        ],
        health_factor: BigDecimal::from_f32(8.3).unwrap(),
    };

    let mut user_hash = HashMap::new();
    user_hash.insert(user_address, user_data);
    user_hash.insert(user_address_2, user_data_2);

    let mut aave_users_hash = AaveUsersHash {
        user_data: user_hash,
        standard_user_ids_by_token: HashMap::new(),
        low_health_user_ids_by_token: HashMap::new(),
    };

    aave_users_hash.intialize_token_user_mapping()?;

    println!("users hash => {:#?}", aave_users_hash);

    Ok(aave_users_hash)
}
