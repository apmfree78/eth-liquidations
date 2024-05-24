#[path = "./mocks/generate_logs.rs"]
mod generate_logs;

use bigdecimal::{BigDecimal, FromPrimitive};
use eth_liquadation::abi::aave_oracle::AddressesProviderCall;
use eth_liquadation::data::erc20::Erc20Token;
use eth_liquadation::events::aave_events::update_users_with_events_from_logs;
use eth_liquadation::exchanges::aave_v3::events::{
    BorrowEvent, RepayEvent, ReserveUsedAsCollateralDisabledEvent,
    ReserveUsedAsCollateralEnabledEvent, SupplyEvent, WithdrawEvent,
};
use eth_liquadation::exchanges::aave_v3::user_data::{AaveToken, AaveUserData, AaveUsersHash};
use ethers::abi::Address;
use ethers::core::types::U256;
use generate_logs::{
    create_log_for_collateral_disable_event, create_log_for_collateral_enable_event,
};

use crate::generate_logs::{
    create_log_for_borrow_event, create_log_for_repay_event, create_log_for_supply_event,
    create_log_for_withdraw_event,
};
use ethers::providers::{Provider, Ws};
use std::collections::HashMap;
use std::sync::Arc;

const WS_URL: &'static str = "ws://localhost:8546";
const AAVE_V3_POOL: &str = "0x87870Bca3F3fD6335C3F4ce8392D69350B4fA4E2";

