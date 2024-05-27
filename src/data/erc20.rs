use super::address::AAVE_ORACLE_ADDRESS;
use crate::abi::aave_oracle::AAVE_ORACLE;
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
    constants::{FeeAmount, FACTORY_ADDRESS},
    extensions::{fraction_to_big_decimal, get_pool},
};

#[derive(Clone, Copy, Debug)]
pub struct Erc20Token {
    pub name: &'static str,
    pub symbol: &'static str,
    pub decimals: u8,
    pub address: &'static str,
    pub liquidation_bonus: u16,
    pub liquidation_threshold: u16,
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
}

#[async_trait]
impl Convert for Erc20Token {
    async fn get_token(&self, chain_id: u64) -> Result<Token, Box<dyn std::error::Error>> {
        return Ok(Token {
            chain_id,
            decimals: self.decimals,
            symbol: Some(self.symbol.into()),
            name: Some(self.name.into()),
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
        if self.symbol == base_token_symbol {
            return Ok(BigDecimal::from(1));
        } else if self.symbol == "crvUSD" && base_token_symbol == "USDC" {
            return Ok(BigDecimal::from(1));
        } else if self.symbol == "sDAI" && base_token_symbol == "USDC" {
            let sdai_price = self.get_token_oracle_price(&client).await?;
            return Ok(sdai_price);
        } else if (self.symbol == "cbETH"
            || self.symbol == "ENS"
            || self.symbol == "SNX"
            || self.symbol == "CRV"
            || self.symbol == "AAVE"
            || self.symbol == "1INCH"
            || self.symbol == "FXS"
            || self.symbol == "LDO"
            || self.symbol == "rETH"
            || self.symbol == "LUSD"
            || self.symbol == "RPL"
            || self.symbol == "weETH"
            || self.symbol == "wstETH"
            || self.symbol == "osETH"
            || self.symbol == "BAL")
            && base_token_symbol == "USDC"
        {
            // 1. get price token price in WETH
            let token_price_in_weth = self.get_token_price_in_("WETH", &client).await?;

            // 2. get ETH price in USDC
            let weth_token: &Erc20Token = TOKEN_DATA.get("WETH").unwrap();
            let weth_price_in_usdc = weth_token.get_token_price_in_("USDC", &client).await?;

            // determine token price in USDC by multipying
            let token_usdc_price = &token_price_in_weth * &weth_price_in_usdc;

            return Ok(token_usdc_price);
        }

        let base_token: &Erc20Token = TOKEN_DATA.get(base_token_symbol).unwrap();
        let base_token = base_token.get_token(1).await?;

        let token = self.get_token(1).await?;

        let scale_factor: i8 = token.decimals as i8 - base_token.decimals as i8;

        let decimal_factor = BigDecimal::from_u64(10_u64.pow(scale_factor.abs() as u32)).unwrap();

        let pool = get_pool(
            1,
            FACTORY_ADDRESS,
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
        let aave_oracle = AAVE_ORACLE::new(*AAVE_ORACLE_ADDRESS, client.clone());
        let oracle_decimal_factor = BigDecimal::from_u64(10_u64.pow(8)).unwrap();

        let token_price_oracle = aave_oracle.get_asset_price(address).call().await?;
        let token_price_oracle = u256_to_big_decimal(&token_price_oracle);
        let token_price_oracle = token_price_oracle / oracle_decimal_factor;

        Ok(token_price_oracle)
    }
}

fn convert_uniswap_to_bigdecimal(
    uniswap_bd: uniswap_sdk_core::prelude::BigDecimal,
) -> bigdecimal::BigDecimal {
    let as_string = uniswap_bd.to_string(); // Convert Uniswap BigDecimal to String
    bigdecimal::BigDecimal::from_str(&as_string).expect("Invalid BigDecimal format")
    // Convert String to bigdecimal BigDecimal
}

pub static TOKEN_DATA: Lazy<HashMap<String, Erc20Token>> = Lazy::new(|| {
    let mut tokens = Vec::<Erc20Token>::new();

    tokens.push(Erc20Token {
        name: "Wrapped Ether",
        symbol: "WETH",
        decimals: 18,
        address: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
        liquidation_bonus: 10500,
        liquidation_threshold: 8300,
    });

    tokens.push(Erc20Token {
        name: "Wrapped liquid staked Ether 2.0",
        symbol: "wstETH",
        decimals: 18,
        address: "0x7f39C581F595B53c5cb19bD0b3f8dA6c935E2Ca0",
        liquidation_bonus: 10600,
        liquidation_threshold: 8100,
    });

    tokens.push(Erc20Token {
        name: "Staked ETH",
        symbol: "osETH",
        decimals: 18,
        address: "0xf1C9acDc66974dFB6dEcB12aA385b9cD01190E38",
        liquidation_bonus: 10750,
        liquidation_threshold: 7500,
    });

    tokens.push(Erc20Token {
        name: "USD Coin",
        symbol: "USDC",
        decimals: 6,
        address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
        liquidation_bonus: 10450,
        liquidation_threshold: 7800,
    });

    tokens.push(Erc20Token {
        name: "Dai Stablecoin",
        symbol: "DAI",
        decimals: 18,
        address: "0x6B175474E89094C44Da98b954EedeAC495271d0F",
        liquidation_bonus: 10500,
        liquidation_threshold: 7700,
    });

    tokens.push(Erc20Token {
        name: "ChainLink Token",
        symbol: "LINK",
        decimals: 18,
        address: "0x514910771AF9Ca656af840dff83E8264EcF986CA",
        liquidation_bonus: 10700,
        liquidation_threshold: 6800,
    });

    tokens.push(Erc20Token {
        name: "Tether USD",
        symbol: "USDT",
        decimals: 6,
        address: "0xdac17f958d2ee523a2206206994597c13d831ec7",
        liquidation_bonus: 10450,
        liquidation_threshold: 7800,
    });

    tokens.push(Erc20Token {
        name: "Wrapped BTC",
        symbol: "WBTC",
        decimals: 8,
        address: "0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599",
        liquidation_bonus: 10500,
        liquidation_threshold: 8300,
    });

    tokens.push(Erc20Token {
        name: "Rocket Pool ETH",
        symbol: "rETH",
        decimals: 18,
        address: "0xae78736Cd615f374D3085123A210448E74Fc6393",
        liquidation_bonus: 10750,
        liquidation_threshold: 7700,
    });

    tokens.push(Erc20Token {
        name: "Maker",
        symbol: "MKR",
        decimals: 18,
        address: "0x9f8F72aA9304c8B593d555F12eF6589cC3A579A2",
        liquidation_bonus: 10850,
        liquidation_threshold: 7000,
    });

    tokens.push(Erc20Token {
        name: "Coinbase Wrapped Staked ETH",
        symbol: "cbETH",
        decimals: 18,
        address: "0xBe9895146f7AF43049ca1c1AE358B0541Ea49704",
        liquidation_bonus: 10750,
        liquidation_threshold: 7700,
    });

    tokens.push(Erc20Token {
        name: "PayPal USD",
        symbol: "PYUSD",
        decimals: 6,
        address: "0x6c3ea9036406852006290770BEdFcAbA0e23A0e8",
        liquidation_bonus: 0,
        liquidation_threshold: 0,
    });

    tokens.push(Erc20Token {
        name: "Lido DAO Token",
        symbol: "LDO",
        decimals: 18,
        address: "0x5A98FcBEA516Cf06857215779Fd812CA3beF1B32",
        liquidation_bonus: 10900,
        liquidation_threshold: 5000,
    });

    tokens.push(Erc20Token {
        name: "Uniswap",
        symbol: "UNI",
        decimals: 18,
        address: "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984",
        liquidation_bonus: 11000,
        liquidation_threshold: 7400,
    });

    tokens.push(Erc20Token {
        name: "Synthetix Network Token",
        symbol: "SNX",
        decimals: 18,
        address: "0xC011a73ee8576Fb46F5E1c5751cA3B9Fe0af2a6F",
        liquidation_bonus: 10850,
        liquidation_threshold: 6400,
    });

    tokens.push(Erc20Token {
        name: "1INCH Token",
        symbol: "1INCH",
        decimals: 18,
        address: "0x111111111117dC0aa78b770fA6A738034120C302",
        liquidation_bonus: 10750,
        liquidation_threshold: 6700,
    });

    tokens.push(Erc20Token {
        name: "LUSD Stablecoin",
        symbol: "LUSD",
        decimals: 18,
        address: "0x5f98805A4E8be255a32880FDeC7F6728C6568bA0",
        liquidation_bonus: 10450,
        liquidation_threshold: 7700,
    });

    tokens.push(Erc20Token {
        name: "Ethereum Name Service",
        symbol: "ENS",
        decimals: 18,
        address: "0xC18360217D8F7Ab5e7c516566761Ea12Ce7F9D72",
        liquidation_bonus: 10800,
        liquidation_threshold: 4900,
    });

    tokens.push(Erc20Token {
        name: "Balancer",
        symbol: "BAL",
        decimals: 18,
        address: "0xba100000625a3754423978a60c9317c58a424e3D",
        liquidation_bonus: 10830,
        liquidation_threshold: 5900,
    });

    tokens.push(Erc20Token {
        name: "Curve DAO Token",
        symbol: "CRV",
        decimals: 18,
        address: "0xD533a949740bb3306d119CC777fa900bA034cd52",
        liquidation_bonus: 10830,
        liquidation_threshold: 4100,
    });

    tokens.push(Erc20Token {
        name: "Frax",
        symbol: "FRAX",
        decimals: 18,
        address: "0x853d955aCEf822Db058eb8505911ED77F175b99e",
        liquidation_bonus: 10600,
        liquidation_threshold: 7200,
    });

    tokens.push(Erc20Token {
        name: "Curve.Fi USD Stablecoin",
        symbol: "crvUSD",
        decimals: 18,
        address: "0xf939E0A03FB07F59A73314E73794Be0E57ac1b4E",
        liquidation_bonus: 0,
        liquidation_threshold: 0,
    });

    tokens.push(Erc20Token {
        name: "Gho Token",
        symbol: "GHO",
        decimals: 18,
        address: "0x40D16FC0246aD3160Ccc09B8D0D3A2cD28aE6C2f",
        liquidation_bonus: 0,
        liquidation_threshold: 0,
    });

    tokens.push(Erc20Token {
        name: "Aave Token",
        symbol: "AAVE",
        decimals: 18,
        address: "0x7Fc66500c84A76Ad7e9c93437bFc5Ac33E2DDaE9",
        liquidation_bonus: 10750,
        liquidation_threshold: 7300,
    });

    tokens.push(Erc20Token {
        name: "Savings Dai",
        symbol: "sDAI",
        decimals: 18,
        address: "0x83F20F44975D03b1b09e64809B757c47f942BEeA",
        liquidation_bonus: 10450,
        liquidation_threshold: 7800,
    });

    tokens.push(Erc20Token {
        name: "Wrapped eETH",
        symbol: "weETH",
        decimals: 18,
        address: "0xCd5fE23C85820F7B72D0926FC9b05b43E359b7ee",
        liquidation_bonus: 10750,
        liquidation_threshold: 7500,
    });

    tokens.push(Erc20Token {
        name: "Rocket Pool Protocol",
        symbol: "RPL",
        decimals: 18,
        address: "0xD33526068D116cE69F19A9ee46F0bd304F21A51f",
        liquidation_bonus: 0,
        liquidation_threshold: 0,
    });

    tokens.push(Erc20Token {
        name: "Frax Share",
        symbol: "FXS",
        decimals: 18,
        address: "0x3432B6A60D23Ca0dFCa7761B7ab56459D9C964D0",
        liquidation_bonus: 11000,
        liquidation_threshold: 4200,
    });

    tokens.push(Erc20Token {
        name: "StargateToken",
        symbol: "STG",
        decimals: 18,
        address: "0xAf5191B0De278C7286d6C7CC6ab6BB8A73bA2Cd6",
        liquidation_bonus: 11000,
        liquidation_threshold: 3700,
    });

    tokens.push(Erc20Token {
        name: "Kyber Network Crystal v2",
        symbol: "KNC",
        decimals: 18,
        address: "0xdeFA4e8a7bcBA345F687a2f1456F5Edd9CE97202",
        liquidation_bonus: 11000,
        liquidation_threshold: 3700,
    });

    let mut token_hash = HashMap::new();

    // create hashmap with token symbol as index
    for token in &tokens {
        token_hash.insert(token.symbol.to_string(), *token);
    }

    // copy hashmap with token address as index
    for token in &tokens {
        token_hash.insert(token.address.to_string().to_lowercase(), *token);
    }

    token_hash
});

pub async fn generate_token(
    chain_id: u64,
    decimals: u8,
    symbol: &str,
    name: &str,
    address: &str,
) -> Result<Token, Box<dyn std::error::Error>> {
    return Ok(Token {
        chain_id,
        decimals,
        symbol: Some(symbol.into()),
        name: Some(name.into()),
        meta: TokenMeta {
            address: address.parse()?,
            buy_fee_bps: None,
            sell_fee_bps: None,
        },
    });
}

pub fn u256_to_big_decimal(value: &U256) -> BigDecimal {
    BigDecimal::from_str(&value.to_string()).unwrap()
}

pub fn address_to_string(address: Address) -> String {
    format!("{:?}", address)
}
