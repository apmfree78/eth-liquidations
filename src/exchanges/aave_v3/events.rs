use crate::data::erc20::{u256_to_big_decimal, Erc20Token};
use crate::utils::type_conversion::address_to_string;
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use ethers::abi::Address;
use ethers::core::types::U256;

pub const WITHDRAW_SIGNATURE: &str = "Withdraw(address,address,address,uint256)";
pub const RESERVE_USED_AS_COLLATERAL_ENABLED_SIGNATURE: &str =
    "ReserveUsedAsCollateralEnabled(address,address)";
pub const RESERVE_USED_AS_COLLATERAL_DISABLED_SIGNATURE: &str =
    "ReserveUsedAsCollateralDisabled(address,address)";
pub const BORROW_SIGNATURE: &str = "Borrow(address,address,address,uint256,uint8,uint256,uint16)";
pub const REPAY_SIGNATURE: &str = "Repay(address,address,address,uint256,bool)";
pub const SUPPLY_SIGNATURE: &str = "Supply(address,address,address,uint256,uint16)";
pub const LIQUIDATION_SIGNATURE: &str =
    "LiquidationCall(address,address,address,uint256,uint256,address,bool)";

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AaveUserEvent {
    WithDraw,
    Borrow,
    Repay,
    Supply,
    ReserveUsedAsCollateralEnabled,
    ReserveUsedAsCollateralDisabled,
    Liquidation,
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
pub struct LiquidationEvent {
    pub collateral_asset: Address, // index
    pub debt_asset: Address,       // index
    pub user: Address,             // index
    pub debt_to_cover: U256,
    pub liquidation_collateral_amount: U256,
    pub liquidator: Address,
    pub received_a_token: bool,
}

impl LiquidationEvent {
    pub fn get_user(&self) -> Address {
        self.user
    }

    pub fn get_collateral_liquidated(&self, decimals: u32) -> f64 {
        let decimal_factor = BigDecimal::from_u64(10_u64.pow(decimals)).unwrap();
        let value = u256_to_big_decimal(&self.liquidation_collateral_amount) / decimal_factor;
        value.to_f64().unwrap()
    }

    pub fn get_amount_debt_reduced(&self, decimals: u32) -> f64 {
        let decimal_factor = BigDecimal::from_u64(10_u64.pow(decimals)).unwrap();
        let value = u256_to_big_decimal(&self.debt_to_cover) / decimal_factor;
        value.to_f64().unwrap()
    }

    pub fn get_debt_token_address(&self) -> String {
        address_to_string(self.debt_asset).to_lowercase()
    }

    pub fn get_collateral_token_address(&self) -> String {
        address_to_string(self.collateral_asset).to_lowercase()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AaveEventType {
    WithdrawEvent(WithdrawEvent),
    BorrowEvent(BorrowEvent),
    RepayEvent(RepayEvent),
    SupplyEvent(SupplyEvent),
    ReserveUsedAsCollateralEnabled(ReserveUsedAsCollateralEnabledEvent),
    ReserveUsedAsCollateralDisabled(ReserveUsedAsCollateralDisabledEvent),
    LiquidationEvent(LiquidationEvent),
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
