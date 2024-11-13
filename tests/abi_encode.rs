use anyhow::anyhow;
use eth_liquadation::mempool::decode_new_price::{
    decode_transmit_report_param_debug, GasPriceUpdate, TokenPriceUpdate,
};
use ethers::abi::{ParamType, Token};
use ethers::prelude::*;
use std::str::FromStr;

#[test]
#[ignore]
fn decode_uint256_array() {
    // First 32 bytes: offset
    let array = "0000000000000000000000000000000000000000000000000000000000000020\
                 0000000000000000000000000000000000000000000000000000000000000003\
                 0000000000000000000000000000000000000000000000000000000000000001\
                 0000000000000000000000000000000000000000000000000000000000000002\
                 0000000000000000000000000000000000000000000000000000000000000003";
    let data = hex::decode(array).expect("could not decode array");

    let offset = U256::from_big_endian(&data[0..32]);
    println!("Offset: {}", offset);

    // Starting from offset:
    // First 32 bytes is length
    let start = offset.as_usize();
    let length = U256::from_big_endian(&data[start..start + 32]);
    println!("Array length: {}", length);

    let mut result = Vec::new();
    let mut pos = start + 32; // Start after length

    // Read each element
    for i in 0..length.as_usize() {
        if pos + 32 > data.len() {
            println!("data too short");
            return ();
        }
        let value = U256::from_big_endian(&data[pos..pos + 32]);
        println!("Element {}: {}", i, value);
        result.push(value);
        pos += 32;
    }

    println!("array => {:?}", result);
}

#[test]
#[ignore]
fn decode_person_struct() {
    // Encoded Person { age: 25, wallet: 0x123..., name: "Alice" }
    // Format:
    // struct Person {
    //   uint256 age;
    //   address wallet;
    //   string name;    // dynamic type
    // }
    // - age (uint256): 25
    // - wallet (address): 0x1234567890123456789012345678901234567890
    // - offset to name (uint256): 0x60 (points to string data)
    // - name length (uint256): 5
    // - name data: "Alice" padded to 32 bytes
    //
    let struct_data = "0000000000000000000000000000000000000000000000000000000000000019\
                       0000000000000000000000001234567890123456789012345678901234567890\
                       0000000000000000000000000000000000000000000000000000000000000060\
                       0000000000000000000000000000000000000000000000000000000000000005\
                       416c696365000000000000000000000000000000000000000000000000000000";

    let data = hex::decode(struct_data).expect("could not decode struct");

    // Read age (fixed size, straight from start)
    let age = U256::from_big_endian(&data[0..32]);
    println!("Age: {}", age);

    // Read wallet address (fixed size, next 32 bytes, but only last 20 bytes are address)
    let wallet = Address::from_slice(&data[44..64]); // Skip first 12 bytes of padding
    println!("Wallet: {:?}", wallet);

    // Read name (dynamic type)
    // First get offset to name data
    let name_offset = U256::from_big_endian(&data[64..96]).as_usize();
    println!("Name offset: {}", name_offset);

    // At offset, first 32 bytes is string length
    let name_length = U256::from_big_endian(&data[name_offset..name_offset + 32]).as_usize();
    println!("Name length: {}", name_length);

    // Next bytes are the actual string data
    let name_data = &data[name_offset + 32..name_offset + 32 + 32]; // Read full 32-byte chunk
    let name = String::from_utf8(name_data[..name_length].to_vec()).expect("Invalid UTF-8");
    println!("Name: {}", name);
}

#[test]
#[ignore]
fn decode_person_struct_array() {
    // Encoding of array of Person[2] containing:
    // [{wallet: 0x1234567890123456789012345678901234567890, age: 25},
    //  {wallet: 0xaabbccddee123456789012345678901234567890, age: 30}]

    let array = "0000000000000000000000000000000000000000000000000000000000000020\
                 0000000000000000000000000000000000000000000000000000000000000002\
                 0000000000000000000000001234567890123456789012345678901234567890\
                 0000000000000000000000000000000000000000000000000000000000000019\
                 000000000000000000000000aabbccddee123456789012345678901234567890\
                 000000000000000000000000000000000000000000000000000000000000001e";

    let data = hex::decode(array).expect("could not decode array");

    // First get the offset to array data
    let offset = U256::from_big_endian(&data[0..32]);
    println!("Array offset: {}", offset);

    // At offset position, get array length
    let start = offset.as_usize();
    let length = U256::from_big_endian(&data[start..start + 32]);
    println!("Array length: {}", length);

    // Now read each Person struct
    let mut pos = start + 32; // Start after length
    for i in 0..length.as_usize() {
        if pos + 64 > data.len() {
            // Each Person is 2 * 32 bytes (wallet + age)
            println!("Data too short");
            return;
        }

        // Read wallet (address)
        let wallet = Address::from_slice(&data[pos + 12..pos + 32]); // Skip 12 bytes padding
        println!("Person {} wallet: {:?}", i, wallet);

        // Read age
        let age = U256::from_big_endian(&data[pos + 32..pos + 64]);
        println!("Person {} age: {}", i, age);

        pos += 64; // Move to next Person struct
    }
}

