use crate::data::token_data_hash::get_token_data;

use super::erc20::{Convert, Erc20Token};
use super::token_data_hash::get_unique_token_data;
use anyhow::Result;
use ethers::providers::{Provider, Ws};
use futures::lock::Mutex;
use log::debug;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;

pub static TOKEN_PRICE_HASH: Lazy<Arc<Mutex<HashMap<String, f64>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::<String, f64>::new())));

pub async fn generate_token_price_hash(client: &Arc<Provider<Ws>>) -> Result<()> {
    let token_price_hash = Arc::clone(&TOKEN_PRICE_HASH);
    let mut token_prices = token_price_hash.lock().await;
    let unique_token_data = get_unique_token_data().await?;

    for token in unique_token_data.values() {
        match token.get_token_oracle_price(client).await {
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
pub async fn add_price_for_new_token(token: &Erc20Token, client: &Arc<Provider<Ws>>) -> Result<()> {
    let token_price_hash = Arc::clone(&TOKEN_PRICE_HASH);
    let mut token_prices = token_price_hash.lock().await;
    // let unique_token_data = get_unique_token_data().await?;

    if token_prices.contains_key(&token.address) {
        return Ok(());
    }

    debug!("adding price for token {}", token.symbol);
    match token.get_token_oracle_price(client).await {
        Ok(token_price) => {
            token_prices.insert(token.address.to_lowercase(), token_price);
        }
        Err(e) => {
            panic!("Failed to fetch price for token {}: {}", token.address, e);
            // Optionally continue to the next token or handle error differently
        }
    }

    Ok(())
}

pub async fn get_saved_token_price(token_address: &str) -> Result<f64> {
    let token_price_hash = Arc::clone(&TOKEN_PRICE_HASH);
    let token_prices = token_price_hash.lock().await;

    let token_price = token_prices
        .get(token_address)
        .unwrap_or_else(|| panic!("token {} not found in token price hash", token_address));

    Ok(*token_price)
}

pub async fn set_saved_token_price(token_address: &str, new_token_price: f64) -> Result<()> {
    let token_price_hash = Arc::clone(&TOKEN_PRICE_HASH);
    let mut token_prices = token_price_hash.lock().await;

    token_prices.insert(token_address.to_lowercase(), new_token_price);
    Ok(())
}

pub async fn print_saved_token_prices() -> Result<()> {
    let token_price_hash = Arc::clone(&TOKEN_PRICE_HASH);
    let token_prices = token_price_hash.lock().await;
    let token_data = get_token_data().await?;

    for (token_address, price) in token_prices.iter() {
        let token = token_data.get(token_address).unwrap();
        debug!("{} price => {:?}", token.symbol, price);
    }

    Ok(())
}
