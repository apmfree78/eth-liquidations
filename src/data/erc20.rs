use super::address::CONTRACT;
use super::token_data_hash::get_token_data;
use super::token_price_hash::get_saved_token_price;
use super::tokens_by_chain::MAINNET_TOKENS;
use crate::abi::aave_oracle::AAVE_ORACLE;
use alloy_primitives;
use async_trait::async_trait;
use bigdecimal::{BigDecimal, FromPrimitive};
use ethers::abi::Address;
use ethers::core::types::U256;
use ethers::providers::{Provider, Ws};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use uniswap_sdk_core::entities::token::{Token, TokenMeta};
use uniswap_v3_sdk::{
    constants::FeeAmount,
    extensions::{fraction_to_big_decimal, get_pool},
};

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
}

#[async_trait]
pub trait Convert {
    async fn get_token(&self, chain_id: u64) -> Result<Token, Box<dyn std::error::Error>>;

    async fn get_token_price_in_(
        &self,
        base_token: &str,
        client: &Arc<Provider<Ws>>,
    ) -> Result<BigDecimal, Box<dyn std::error::Error>>;

    async fn get_token_oracle_price(
        &self,
        client: &Arc<Provider<Ws>>,
    ) -> Result<BigDecimal, Box<dyn std::error::Error>>;

    async fn get_saved_price_from_token_price_hash(
        &self,
    ) -> Result<BigDecimal, Box<dyn std::error::Error>>;
}

