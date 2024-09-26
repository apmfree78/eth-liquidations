#[path = "./mocks/generate_logs.rs"]
mod generate_logs;

#[path = "./mocks/generate_mock_users.rs"]
mod generate_mock_users;

use eth_liquadation::data::token_data_hash::save_erc20_tokens_from_static_data;
use eth_liquadation::data::token_price_hash::generate_token_price_hash;
use eth_liquadation::events::aave_events::update_users_with_event_from_log;
use eth_liquadation::exchanges::aave_v3::events::{
    BorrowEvent, LiquidationEvent, RepayEvent, ReserveUsedAsCollateralDisabledEvent,
    ReserveUsedAsCollateralEnabledEvent, SupplyEvent, WithdrawEvent,
};
use ethers::core::types::U256;
use generate_logs::{
    create_log_for_collateral_disable_event, create_log_for_collateral_enable_event,
};
use generate_mock_users::{
    generate_mock_user_hash, USDT_USER_BALANCE, USDT_USER_BALANCE_UNSCALED, USDT_USER_DEBT,
    USDT_USER_DEBT_UNSCALED, WETH_USER_BALANCE, WETH_USER_BALANCE_UNSCALED,
};

use crate::generate_logs::{
    create_log_for_borrow_event, create_log_for_liquidation_event, create_log_for_repay_event,
    create_log_for_supply_event, create_log_for_withdraw_event,
};
use anyhow::Result;
use ethers::providers::{Provider, Ws};
use std::sync::Arc;

const WS_URL: &str = "ws://localhost:8546";
const AAVE_V3_POOL: &str = "0x87870Bca3F3fD6335C3F4ce8392D69350B4fA4E2";

