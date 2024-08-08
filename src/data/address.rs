use ethers::types::Address;
use once_cell::sync::Lazy;

pub static WETH_ADDRESS: Lazy<Address> = Lazy::new(|| {
    "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
        .parse()
        .expect("Invalid address")
});
pub static AAVE_ORACLE_ADDRESS: Lazy<Address> = Lazy::new(|| {
    "0x54586bE62E3c3580375aE3723C145253060Ca0C2"
        .parse()
        .expect("Invalid address")
});
pub static AAVE_V3_POOL_ADDRESS: Lazy<Address> = Lazy::new(|| {
    "0x87870Bca3F3fD6335C3F4ce8392D69350B4fA4E2"
        .parse()
        .expect("Invalid address")
});

pub static AAVE_V3_DATA_PROVIDER_ADDRESS: Lazy<Address> = Lazy::new(|| {
    "0x7B4EB56E7CD4b454BA8ff71E4518426369a138a3"
        .parse()
        .expect("Invalid address")
});

pub static LIQUIDATE_USER_ADDRESS: Lazy<Address> = Lazy::new(|| {
    "0x0000000000000000000000000000000000000000"
        .parse()
        .expect("Invalid address")
});
