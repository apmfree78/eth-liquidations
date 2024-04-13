use once_cell::sync::Lazy;
use std::collections::HashMap;
use uniswap_sdk_core::entities::token::{Token, TokenMeta};

#[derive(Clone, Copy, Debug)]
pub struct Erc20Token {
    pub name: &'static str,
    pub symbol: &'static str,
    pub decimals: u8,
    pub address: &'static str,
}

pub trait Convert {
    async fn get_token(&self, chain_id: u64) -> Result<Token, Box<dyn std::error::Error>>;
}

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
}

pub static TOKEN_DATA: Lazy<HashMap<&str, Erc20Token>> = Lazy::new(|| {
    let mut t = HashMap::new();
    t.insert(
        "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
        Erc20Token {
            name: "Wrapped Ether",
            symbol: "WETH",
            decimals: 18,
            address: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
        },
    );

    t.insert(
        "0x7f39C581F595B53c5cb19bD0b3f8dA6c935E2Ca0",
        Erc20Token {
            name: "Wrapped liquid staked Ether 2.0",
            symbol: "wstETH",
            decimals: 18,
            address: "0x7f39C581F595B53c5cb19bD0b3f8dA6c935E2Ca0",
        },
    );

    t.insert(
        "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
        Erc20Token {
            name: "USD Coin",
            symbol: "USDC",
            decimals: 6,
            address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
        },
    );

    t.insert(
        "0x6B175474E89094C44Da98b954EedeAC495271d0F",
        Erc20Token {
            name: "Dai Stablecoin",
            symbol: "DAI",
            decimals: 18,
            address: "0x6B175474E89094C44Da98b954EedeAC495271d0F",
        },
    );

    t.insert(
        "0x514910771AF9Ca656af840dff83E8264EcF986CA",
        Erc20Token {
            name: "ChainLink Token",
            symbol: "LINK",
            decimals: 18,
            address: "0x514910771AF9Ca656af840dff83E8264EcF986CA",
        },
    );

    t.insert(
        "0xdac17f958d2ee523a2206206994597c13d831ec7",
        Erc20Token {
            name: "Tether USD",
            symbol: "USDT",
            decimals: 6,
            address: "0xdac17f958d2ee523a2206206994597c13d831ec7",
        },
    );

    t.insert(
        "0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599",
        Erc20Token {
            name: "Wrapped BTC",
            symbol: "WBTC",
            decimals: 8,
            address: "0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599",
        },
    );

    t.insert(
        "0xae78736Cd615f374D3085123A210448E74Fc6393",
        Erc20Token {
            name: "Rocket Pool ETH",
            symbol: "rETH",
            decimals: 18,
            address: "0xae78736Cd615f374D3085123A210448E74Fc6393",
        },
    );

    t.insert(
        "0x9f8F72aA9304c8B593d555F12eF6589cC3A579A2",
        Erc20Token {
            name: "Maker",
            symbol: "MKR",
            decimals: 18,
            address: "0x9f8F72aA9304c8B593d555F12eF6589cC3A579A2",
        },
    );

    t.insert(
        "0xBe9895146f7AF43049ca1c1AE358B0541Ea49704",
        Erc20Token {
            name: "Coinbase Wrapped Staked ETH",
            symbol: "cbETH",
            decimals: 18,
            address: "0xBe9895146f7AF43049ca1c1AE358B0541Ea49704",
        },
    );

    t.insert(
        "0x6c3ea9036406852006290770BEdFcAbA0e23A0e8",
        Erc20Token {
            name: "PayPal USD",
            symbol: "PYUSD",
            decimals: 6,
            address: "0x6c3ea9036406852006290770BEdFcAbA0e23A0e8",
        },
    );

    t.insert(
        "0x5A98FcBEA516Cf06857215779Fd812CA3beF1B32",
        Erc20Token {
            name: "Lido DAO Token",
            symbol: "LDO",
            decimals: 18,
            address: "0x5A98FcBEA516Cf06857215779Fd812CA3beF1B32",
        },
    );

    t.insert(
        "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984",
        Erc20Token {
            name: "Uniswap",
            symbol: "UNI",
            decimals: 18,
            address: "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984",
        },
    );

    t.insert(
        "0xC011a73ee8576Fb46F5E1c5751cA3B9Fe0af2a6F",
        Erc20Token {
            name: "Synthetix Network Token",
            symbol: "SNX",
            decimals: 18,
            address: "0xC011a73ee8576Fb46F5E1c5751cA3B9Fe0af2a6F",
        },
    );

    t.insert(
        "0x111111111117dC0aa78b770fA6A738034120C302",
        Erc20Token {
            name: "1INCH Token",
            symbol: "1INCH",
            decimals: 18,
            address: "0x111111111117dC0aa78b770fA6A738034120C302",
        },
    );

    t
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
