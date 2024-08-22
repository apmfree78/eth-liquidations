use super::chainlink_feed_map::{
    get_chainlink_aggregator, get_chainlink_price_feed_for_token_, CHAINLINK_AGGREGATOR_HASH,
};
use super::erc20::Erc20Token;
use super::tokens_by_chain::MAINNET_TOKENS;
use crate::abi::aave_v3_data_provider::AAVE_V3_DATA_PROVIDER;
use crate::abi::erc20::ERC20;
use crate::data::address::CONTRACT;
use ethers::providers::{Provider, Ws};
use ethers::types::{Address, U256};
use futures::lock::Mutex;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;

static TOKEN_DATA_HASH: Lazy<Arc<Mutex<HashMap<String, Erc20Token>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::<String, Erc20Token>::new())));

static UNIQUE_TOKEN_DATA_HASH: Lazy<Arc<Mutex<HashMap<String, Erc20Token>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::<String, Erc20Token>::new())));

// TODO - Fix bug where WETH and other eth tokens do not have chain link feeds
pub async fn save_erc20_token(
    token: &Erc20Token,
    client: &Arc<Provider<Ws>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let token_data_hash = Arc::clone(&TOKEN_DATA_HASH);
    let mut tokens = token_data_hash.lock().await;
    let chainlink_aggregator_hash = Arc::clone(&CHAINLINK_AGGREGATOR_HASH);
    let mut aggregators = chainlink_aggregator_hash.lock().await;
    let unique_data_hash = Arc::clone(&UNIQUE_TOKEN_DATA_HASH);
    let mut unique_tokens = unique_data_hash.lock().await;

    // make sure token is not already in hashmap
    if tokens.contains_key(&token.address) {
        return Ok(());
    }

    let chainlink_price_feed = get_chainlink_price_feed_for_token_(&token.symbol, token).await;
    let aggregator_address = if !chainlink_price_feed.is_empty() {
        get_chainlink_aggregator(&chainlink_price_feed, client).await?
    } else {
        "".to_string()
    };

    let updated_token = Erc20Token {
        chain_link_price_feed: chainlink_price_feed.to_string(),
        chainlink_aggregator: aggregator_address.clone(),
        ..token.clone()
    };

    tokens.insert(token.symbol.clone(), updated_token.clone());
    tokens.insert(token.address.to_lowercase(), updated_token.clone());
    unique_tokens.insert(token.address.to_lowercase(), updated_token.clone());

    if !aggregator_address.is_empty() {
        aggregators.insert(aggregator_address, updated_token);
    }

    Ok(())
}

pub async fn save_erc20_tokens_from_static_data(
    client: &Arc<Provider<Ws>>,
) -> Result<(), Box<dyn std::error::Error>> {
    for static_token in MAINNET_TOKENS {
        let token = Erc20Token {
            name: static_token.name.to_string(),
            symbol: static_token.symbol.to_string(),
            decimals: static_token.decimals,
            address: static_token.address.to_string(),
            liquidation_bonus: static_token.liquidation_bonus,
            liquidation_threshold: static_token.liquidation_threshold,
            ..Default::default()
        };

        save_erc20_token(&token, client).await?;
    }

    Ok(())
}

pub async fn get_and_save_erc20_by_token_address(
    token_address_str: &str,
    client: &Arc<Provider<Ws>>,
) -> Result<Erc20Token, Box<dyn std::error::Error>> {
    let token_data_hash = Arc::clone(&TOKEN_DATA_HASH);
    let mut tokens = token_data_hash.lock().await;
    let chainlink_aggregator_hash = Arc::clone(&CHAINLINK_AGGREGATOR_HASH);
    let mut aggregators = chainlink_aggregator_hash.lock().await;
    let unique_data_hash = Arc::clone(&UNIQUE_TOKEN_DATA_HASH);
    let mut unique_tokens = unique_data_hash.lock().await;

    // make sure token is not already in hashmap
    if tokens.contains_key(token_address_str) {
        let token = tokens.get(token_address_str).unwrap();
        return Ok(token.clone());
    }

    let token_address: Address = token_address_str.parse()?;
    let token_contract = ERC20::new(token_address, client.clone());
    let data_provider_address: Address = CONTRACT.get_address().aave_v3_data_provider.parse()?;
    let aave_v3_data_provider = AAVE_V3_DATA_PROVIDER::new(data_provider_address, client.clone());

    // get basic toke data
    let symbol = token_contract.symbol().call().await?;
    let decimals = token_contract.decimals().call().await?;
    let name = token_contract.name().call().await?;

    // get aave data
    let (_, _, liquidation_threshold, liquidation_bonus, _, _, _, _, _, _) = aave_v3_data_provider
        .get_reserve_configuration_data(token_address)
        .call()
        .await?;

    // convert u256 to u16 !
    let liquidation_bonus = u16::try_from(U256::low_u64(&liquidation_bonus))?;
    let liquidation_threshold = u16::try_from(U256::low_u64(&liquidation_threshold))?;

    let mut token = Erc20Token {
        name,
        symbol,
        decimals,
        address: token_address_str.to_string(),
        liquidation_bonus,
        liquidation_threshold,
        ..Default::default()
    };

    let chainlink_price_feed = get_chainlink_price_feed_for_token_(&token.symbol, &token).await;
    let aggregator_address = if !chainlink_price_feed.is_empty() {
        get_chainlink_aggregator(&chainlink_price_feed, client).await?
    } else {
        "".to_string()
    };

    token = Erc20Token {
        chain_link_price_feed: chainlink_price_feed.to_string(),
        chainlink_aggregator: aggregator_address.clone(),
        ..token.clone()
    };

    tokens.insert(token.symbol.clone(), token.clone());
    tokens.insert(token.address.clone(), token.clone());
    unique_tokens.insert(token.address.clone(), token.clone());

    if !aggregator_address.is_empty() {
        aggregators.insert(aggregator_address, token.clone());
    }

    Ok(token)
}

pub async fn get_token_data() -> Result<HashMap<String, Erc20Token>, Box<dyn std::error::Error>> {
    let token_data_hash = Arc::clone(&TOKEN_DATA_HASH);
    let tokens = token_data_hash.lock().await;

    Ok(tokens.clone())
}

pub async fn get_unique_token_data(
) -> Result<HashMap<String, Erc20Token>, Box<dyn std::error::Error>> {
    let token_data_hash = Arc::clone(&UNIQUE_TOKEN_DATA_HASH);
    let tokens = token_data_hash.lock().await;

    Ok(tokens.clone())
}

// TODO - CREATE METHOD TO UPDATE AGGREGATORS

pub static TOKENS_WITH_PRICE_CONNECTED_TO_ETH: Lazy<Arc<Mutex<HashMap<String, Erc20Token>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::<String, Erc20Token>::new())));

pub async fn set_token_connected_to_eth(token_symbol: String, token: &Erc20Token) {
    let tokens_connected_to_eth_hash = Arc::clone(&TOKENS_WITH_PRICE_CONNECTED_TO_ETH);
    let mut tokens = tokens_connected_to_eth_hash.lock().await;

    tokens.entry(token_symbol).or_insert(token.clone());
}

pub async fn get_token_connected_to_eth(
) -> Result<HashMap<String, Erc20Token>, Box<dyn std::error::Error>> {
    let tokens_connected_to_eth_hash = Arc::clone(&TOKENS_WITH_PRICE_CONNECTED_TO_ETH);
    let tokens = tokens_connected_to_eth_hash.lock().await;

    Ok(tokens.clone())
}
