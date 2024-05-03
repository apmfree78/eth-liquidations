use super::data::AaveUserData;
use crate::data::erc20::{u256_to_big_decimal, Erc20Token, TOKEN_DATA};
use bigdecimal::BigDecimal;
use ethers::abi::Address;
use ethers::core::abi::RawLog;
use ethers::core::types::{Log, U256};
use open_fastrlp::Decodable;

#[derive(Clone, Debug)]
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

pub struct WithdrawEvent {
    pub from: Address,
    pub reserve: Address,
    pub to: Address,
    pub amount: U256,
}

pub struct BorrowEvent {
    pub from: Address,
    pub reserve: Address,
    pub to: Address,
    pub amount: U256,
}

pub struct RepayEvent {
    pub from: Address,
    pub reserve: Address,
    pub to: Address,
    pub amount: U256,
}

pub struct SupplyEvent {
    pub from: Address,
    pub reserve: Address,
    pub on_behalf_of: Address,
    pub amount: U256,
    pub referral_code: u16,
}

pub enum AaveEventType {
    WithdrawEvent(WithdrawEvent),
    BorrowEvent(BorrowEvent),
    RepayEvent(RepayEvent),
    SupplyEvent(SupplyEvent),
    Unknown,
}

pub trait AaveEvent {
    fn get_from(&self) -> Address;
    fn get_reserve(&self) -> Address;
    fn get_amount(&self) -> U256;
    fn get_type(&self) -> AaveUserEvent;
}

macro_rules! impl_aave_event {
    ($struct_name:ident, $event_variant:expr) => {
        impl AaveEvent for $struct_name {
            fn get_from(&self) -> Address {
                self.from
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

pub fn get_user_action_from_event<T: AaveEvent>(event: &T) -> AaveUserAction {

    let token_address = event.get_reserve().to_string();
    let token = TOKEN_DATA.get(&token_address).unwrap();
    let amount = event.get_amount();
    let amount = u256_to_big_decimal(&amount);

    return AaveUserAction {
        user_event: event.get_type(),
        user_address: event.get_from(),
        token:*token,
        amount_transferred: amount,
    }
}

pub trait Update {
    fn update_user(
        &mut self,
        aave_action: AaveUserAction,
    ) -> Result<(), Box<dyn std::error::Error>>;

}


impl Update for AaveUserData {
    fn update_user(
        &mut self,
        aave_action: AaveUserAction,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match aave_action.user_event {
            AaveUserEvent::WithDraw => { }
            AaveUserEvent::Borrow => {}
            AaveUserEvent::Repay => {}
            AaveUserEvent::Supply => {}
            _ => {}
        }
        Ok(())
    }
}


fn create_aave_event_from_log(event_type:AaveUserEvent,log:&Log) -> AaveEventType {

    let raw_log:RawLog = RawLog::from(log.clone());

        match event_type {
            AaveUserEvent::WithDraw => {
                let amount = vec_u8_to_u256(&raw_log.data);
                return AaveEventType::WithdrawEvent(WithdrawEvent {
                    from: raw_log.topics[1].into(),
                    reserve: raw_log.topics[2].into(),
                    to: raw_log.topics[3].into(),
                    amount,
                });
        }
            AaveUserEvent::Borrow => {
                let amount = vec_u8_to_u256(&raw_log.data);
                return AaveEventType::BorrowEvent(BorrowEvent {
                    from: raw_log.topics[1].into(),
                    reserve: raw_log.topics[2].into(),
                    to: raw_log.topics[3].into(),
                    amount,
                });
        }
            AaveUserEvent::Repay => {

                let amount = vec_u8_to_u256(&raw_log.data);
                return AaveEventType::RepayEvent(RepayEvent {
                    from: raw_log.topics[1].into(),
                    reserve: raw_log.topics[2].into(),
                    to: raw_log.topics[3].into(),
                    amount,
                });

        }
            AaveUserEvent::Supply => {

            // TODO - fix data fields
                let amount = vec_u8_to_u256(&raw_log.data);
                return AaveEventType::SupplyEvent(SupplyEvent {
                    from: raw_log.topics[1].into(),
                    reserve: raw_log.topics[2].into(),
                    on_behalf_of: raw_log.topics[3].into(),
                    amount,
                    referral_code: 0,
                });
        }
            _ => AaveEventType::Unknown
        }

}

fn vec_u8_to_u256(data:&Vec<u8>)-> U256 {
   let mut data_slice = data.as_slice(); 
    U256::decode(&mut data_slice).unwrap()
}

