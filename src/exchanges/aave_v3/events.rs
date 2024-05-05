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
    pub interate_rate_mode: u8,
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
            let amount = vec_u8_to_u256(&raw_log.data).expect("vec u8 to u256:invalid call");
            return AaveEventType::WithdrawEvent(WithdrawEvent {
                from: raw_log.topics[1].into(),
                reserve: raw_log.topics[2].into(),
                to: raw_log.topics[3].into(),
                amount,
            });
        }
        AaveUserEvent::Borrow => {
            let amount = vec_u8_to_u256(&raw_log.data).expect("vec u8 to u256:invalid call");
            return AaveEventType::BorrowEvent(BorrowEvent {
                from: raw_log.topics[1].into(),
                reserve: raw_log.topics[2].into(),
                to: raw_log.topics[3].into(),
                amount,
            });
        }
        AaveUserEvent::Repay => {
            let amount = vec_u8_to_u256(&raw_log.data).expect("vec u8 to u256: invalid call");
            return AaveEventType::RepayEvent(RepayEvent {
                from: raw_log.topics[1].into(),
                reserve: raw_log.topics[2].into(),
                to: raw_log.topics[3].into(),
                amount,
            });
        }
        AaveUserEvent::Supply => {
            let (amount, referral_code) = decode_supply_event_data(&raw_log.data).unwrap();
            return AaveEventType::SupplyEvent(SupplyEvent {
                from: raw_log.topics[1].into(),
                reserve: raw_log.topics[2].into(),
                on_behalf_of: raw_log.topics[3].into(),
                amount,
                referral_code,
            });
        }
        _ => AaveEventType::Unknown,
    }
}

fn vec_u8_to_u256(data: &Vec<u8>) -> Result<U256, Box<dyn std::error::Error>> {
    if data.len() != 32 {
        return Err("Data length incorrect for U256, must be 32 bytes".into());
    }
    let data_slice = data.as_slice();
    Ok(U256::from_big_endian(data_slice))
}

fn decode_supply_event_data(data: &Vec<u8>) -> Result<(U256, u16), ethers::abi::Error> {
    // Decode data assuming it contains a U256 followed by a u16
    // Both U256 and u16 are ABI-encoded as 32 bytes, with u16 being padded on the right
    let data_slice = data.as_slice();
    let tokens = decode(&[ParamType::Uint(256), ParamType::Uint(16)], data_slice)?;

    let amount = match &tokens[0] {
        Token::Uint(value) => *value,
        _ => return Err(ethers::abi::Error::InvalidData),
    };

    let referral_code = match &tokens[1] {
        Token::Uint(value) => {
            // u16 should be within the last 2 bytes of the 32-byte aligned Uint
            (*value & U256::from(u16::MAX)).as_u32() as u16
        }
        _ => return Err(ethers::abi::Error::InvalidData),
    };

    Ok((amount, referral_code))
}
