use bigdecimal::{BigDecimal, FromPrimitive};
use eth_liquadation::data::erc20::Erc20Token;
use eth_liquadation::data::token_price_hash::generate_token_price_hash;
use eth_liquadation::exchanges::aave_v3::implementations::aave_users_hash::UpdateUsers;
use eth_liquadation::exchanges::aave_v3::user_structs::{AaveToken, AaveUserData, AaveUsersHash};
use ethers::abi::Address;
use ethers::providers::{Provider, Ws};
use std::collections::HashMap;
use std::sync::Arc;

const WS_URL: &str = "ws://localhost:8546";

#[tokio::test]
async fn test_user_is_placed_in_correct_mapping() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    generate_token_price_hash(&client).await?;

    let users_hash = generate_mock_user_hash()?;

    println!(
        "low health users {:?}",
        users_hash.low_health_user_ids_by_token
    );
    println!("standard users {:?}", users_hash.standard_user_ids_by_token);

    // test that both values are in low health mapping since
    // initial health factor is 0.9
    for user_id_array in users_hash.low_health_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 1);
    }

    for user_id_array in users_hash.standard_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 0);
    }

    Ok(())
}

#[tokio::test]
async fn test_users_are_placed_in_correct_mapping() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    generate_token_price_hash(&client).await?;

    let users_hash = generate_mock_2_user_hash()?;

    println!(
        "low health users {:?}",
        users_hash.low_health_user_ids_by_token
    );
    println!("standard users {:?}", users_hash.standard_user_ids_by_token);

    // test that both values are in low health mapping since
    // initial health factor is 0.9
    for user_id_array in users_hash.low_health_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 2);
    }

    for user_id_array in users_hash.standard_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 0);
    }

    Ok(())
}

#[tokio::test]
async fn test_both_users_are_moved_to_correct_mapping() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);

    let user_id: Address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;

    generate_token_price_hash(&client).await?;

    let mut users_hash = generate_mock_2_user_hash()?;

    println!(
        "low health users {:?}",
        users_hash.low_health_user_ids_by_token
    );
    println!("standard users {:?}", users_hash.standard_user_ids_by_token);

    // test that both values are in low health mapping since
    // initial health factor is 0.9
    for user_id_array in users_hash.low_health_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 2);
    }

    for user_id_array in users_hash.standard_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 0);
    }

    // now let bump the healh factor up and update token user mapping and check
    // user was moved to standard mapping
    let user = users_hash
        .user_data
        .get_mut(&user_id)
        .expect("invalid user id");
    user.health_factor = BigDecimal::from_f32(2.0).unwrap();

    users_hash.update_token_user_mapping_for_(user_id)?;

    println!(
        "low health users {:?}",
        users_hash.low_health_user_ids_by_token
    );
    println!("standard users {:?}", users_hash.standard_user_ids_by_token);
    // now expect 1 user to be in low health mapping while other is in standard
    for user_id_array in users_hash.low_health_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 1);
    }

    for user_id_array in users_hash.standard_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 1);
    }

    Ok(())
}

