use crate::data::erc20::Erc20Token;
use bigdecimal::BigDecimal;
use ethers::abi::Address;
use ethers::core::types::U256;

#[derive(Clone, Copy, Debug)]
pub enum AaveUserEvent {
    WithDraw,
    Borrow,
    Repay,
    Supply,
    ReserveUsedAsCollateralEnabled,
    ReserveUsedAsCollateralDisabled,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct AaveUserAction {
    pub user_event: AaveUserEvent,
    pub user_address: Address,
    pub token: Erc20Token,
    pub amount_transferred: BigDecimal,
    pub use_a_tokens: bool,
}

pub trait ReserveCollateralEvent {
    fn new(reserve: Address, user: Address) -> Self
    where
        Self: Sized;
    fn get_reserve(&self) -> Address;
    fn get_user(&self) -> Address;
}

#[derive(Clone, Copy, Debug)]
pub struct ReserveUsedAsCollateralEnabledEvent {
    pub reserve: Address,
    pub user: Address,
}

#[derive(Clone, Copy, Debug)]
pub struct ReserveUsedAsCollateralDisabledEvent {
    pub reserve: Address,
    pub user: Address,
}

impl ReserveCollateralEvent for ReserveUsedAsCollateralEnabledEvent {
    fn new(reserve: Address, user: Address) -> Self {
        ReserveUsedAsCollateralEnabledEvent { reserve, user }
    }
    fn get_reserve(&self) -> Address {
        self.reserve
    }
    fn get_user(&self) -> Address {
        self.user
    }
}

impl ReserveCollateralEvent for ReserveUsedAsCollateralDisabledEvent {
    fn new(reserve: Address, user: Address) -> Self {
        ReserveUsedAsCollateralDisabledEvent { reserve, user }
    }
    fn get_reserve(&self) -> Address {
        self.reserve
    }
    fn get_user(&self) -> Address {
        self.user
    }
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
    ReserveUsedAsCollateralEnabled(ReserveUsedAsCollateralEnabledEvent),
    ReserveUsedAsCollateralDisabled(ReserveUsedAsCollateralDisabledEvent),
    Unknown,
}

pub trait AaveEvent {
    fn get_user(&self) -> Address;
    fn get_reserve(&self) -> Address;
    fn get_amount(&self) -> U256 {
        U256::from(0)
    }
    fn get_type(&self) -> AaveUserEvent;
    fn get_use_a_tokens(&self) -> bool {
        false
    }
}

impl AaveEvent for BorrowEvent {
    fn get_user(&self) -> Address {
        // on_behalf_of is user who will be update
        self.on_behalf_of
    }

    fn get_reserve(&self) -> Address {
        self.reserve
    }

    fn get_amount(&self) -> U256 {
        self.amount
    }

    fn get_type(&self) -> AaveUserEvent {
        AaveUserEvent::Borrow
    }
}

impl AaveEvent for RepayEvent {
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
        AaveUserEvent::Repay
    }
    fn get_use_a_tokens(&self) -> bool {
        self.use_a_tokens
    }
}

impl AaveEvent for SupplyEvent {
    fn get_user(&self) -> Address {
        // on_behalf_of is user who will be update
        self.on_behalf_of
    }

    fn get_reserve(&self) -> Address {
        self.reserve
    }

    fn get_amount(&self) -> U256 {
        self.amount
    }

    fn get_type(&self) -> AaveUserEvent {
        AaveUserEvent::Supply
    }
}

impl AaveEvent for WithdrawEvent {
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
        AaveUserEvent::WithDraw
    }
}

impl AaveEvent for ReserveUsedAsCollateralEnabledEvent {
    fn get_user(&self) -> Address {
        self.user
    }

    fn get_reserve(&self) -> Address {
        self.reserve
    }

    fn get_type(&self) -> AaveUserEvent {
        AaveUserEvent::ReserveUsedAsCollateralEnabled
    }
}

impl AaveEvent for ReserveUsedAsCollateralDisabledEvent {
    fn get_user(&self) -> Address {
        self.user
    }

    fn get_reserve(&self) -> Address {
        self.reserve
    }

    fn get_type(&self) -> AaveUserEvent {
        AaveUserEvent::ReserveUsedAsCollateralDisabled
    }
}
