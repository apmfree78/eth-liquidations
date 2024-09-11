#[path = "./mocks/generate_mock_users.rs"]
mod generate_mock_users;

use anyhow::Result;
use bigdecimal::{BigDecimal, FromPrimitive};
use eth_liquadation::data::token_data_hash::{get_token_data, save_erc20_tokens_from_static_data};
use eth_liquadation::data::token_price_hash::generate_token_price_hash;
use eth_liquadation::exchanges::aave_v3::implementations::aave_users_hash::UpdateUsers;
use eth_liquadation::exchanges::aave_v3::user_structs::{UserType, UsersToLiquidate};
use ethers::abi::Address;
use ethers::providers::{Provider, Ws};
use generate_mock_users::{
    generate_mock_2_user_hash, generate_mock_2_user_hash_v2, generate_mock_user_hash,
};
use std::sync::Arc;

const WS_URL: &str = "ws://localhost:8546";

#[tokio::test]
async fn test_user_is_placed_in_correct_mapping() -> Result<()> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;
    generate_token_price_hash(&client).await?;

    let users_hash = generate_mock_user_hash().await?;

    println!("low health users {:?}", users_hash.whale_user_ids_by_token);
    println!("standard users {:?}", users_hash.standard_user_ids_by_token);

    // test that both values are in low health mapping since
    // initial health factor is 0.9
    for user_id_array in users_hash.whale_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 1);
    }

    for user_id_array in users_hash.standard_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 0);
    }

    Ok(())
}

#[tokio::test]
async fn test_users_are_placed_in_correct_mapping() -> Result<()> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;
    generate_token_price_hash(&client).await?;

    let users_hash = generate_mock_2_user_hash().await?;

    println!("low health users {:?}", users_hash.whale_user_ids_by_token);
    println!("standard users {:?}", users_hash.standard_user_ids_by_token);

    // test that both values are in low health mapping since
    // initial health factor is 0.9
    for user_id_array in users_hash.whale_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 2);
    }

    for user_id_array in users_hash.standard_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 0);
    }

    Ok(())
}

#[tokio::test]
async fn test_both_users_are_moved_to_correct_mapping() -> Result<()> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);

    let user_id: Address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;
    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;

    generate_token_price_hash(&client).await?;

    let mut users_hash = generate_mock_2_user_hash().await?;

    println!("low health users {:?}", users_hash.whale_user_ids_by_token);
    println!("standard users {:?}", users_hash.standard_user_ids_by_token);

    // test that both values are in low health mapping since
    // initial health factor is 0.9
    for user_id_array in users_hash.whale_user_ids_by_token.values() {
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

    users_hash
        .update_token_user_mapping_for_(user_id, &client)
        .await?;

    println!("low health users {:?}", users_hash.whale_user_ids_by_token);
    println!("standard users {:?}", users_hash.standard_user_ids_by_token);
    // now expect 1 user to be in low health mapping while other is in standard
    for user_id_array in users_hash.whale_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 1);
    }

    for user_id_array in users_hash.standard_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 1);
    }

    Ok(())
}

#[tokio::test]
async fn test_both_users_mappings_update_by_token() -> Result<()> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    let token_data = get_token_data().await?;

    let user_id: Address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;
    let user_id_2: Address = "0x922389be330d20bfb132faf5c73ee0fd81e9ad21".parse()?;
    let token_address = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
    let token = token_data.get(&token_address.to_lowercase()).unwrap();

    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;
    generate_token_price_hash(&client).await?;

    let mut users_hash = generate_mock_2_user_hash().await?;

    println!("low health users {:?}", users_hash.whale_user_ids_by_token);
    println!("standard users {:?}", users_hash.standard_user_ids_by_token);

    // test that both values are in low health mapping since
    // initial health factor is 0.9
    for user_id_array in users_hash.whale_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 2);
    }

    for user_id_array in users_hash.standard_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 0);
    }

    users_hash
        .update_token_to_user_mapping_for_all_users_with_token_(token, &client)
        .await?;

    println!("low health users {:?}", users_hash.whale_user_ids_by_token);
    println!("standard users {:?}", users_hash.standard_user_ids_by_token);

    // since nothing should have changed should be unchanged
    for user_id_array in users_hash.whale_user_ids_by_token.values() {
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

    users_hash
        .update_token_to_user_mapping_for_all_users_with_token_(token, &client)
        .await?;

    println!("low health users {:?}", users_hash.whale_user_ids_by_token);
    println!("standard users {:?}", users_hash.standard_user_ids_by_token);

    // now expect 1 user to be in low health mapping while other is in standard
    for user_id_array in users_hash.whale_user_ids_by_token.values() {
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

    users_hash
        .update_token_to_user_mapping_for_all_users_with_token_(token, &client)
        .await?;

    println!("low health users {:?}", users_hash.whale_user_ids_by_token);
    println!("standard users {:?}", users_hash.standard_user_ids_by_token);

    // now both user to be in low health mapping while other is in standard
    for user_id_array in users_hash.whale_user_ids_by_token.values() {
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

    users_hash
        .update_token_to_user_mapping_for_all_users_with_token_(token, &client)
        .await?;

    println!("low health users {:?}", users_hash.whale_user_ids_by_token);
    println!("standard users {:?}", users_hash.standard_user_ids_by_token);

    // now expect 1 user to be in low health mapping while other is in standard
    for user_id_array in users_hash.whale_user_ids_by_token.values() {
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
    user_2.health_factor = BigDecimal::from_f32(1.04).unwrap();

    users_hash
        .update_token_to_user_mapping_for_all_users_with_token_(token, &client)
        .await?;

    println!("low health users {:?}", users_hash.whale_user_ids_by_token);
    println!("standard users {:?}", users_hash.standard_user_ids_by_token);

    // now both user to be in low health mapping while other is in standard
    for user_id_array in users_hash.whale_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 2);
    }

    for user_id_array in users_hash.standard_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 0);
    }

    Ok(())
}

