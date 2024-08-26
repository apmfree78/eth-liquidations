use ethers::abi::{Abi, Token};
use ethers::prelude::*;
use eyre::Result;
use std::error::Error;

pub fn decode_transmit_report(data: Vec<u8>) -> Result<Vec<U256>, Box<dyn Error>> {
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
                            println!("Observation: {:?}", value);
                            price_observations.push(*value);
                        }
                    }
                }
                _ => return Err("Failed to match decoded data with expected types".into()),
            }
        } else {
            return Err("Failed to decode report data".into());
        }
    } else {
        return Err("decodeReport function not found in ABI".into());
    }

    Ok(price_observations)
}