#[test]
#[ignore]
fn decode_price_updates_struct() {
    let array = "0000000000000000000000000000000000000000000000000000000000000020\
0000000000000000000000000000000000000000000000000000000000000002\
0000000000000000000000007f39c581f595b53c5cb19bd0b3f8da6c935e2ca0\
0000000000000000000000000000000000000000001bc16d674ec80000000000\
000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2\
0000000000000000000000000000000000000000001e87f85809dc0000000000\
000000000000000000000000000000000000000000000000000000000a4a1000\
0000000000000000000000000000000000000000000000000000017d78400000";

    // Debug prints
    println!("String length: {}", array.len());
    println!("String chars: {}", array.chars().count());

    // Remove any hidden characters
    let array = array
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    println!("Cleaned string length: {}", array.len());

    let data = hex::decode(&array).expect("could not decode array");
    println!("Data length: {}", data.len());

    // Rest of your code...
    // First get the array offset (for tokenPriceUpdates array)
    let offset = U256::from_big_endian(&data[0..32]);
    println!("Array offset: {}", offset);

    // At offset position, get array length
    let start = offset.as_usize();
    let length = U256::from_big_endian(&data[start..start + 32]);
    println!("TokenPriceUpdates array length: {}", length);

    // Read each TokenPriceUpdate
    let mut pos = start + 32; // Start after length
    for i in 0..length.as_usize() {
        if pos + 64 > data.len() {
            // Each TokenPriceUpdate is 2 * 32 bytes (sourceToken + usdPerToken)
            println!("Data too short");
            return;
        }

        // Read sourceToken (address)
        let source_token = Address::from_slice(&data[pos + 12..pos + 32]); // Skip 12 bytes padding
        println!("TokenPriceUpdate {} sourceToken: {:?}", i, source_token);

        // Read usdPerToken (uint192)
        let usd_per_token = U256::from_big_endian(&data[pos + 32..pos + 64]);
        println!("TokenPriceUpdate {} usdPerToken: {}", i, usd_per_token);

        pos += 64; // Move to next TokenPriceUpdate
    }

    // After the array, read destChainSelector (uint64)
    let dest_chain_selector = U256::from_big_endian(&data[pos..pos + 32]);
    println!("destChainSelector: {}", dest_chain_selector);
    pos += 32;

    // Finally read usdPerUnitGas (uint192)
    let usd_per_unit_gas = U256::from_big_endian(&data[pos..pos + 32]);
    println!("usdPerUnitGas: {}", usd_per_unit_gas);
}

#[test]
#[ignore]
fn decode_commit_report() {
    // CommitReport containing:
    // - PriceUpdates offset (points to PriceUpdates data because it's dynamic)
    // - range (uint256)
    // - merkleRoot (bytes32)
    let commit_report = "0000000000000000000000000000000000000000000000000000000000000060\
0000000000000000000000000000000000000000000000000000000000000123\
1234567890123456789012345678901234567890123456789012345678901234\
0000000000000000000000000000000000000000000000000000000000000002\
0000000000000000000000007f39c581f595b53c5cb19bd0b3f8da6c935e2ca0\
0000000000000000000000000000000000000000001bc16d674ec80000000000\
000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2\
0000000000000000000000000000000000000000001e87f85809dc0000000000\
000000000000000000000000000000000000000000000000000000000a4a1000\
0000000000000000000000000000000000000000000000000000017d78400000";

    let data = hex::decode(commit_report).expect("could not decode array");
    println!("Data length: {}", data.len());

    // First word is offset to PriceUpdates data (because it's dynamic)
    let price_updates_offset = U256::from_big_endian(&data[0..32]);
    println!("PriceUpdates offset: {}", price_updates_offset);

    // Read range (uint256)
    let range = U256::from_big_endian(&data[32..64]);
    println!("Range: {}", range);

    // Read merkleRoot (bytes32)
    let merkle_root = &data[64..96];
    println!("MerkleRoot: 0x{}", hex::encode(merkle_root));

    // Now decode PriceUpdates at its offset
    let price_updates_start = price_updates_offset.as_usize();

    // Get token updates array length
    let token_updates_length =
        U256::from_big_endian(&data[price_updates_start..price_updates_start + 32]);
    println!("\nTokenPriceUpdates length: {}", token_updates_length);

    // Read each TokenPriceUpdate
    let mut pos = price_updates_start + 32; // Start after length
    for i in 0..token_updates_length.as_usize() {
        if pos + 64 > data.len() {
            println!("Data too short for token update {}", i);
            return;
        }

        // Read sourceToken (address)
        let source_token = Address::from_slice(&data[pos + 12..pos + 32]);
        println!("\nTokenPriceUpdate {}:", i);
        println!("  sourceToken: {:?}", source_token);

        // Read usdPerToken (uint192)
        let usd_per_token = U256::from_big_endian(&data[pos + 32..pos + 64]);
        println!("  usdPerToken: {}", usd_per_token);

        pos += 64;
    }

    // After token updates array, read remaining PriceUpdates fields
    // Read destChainSelector (uint64)
    let dest_chain_selector = U256::from_big_endian(&data[pos..pos + 32]);
    println!("\nDestChainSelector: {}", dest_chain_selector);
    pos += 32;

    // Read usdPerUnitGas (uint192)
    let usd_per_unit_gas = U256::from_big_endian(&data[pos..pos + 32]);
    println!("UsdPerUnitGas: {}", usd_per_unit_gas);
}