#[tokio::test]
async fn test_user_update_with_repay_event() -> Result<()> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);

    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;
    generate_token_price_hash(&client).await?;

    let user_address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;

    let amount_to_repay: u64 = USDT_USER_DEBT_UNSCALED / 2;
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

    let log = create_log_for_repay_event(&repay_event, AAVE_V3_POOL);

    let mut user_hash = generate_mock_user_hash().await?;
    // let mut users = user_hash.user_data.values();
    // println!("users => {:#?}", users);

    // now lets repay user debt and see if amount is updated
    update_users_with_event_from_log(log, &mut user_hash, &client).await?;

    // println!("update users => {:#?}", users);

    let a_token_balance = USDT_USER_BALANCE;
    let remaining_debt = USDT_USER_DEBT / 2.0;

    // get user
    let user = user_hash.user_data.get(&user_address).unwrap();

    for tokens in &user.tokens {
        if tokens.token.address == reserve_token {
            assert_eq!(
                tokens.current_stable_debt + tokens.current_variable_debt,
                remaining_debt
            );
            assert_eq!(tokens.current_atoken_balance, a_token_balance);
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_user_update_with_full_repay_then_withdraw_event() -> Result<()> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;

    generate_token_price_hash(&client).await?;
    let user_address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;

    // test that  when  user repays debt and withdraws a token balance token is removed
    let amount_to_repay: u64 = USDT_USER_DEBT_UNSCALED as u64;
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

    let log = create_log_for_repay_event(&repay_event, AAVE_V3_POOL);

    let mut user_hash = generate_mock_user_hash().await?;
    // let mut users = user_hash.user_data.values();
    // println!("users => {:#?}", users);

    // now lets repay user debt and see if amount is updated
    update_users_with_event_from_log(log, &mut user_hash, &client).await?;

    let a_token_balance = USDT_USER_BALANCE; // should be unchanged;
    let remaining_debt = 0.0; // lower remaining debt

    // get user
    let user = user_hash.user_data.get(&user_address).unwrap();

    for tokens in &user.tokens {
        if tokens.token.address == reserve_token {
            assert_eq!(
                tokens.current_stable_debt + tokens.current_variable_debt,
                remaining_debt
            );
            assert_eq!(tokens.current_atoken_balance, a_token_balance);
        }
    }

    let amount_to_withdraw: u64 = USDT_USER_BALANCE_UNSCALED as u64;
    let withdraw_event = WithdrawEvent {
        reserve: reserve_token.parse().unwrap(),
        user: user_address,
        to: user_address,
        amount: amount_to_withdraw.into(),
    };

    let log = create_log_for_withdraw_event(&withdraw_event, AAVE_V3_POOL);

    // now lets withdraw remaining a token balance
    update_users_with_event_from_log(log, &mut user_hash, &client).await?;

    // get user
    let user = user_hash.user_data.get(&user_address).unwrap();

    println!("update users => {:#?}", user);
    // check that token was removed
    assert_eq!(user.tokens.len(), 1);

    Ok(())
}

#[tokio::test]
async fn test_user_update_with_full_withdraw_then_repay_event() -> Result<()> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;

    generate_token_price_hash(&client).await?;
    let user_address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;
    let reserve_token = "0xdac17f958d2ee523a2206206994597c13d831ec7";

    let amount_to_withdraw: u64 = USDT_USER_BALANCE_UNSCALED as u64;
    let withdraw_event = WithdrawEvent {
        reserve: reserve_token.parse().unwrap(),
        user: user_address,
        to: user_address,
        amount: amount_to_withdraw.into(),
    };

    let log = create_log_for_withdraw_event(&withdraw_event, AAVE_V3_POOL);

    let mut user_hash = generate_mock_user_hash().await?;
    // let mut users = user_hash.user_data.values();
    // println!("users => {:#?}", users);

    // now lets repay user debt and see if amount is updated
    update_users_with_event_from_log(log, &mut user_hash, &client).await?;

    let remaining_debt = USDT_USER_DEBT; // should be unchanged
    let a_token_balance = 0.0; // lower remaining debt

    // get user
    let user = user_hash.user_data.get(&user_address).unwrap();

    for tokens in &user.tokens {
        if tokens.token.address == reserve_token {
            assert_eq!(
                tokens.current_stable_debt + tokens.current_variable_debt,
                remaining_debt
            );
            assert_eq!(tokens.current_atoken_balance, a_token_balance);
        }
    }

    // test that  when  user repays debt and withdraws a token balance token is removed
    let amount_to_repay: u64 = USDT_USER_DEBT_UNSCALED as u64;
    let repay_event = RepayEvent {
        reserve: reserve_token.parse().unwrap(),
        user: user_address,
        repayer: "0x024889be330d20bfb132faf5c73ee0fd81e96e71"
            .parse()
            .unwrap(),
        amount: amount_to_repay.into(),
        use_a_tokens: false,
    };

    let log = create_log_for_repay_event(&repay_event, AAVE_V3_POOL);

    // now lets withdraw remaining a token balance
    update_users_with_event_from_log(log, &mut user_hash, &client).await?;

    // get user
    let user = user_hash.user_data.get(&user_address).unwrap();

    println!("update users => {:#?}", user);
    // check that token was removed
    assert_eq!(user.tokens.len(), 1);

    Ok(())
}

