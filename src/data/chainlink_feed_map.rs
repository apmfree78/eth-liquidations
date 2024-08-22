use crate::utils::type_conversion::address_to_string;

use super::address::CHAIN;
use super::chainlink_data::{
    ChainlinkPriceFeed, ARBITRUM_PRICE_FEEDS, BNB_PRICE_FEEDS, ETHEREUM_PRICE_FEEDS,
    OPTIMISM_PRICE_FEEDS, POLYGON_PRICE_FEEDS, ZKSYNC_PRICE_FEEDS,
};
use super::erc20::Erc20Token;
use super::token_data_hash::set_token_connected_to_eth;
use ethers::contract::abigen;
use ethers::providers::{Provider, Ws};
use ethers::types::{Address, Chain};
use futures::lock::Mutex;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;

static STABLE_COINS: &[&str] = &[
    "USDC", "USDE", "CRVUSD", "SUSDE", "SDAI", "DAI", "USDT", "PYUSD", "USDD", "FRAX", "TUSD",
    "USDB", "GHO", "GUSD", "USDX", "LUSD", "BUSD", "MIM", "USDM", "CUSD",
];

pub static CHAINLINK_AGGREGATOR_HASH: Lazy<Arc<Mutex<HashMap<String, Erc20Token>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::<String, Erc20Token>::new())));

static CHAINLINK_FEED_MAP: Lazy<HashMap<String, ChainlinkPriceFeed>> = Lazy::new(|| {
    let mut price_feed_hash = HashMap::new();

    let chainlink_price_feeds = get_chainlink_price_feeds_by_chain();

    // create hashmap with token symbol as index
    for feed in chainlink_price_feeds {
        if feed.base_currency == "USD" {
            if feed.token_symbol == "ETH" {
                price_feed_hash.insert("weth/USD".to_string(), feed);
            } else {
                price_feed_hash.insert(feed.token_symbol.to_lowercase() + "/USD", feed);
            }
        } else if feed.base_currency == "ETH" {
            price_feed_hash.insert(feed.token_symbol.to_lowercase() + "/ETH", feed);
        }
    }

    price_feed_hash
});

pub async fn get_chainlink_aggregator(
    price_feed: &str,
    client: &Arc<Provider<Ws>>,
) -> Result<String, Box<dyn std::error::Error>> {
    abigen!(
        AGGREGATOR,
        r#"[function aggregator() external view returns (address)]"#
    );

    let price_feed_address: Address = price_feed.parse()?;

    let aggregator_contract = AGGREGATOR::new(price_feed_address, client.clone());
    let aggregator_address = aggregator_contract.aggregator().call().await?;
    let aggregator_address = address_to_string(aggregator_address);

    Ok(aggregator_address)
}

pub async fn get_chainlink_price_feed_for_token_(token_symbol: &str, token: &Erc20Token) -> String {
    let usd_feed_symbol = token_symbol.to_lowercase() + "/USD";
    let eth_feed_symbol = token_symbol.to_lowercase() + "/ETH";

    // check for BTC TOKEN
    if token_symbol.to_string().contains("BTC") {
        return CHAINLINK_FEED_MAP
            .get("btc/USD")
            .unwrap()
            .address
            .to_string();
    }

    // check for stable coin
    if STABLE_COINS.contains(&token_symbol.to_uppercase().as_str()) {
        return "".to_string();
    }

    // check if chainlink price feed exists for USD or ETH base pair
    // and return , if neither exists then just retrun ""
    if CHAINLINK_FEED_MAP.contains_key(&usd_feed_symbol) {
        CHAINLINK_FEED_MAP
            .get(&usd_feed_symbol)
            .unwrap()
            .address
            .to_string()
    } else if CHAINLINK_FEED_MAP.contains_key(&eth_feed_symbol) {
        set_token_connected_to_eth(token_symbol.to_string(), token).await;
        CHAINLINK_FEED_MAP
            .get(&eth_feed_symbol)
            .unwrap()
            .address
            .to_string()
    } else {
        if token_symbol.to_lowercase().contains("eth") {
            set_token_connected_to_eth(token_symbol.to_string(), token).await;
        }
        "".to_string()
    }
}

pub async fn get_chainlink_aggregator_map(
) -> Result<HashMap<String, Erc20Token>, Box<dyn std::error::Error>> {
    let chainlink_aggregator_hash = Arc::clone(&CHAINLINK_AGGREGATOR_HASH);
    let aggregators = chainlink_aggregator_hash.lock().await;

    Ok(aggregators.clone())
}

fn get_chainlink_price_feeds_by_chain() -> Vec<ChainlinkPriceFeed> {
    let price_feed = match CHAIN {
        Chain::Mainnet => ETHEREUM_PRICE_FEEDS,
        Chain::Polygon => POLYGON_PRICE_FEEDS,
        Chain::Arbitrum => ARBITRUM_PRICE_FEEDS,
        Chain::Optimism => OPTIMISM_PRICE_FEEDS,
        Chain::BinanceSmartChain => BNB_PRICE_FEEDS,
        Chain::ZkSync => ZKSYNC_PRICE_FEEDS,
        _ => ETHEREUM_PRICE_FEEDS,
    };
    price_feed.to_vec()
}