#[test]
#[ignore]
fn decode_price_updates_struct_debug() {
    let chunks = [
        "0000000000000000000000000000000000000000000000000000000000000020",
        "0000000000000000000000000000000000000000000000000000000000000002",
        "0000000000000000000000007f39c581f595b53c5cb19bd0b3f8da6c935e2ca0",
        "0000000000000000000000000000000000000000001bc16d674ec80000000000",
        "000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
        "0000000000000000000000000000000000000000001e87f85809dc0000000000",
        "000000000000000000000000000000000000000000000000000000000a4a1000",
        "0000000000000000000000000000000000000000000000000000017d78400000", // Fixed: added '0' at end
    ];

    // Verify each chunk is 64 characters
    for (i, chunk) in chunks.iter().enumerate() {
        println!("Chunk {} length: {}", i, chunk.len());
        assert_eq!(chunk.len(), 64, "Chunk {} is not 64 characters", i);
    }

    let array = chunks.join("");
    println!("Final string length: {}", array.len());
    println!("Is length even? {}", array.len() % 2 == 0);

    let data = hex::decode(&array).expect("could not decode array");
    println!("Decoded data length: {}", data.len());

    // Rest of decoding logic...
    let offset = U256::from_big_endian(&data[0..32]);
    println!("Array offset: {}", offset);

    let start = offset.as_usize();
    let length = U256::from_big_endian(&data[start..start + 32]);
    println!("TokenPriceUpdates array length: {}", length);

    let mut pos = start + 32;
    for i in 0..length.as_usize() {
        if pos + 64 > data.len() {
            println!("Data too short");
            return;
        }

        let source_token = Address::from_slice(&data[pos + 12..pos + 32]);
        println!("TokenPriceUpdate {} sourceToken: {:?}", i, source_token);

        let usd_per_token = U256::from_big_endian(&data[pos + 32..pos + 64]);
        println!("TokenPriceUpdate {} usdPerToken: {}", i, usd_per_token);

        pos += 64;
    }
}