#[tokio::test]
async fn test_both_users_mappings_update_by_token() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);

    let user_id: Address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;
    let user_id_2: Address = "0x922389be330d20bfb132faf5c73ee0fd81e9ad21".parse()?;
    let token_address: Address = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse()?;

    generate_token_price_hash(&client).await?;

    let mut users_hash = generate_mock_2_user_hash()?;

    println!(
        "low health users {:?}",
        users_hash.low_health_user_ids_by_token
    );
    println!("standard users {:?}", users_hash.standard_user_ids_by_token);

    // test that both values are in low health mapping since
    // initial health factor is 0.9
    for user_id_array in users_hash.low_health_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 2);
    }

    for user_id_array in users_hash.standard_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 0);
    }

    users_hash.update_token_to_user_mapping_for_all_users_with_token_(token_address)?;

    println!(
        "low health users {:?}",
        users_hash.low_health_user_ids_by_token
    );
    println!("standard users {:?}", users_hash.standard_user_ids_by_token);

    // since nothing should have changed should be unchanged
    for user_id_array in users_hash.low_health_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 2);
    }

    for user_id_array in users_hash.standard_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 0);
    }

    // now let bump the healh factor up and update token user mapping and check
    // user was moved to standard mapping
    let user = users_hash
        .user_data
        .get_mut(&user_id)
        .expect("invalid user id");
    user.health_factor = BigDecimal::from_f32(2.0).unwrap();

    users_hash.update_token_to_user_mapping_for_all_users_with_token_(token_address)?;

    println!(
        "low health users {:?}",
        users_hash.low_health_user_ids_by_token
    );
    println!("standard users {:?}", users_hash.standard_user_ids_by_token);

    // now expect 1 user to be in low health mapping while other is in standard
    for user_id_array in users_hash.low_health_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 1);
    }

    for user_id_array in users_hash.standard_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 1);
    }

    // now let bump the healh factor up and update token user mapping and check
    // user was moved to standard mapping
    let user_2 = users_hash
        .user_data
        .get_mut(&user_id_2)
        .expect("invalid user id");
    user_2.health_factor = BigDecimal::from_f32(4.0).unwrap();

    users_hash.update_token_to_user_mapping_for_all_users_with_token_(token_address)?;

    println!(
        "low health users {:?}",
        users_hash.low_health_user_ids_by_token
    );
    println!("standard users {:?}", users_hash.standard_user_ids_by_token);

    // now both user to be in low health mapping while other is in standard
    for user_id_array in users_hash.low_health_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 0);
    }

    for user_id_array in users_hash.standard_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 2);
    }

    // now let lower the healh factor up and update token user mapping and check
    // user was moved to standard mapping
    let user = users_hash
        .user_data
        .get_mut(&user_id)
        .expect("invalid user id");
    user.health_factor = BigDecimal::from_f32(1.0).unwrap();

    users_hash.update_token_to_user_mapping_for_all_users_with_token_(token_address)?;

    println!(
        "low health users {:?}",
        users_hash.low_health_user_ids_by_token
    );
    println!("standard users {:?}", users_hash.standard_user_ids_by_token);

    // now expect 1 user to be in low health mapping while other is in standard
    for user_id_array in users_hash.low_health_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 1);
    }

    for user_id_array in users_hash.standard_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 1);
    }

    // now let lower the healh factor up and update token user mapping and check
    // user was moved to standard mapping
    let user_2 = users_hash
        .user_data
        .get_mut(&user_id_2)
        .expect("invalid user id");
    user_2.health_factor = BigDecimal::from_f32(1.05).unwrap();

    users_hash.update_token_to_user_mapping_for_all_users_with_token_(token_address)?;

    println!(
        "low health users {:?}",
        users_hash.low_health_user_ids_by_token
    );
    println!("standard users {:?}", users_hash.standard_user_ids_by_token);

    // now both user to be in low health mapping while other is in standard
    for user_id_array in users_hash.low_health_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 2);
    }

    for user_id_array in users_hash.standard_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 0);
    }

    Ok(())
}

#[tokio::test]
async fn test_moving_user_to_correct_mapping() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    generate_token_price_hash(&client).await?;
    let user_id: Address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;

    let mut users_hash = generate_mock_user_hash()?;

    // test that both values are in low health mapping since
    // initial health factor is 0.9
    for user_id_array in users_hash.low_health_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 1);
    }

    for user_id_array in users_hash.standard_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 0);
    }

    // now let bump the healh factor up and update token user mapping and check
    // user was moved to standard mapping
    let user = users_hash
        .user_data
        .get_mut(&user_id)
        .expect("invalid user id");
    user.health_factor = BigDecimal::from_f32(2.0).unwrap();

    // update token user mapping
    users_hash.update_token_user_mapping_for_(user_id)?;

    // test that both values are in standard mapping since
    // initial health factor is 2.0
    for user_id_array in users_hash.low_health_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 0);
    }

    for user_id_array in users_hash.standard_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 1);
    }

    // now let lower the healh factor and update token user mapping and check
    // user was moved to low health mapping
    let user = users_hash
        .user_data
        .get_mut(&user_id)
        .expect("invalid user id");
    user.health_factor = BigDecimal::from_f32(1.05).unwrap();

    // update token user mapping
    users_hash.update_token_user_mapping_for_(user_id)?;

    // test that both values are in low health mapping since
    // initial health factor is 1.05
    for user_id_array in users_hash.low_health_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 1);
    }

    for user_id_array in users_hash.standard_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 0);
    }

    Ok(())
}

#[tokio::test]
async fn test_user_is_removed_from_mapping() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    generate_token_price_hash(&client).await?;

    let user_id: Address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;
    let token_address: Address = "0xdac17f958d2ee523a2206206994597c13d831ec7".parse()?;

    let mut users_hash = generate_mock_user_hash()?;

    println!(
        "low health users {:?}",
        users_hash.low_health_user_ids_by_token
    );
    println!("standard users {:?}", users_hash.standard_user_ids_by_token);

    // test that both values are in low health mapping since
    // initial health factor is 0.9
    for user_id_array in users_hash.low_health_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 1);
    }

    for user_id_array in users_hash.standard_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 0);
    }

    users_hash.remove_user_from_token_user_mapping(user_id, token_address)?;

    // test that both values are in low health mapping since
    // initial health factor is 0.9
    for (token, user) in users_hash.low_health_user_ids_by_token {
        if token == token_address {
            assert_eq!(user.len(), 0);
        } else {
            assert_eq!(user.len(), 1);
        }
    }

    for user_id_array in users_hash.standard_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 0);
    }

    Ok(())
}

