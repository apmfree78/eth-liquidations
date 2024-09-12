use crate::data::address::BTC;
use crate::data::erc20::{u256_to_big_decimal, Erc20Token};
use crate::data::token_data_hash::{
    get_token_data, get_tokens_priced_in_btc, get_tokens_priced_in_eth,
};
use crate::data::token_price_hash::get_saved_token_price;
use anyhow::{anyhow, Result};
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use ethers::abi::{Abi, Bytes, Token};
use ethers::prelude::*;

pub async fn get_chainlink_price_from_transmit_tx(data: &Bytes, token: &Erc20Token) -> Result<f64> {
    let token_priced_in_eth = get_tokens_priced_in_eth().await?;
    let token_priced_in_btc = get_tokens_priced_in_btc().await?;
    // 1. get observations
    let observations = decode_transmit_tx(data)?;

    // 2. find median
    let chainlink_price_unscaled = observations.get(observations.len() / 2).unwrap();

    // 3. scale price
    let is_priced_in_eth =
        token_priced_in_eth.contains_key(&token.symbol) && token.symbol != "WETH";
    let is_priced_in_btc = token_priced_in_btc.contains_key(&token.symbol) && token.symbol != "BTC";

    let scale = if is_priced_in_eth {
        BigDecimal::from_u64(10_u64.pow(18)).unwrap()
    } else {
        BigDecimal::from_u64(10_u64.pow(8)).unwrap()
    };

    let chainlink_price = u256_to_big_decimal(chainlink_price_unscaled) / scale;
    let mut chainlink_price = chainlink_price.to_f64().unwrap();

    // TODO - 4. tackle edge cases where price in ETH, convert to USD price
    if is_priced_in_eth {
        let token_data = get_token_data().await?;
        let weth = token_data.get("WETH").unwrap();
        let weth_price = get_saved_token_price(weth.address.to_lowercase()).await?;

        // convert price fromt TOKEN/ETH -> TOKEN/USD
        chainlink_price *= weth_price;
    } else if is_priced_in_btc {
        let btc_price = get_saved_token_price(BTC.to_lowercase()).await?;

        // convert price fromt TOKEN/BTC -> TOKEN/USD
        chainlink_price *= btc_price
    }

    Ok(chainlink_price)
}

fn decode_transmit_tx(data: &Bytes) -> Result<Vec<U256>> {
    let transmit_abi = r#"
    [
        {
            "constant": false,
            "inputs": [
                {"name": "report", "type": "bytes"},
                {"name": "rs", "type": "bytes32[]"},
                {"name": "ss", "type": "bytes32[]"},
                {"name": "rawVs", "type": "bytes32"}
            ],
            "name": "transmit",
            "outputs": [],
            "payable": false,
            "stateMutability": "nonpayable",
            "type": "function"
        }
    ]
    "#;

    let abi = Abi::load(transmit_abi.as_bytes())?;

    if let Ok(function) = abi.function("transmit") {
        if let Ok(decoded) = function.decode_input(&data[4..]) {
            // Skip selector if included
            match decoded.as_slice() {
                [Token::Bytes(report), Token::Array(_), Token::Array(_), Token::FixedBytes(_)] => {
                    let observations = decode_transmit_report(report.clone())?;
                    Ok(observations)
                }
                _ => Err(anyhow!(
                    "Failed to match decoded arguments with expected types"
                )),
            }
        } else {
            Err(anyhow!("Failed to decode input data for `transmit`"))
        }
    } else {
        Err(anyhow!("Transmit function not found in ABI"))
    }
}

fn decode_transmit_report(data: Vec<u8>) -> Result<Vec<U256>> {
    // Correctly formatted ABI JSON
    let decode_json = r#"
        [{
            "inputs": [
                {"type": "bytes32", "name": "first"},
                {"type": "bytes32", "name": "second"},
                {"type": "int192[]", "name": "observations"}
            ],
            "name": "decodeReport",
            "outputs": [],
            "type": "function"
        }]
    "#;

    // Load ABI
    let abi = Abi::load(decode_json.as_bytes())?;

    // Example report data as Vec<u8>, correctly formatted
    let mut price_observations = Vec::new();

    // Find the decodeReport function in the ABI
    if let Ok(decode_fn) = abi.function("decodeReport") {
        // Decode the input assuming the correct data is provided
        if let Ok(decoded) = decode_fn.decode_input(&data) {
            match decoded.as_slice() {
                [Token::FixedBytes(_), Token::FixedBytes(_), Token::Array(observations)] => {
                    for obs in observations {
                        if let Token::Int(value) = obs {
                            price_observations.push(*value);
                        }
                    }
                }
                _ => return Err(anyhow!("Failed to match decoded data with expected types")),
            }
        } else {
            return Err(anyhow!("Failed to decode report data"));
        }
    } else {
        return Err(anyhow!("decodeReport function not found in ABI"));
    }

    Ok(price_observations)
}