#[tokio::test]
async fn test_user_update_with_repay_event() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    let user_address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;

    let amount_to_repay: u64 = 6000000000;
    let reserve_token = "0xdac17f958d2ee523a2206206994597c13d831ec7";
    let repay_event = RepayEvent {
        reserve: reserve_token.parse().unwrap(),
        user: user_address,
        repayer: "0x024889be330d20bfb132faf5c73ee0fd81e96e71"
            .parse()
            .unwrap(),
        amount: amount_to_repay.into(),
        use_a_tokens: false,
    };

    let logs = vec![create_log_for_repay_event(&repay_event, AAVE_V3_POOL)];

    let mut user_hash = generate_mock_user_hash()?;
    // let mut users = user_hash.user_data.values();
    // println!("users => {:#?}", users);

    // now lets repay user debt and see if amount is updated
    update_users_with_events_from_logs(&logs, &mut user_hash, &client).await?;

    // println!("update users => {:#?}", users);

    let a_token_balance = BigDecimal::from_u64(30000000000).unwrap(); // should be unchanged
    let remaining_debt = BigDecimal::from_u64(20000000000).unwrap(); // lower remaining debt

    // get user
    let user = user_hash.user_data.get(&user_address).unwrap();

    for tokens in &user.tokens {
        if tokens.token.address == reserve_token {
            assert_eq!(tokens.current_total_debt, remaining_debt);
            assert_eq!(tokens.current_atoken_balance, a_token_balance);
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_user_update_with_repay_with_a_token_event() -> Result<(), Box<dyn std::error::Error>>
{
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    let user_address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;

    let amount_to_repay: u64 = 6000000000;
    let reserve_token = "0xdac17f958d2ee523a2206206994597c13d831ec7";
    let repay_event = RepayEvent {
        reserve: reserve_token.parse().unwrap(),
        user: user_address,
        repayer: user_address,
        amount: amount_to_repay.into(),
        use_a_tokens: true,
    };

    let logs = vec![create_log_for_repay_event(&repay_event, AAVE_V3_POOL)];

    let mut user_hash = generate_mock_user_hash()?;

    // now lets repay user debt and see if amount is updated
    update_users_with_events_from_logs(&logs, &mut user_hash, &client).await?;

    let a_token_balance = BigDecimal::from_u64(24000000000).unwrap(); // should be unchanged
    let remaining_debt = BigDecimal::from_u64(20000000000).unwrap(); // lower remaining debt

    // get user
    let user = user_hash.user_data.get(&user_address).unwrap();

    for tokens in &user.tokens {
        if tokens.token.address == reserve_token {
            assert_eq!(tokens.current_total_debt, remaining_debt);
            assert_eq!(tokens.current_atoken_balance, a_token_balance);
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_user_update_with_borrow() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    let user_address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;

    let amount_to_borrow: u64 = 4000000000;
    let reserve_token = "0xdac17f958d2ee523a2206206994597c13d831ec7";
    let borrow_event = BorrowEvent {
        reserve: reserve_token.parse().unwrap(),
        user: "0x893411580e590d62ddbca8a703d61cc4a8c7b2b9"
            .parse()
            .unwrap(),
        on_behalf_of: user_address,
        amount: amount_to_borrow.into(),
        interest_rate_mode: 1,
        borrow_rate: U256::from(1000),
        referral_code: 0,
    };

    let logs = vec![create_log_for_borrow_event(&borrow_event, AAVE_V3_POOL)];

    let mut user_hash = generate_mock_user_hash()?;

    // now lets borrow tokens and take on more debt
    update_users_with_events_from_logs(&logs, &mut user_hash, &client).await?;

    let updated_debt = BigDecimal::from_u64(30000000000).unwrap(); // lower remaining debt

    // get user
    let user = user_hash.user_data.get(&user_address).unwrap();

    for tokens in &user.tokens {
        if tokens.token.address == reserve_token {
            assert_eq!(tokens.current_total_debt, updated_debt);
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_user_update_with_borrow_new_token() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    let user_address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;

    let amount_to_borrow: u64 = 4000000000;
    let reserve_token = "0x6B175474E89094C44Da98b954EedeAC495271d0F"; //DAI
    let borrow_event = BorrowEvent {
        reserve: reserve_token.parse().unwrap(),
        user: "0x893411580e590d62ddbca8a703d61cc4a8c7b2b9"
            .parse()
            .unwrap(),
        on_behalf_of: user_address,
        amount: amount_to_borrow.into(),
        interest_rate_mode: 1,
        borrow_rate: U256::from(1000),
        referral_code: 0,
    };

    let logs = vec![create_log_for_borrow_event(&borrow_event, AAVE_V3_POOL)];

    let mut user_hash = generate_mock_user_hash()?;

    // now lets borrow tokens and take on more debt
    update_users_with_events_from_logs(&logs, &mut user_hash, &client).await?;

    let amount_to_borrow = BigDecimal::from_u64(amount_to_borrow).unwrap(); // lower remaining debt

    // get user
    let user = user_hash.user_data.get(&user_address).unwrap();

    // confirm new DAI token was added
    assert_eq!(user.tokens.len(), 3);

    for tokens in &user.tokens {
        if tokens.token.address == reserve_token {
            assert_eq!(tokens.current_atoken_balance, BigDecimal::from(0));
            assert_eq!(tokens.current_total_debt, amount_to_borrow);
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_user_update_with_supply() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    let user_address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;

    let amount_to_supply: u128 = 500000000000000000;
    let reserve_token = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
    let supply_event = SupplyEvent {
        reserve: reserve_token.parse().unwrap(),
        user: "0x893411580e590d62ddbca8a703d61cc4a8c7b2b9"
            .parse()
            .unwrap(),
        on_behalf_of: user_address,
        amount: amount_to_supply.into(),
        referral_code: 0,
    };

    let logs = vec![create_log_for_supply_event(&supply_event, AAVE_V3_POOL)];

    let mut user_hash = generate_mock_user_hash()?;

    // now lets supply tokens to exchange
    update_users_with_events_from_logs(&logs, &mut user_hash, &client).await?;

    let new_supply = BigDecimal::from_u128(15500000000000000000).unwrap();

    // get user
    let user = user_hash.user_data.get(&user_address).unwrap();

    for tokens in &user.tokens {
        if tokens.token.address == reserve_token {
            assert_eq!(tokens.current_atoken_balance, new_supply);
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_user_update_with_supply_to_new_token() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    let user_address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;

    let amount_to_supply: u128 = 500000000000000000;
    let reserve_token = "0x6B175474E89094C44Da98b954EedeAC495271d0F"; //DAI
    let supply_event = SupplyEvent {
        reserve: reserve_token.parse().unwrap(),
        user: "0x893411580e590d62ddbca8a703d61cc4a8c7b2b9"
            .parse()
            .unwrap(),
        on_behalf_of: user_address,
        amount: amount_to_supply.into(),
        referral_code: 0,
    };

    let logs = vec![create_log_for_supply_event(&supply_event, AAVE_V3_POOL)];

    let mut user_hash = generate_mock_user_hash()?;

    // now lets supply tokens to exchange
    update_users_with_events_from_logs(&logs, &mut user_hash, &client).await?;

    let amount_to_supply = BigDecimal::from_u128(amount_to_supply).unwrap();

    // get user
    let user = user_hash.user_data.get(&user_address).unwrap();

    // confirm new DAI token was added
    assert_eq!(user.tokens.len(), 3);

    for tokens in &user.tokens {
        if tokens.token.address == reserve_token {
            assert_eq!(tokens.current_atoken_balance, amount_to_supply);
            assert_eq!(tokens.current_total_debt, BigDecimal::from(0));
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_user_update_with_withdraw() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    let user_address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;

    let amount_to_withdraw: u128 = 15000000000000000000;
    let reserve_token = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
    let withdraw_event = WithdrawEvent {
        reserve: reserve_token.parse().unwrap(),
        user: user_address,
        to: user_address,
        amount: amount_to_withdraw.into(),
    };

    let logs = vec![create_log_for_withdraw_event(&withdraw_event, AAVE_V3_POOL)];

    let mut user_hash = generate_mock_user_hash()?;

    // now lets withdraw user tokens
    update_users_with_events_from_logs(&logs, &mut user_hash, &client).await?;

    let new_supply = BigDecimal::from(0);

    // get user
    let user = user_hash.user_data.get(&user_address).unwrap();

    for tokens in &user.tokens {
        if tokens.token.address == reserve_token {
            assert_eq!(tokens.current_atoken_balance, new_supply);
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_user_update_with_collateral_enable_disable() -> Result<(), Box<dyn std::error::Error>>
{
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    let user_address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;

    // DISABLE
    let reserve_token = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
    let collateral_disable_event = ReserveUsedAsCollateralDisabledEvent {
        reserve: reserve_token.parse().unwrap(),
        user: user_address,
    };

    let logs = vec![create_log_for_collateral_disable_event(
        collateral_disable_event,
        AAVE_V3_POOL,
    )];

    let mut user_hash = generate_mock_user_hash()?;

    // disable token usage as collateral
    update_users_with_events_from_logs(&logs, &mut user_hash, &client).await?;

    // get user
    let user = user_hash.user_data.get(&user_address).unwrap();

    for tokens in &user.tokens {
        if tokens.token.address == reserve_token {
            assert_eq!(tokens.usage_as_collateral_enabled, false);
        }
    }

    // ENABLE
    let collateral_enable_event = ReserveUsedAsCollateralEnabledEvent {
        reserve: reserve_token.parse().unwrap(),
        user: "0x024889be330d20bfb132faf5c73ee0fd81e96e71"
            .parse()
            .unwrap(),
    };

    let logs = vec![create_log_for_collateral_enable_event(
        collateral_enable_event,
        AAVE_V3_POOL,
    )];

    // enable token usage as collateral
    update_users_with_events_from_logs(&logs, &mut user_hash, &client).await?;

    let user = user_hash.user_data.get(&user_address).unwrap();

    for tokens in &user.tokens {
        if tokens.token.address == reserve_token {
            assert_eq!(tokens.usage_as_collateral_enabled, true);
        }
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
                },
                current_total_debt: BigDecimal::from_u64(26000000000).unwrap(),
                usage_as_collateral_enabled: true,
                current_atoken_balance: BigDecimal::from_u64(30000000000).unwrap(),
                reserve_liquidation_threshold: BigDecimal::from(7800),
                reserve_liquidation_bonus: BigDecimal::from(10450),
            },
        ],
        health_factor: BigDecimal::from_f32(1.545587076407477097).unwrap(),
    };

    let mut user_hash = HashMap::new();
    user_hash.insert(user_address, user_data);

    Ok(AaveUsersHash {
        user_data: user_hash,
        user_ids_by_token: HashMap::new(),
        low_health_user_ids_by_token: HashMap::new(),
    })
}
