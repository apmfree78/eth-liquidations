use super::data::{AaveToken, AaveUserData};
use crate::data::erc20::{u256_to_big_decimal, Erc20Token, TOKEN_DATA};
use bigdecimal::BigDecimal;
use ethers::abi::Address;
use ethers::abi::{self, decode, ParamType, Token};
use ethers::core::abi::RawLog;
use ethers::core::types::{Log, U256};
use open_fastrlp::Decodable;

#[derive(Clone, Copy, Debug)]
pub enum AaveUserEvent {
    WithDraw,
    Borrow,
    Repay,
    Supply,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct AaveUserAction {
    pub user_event: AaveUserEvent,
    pub user_address: Address,
    pub token: Erc20Token,
    pub amount_transferred: BigDecimal,
}

#[derive(Clone, Copy, Debug)]
pub struct WithdrawEvent {
    pub reserve: Address,
    pub user: Address,
    pub to: Address,
    pub amount: U256,
}

#[derive(Clone, Copy, Debug)]
pub struct BorrowEvent {
    pub reserve: Address,
    pub user: Address,
    pub on_behalf_of: Address,
    pub amount: U256,
    pub interest_rate_mode: u8,
    pub borrow_rate: U256,
    pub referral_code: u16,
}

#[derive(Clone, Copy, Debug)]
pub struct RepayEvent {
    pub reserve: Address,
    pub user: Address,
    pub repayer: Address,
    pub amount: U256,
    pub use_a_tokens: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct SupplyEvent {
    pub reserve: Address,
    pub user: Address,
    pub on_behalf_of: Address,
    pub amount: U256,
    pub referral_code: u16,
}

#[derive(Clone, Copy, Debug)]
pub enum AaveEventType {
    WithdrawEvent(WithdrawEvent),
    BorrowEvent(BorrowEvent),
    RepayEvent(RepayEvent),
    SupplyEvent(SupplyEvent),
    Unknown,
}

pub trait AaveEvent {
    fn get_user(&self) -> Address;
    fn get_reserve(&self) -> Address;
    fn get_amount(&self) -> U256;
    fn get_type(&self) -> AaveUserEvent;
}

macro_rules! impl_aave_event {
    ($struct_name:ident, $event_variant:expr) => {
        impl AaveEvent for $struct_name {
            fn get_user(&self) -> Address {
                self.user
            }

            fn get_reserve(&self) -> Address {
                self.reserve
            }

            fn get_amount(&self) -> U256 {
                self.amount
            }

            fn get_type(&self) -> AaveUserEvent {
                $event_variant
            }
        }
    };
}

impl_aave_event!(BorrowEvent, AaveUserEvent::Borrow);
impl_aave_event!(RepayEvent, AaveUserEvent::Repay);
impl_aave_event!(SupplyEvent, AaveUserEvent::Supply);
impl_aave_event!(WithdrawEvent, AaveUserEvent::WithDraw);

pub fn get_user_action_from_event(event: Box<dyn AaveEvent>) -> AaveUserAction {
    let token_address = event.get_reserve();
    println!("token address {:#?}", token_address);
    let token = TOKEN_DATA.get(&token_address.to_string()).unwrap();
    let amount = event.get_amount();
    let amount = u256_to_big_decimal(&amount);

    return AaveUserAction {
        user_event: event.get_type(),
        user_address: event.get_user(),
        token: *token,
        amount_transferred: amount,
    };
}

pub trait Update {
    fn update(&mut self, aave_action: &AaveUserAction) -> Result<(), Box<dyn std::error::Error>>;
}

impl Update for AaveUserData {
    fn update(&mut self, aave_action: &AaveUserAction) -> Result<(), Box<dyn std::error::Error>> {
        let token_address = &aave_action.token.address;
        match aave_action.user_event {
            AaveUserEvent::WithDraw => {
                for token in &mut self.tokens {
                    if token.token.address == *token_address {
                        // update
                        token.current_atoken_balance -= aave_action.amount_transferred.clone();
                        return Ok(());
                    };
                }
            }
            AaveUserEvent::Borrow => {
                // find token in aave user data
                for token in &mut self.tokens {
                    if token.token.address == *token_address {
                        // update
                        token.current_total_debt += aave_action.amount_transferred.clone();
                        return Ok(());
                    };
                }

                // token does not exist , add it
                let token = TOKEN_DATA.get(*token_address).unwrap();
                self.tokens.push(AaveToken {
                    token: *token,
                    current_total_debt: aave_action.amount_transferred.clone(),
                    usage_as_collateral_enabled: false,
                    current_atoken_balance: BigDecimal::from(0),
                    reserve_liquidation_threshold: BigDecimal::from(8000),
                    reserve_liquidation_bonus: BigDecimal::from(10000),
                })
            }
            AaveUserEvent::Repay => {
                // find token in aave user data
                for token in &mut self.tokens {
                    if token.token.address == *token_address {
                        // update
                        token.current_total_debt -= aave_action.amount_transferred.clone();
                        return Ok(());
                    };
                }
            }
            AaveUserEvent::Supply => {
                for token in &mut self.tokens {
                    if token.token.address == *token_address {
                        // update
                        token.current_atoken_balance += aave_action.amount_transferred.clone();
                        return Ok(());
                    };
                }
                // token does not exist , add it
                let token = TOKEN_DATA.get(*token_address).unwrap();
                self.tokens.push(AaveToken {
                    token: *token,
                    current_total_debt: BigDecimal::from(0),
                    usage_as_collateral_enabled: true,
                    current_atoken_balance: aave_action.amount_transferred.clone(),
                    reserve_liquidation_threshold: BigDecimal::from(8000),
                    reserve_liquidation_bonus: BigDecimal::from(10000),
                })
            }
            _ => {}
        }
        Ok(())
    }
}

pub fn create_aave_event_from_log(event_type: AaveUserEvent, log: &Log) -> AaveEventType {
    let raw_log: RawLog = RawLog::from(log.clone());
    println!(" raw log data {:#?}", raw_log);

    match event_type {
        AaveUserEvent::WithDraw => {
            let withdraw_event = decode_withdraw_event(log).unwrap();
            AaveEventType::WithdrawEvent(withdraw_event)
        }
        AaveUserEvent::Borrow => {
            let borrow_event = decode_borrow_event(log).unwrap();
            AaveEventType::BorrowEvent(borrow_event)
        }
        AaveUserEvent::Repay => {
            let repay_event = decode_repay_event(log).unwrap();
            AaveEventType::RepayEvent(repay_event)
        }
        AaveUserEvent::Supply => {
            let supply_event = decode_supply_event(log).unwrap();
            AaveEventType::SupplyEvent(supply_event)
        }
        _ => AaveEventType::Unknown,
    }
}

fn decode_borrow_event(log: &Log) -> Result<BorrowEvent, Box<dyn std::error::Error>> {
    if log.topics.len() < 3 {
        // Check the number of indexed parameters
        return Err("Incorrect number of topics for Borrow event".into());
    }

    let reserve = Address::from_slice(&log.topics[1].as_bytes());
    let on_behalf_of = Address::from_slice(&log.topics[2].as_bytes());
    let referral_code = U256::from_big_endian(&log.topics[3].as_bytes());
    let referral_code: u16 = referral_code
        .low_u64()
        .try_into()
        .map_err(|_| "Referral code is too large for u16")?;

    // Assuming the data contains the rest in order: user, amount, interestRateMode, borrowRate
    let mut data_slice = log.data.as_ref();
    let user = Address::decode(&mut data_slice)?;
    let amount = U256::decode(&mut data_slice)?;
    let interest_rate_mode = u8::decode(&mut data_slice)?;
    let borrow_rate = U256::decode(&mut data_slice)?;

    Ok(BorrowEvent {
        reserve,
        user,
        on_behalf_of,
        amount,
        interest_rate_mode,
        borrow_rate,
        referral_code,
    })
}

fn decode_repay_event(log: &Log) -> Result<RepayEvent, Box<dyn std::error::Error>> {
    if log.topics.len() < 3 {
        return Err("Must have 3 topics for Repay Event".into());
    }

    let reserve = Address::from_slice(log.topics[1].as_bytes());
    let user = Address::from_slice(log.topics[2].as_bytes());
    let repayer = Address::from_slice(log.topics[3].as_bytes());

    let mut data_slice = log.data.as_ref();
    let amount = U256::decode(&mut data_slice)?;
    let use_a_tokens = bool::decode(&mut data_slice)?;

    Ok(RepayEvent {
        reserve,
        user,
        repayer,
        amount,
        use_a_tokens,
    })
}

fn decode_withdraw_event(log: &Log) -> Result<WithdrawEvent, Box<dyn std::error::Error>> {
    if log.topics.len() < 3 {
        return Err("Must have 3 topics for WithDraw Event".into());
    }

    let reserve = Address::from_slice(log.topics[1].as_bytes());
    let user = Address::from_slice(log.topics[2].as_bytes());
    let to = Address::from_slice(log.topics[3].as_bytes());

    let mut data_slice = log.data.as_ref();
    let amount = U256::decode(&mut data_slice)?;

    Ok(WithdrawEvent {
        reserve,
        user,
        to,
        amount,
    })
}

fn decode_supply_event(log: &Log) -> Result<SupplyEvent, Box<dyn std::error::Error>> {
    if log.topics.len() < 3 {
        // Check the number of indexed parameters
        return Err("Incorrect number of topics for Borrow event".into());
    }

    let reserve = Address::from_slice(&log.topics[1].as_bytes());
    let user = Address::from_slice(&log.topics[2].as_bytes());
    let on_behalf_of = Address::from_slice(&log.topics[3].as_bytes());

    // Assuming the data contains the rest in order: user, amount, interestRateMode, borrowRate
    let mut data_slice = log.data.as_ref();
    let amount = U256::decode(&mut data_slice)?;
    let referral_code = u16::decode(&mut data_slice)?;

    Ok(SupplyEvent {
        reserve,
        user,
        on_behalf_of,
        amount,
        referral_code,
    })
}