#[tokio::test]
async fn test_moving_user_to_correct_mapping() -> Result<()> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;
    generate_token_price_hash(&client).await?;
    let user_id: Address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;

    let mut users_hash = generate_mock_user_hash().await?;

    // test that both values are in low health mapping since
    // initial health factor is 0.9
    for user_id_array in users_hash.whale_user_ids_by_token.values() {
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
    users_hash
        .update_token_user_mapping_for_(user_id, &client)
        .await?;

    // test that both values are in standard mapping since
    // initial health factor is 2.0
    for user_id_array in users_hash.whale_user_ids_by_token.values() {
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
    user.health_factor = BigDecimal::from_f32(1.04).unwrap();

    // update token user mapping
    users_hash
        .update_token_user_mapping_for_(user_id, &client)
        .await?;

    // test that both values are in low health mapping since
    // initial health factor is 1.05
    for user_id_array in users_hash.whale_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 1);
    }

    for user_id_array in users_hash.standard_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 0);
    }

    Ok(())
}

#[tokio::test]
async fn test_user_is_removed_from_mapping() -> Result<()> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;
    generate_token_price_hash(&client).await?;

    let user_id: Address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;
    let token_address: Address = "0xdac17f958d2ee523a2206206994597c13d831ec7".parse()?;

    let mut users_hash = generate_mock_user_hash().await?;

    println!("low health users {:?}", users_hash.whale_user_ids_by_token);
    println!("standard users {:?}", users_hash.standard_user_ids_by_token);

    // test that both values are in low health mapping since
    // initial health factor is 0.9
    for user_id_array in users_hash.whale_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 1);
    }

    for user_id_array in users_hash.standard_user_ids_by_token.values() {
        assert_eq!(user_id_array.len(), 0);
    }

    users_hash.remove_user_from_token_user_mapping(user_id, token_address)?;

    // test that both values are in low health mapping since
    // initial health factor is 0.9
    for (token, user) in users_hash.whale_user_ids_by_token {
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

#[tokio::test]
async fn test_correct_users_to_liquidate_are_found_for_low_health_users() -> Result<()> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;
    generate_token_price_hash(&client).await?;
    let token_data = get_token_data().await?;

    let user_address: Address = "0x922389be330d20bfb132faf5c73ee0fd81e9ad21".parse()?;
    let token_address = "0xdac17f958d2ee523a2206206994597c13d831ec7";

    let token = token_data.get(token_address).unwrap();

    let mut users_hash = generate_mock_2_user_hash_v2().await?;

    println!("low health users {:?}", users_hash.whale_user_ids_by_token);
    println!("standard users {:?}", users_hash.standard_user_ids_by_token);

    let users_to_liquidate_enum = users_hash
        .update_users_health_factor_by_token_and_return_liquidation_candidates(
            token,
            UserType::Whale,
            &client,
        )
        .await?;

    let users = match users_to_liquidate_enum {
        UsersToLiquidate::Users(users) => Some(users),
        UsersToLiquidate::None => None,
    };

    let users = users.unwrap();
    assert_eq!(users.len(), 1);

    let user = users.first().unwrap();

    assert_eq!(user.user_id, user_address);

    Ok(())
}

#[tokio::test]
async fn test_correct_users_to_liquidate_are_found_for_standard_users() -> Result<()> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;
    generate_token_price_hash(&client).await?;
    let token_data = get_token_data().await?;

    let token_address = "0xdac17f958d2ee523a2206206994597c13d831ec7";

    let token = token_data.get(token_address).unwrap();
    let mut users_hash = generate_mock_2_user_hash_v2().await?;

    println!("low health users {:?}", users_hash.whale_user_ids_by_token);
    println!("standard users {:?}", users_hash.standard_user_ids_by_token);

    let users_to_liquidate_enum = users_hash
        .update_users_health_factor_by_token_and_return_liquidation_candidates(
            token,
            UserType::Standard,
            &client,
        )
        .await?;

    let users = match users_to_liquidate_enum {
        UsersToLiquidate::Users(users) => Some(users),
        UsersToLiquidate::None => None,
    };

    let users = users.unwrap_or_default();
    assert!(
        users.is_empty(),
        "Expected no users to liquidate, but some found"
    );

    Ok(())
}