#[test]
#[ignore]
fn decode_commit_report_with_interval() {
    // CommitReport layout:
    // [0]:  PriceUpdates offset (0x80)
    // [1]:  interval.min
    // [2]:  interval.max
    // [3]:  merkleRoot
    // At PriceUpdates offset:
    // [4]:  offset to tokenPriceUpdates array
    // [5]:  offset to gasPriceUpdates array
    // At tokenPriceUpdates offset:
    // [6]:  tokenPriceUpdates length
    // [7+]: tokenPriceUpdates elements (each element is address + uint224)
    // At gasPriceUpdates offset:
    // [N]:  gasPriceUpdates length
    // [N+]: gasPriceUpdates elements (each element is uint64 + uint224)

    // First let's verify our hex string is valid
    let commit_report = [
        // Offset to PriceUpdates (128 bytes = 0x80)
        "0000000000000000000000000000000000000000000000000000000000000080",
        // interval.min (1000 = 0x3e8)
        "00000000000000000000000000000000000000000000000000000000000003e8",
        // interval.max (2000 = 0x7d0)
        "00000000000000000000000000000000000000000000000000000000000007d0",
        // merkleRoot
        "0000000000000000000000000000000000000000000000000000000000001234",
        // Offset to tokenPriceUpdates array
        "0000000000000000000000000000000000000000000000000000000000000040",
        // Offset to gasPriceUpdates array
        "00000000000000000000000000000000000000000000000000000000000000E0",
        // TokenPriceUpdates length (2)
        "0000000000000000000000000000000000000000000000000000000000000002",
        // First token address
        "0000000000000000000000007f39c581f595b53c5cb19bd0b3f8da6c935e2ca0",
        // First token price
        "0000000000000000000000000000000000000000001bc16d674ec80000000000",
        // Second token address
        "000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
        // Second token price
        "0000000000000000000000000000000000000000001e87f85809dc0000000000",
        // GasPriceUpdates length (2)
        "0000000000000000000000000000000000000000000000000000000000000002",
        // First gas chain selector
        "00000000000000000000000000000000000000000000000000000000000a4a10",
        // First gas price
        "0000000000000000000000000000000000000000000000000000017d78400000",
        // Second gas chain selector
        "00000000000000000000000000000000000000000000000000000000001e8480",
        // Second gas price
        "0000000000000000000000000000000000000000000000000000000174876e80", // Removed extra '0'
    ]
    .join("");

    println!("Hex string length: {}", commit_report.len());
    assert_eq!(
        commit_report.len() % 64,
        0,
        "Hex string length should be multiple of 64"
    );
    let data = hex::decode(&commit_report).expect("could not decode array");

    // Debug prints for each 32-byte word
    for (i, chunk) in data.chunks(32).enumerate() {
        println!("Word {}: 0x{}", i, hex::encode(chunk));
    }

    // First word is offset to PriceUpdates data
    let price_updates_offset = U256::from_big_endian(&data[0..32]);
    println!("\nPriceUpdates offset: {}", price_updates_offset);

    // Read Interval struct
    let interval_min = U256::from_big_endian(&data[32..64]);
    let interval_max = U256::from_big_endian(&data[64..96]);
    println!("Interval:");
    println!("  min: {}", interval_min);
    println!("  max: {}", interval_max);

    // Read merkleRoot (bytes32)
    let merkle_root = &data[96..128];
    println!("MerkleRoot: 0x{}", hex::encode(merkle_root));

    // At PriceUpdates offset, read the two array offsets
    let price_struct_start = price_updates_offset.as_usize();
    let token_updates_offset =
        U256::from_big_endian(&data[price_struct_start..price_struct_start + 32]);
    let gas_updates_offset =
        U256::from_big_endian(&data[price_struct_start + 32..price_struct_start + 32 + 32]);
    println!("Token updates offset: {}", token_updates_offset);
    println!("Gas updates offset: {}", gas_updates_offset);

    // TokenPriceUpdates array starts at PriceUpdates offset + token_updates_offset
    let token_array_start = price_struct_start + token_updates_offset.as_usize();
    let token_length = U256::from_big_endian(&data[token_array_start..token_array_start + 32]);
    println!("\nTokenPriceUpdates length: {}", token_length);

    // Read TokenPriceUpdates
    let mut pos = token_array_start + 32;
    for i in 0..token_length.as_usize() {
        let source_token = Address::from_slice(&data[pos + 12..pos + 32]);
        println!("\nTokenPriceUpdate {}:", i);
        println!("  sourceToken: {:?}", source_token);
        let usd_per_token = U256::from_big_endian(&data[pos + 32..pos + 64]);
        println!("  usdPerToken: {}", usd_per_token);
        pos += 64;
    }

    // GasPriceUpdates array starts at PriceUpdates offset + gas_updates_offset
    let gas_array_start = price_struct_start + gas_updates_offset.as_usize();
    let gas_length = U256::from_big_endian(&data[gas_array_start..gas_array_start + 32]);
    println!("\nGasPriceUpdates length: {}", gas_length);

    // Read GasPriceUpdates
    pos = gas_array_start + 32;
    for i in 0..gas_length.as_usize() {
        let chain_selector = U256::from_big_endian(&data[pos..pos + 32]);
        println!("\nGasPriceUpdate {}:", i);
        println!("  destChainSelector: {}", chain_selector);
        let usd_per_unit_gas = U256::from_big_endian(&data[pos + 32..pos + 64]);
        println!("  usdPerUnitGas: {}", usd_per_unit_gas);
        pos += 64;
    }
}

