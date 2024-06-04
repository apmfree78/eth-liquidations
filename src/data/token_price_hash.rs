use super::erc20::{Convert, TOKEN_DATA};
use bigdecimal::BigDecimal;
use ethers::providers::{Provider, Ws};
use futures::lock::Mutex;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;

const WS_URL: &str = "ws://localhost:8546";
pub static TOKEN_PRICE_HASH: Lazy<Arc<Mutex<HashMap<String, BigDecimal>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::<String, BigDecimal>::new())));

pub async fn generate_token_price_hash() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    let token_price_hash = Arc::clone(&TOKEN_PRICE_HASH);
    let mut token_prices = token_price_hash.lock().await;

    for token in TOKEN_DATA.values() {
        let token_price = token.get_token_oracle_price(&client).await?;
        token_prices.insert(token.address.to_lowercase(), token_price);
    }

    Ok(())
}