fn generate_mock_user_hash() -> Result<AaveUsersHash, Box<dyn std::error::Error>> {
    let user_address: Address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;
    let user_data = AaveUserData {
        id: user_address,
        total_debt: BigDecimal::from_u64(2603060364429).unwrap(),
        colladeral_times_liquidation_factor: BigDecimal::from_f32(4023256458369.85).unwrap(),
        tokens: vec![
            AaveToken {
                token: Erc20Token {
                    name: "Wrapped Ether",
                    symbol: "WETH",
                    decimals: 18,
                    address: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
                    liquidation_bonus: 10500,
                    liquidation_threshold: 8300,
                    chain_link_price_feed: "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419",
                    chainlink_aggregator: "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419",
                },
                current_total_debt: BigDecimal::from(0),
                usage_as_collateral_enabled: true,
                current_atoken_balance: BigDecimal::from_u128(15000000000000000000).unwrap(),
                reserve_liquidation_threshold: BigDecimal::from(8300),
                reserve_liquidation_bonus: BigDecimal::from(10500),
            },
            AaveToken {
                token: Erc20Token {
                    name: "Tether USD",
                    symbol: "USDT",
                    decimals: 6,
                    address: "0xdac17f958d2ee523a2206206994597c13d831ec7",
                    liquidation_bonus: 10450,
                    liquidation_threshold: 7800,
                    chain_link_price_feed: "0x3E7d1eAB13ad0104d2750B8863b489D65364e32D",
                    chainlink_aggregator: "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419",
                },
                current_total_debt: BigDecimal::from_u64(26000000000).unwrap(),
                usage_as_collateral_enabled: true,
                current_atoken_balance: BigDecimal::from_u64(30000000000).unwrap(),
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

fn generate_mock_2_user_hash() -> Result<AaveUsersHash, Box<dyn std::error::Error>> {
    let user_address: Address = "0x922389be330d20bfb132faf5c73ee0fd81e9ad21".parse()?;
    let user_data = AaveUserData {
        id: user_address,
        total_debt: BigDecimal::from_u64(2603060364429).unwrap(),
        colladeral_times_liquidation_factor: BigDecimal::from_f32(4023256458369.85).unwrap(),
        tokens: vec![
            AaveToken {
                token: Erc20Token {
                    name: "Wrapped Ether",
                    symbol: "WETH",
                    decimals: 18,
                    address: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
                    liquidation_bonus: 10500,
                    liquidation_threshold: 8300,
                    chain_link_price_feed: "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419",
                    chainlink_aggregator: "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419",
                },
                current_total_debt: BigDecimal::from(0),
                usage_as_collateral_enabled: true,
                current_atoken_balance: BigDecimal::from_u128(15000000000000000000).unwrap(),
                reserve_liquidation_threshold: BigDecimal::from(8300),
                reserve_liquidation_bonus: BigDecimal::from(10500),
            },
            AaveToken {
                token: Erc20Token {
                    name: "Tether USD",
                    symbol: "USDT",
                    decimals: 6,
                    address: "0xdac17f958d2ee523a2206206994597c13d831ec7",
                    liquidation_bonus: 10450,
                    liquidation_threshold: 7800,
                    chain_link_price_feed: "0x3E7d1eAB13ad0104d2750B8863b489D65364e32D",
                    chainlink_aggregator: "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419",
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
        colladeral_times_liquidation_factor: BigDecimal::from_f32(4023256458369.85).unwrap(),
        tokens: vec![
            AaveToken {
                token: Erc20Token {
                    name: "Wrapped Ether",
                    symbol: "WETH",
                    decimals: 18,
                    address: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
                    liquidation_bonus: 10500,
                    liquidation_threshold: 8300,
                    chain_link_price_feed: "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419",
                    chainlink_aggregator: "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419",
                },
                current_total_debt: BigDecimal::from(0),
                usage_as_collateral_enabled: true,
                current_atoken_balance: BigDecimal::from_u128(15000000000000000000).unwrap(),
                reserve_liquidation_threshold: BigDecimal::from(8300),
                reserve_liquidation_bonus: BigDecimal::from(10500),
            },
            AaveToken {
                token: Erc20Token {
                    name: "Tether USD",
                    symbol: "USDT",
                    decimals: 6,
                    address: "0xdac17f958d2ee523a2206206994597c13d831ec7",
                    liquidation_bonus: 10450,
                    liquidation_threshold: 7800,
                    chain_link_price_feed: "0x3E7d1eAB13ad0104d2750B8863b489D65364e32D",
                    chainlink_aggregator: "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419",
                },
                current_total_debt: BigDecimal::from_u64(26000000000).unwrap(),
                usage_as_collateral_enabled: true,
                current_atoken_balance: BigDecimal::from_u64(30000000000).unwrap(),
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