#[test]
#[ignore]
fn decode_commit_report_with_interval_2() {
    // CommitReport
    // [0]:  PriceUpdates offset (0x80)
    // [1]:  interval.min (1000)
    // [2]:  interval.max (2000)
    // [3]:  merkleRoot
    // [4]:  array length (2)
    // [5]:  first token address
    // [6]:  first token price
    // [7]:  second token address
    // [8]:  second token price
    // [9]:  destChainSelector
    // [10]: usdPerUnitGas
    let commit_report = "0000000000000000000000000000000000000000000000000000000000000080\
00000000000000000000000000000000000000000000000000000000000003e8\
00000000000000000000000000000000000000000000000000000000000007d0\
0000000000000000000000000000000000000000000000000000000000001234\
0000000000000000000000000000000000000000000000000000000000000002\
0000000000000000000000007f39c581f595b53c5cb19bd0b3f8da6c935e2ca0\
0000000000000000000000000000000000000000001bc16d674ec80000000000\
000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2\
0000000000000000000000000000000000000000001e87f85809dc0000000000\
000000000000000000000000000000000000000000000000000000000a4a1000\
0000000000000000000000000000000000000000000000000000017d78400000";

    let data = hex::decode(commit_report).expect("could not decode array");

    // Debug prints for each 32-byte word
    for (i, chunk) in data.chunks(32).enumerate() {
        println!("Word {}: 0x{}", i, hex::encode(chunk));
    }

    // First word is offset to PriceUpdates data
    let price_updates_offset = U256::from_big_endian(&data[0..32]);
    println!("\nPriceUpdates offset: {}", price_updates_offset);

    // Read Interval struct
    let interval_min = U256::from_big_endian(&data[32..64]);
    let interval_max = U256::from_big_endian(&data[64..96]);
    println!("Interval:");
    println!("  min: {}", interval_min);
    println!("  max: {}", interval_max);

    // Read merkleRoot (bytes32)
    let merkle_root = &data[96..128];
    println!("MerkleRoot: 0x{}", hex::encode(merkle_root));

    // Debug print the word at price_updates_start
    let price_updates_start = price_updates_offset.as_usize();
    println!("\nReading array length at offset {}", price_updates_start);
    let array_data = &data[price_updates_start..price_updates_start + 32];
    println!("Array length word: 0x{}", hex::encode(array_data));

    let token_updates_length = U256::from_big_endian(array_data);
    println!("TokenPriceUpdates length: {}", token_updates_length);
    assert!(
        token_updates_length == U256::from(2),
        "Expected array length of 2"
    );

    let mut pos = price_updates_start + 32;
    for i in 0..token_updates_length.as_usize() {
        // Read sourceToken (address)
        let source_token = Address::from_slice(&data[pos + 12..pos + 32]);
        println!("\nTokenPriceUpdate {}:", i);
        println!("  sourceToken: {:?}", source_token);

        // Read usdPerToken (uint192)
        let usd_per_token = U256::from_big_endian(&data[pos + 32..pos + 64]);
        println!("  usdPerToken: {}", usd_per_token);

        pos += 64;
    }

    // Read destChainSelector (uint64)
    let dest_chain_selector = U256::from_big_endian(&data[pos..pos + 32]);
    println!("\nDestChainSelector: {}", dest_chain_selector);
    pos += 32;

    // Read usdPerUnitGas (uint192)
    let usd_per_unit_gas = U256::from_big_endian(&data[pos..pos + 32]);
    println!("UsdPerUnitGas: {}", usd_per_unit_gas);
}

