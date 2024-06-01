use ethers::abi::Address;
use ethers::core::types::U256;
use ethers::types::H256;
use ethers::utils::keccak256;

pub fn str_to_h256_hash(str: &str) -> H256 {
    H256::from(keccak256(str.as_bytes()))
}

pub fn u256_to_bytes_array(number: U256) -> [u8; 32] {
    let mut number_bytes = [0u8; 32];
    number.to_big_endian(&mut number_bytes);
    number_bytes
}

pub fn boolean_to_bytes_array(boolean: bool) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes[31] = if boolean { 1 } else { 0 };
    bytes
}

pub fn u8_to_bytes_array(value: u8) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes[31] = value;
    bytes
}

pub fn u16_to_bytes_array(value: u16) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes[30..32].copy_from_slice(&value.to_be_bytes());
    bytes
}

pub fn address_to_bytes_array(address: Address) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes[12..32].copy_from_slice(&address.as_bytes());
    bytes
}

pub fn address_to_string(address: Address) -> String {
    format!("{:?}", address)
}
