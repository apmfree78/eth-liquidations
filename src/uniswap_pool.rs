// let pool_address = get_pool_address(&token_1, &token_2).await?;
// println!("Calculated Pool Address: {:?}", pool_address);

// let pool_contract = get_pool_contract(
//     FACTORY_ADDRESS,
//     token_1.meta.address,
//     token_2.meta.address,
//     FeeAmount::MEDIUM,
//     client.clone(),
// );

// println!("Pool Contract: {:?}", pool_contract);
pub async fn get_pool_address(
    token_a: &Token,
    token_b: &Token,
) -> Result<Address, Box<dyn std::error::Error>> {
    // Fee Tier
    let fee = FeeAmount::MEDIUM; // Assuming this matches the Uniswap SDK's representation

    // Calculate Pool Address
    let pool_address = pool::get_address(
        token_a, token_b, fee, None, // init_code_hash_manual_override
        None, // factory_address_override
    );

    Ok(pool_address)
}
