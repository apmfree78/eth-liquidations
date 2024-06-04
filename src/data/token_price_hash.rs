use super::erc20::{Convert, Erc20Token, TOKEN_DATA};
use bigdecimal::BigDecimal;
use ethers::providers::{Provider, Ws};
use futures::lock::Mutex;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;

const WS_URL: &str = "ws://localhost:8546";
pub static TOKEN_PRICE_HASH: Lazy<Arc<Mutex<HashMap<String, BigDecimal>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::<String, BigDecimal>::new())));

pub async fn generate_token_price_hash(
    client: &Arc<Provider<Ws>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let token_price_hash = Arc::clone(&TOKEN_PRICE_HASH);
    let mut token_prices = token_price_hash.lock().await;

    for token in TOKEN_DATA.values() {
        match token.get_token_oracle_price(&client).await {
            Ok(token_price) => {
                token_prices.insert(token.address.to_lowercase(), token_price);
            }
            Err(e) => {
                panic!("Failed to fetch price for token {}: {}", token.address, e);
                // Optionally continue to the next token or handle error differently
            }
        }
    }

    Ok(())
}

pub async fn get_saved_token_price(
    token_address: String,
) -> Result<BigDecimal, Box<dyn std::error::Error>> {
    let token_price_hash = Arc::clone(&TOKEN_PRICE_HASH);
    let token_prices = token_price_hash.lock().await;

    let token_price = token_prices
        .get(&token_address)
        .unwrap_or_else(|| panic!("token not found in token price hash"));

    Ok(token_price.clone())
}

pub async fn set_saved_token_price(
    token_address: &str,
    new_token_price: BigDecimal,
) -> Result<(), Box<dyn std::error::Error>> {
    let token_price_hash = Arc::clone(&TOKEN_PRICE_HASH);
    let mut token_prices = token_price_hash.lock().await;

    token_prices.insert(token_address.to_lowercase(), new_token_price);
    Ok(())
}