#[test]
#[ignore]
fn decode_actual_commit_report_with_interval() {
    // CommitReport
    // [0]:  PriceUpdates offset (0x80)
    // [1]:  interval.min
    // [2]:  interval.max
    // [3]:  merkleRoot
    // [4]:  array length
    // [5]:  first token address
    // [6]:  first token price
    // [7]:  second token address
    // [8]:  second token price
    // [9]:  destChainSelector
    // [10]: usdPerUnitGas

    let commit_report = "0000000000000000000000000000000000000000000000000000000000000080\
0000000000000000000000000000000000000000000000000000000000000900\
0000000000000000000000000000000000000000000000000000000000000c20\
0000000000000000000000000000000000000000000000000000000000000001\
0000000000000000000000000000000000000000000000000000000000000002\
0000000000000000000000000000000000000000000000000000000000000040\
0000000000000000000000000000000000000000000000000000000000000300\
000000000000000000000000000000000000000000000000dda641cfe44aff82\
0000000000000000000000003b41e21094611d152a08d3691a70837f1a077dae\
00000000000000000000000088135dd0e7a8a2e42272dda89849a997ce2e83f7\
00000000000000000000000000000000000000000000000000000000000013a1\
0000000000000000000000000000000000000000000000000000000000030d40\
0000000000000000000000000000000000000000000000000000000000000000\
0000000000000000000000000000000000000000000000000000000000002710\
0000000000000000000000004200000000000000000000000000000000000006\
000000000000000000000000000000000000000000000000001097c0c999165b\
00000000000000000000000000000000000000000000000000000000000001a0\
0000000000000000000000000000000000000000000000000000000000000280\
00000000000000000000000000000000000000000000000000000000000002a0\
397883be58f14c50cfdfdecc76603b426dc2eeb9a1425e4a02a4e446922ec9a5\
00000000000000000000000000000000000000000000000000000000000000c0\
0000000000000000000000000000000000000000000000000000000000000040\
0000000000000000000000000000000000000000000000000000000000000080\
0000000000000000000000000000000000000000000000000000000000000001\
000000000000000000000000099b65a61e66bdb247c0926df8b6bc3aa34f5d5e\
0000000000000000000000000000000000000000000000000000000000000001\
000000000000000000000000000000000000000000000094745268ce1f3a0000\
0000000000000000000000000000000000000000000000000000000000000000\
0000000000000000000000000000000000000000000000000000000000000000\
000000000000000000000000000000000000000000000000dda641cfe44aff82\
000000000000000000000000a0950842b58fce378d2f546928b3e94d71bdffd4\
0000000000000000000000009dbedc5da9a9eeda65d0952a5c8a21039280c38b\
00000000000000000000000000000000000000000000000000000000000013a2\
000000000000000000000000000000000000000000000000000000000016e360\
0000000000000000000000000000000000000000000000000000000000000000\
0000000000000000000000000000000000000000000000000000000000000001\
0000000000000000000000004200000000000000000000000000000000000006\
000000000000000000000000000000000000000000000000003b039529a3ef98\
00000000000000000000000000000000000000000000000000000000000001a0\
0000000000000000000000000000000000000000000000000000000000000460\
00000000000000000000000000000000000000000000000000000000000004c0\
dba5579ccfb7020b09594a029b3993da3eb247fa0d2b4dc2f103c681f991928b\
00000000000000000000000000000000000000000000000000000000000002a0\
0000000000000000000000000000000000000000000000000000000000000020\
0000000000000000000000000000000000000000000000000000000000000260\
0000000000000000000000000000000000000000000000000000000000000020\
0000000000000000000000000000000000000000000000000000000000000001\
0000000000000000000000000000000000000000000000000000000000000002\
0000000000000000000000000000000000000000000000000000000000018574\
0000000000000000000000000000000000000000000000000000000000000000\
0000000000000000000000000000000000000000000000000000000000000140\
000000000000000000000000000000000000000000000000de0b6b3a763ffff0\
00000000000000000000000000000000000000000000000000000000000000000\
000000000000000000000068b3465833fb72a70ecdf485e0e4c7bd8665fc4500\
00000000000000000000000000000000000000000000000000000000000001a0\
000000000000000000000000686c0072df3df7a13ef666a3b661803a48558a90\
0000000000000000000000000000000000000000000000000000000000000001\
0000000000000000000000000000000000000000000000000000000000000064\
0000000000000000000000000000000000000000000000000000000000000002\
000000000000000000000000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48\
0000000000000000000000000000000000000000000000000000000000000000\
000000000000000000000000000000000000000000000000000000000000002b\
a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48000064c02aaa39b223fe8d0a\
0e5c4f27ead9083c756cc20000000000000000000000000000000000000000000\
0000000000000000000000000000000000000000000000000000000000000001\
000000000000000000000000833589fcd6edb6e08f4c7c32d4f71b54bda02913\
0000000000000000000000000000000000000000000000000000000000018574\
0000000000000000000000000000000000000000000000000000000000000001\
0000000000000000000000000000000000000000000000000000000000000002\
0000000000000000000000000000000000000000000000000000000000000040\
000000000000000000000000000000000000000000000000000000000004f2de\
0000000000000000000000000000000000000000000000000000000000000006\
0000000000000000000000000000000000000000000000000000000000000002\
0000000000000000000000000000000000000000000000000000000000000040\
0000000000000000000000000000000000000000000000000000000000000060\
0000000000000000000000000000000000000000000000000000000000000000\
0000000000000000000000000000000000000000000000000000000000000001\
0000000000000000000000000000000000000000000000000000000000000020\
0000000000000000000000000000000000000000000000000000000000000240\
0000000000000000000000000000000000000000000000000000000000000002\
0000000000000000000000000000000000000000000000000000000000000040\
0000000000000000000000000000000000000000000000000000000000000160\
00000000000000000000000000000000000000000000000000000000000000f8\
000000000000000600000000000000000004f2de00000000000000000000000016\
82ae6375c4e4a97e4b583bc394c861a46d8962000000000000000000000000bd3f\
a81b58ba92a82136038b25adec7066af3155000000000000000000000000a81f4a\
b595de5c14759245de5ce9899d380fefda00000000000000000000000000000000\
833589fcd6edb6e08f4c7c32d4f71b54bda029130000000000000000000000009d\
bedc5da9a9eeda65d0952a5c8a21039280c38b0000000000000000000000000000\
00000000000000000000000000000001857400000000000000000000000055a578\
6ca51c31623f3efb8bbfcc8df9a4c61ba90000000000000000000000000000000000\
0000000000000000000000000000000000000000000082b3d760455b2f5e23a21ec\
874911a6d786e1237ed26431f58d27a069f2fa1c0347d7dae01e2a1c4ff0b08572d\
0ffa91c73bc48fb25f896ed3b07edcc24d4f7a171bcae58f1fdbac32582ec9df573\
b040d2de73178653905907a7c145916f63f1fc546ee800facb0bc7d4e10399b6294\
f8c28ba42eedc06df8dfc2e4ee16e29562641b000000000000000000000000000000\
0000000000000000000000000000000000000000000000000000000000000000000000\
000000000000000000000000";

    let data = hex::decode(commit_report).expect("could not decode array");

    // Debug prints for each 32-byte word
    for (i, chunk) in data.chunks(32).enumerate() {
        println!("Word {}: 0x{}", i, hex::encode(chunk));
    }

    // First word is offset to PriceUpdates data
    let price_updates_offset = U256::from_big_endian(&data[0..32]);
    println!("\nPriceUpdates offset: {}", price_updates_offset);

    // Read Interval struct
    let interval_min = U256::from_big_endian(&data[32..64]);
    let interval_max = U256::from_big_endian(&data[64..96]);
    println!("Interval:");
    println!("  min: {}", interval_min);
    println!("  max: {}", interval_max);

    // Read merkleRoot (bytes32)
    let merkle_root = &data[96..128];
    println!("MerkleRoot: 0x{}", hex::encode(merkle_root));

    // Debug print the word at price_updates_start
    let price_updates_start = price_updates_offset.as_usize();
    println!("\nReading array length at offset {}", price_updates_start);
    let array_data = &data[price_updates_start..price_updates_start + 32];
    println!("Array length word: 0x{}", hex::encode(array_data));

    let token_updates_length = U256::from_big_endian(array_data);
    println!("TokenPriceUpdates length: {}", token_updates_length);

    let mut pos = price_updates_start + 32;
    for i in 0..token_updates_length.as_usize() {
        // Read sourceToken (address)
        let source_token = Address::from_slice(&data[pos + 12..pos + 32]);
        println!("\nTokenPriceUpdate {}:", i);
        println!("  sourceToken: {:?}", source_token);

        // Read usdPerToken (uint192)
        let usd_per_token = U256::from_big_endian(&data[pos + 32..pos + 64]);
        println!("  usdPerToken: {}", usd_per_token);

        pos += 64;
    }

    // Read destChainSelector (uint64)
    let dest_chain_selector = U256::from_big_endian(&data[pos..pos + 32]);
    println!("\nDestChainSelector: {}", dest_chain_selector);
    pos += 32;

    // Read usdPerUnitGas (uint192)
    let usd_per_unit_gas = U256::from_big_endian(&data[pos..pos + 32]);
    println!("UsdPerUnitGas: {}", usd_per_unit_gas);
}

