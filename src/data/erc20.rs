use super::address::CONTRACT;
use super::token_price_hash::get_saved_token_price;
use crate::abi::aave_oracle::AAVE_ORACLE;
use crate::abi::chainlink_aggregator::CHAINLINK_AGGREGATOR;
use crate::utils::type_conversion::{i256_to_f64, u256_to_f64};
use anyhow::Result;
use async_trait::async_trait;
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use ethers::abi::Address;
use ethers::core::types::U256;
use ethers::providers::{Provider, Ws};
use std::str::FromStr;
use std::sync::Arc;
use uniswap_sdk_core::entities::token::{Token, TokenMeta};

#[derive(Clone, Default, Debug)]
pub struct Erc20Token {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub address: String,
    pub liquidation_bonus: u16,
    pub liquidation_threshold: u16,
    pub chain_link_price_feed: String,
    pub chainlink_aggregator: String,
    pub stable_borrow_rate: f64,
    pub variable_borrow_rate: f64,
    pub liquidity_rate: f64,
}

#[derive(Clone, Debug)]
pub struct Erc20TokenStatic {
    pub name: &'static str,
    pub symbol: &'static str,
    pub decimals: u8,
    pub address: &'static str,
    pub liquidation_bonus: u16,
    pub liquidation_threshold: u16,
    pub chain_link_price_feed: &'static str,
    pub chainlink_aggregator: &'static str,
    pub stable_borrow_rate: f64,
    pub variable_borrow_rate: f64,
    pub liquidity_rate: f64,
}

#[async_trait]
pub trait Convert {
    async fn get_token(&self, chain_id: u64) -> Result<Token>;
    async fn get_token_oracle_price(&self, client: &Arc<Provider<Ws>>) -> Result<f64>;
    async fn get_saved_price_from_token_price_hash(&self) -> Result<f64>;
}

#[async_trait]
impl Convert for Erc20Token {
    async fn get_token(&self, chain_id: u64) -> Result<Token> {
        return Ok(Token {
            chain_id,
            decimals: self.decimals,
            symbol: Some(self.symbol.clone()),
            name: Some(self.name.clone()),
            meta: TokenMeta {
                address: self.address.parse()?,
                buy_fee_bps: None,
                sell_fee_bps: None,
            },
        });
    }

    async fn get_token_oracle_price(&self, client: &Arc<Provider<Ws>>) -> Result<f64> {
        let oracle_decimal_factor = 10_u64.pow(8) as f64;

        if self.symbol == "BTC" {
            let feed_address: Address = self.chain_link_price_feed.parse()?;
            let chainlink_btc_oracle = CHAINLINK_AGGREGATOR::new(feed_address, client.clone());

            let btc_price_oracle = chainlink_btc_oracle.latest_answer().call().await?;
            let btc_price_oracle = i256_to_f64(btc_price_oracle).unwrap() / oracle_decimal_factor;

            return Ok(btc_price_oracle);
        }

        let address: Address = self.address.parse()?;
        let aave_oracle_address: Address = CONTRACT.get_address().aave_oracle.parse()?;
        let aave_oracle = AAVE_ORACLE::new(aave_oracle_address, client.clone());

        let token_price_oracle = aave_oracle.get_asset_price(address).call().await?;
        let token_price_oracle = u256_to_f64(token_price_oracle).unwrap() / oracle_decimal_factor;

        Ok(token_price_oracle)
    }

    async fn get_saved_price_from_token_price_hash(&self) -> Result<f64> {
        let token_price = get_saved_token_price(self.address.to_lowercase()).await?;

        Ok(token_price)
    }
}

pub async fn generate_token(
    chain_id: u64,
    decimals: u8,
    symbol: &str,
    name: &str,
    address: &str,
) -> Result<Token, Box<dyn std::error::Error>> {
    Ok(Token {
        chain_id,
        decimals,
        symbol: Some(symbol.into()),
        name: Some(name.into()),
        meta: TokenMeta {
            address: address.parse()?,
            buy_fee_bps: None,
            sell_fee_bps: None,
        },
    })
}

pub fn u256_to_big_decimal(value: &U256) -> BigDecimal {
    BigDecimal::from_str(&value.to_string()).unwrap()
}