#[async_trait]
impl Convert for Erc20Token {
    async fn get_token(&self, chain_id: u64) -> Result<Token, Box<dyn std::error::Error>> {
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

    async fn get_token_price_in_(
        &self,
        base_token_symbol: &str,
        client: &Arc<Provider<Ws>>,
    ) -> Result<BigDecimal, Box<dyn std::error::Error>> {
        let token_data = get_token_data().await?;

        if self.symbol == base_token_symbol || self.symbol == "USDC" || self.symbol == "USDe" {
            return Ok(BigDecimal::from(1));
        } else if (self.symbol == "sDAI"
            || self.symbol == "MKR"
            || self.symbol == "crvUSD"
            || self.symbol == "sUSDe"
            || self.symbol == "ETHx"
            || self.symbol == "KNC"
            || self.symbol == "osETH")
            && base_token_symbol == "USDC"
        {
            let price = self.get_token_oracle_price(client).await?;
            return Ok(price);
        } else if (self.symbol == "cbETH"
            || self.symbol == "weETH"
            || self.symbol == "wstETH"
            || self.symbol == "AAVE"
            || self.symbol == "1INCH"
            || self.symbol == "rETH"
            || self.symbol == "ENS"
            || self.symbol == "SNX"
            || self.symbol == "STG"
            || self.symbol == "CRV"
            || self.symbol == "FXS"
            || self.symbol == "LDO"
            || self.symbol == "LUSD"
            || self.symbol == "RPL"
            || self.symbol == "BAL")
            && base_token_symbol == "USDC"
        {
            // 1. get price token price in WETH
            let token_price_in_weth = self.get_token_price_in_("WETH", client).await?;

            // 2. get ETH price in USDC
            let weth_token: &Erc20Token = token_data.get("WETH").unwrap();
            let weth_price_in_usdc = weth_token.get_token_price_in_("USDC", client).await?;

            // determine token price in USDC by multipying
            let token_usdc_price = &token_price_in_weth * &weth_price_in_usdc;

            return Ok(token_usdc_price);
        }

        let base_token: &Erc20Token = token_data.get(base_token_symbol).unwrap();
        let base_token = base_token.get_token(1).await?;

        let token = self.get_token(1).await?;

        let scale_factor: i8 = token.decimals as i8 - base_token.decimals as i8;

        let decimal_factor =
            BigDecimal::from_u64(10_u64.pow(scale_factor.unsigned_abs() as u32)).unwrap();

        let factory_address: alloy_primitives::Address =
            CONTRACT.get_address().uniswap_factory.parse()?;
        let pool = get_pool(
            1,
            factory_address,
            token.meta.address,
            base_token.meta.address,
            FeeAmount::MEDIUM,
            client.clone(),
            None,
        )
        .await?;
        let token0_symbol = pool.token0.symbol.as_ref().unwrap();

        let token_price_in_base_token = if token0_symbol == base_token_symbol {
            pool.token1_price().clone()
        } else {
            pool.token0_price().clone()
        };

        let token_price_in_base_token = fraction_to_big_decimal(&token_price_in_base_token);
        let mut token_price_in_base_token =
            convert_uniswap_to_bigdecimal(token_price_in_base_token);

        if scale_factor > 0 {
            token_price_in_base_token = &token_price_in_base_token * &decimal_factor;
        } else {
            token_price_in_base_token = &token_price_in_base_token / &decimal_factor;
        }

        Ok(token_price_in_base_token)
    }

    async fn get_token_oracle_price(
        &self,
        client: &Arc<Provider<Ws>>,
    ) -> Result<BigDecimal, Box<dyn std::error::Error>> {
        let address: Address = self.address.parse()?;
        let aave_oracle_address: Address = CONTRACT.get_address().aave_oracle.parse()?;
        let aave_oracle = AAVE_ORACLE::new(aave_oracle_address, client.clone());
        let oracle_decimal_factor = BigDecimal::from_u64(10_u64.pow(8)).unwrap();

        let token_price_oracle = aave_oracle.get_asset_price(address).call().await?;
        let token_price_oracle = u256_to_big_decimal(&token_price_oracle);
        let token_price_oracle = token_price_oracle / oracle_decimal_factor;

        Ok(token_price_oracle)
    }

    async fn get_saved_price_from_token_price_hash(
        &self,
    ) -> Result<BigDecimal, Box<dyn std::error::Error>> {
        let token_price = get_saved_token_price(self.address.to_lowercase()).await?;

        Ok(token_price)
    }
}

fn convert_uniswap_to_bigdecimal(
    uniswap_bd: uniswap_sdk_core::prelude::BigDecimal,
) -> bigdecimal::BigDecimal {
    let as_string = uniswap_bd.to_string(); // Convert Uniswap BigDecimal to String
    bigdecimal::BigDecimal::from_str(&as_string).expect("Invalid BigDecimal format")
    // Convert String to bigdecimal BigDecimal
}
//
// pub static token_data: Lazy<HashMap<String, Erc20TokenStatic>> = Lazy::new(|| {
//     let mut token_hash = HashMap::new();
//
//     // create hashmap with token symbol as index
//     for token in MAINNET_TOKENS {
//         token_hash.insert(token.symbol.to_string(), token.clone());
//     }
//
//     // copy hashmap with token address as index
//     for token in MAINNET_TOKENS {
//         token_hash.insert(token.address.to_lowercase(), token.clone());
//     }
//
//     // copy hashmap with chain link price feed as index
//     for token in MAINNET_TOKENS {
//         if !token.chainlink_aggregator.is_empty() {
//             token_hash.insert(token.chainlink_aggregator.to_lowercase(), token.clone());
//         }
//     }
//
//     token_hash
// });

// pub static UNIQUE_token_data: Lazy<HashMap<String, Erc20Token>> = Lazy::new(|| {
//     let unique_tokens = token_data
//         .values()
//         .map(|token| (token.address.to_lowercase(), token.clone()))
//         .collect();
//     unique_tokens
// });
//
// pub static TOKENS_WITH_PRICE_CONNECTED_TO_ETH: Lazy<Vec<&Erc20Token>> = Lazy::new(|| {
//     let token_symbols_connected_to_eth = vec![
//         "WETH", "wstETH", "osETH", "WBTC", "rETH", "cbETH", "LDO", "weETH", "ETHx",
//     ];
//
//     let mut tokens_with_price_connected_to_eth = Vec::<&Erc20Token>::new();
//
//     for token_symbol in token_symbols_connected_to_eth {
//         let token = token_data
//             .get(token_symbol)
//             .unwrap_or_else(|| panic!("invalid token symbol"));
//
//         tokens_with_price_connected_to_eth.push(token);
//     }
//     tokens_with_price_connected_to_eth
// });
//

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