#[test]
fn decode_transmit() {
    let input = "Bytes(0xb1dc65a40001b728806c50675d907f631fb922ee3658492327dfb556b5638ce5ac48a329000000000000000000000000000000000000000000000000000000000063c4038f09bb89dc580d512e5d37af506dca38299a1b93d271d88f2b9eb835a6d7664700000000000000000000000000000000000000000000000000000000000000e000000000000000000000000000000000000000000000000000000000000002a00000000000000000000000000000000000000000000000000000000000000380010100000101000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001a00000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000674b9172b4b4974000000000000000000000000000000000000000000000000000000037a7cad14000000000000000000000000000000000000000000000000eeac290d90ce4b09000000000000000000000000000ed825f22d0000000000000000000cc0e8cf390000000000000000000000000000000000000000000000000000000000000006c8cccd655df8821f1f1f83ae0fbd41d2f1be00029d11e3d88877125b47b7d4ad70cc6534b2ae946f7820c79d51e119cca83a12ac8375264557987c96bf0636b62ee12d04755282cb7bf87aa4fc1b8a2b45b3c2083f68962b06c5dbddc0306fbc112c9b5e0f359eb81afc9cbc337ba163201f1b2591d4007de19d866d624cb75a768905545a3a7ae8cf6541917d8b3049f82163f01f51c5422afe7b47695412bffe657acbf1e6c2abdd2edfd33d48e1f3209a1e0282e61cd0a0f9aa28cf37372500000000000000000000000000000000000000000000000000000000000000061c482f21f86b484af74a0dc4420506d912f4bbaf158fa213065db5c1fd2b22ad4fdc667aded741704ec5c0f80e385b1d330855a32182f5ad038b3a977ed7a6e971e2400384dc084902c1cbee42c6942a0b49f52198343ee7e6fa0236bc580fd103c0af2395df5a54d0daf5e7d5526f356fd6b5f022a1a48003e2e125a57bc40e7feac40b00c561901b154708ce56233eb21ff4a8ac4845621aff134a17aa019b4d76e7ff5ff74b844db81b18c12f41a36f6ba233635613007e0bb10fe79f7e38)";

    // Strip "Bytes(0x" prefix and ")" suffix
    let hex_str = input
        .strip_prefix("Bytes(0x")
        .and_then(|s| s.strip_suffix(")"))
        .unwrap();

    let bytes = Bytes::from_str(hex_str).unwrap();

    // TODO - REMOVE first 4 bytes
    let (token_price_updates, _) = decode_transmit_report(&bytes).expect("could not decode report");

    println!("token_price_update ==> {:#?}", token_price_updates);
}