#[tokio::test]
async fn test_user_update_with_repay_with_a_token_event() -> Result<()> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;

    generate_token_price_hash(&client).await?;
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

    let log = create_log_for_repay_event(&repay_event, AAVE_V3_POOL);

    let mut user_hash = generate_mock_user_hash().await?;

    // now lets repay user debt and see if amount is updated
    update_users_with_event_from_log(log, &mut user_hash, &client).await?;

    let amount_to_repay_scaled = amount_to_repay as f64 / 1e6;

    let a_token_balance: f64 = USDT_USER_BALANCE - amount_to_repay_scaled;
    let remaining_debt: f64 = USDT_USER_DEBT - amount_to_repay_scaled;

    // get user
    let user = user_hash.user_data.get(&user_address).unwrap();

    for tokens in &user.tokens {
        if tokens.token.address == reserve_token {
            assert_eq!(
                tokens.current_stable_debt + tokens.current_variable_debt,
                remaining_debt
            );
            assert_eq!(tokens.current_atoken_balance, a_token_balance);
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_user_update_with_borrow() -> Result<()> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;

    generate_token_price_hash(&client).await?;
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

    let log = create_log_for_borrow_event(&borrow_event, AAVE_V3_POOL);

    let mut user_hash = generate_mock_user_hash().await?;

    // now lets borrow tokens and take on more debt
    update_users_with_event_from_log(log, &mut user_hash, &client).await?;

    let amount_to_borrow_unscaled = amount_to_borrow as f64 / 1e6;

    let updated_debt = USDT_USER_DEBT + amount_to_borrow_unscaled;

    // get user
    let user = user_hash.user_data.get(&user_address).unwrap();

    for tokens in &user.tokens {
        if tokens.token.address == reserve_token {
            assert_eq!(
                tokens.current_stable_debt + tokens.current_variable_debt,
                updated_debt
            );
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_user_liquidation() -> Result<()> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;

    generate_token_price_hash(&client).await?;
    let user_address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;

    let liquidation_collateral_amount: u128 = WETH_USER_BALANCE_UNSCALED as u128 / 2;
    let debt_to_cover: u64 = USDT_USER_DEBT_UNSCALED as u64 / 2;
    let reserve_token = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
    let debt_token = "0xdac17f958d2ee523a2206206994597c13d831ec7";

    let liquidation_event = LiquidationEvent {
        collateral_asset: reserve_token.parse().unwrap(),
        debt_asset: debt_token.parse().unwrap(),
        user: user_address,
        debt_to_cover: debt_to_cover.into(),
        liquidation_collateral_amount: liquidation_collateral_amount.into(),
        liquidator: "0x893411580e590d62ddbca8a703d61cc4a8c7b2b9"
            .parse()
            .unwrap(),
        received_a_token: false,
    };

    let log = create_log_for_liquidation_event(&liquidation_event, AAVE_V3_POOL);

    let mut user_hash = generate_mock_user_hash().await?;

    // now lets borrow tokens and take on more debt
    update_users_with_event_from_log(log, &mut user_hash, &client).await?;

    let updated_debt = USDT_USER_DEBT / 2.0;
    let updated_collateral = WETH_USER_BALANCE / 2.0; // lower remaining debt

    // get user
    let user = user_hash.user_data.get(&user_address).unwrap();

    println!("updated user => {:#?}", user);

    for token in &user.tokens {
        if token.token.address == debt_token {
            assert_eq!(
                token.current_variable_debt + token.current_stable_debt,
                updated_debt
            );
        } else if token.token.address == reserve_token {
            assert_eq!(token.current_atoken_balance, updated_collateral);
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_user_update_with_borrow_new_token() -> Result<()> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;

    generate_token_price_hash(&client).await?;
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

    let log = create_log_for_borrow_event(&borrow_event, AAVE_V3_POOL);

    let mut user_hash = generate_mock_user_hash().await?;

    // now lets borrow tokens and take on more debt
    update_users_with_event_from_log(log, &mut user_hash, &client).await?;

    // get user
    let user = user_hash.user_data.get(&user_address).unwrap();

    // confirm new DAI token was added
    assert_eq!(user.tokens.len(), 3);

    for tokens in &user.tokens {
        if tokens.token.address == reserve_token {
            assert_eq!(tokens.current_atoken_balance, 0.0);
            assert_eq!(
                tokens.current_stable_debt + tokens.current_variable_debt,
                amount_to_borrow as f64
            );
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_user_update_with_supply() -> Result<()> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;

    generate_token_price_hash(&client).await?;
    let user_address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;

    let amount_to_supply: u64 = WETH_USER_BALANCE_UNSCALED as u64 / 10;
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

    let log = create_log_for_supply_event(&supply_event, AAVE_V3_POOL);

    let mut user_hash = generate_mock_user_hash().await?;

    // now lets supply tokens to exchange
    update_users_with_event_from_log(log, &mut user_hash, &client).await?;

    let new_supply = WETH_USER_BALANCE + WETH_USER_BALANCE / 10.0;

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
async fn test_user_update_with_supply_to_new_token() -> Result<()> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;

    generate_token_price_hash(&client).await?;
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

    let log = create_log_for_supply_event(&supply_event, AAVE_V3_POOL);

    let mut user_hash = generate_mock_user_hash().await?;

    // now lets supply tokens to exchange
    update_users_with_event_from_log(log, &mut user_hash, &client).await?;

    // get user
    let user = user_hash.user_data.get(&user_address).unwrap();

    // confirm new DAI token was added
    assert_eq!(user.tokens.len(), 3);

    let scaled_amount_to_supply = amount_to_supply / 10_u128.pow(18);

    for tokens in &user.tokens {
        if tokens.token.address == reserve_token {
            assert_eq!(
                tokens.current_atoken_balance as u64,
                scaled_amount_to_supply as u64
            );
            assert_eq!(
                tokens.current_stable_debt + tokens.current_variable_debt,
                0.0
            );
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_user_update_with_withdraw() -> Result<()> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;

    generate_token_price_hash(&client).await?;
    let user_address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;

    let amount_to_withdraw: u64 = WETH_USER_BALANCE_UNSCALED as u64;
    let reserve_token = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
    let withdraw_event = WithdrawEvent {
        reserve: reserve_token.parse().unwrap(),
        user: user_address,
        to: user_address,
        amount: amount_to_withdraw.into(),
    };

    let log = create_log_for_withdraw_event(&withdraw_event, AAVE_V3_POOL);

    let mut user_hash = generate_mock_user_hash().await?;

    // now lets withdraw user tokens
    update_users_with_event_from_log(log, &mut user_hash, &client).await?;

    let new_supply = 0.0;

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
async fn test_user_update_with_collateral_enable_disable() -> Result<()> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    // populate token state
    save_erc20_tokens_from_static_data(&client).await?;

    generate_token_price_hash(&client).await?;
    let user_address = "0x024889be330d20bfb132faf5c73ee0fd81e96e71".parse()?;

    // DISABLE
    let reserve_token = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
    let collateral_disable_event = ReserveUsedAsCollateralDisabledEvent {
        reserve: reserve_token.parse().unwrap(),
        user: user_address,
    };

    let log = create_log_for_collateral_disable_event(collateral_disable_event, AAVE_V3_POOL);

    let mut user_hash = generate_mock_user_hash().await?;

    // disable token usage as collateral
    update_users_with_event_from_log(log, &mut user_hash, &client).await?;

    // get user
    let user = user_hash.user_data.get(&user_address).unwrap();

    for tokens in &user.tokens {
        if tokens.token.address == reserve_token {
            assert!(!tokens.usage_as_collateral_enabled);
        }
    }

    // ENABLE
    let collateral_enable_event = ReserveUsedAsCollateralEnabledEvent {
        reserve: reserve_token.parse().unwrap(),
        user: "0x024889be330d20bfb132faf5c73ee0fd81e96e71"
            .parse()
            .unwrap(),
    };

    let log = create_log_for_collateral_enable_event(collateral_enable_event, AAVE_V3_POOL);

    // enable token usage as collateral
    update_users_with_event_from_log(log, &mut user_hash, &client).await?;

    let user = user_hash.user_data.get(&user_address).unwrap();

    for tokens in &user.tokens {
        if tokens.token.address == reserve_token {
            assert!(tokens.usage_as_collateral_enabled);
        }
    }

    Ok(())
}