fn decode_transmit_report(
    report: &[u8],
) -> anyhow::Result<(Vec<TokenPriceUpdate>, Vec<GasPriceUpdate>)> {
    // Define the struct types using ParamType
    let token_price_update = ParamType::Tuple(vec![
        ParamType::Address,   // sourceToken
        ParamType::Uint(224), // usdPerToken
    ]);

    let gas_price_update = ParamType::Tuple(vec![
        ParamType::Uint(64),  // chain selector
        ParamType::Uint(224), // usdPerUnitGas
    ]);

    // Note: arrays are dynamic types, so they start with an offset
    let price_updates = ParamType::Tuple(vec![
        ParamType::Array(Box::new(token_price_update.clone())),
        ParamType::Array(Box::new(gas_price_update.clone())),
    ]);

    let interval = ParamType::Tuple(vec![ParamType::Uint(64), ParamType::Uint(64)]);

    let commit_report = ParamType::Tuple(vec![
        price_updates, // priceUpdates (starts with offset)
        interval,
        ParamType::FixedBytes(32), // merkleRoot
    ]);

    // Decode the report bytes
    let decoded = ethers::abi::decode(&[commit_report], report).expect("could not decode report");

    match &decoded[..] {
        [Token::Tuple(outer_tuple)] => {
            match &outer_tuple[..] {
                [Token::Tuple(price_updates), Token::Tuple(interval), Token::FixedBytes(_)] => {
                    println!("interval {:?}", interval);
                    match price_updates.as_slice() {
                        [Token::Array(token_updates), Token::Array(gas_updates)] => {
                            let mut token_results = Vec::new();
                            let mut gas_results = Vec::new();

                            // Process token price updates
                            for token in token_updates {
                                if let Token::Tuple(values) = token {
                                    if let [Token::Address(addr), Token::Uint(price)] =
                                        values.as_slice()
                                    {
                                        token_results.push(TokenPriceUpdate {
                                            source_token: *addr,
                                            usd_per_token: *price,
                                        });
                                    }
                                }
                            }

                            // Process gas price updates
                            for gas in gas_updates {
                                if let Token::Tuple(values) = gas {
                                    if let [Token::Uint(selector), Token::Uint(price)] =
                                        values.as_slice()
                                    {
                                        gas_results.push(GasPriceUpdate {
                                            dest_chain_selector: selector.as_u64(),
                                            usd_per_unit_gas: *price,
                                        });
                                    }
                                }
                            }

                            Ok((token_results, gas_results))
                        }
                        _ => Err(anyhow!("Invalid price updates format")),
                    }
                }
                _ => Err(anyhow!("Invalid commit report format")),
            }
        }
        _ => Err(anyhow!("Failed to decode commit report")),
    }
}

#[test]
fn hex_padding() {
    let input = "Bytes(0xb1dc65a40001b728806c50675d907f631fb922ee3658492327dfb556b5638ce5ac48a329000000000000000000000000000000000000000000000000000000000063c4038f09bb89dc580d512e5d37af506dca38299a1b93d271d88f2b9eb835a6d7664700000000000000000000000000000000000000000000000000000000000000e000000000000000000000000000000000000000000000000000000000000002a00000000000000000000000000000000000000000000000000000000000000380010100000101000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001a00000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000674b9172b4b4974000000000000000000000000000000000000000000000000000000037a7cad14000000000000000000000000000000000000000000000000eeac290d90ce4b09000000000000000000000000000ed825f22d0000000000000000000cc0e8cf390000000000000000000000000000000000000000000000000000000000000006c8cccd655df8821f1f1f83ae0fbd41d2f1be00029d11e3d88877125b47b7d4ad70cc6534b2ae946f7820c79d51e119cca83a12ac8375264557987c96bf0636b62ee12d04755282cb7bf87aa4fc1b8a2b45b3c2083f68962b06c5dbddc0306fbc112c9b5e0f359eb81afc9cbc337ba163201f1b2591d4007de19d866d624cb75a768905545a3a7ae8cf6541917d8b3049f82163f01f51c5422afe7b47695412bffe657acbf1e6c2abdd2edfd33d48e1f3209a1e0282e61cd0a0f9aa28cf37372500000000000000000000000000000000000000000000000000000000000000061c482f21f86b484af74a0dc4420506d912f4bbaf158fa213065db5c1fd2b22ad4fdc667aded741704ec5c0f80e385b1d330855a32182f5ad038b3a977ed7a6e971e2400384dc084902c1cbee42c6942a0b49f52198343ee7e6fa0236bc580fd103c0af2395df5a54d0daf5e7d5526f356fd6b5f022a1a48003e2e125a57bc40e7feac40b00c561901b154708ce56233eb21ff4a8ac4845621aff134a17aa019b4d76e7ff5ff74b844db81b18c12f41a36f6ba233635613007e0bb10fe79f7e38)";

    // Strip "Bytes(0x" prefix and ")" suffix
    let hex_str = input
        .strip_prefix("Bytes(0x")
        .and_then(|s| s.strip_suffix(")"))
        .unwrap();

    let bytes = Bytes::from_str(hex_str).unwrap();

    // Skip first 4 bytes (function selector)
    let (selector, bytes) = bytes.split_at(4);

    // Print function selector for verification
    println!("Function selector: 0x{}", hex::encode(selector));

    // Check each 32-byte word
    for (i, chunk) in bytes.chunks(32).enumerate() {
        let hex_word = hex::encode(chunk);
        println!(
            "Word {}: length = {}, value = {}",
            i,
            hex_word.len(),
            hex_word
        );
        assert_eq!(hex_word.len(), 64, "Word {} is not 64 characters", i);
    }
}
