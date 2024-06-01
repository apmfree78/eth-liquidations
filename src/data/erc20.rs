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
        chain_link_price_feed: "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419",
        chainlink_aggregator: "0xe62b71cf983019bff55bc83b48601ce8419650cc",
    });

    tokens.push(Erc20Token {
        name: "Wrapped liquid staked Ether 2.0",
        symbol: "wstETH",
        decimals: 18,
        address: "0x7f39C581F595B53c5cb19bD0b3f8dA6c935E2Ca0",
        liquidation_bonus: 10600,
        liquidation_threshold: 8100,
        // track with movement of ETH
        chain_link_price_feed: "0xB4aB0c94159bc2d8C133946E7241368fc2F2a010",
        chainlink_aggregator: "",
    });

    tokens.push(Erc20Token {
        name: "Staked ETH",
        symbol: "osETH",
        decimals: 18,
        address: "0xf1C9acDc66974dFB6dEcB12aA385b9cD01190E38",
        liquidation_bonus: 10750,
        liquidation_threshold: 7500,
        // track with movement of ETH
        chain_link_price_feed: "0x0a2af898cec35197e6944d9e0f525c2626393442",
        chainlink_aggregator: "",
    });

    tokens.push(Erc20Token {
        name: "USD Coin",
        symbol: "USDC",
        decimals: 6,
        address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
        liquidation_bonus: 10450,
        liquidation_threshold: 7800,
        // do not track
        chain_link_price_feed: "0x736bF902680e68989886e9807CD7Db4B3E015d3C",
        chainlink_aggregator: "",
    });

    tokens.push(Erc20Token {
        name: "Dai Stablecoin",
        symbol: "DAI",
        decimals: 18,
        address: "0x6B175474E89094C44Da98b954EedeAC495271d0F",
        liquidation_bonus: 10500,
        liquidation_threshold: 7700,
        // do not track
        chain_link_price_feed: "0xaEb897E1Dc6BbdceD3B9D551C71a8cf172F27AC4",
        chainlink_aggregator: "",
    });

    tokens.push(Erc20Token {
        name: "ChainLink Token",
        symbol: "LINK",
        decimals: 18,
        address: "0x514910771AF9Ca656af840dff83E8264EcF986CA",
        liquidation_bonus: 10700,
        liquidation_threshold: 6800,
        chain_link_price_feed: "0x2c1d072e956AFFC0D435Cb7AC38EF18d24d9127c",
        chainlink_aggregator: "0x20807cf61ad17c31837776fa39847a2fa1839e81",
    });

    tokens.push(Erc20Token {
        name: "Tether USD",
        symbol: "USDT",
        decimals: 6,
        address: "0xdac17f958d2ee523a2206206994597c13d831ec7",
        liquidation_bonus: 10450,
        liquidation_threshold: 7800,
        // do not track
        chain_link_price_feed: "0xc26d4a1c46d884cff6de9800b6ae7a8cf48b4ff8",
        chainlink_aggregator: "",
    });

    tokens.push(Erc20Token {
        name: "Wrapped BTC",
        symbol: "WBTC",
        decimals: 8,
        address: "0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599",
        liquidation_bonus: 10500,
        liquidation_threshold: 8300,
        chain_link_price_feed: "0x230e0321cf38f09e247e50afc7801ea2351fe56f",
        chainlink_aggregator: "0xdBe1941BFbe4410D6865b9b7078e0b49af144D2d", // BTC/USD
    });

    tokens.push(Erc20Token {
        name: "Rocket Pool ETH",
        symbol: "rETH",
        decimals: 18,
        address: "0xae78736Cd615f374D3085123A210448E74Fc6393",
        liquidation_bonus: 10750,
        liquidation_threshold: 7700,
        // also track with ETH movement
        chain_link_price_feed: "0x536218f9E9Eb48863970252233c8F271f554C2d0", // rETH/ETH
        chainlink_aggregator: "0x9cb248e68fb81d0cfe7d6b3265fe6bf123a71fe0",
    });

    tokens.push(Erc20Token {
        name: "Maker",
        symbol: "MKR",
        decimals: 18,
        address: "0x9f8F72aA9304c8B593d555F12eF6589cC3A579A2",
        liquidation_bonus: 10850,
        liquidation_threshold: 7000,
        chain_link_price_feed: "0xec1D1B3b0443256cc3860e24a46F108e699484Aa",
        chainlink_aggregator: "0x71febc2f741f113af322e1b576ef005a4424574f",
    });

    tokens.push(Erc20Token {
        name: "Coinbase Wrapped Staked ETH",
        symbol: "cbETH",
        decimals: 18,
        address: "0xBe9895146f7AF43049ca1c1AE358B0541Ea49704",
        liquidation_bonus: 10750,
        liquidation_threshold: 7700,
        // track with movement of ETH
        chain_link_price_feed: "0x6243d2F41b4ec944F731f647589E28d9745a2674",
        chainlink_aggregator: "",
    });

    tokens.push(Erc20Token {
        name: "PayPal USD",
        symbol: "PYUSD",
        decimals: 6,
        address: "0x6c3ea9036406852006290770BEdFcAbA0e23A0e8",
        liquidation_bonus: 0,
        liquidation_threshold: 0,
        // do not track
        chain_link_price_feed: "0x150bae7ce224555d39afdbc6cb4b8204e594e022",
        chainlink_aggregator: "",
    });

    tokens.push(Erc20Token {
        name: "Lido DAO Token",
        symbol: "LDO",
        decimals: 18,
        address: "0x5A98FcBEA516Cf06857215779Fd812CA3beF1B32",
        liquidation_bonus: 10900,
        liquidation_threshold: 5000,
        chain_link_price_feed: "0xb01e6c9af83879b8e06a092f0dd94309c0d497e4",
        // track with movement of ETH
        chainlink_aggregator: "0x7898AcCC83587C3C55116c5230C17a6Cd9C71bad", // LDO/ETH
    });

    tokens.push(Erc20Token {
        name: "Uniswap",
        symbol: "UNI",
        decimals: 18,
        address: "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984",
        liquidation_bonus: 11000,
        liquidation_threshold: 7400,
        chain_link_price_feed: "0x553303d460EE0afB37EdFf9bE42922D8FF63220e",
        chainlink_aggregator: "0x373bce97bec13bfa8a5f07cc578ec2d77f80c589",
    });

    tokens.push(Erc20Token {
        name: "Synthetix Network Token",
        symbol: "SNX",
        decimals: 18,
        address: "0xC011a73ee8576Fb46F5E1c5751cA3B9Fe0af2a6F",
        liquidation_bonus: 10850,
        liquidation_threshold: 6400,
        // 0x373BCe97bec13BfA8A5f07Cc578EC2D77f80c589
        chain_link_price_feed: "0xDC3EA94CD0AC27d9A86C180091e7f78C683d3699",
        chainlink_aggregator: "0x06ce8be8729b6ba18dd3416e3c223a5d4db5e755",
    });

    tokens.push(Erc20Token {
        name: "1INCH Token",
        symbol: "1INCH",
        decimals: 18,
        address: "0x111111111117dC0aa78b770fA6A738034120C302",
        liquidation_bonus: 10750,
        liquidation_threshold: 6700,
        chain_link_price_feed: "0xc929ad75B72593967DE83E7F7Cda0493458261D9",
        chainlink_aggregator: "0xd2bdd1e01fd2f8d7d42b209c111c7b32158b5a42",
    });

    tokens.push(Erc20Token {
        name: "LUSD Stablecoin",
        symbol: "LUSD",
        decimals: 18,
        address: "0x5f98805A4E8be255a32880FDeC7F6728C6568bA0",
        liquidation_bonus: 10450,
        liquidation_threshold: 7700,
        // do not track
        chain_link_price_feed: "0x9ecdfacca946614cc32af63f3dbe50959244f3af",
        chainlink_aggregator: "",
    });

    tokens.push(Erc20Token {
        name: "Ethereum Name Service",
        symbol: "ENS",
        decimals: 18,
        address: "0xC18360217D8F7Ab5e7c516566761Ea12Ce7F9D72",
        liquidation_bonus: 10800,
        liquidation_threshold: 4900,
        chain_link_price_feed: "0x5C00128d4d1c2F4f652C267d7bcdD7aC99C16E16",
        chainlink_aggregator: "0x780f1bd91a5a22ede36d4b2b2c0eccb9b1726a28",
    });

    tokens.push(Erc20Token {
        name: "Balancer",
        symbol: "BAL",
        decimals: 18,
        address: "0xba100000625a3754423978a60c9317c58a424e3D",
        liquidation_bonus: 10830,
        liquidation_threshold: 5900,
        chain_link_price_feed: "0xdF2917806E30300537aEB49A7663062F4d1F2b5F",
        chainlink_aggregator: "0xbd9350a3a2fd6e3ad0a053a567f2609a1bf6c505",
    });

    tokens.push(Erc20Token {
        name: "Curve DAO Token",
        symbol: "CRV",
        decimals: 18,
        address: "0xD533a949740bb3306d119CC777fa900bA034cd52",
        liquidation_bonus: 10830,
        liquidation_threshold: 4100,
        chain_link_price_feed: "0xCd627aA160A6fA45Eb793D19Ef54f5062F20f33f",
        chainlink_aggregator: "0xb4c4a493ab6356497713a78ffa6c60fb53517c63",
    });

    tokens.push(Erc20Token {
        name: "Frax",
        symbol: "FRAX",
        decimals: 18,
        address: "0x853d955aCEf822Db058eb8505911ED77F175b99e",
        liquidation_bonus: 10600,
        liquidation_threshold: 7200,
        // custom oracle
        chain_link_price_feed: "0x45d270263bbee500cf8adcf2abc0ac227097b036",
        chainlink_aggregator: "0x9d78092775dfe715dfe1b0d71ac1a4d6e3652559",
    });

    tokens.push(Erc20Token {
        name: "Curve.Fi USD Stablecoin",
        symbol: "crvUSD",
        decimals: 18,
        address: "0xf939E0A03FB07F59A73314E73794Be0E57ac1b4E",
        liquidation_bonus: 0,
        liquidation_threshold: 0,
        // do not track
        chain_link_price_feed: "0x02AeE5b225366302339748951E1a924617b8814F",
        chainlink_aggregator: "",
    });

    tokens.push(Erc20Token {
        name: "Gho Token",
        symbol: "GHO",
        decimals: 18,
        address: "0x40D16FC0246aD3160Ccc09B8D0D3A2cD28aE6C2f",
        liquidation_bonus: 0,
        liquidation_threshold: 0,
        // gho custom oracle -- do not track stablecoin
        chain_link_price_feed: "0xd110cac5d8682a3b045d5524a9903e031d70fccd",
        chainlink_aggregator: "",
    });

    tokens.push(Erc20Token {
        name: "Aave Token",
        symbol: "AAVE",
        decimals: 18,
        address: "0x7Fc66500c84A76Ad7e9c93437bFc5Ac33E2DDaE9",
        liquidation_bonus: 10750,
        liquidation_threshold: 7300,
        chain_link_price_feed: "0x547a514d5e3769680Ce22B2361c10Ea13619e8a9",
        chainlink_aggregator: "0x8116b273cd75d79c382afacc706659ded5e0a59d",
    });

    tokens.push(Erc20Token {
        name: "Savings Dai",
        symbol: "sDAI",
        decimals: 18,
        address: "0x83F20F44975D03b1b09e64809B757c47f942BEeA",
        liquidation_bonus: 10450,
        liquidation_threshold: 7800,
        // do not track
        chain_link_price_feed: "0x29081f7ab5a644716efcdc10d5c926c5fee9f72b",
        chainlink_aggregator: "",
    });

    tokens.push(Erc20Token {
        name: "Wrapped eETH",
        symbol: "weETH",
        decimals: 18,
        address: "0xCd5fE23C85820F7B72D0926FC9b05b43E359b7ee",
        liquidation_bonus: 10750,
        liquidation_threshold: 7500,
        // ALSO track with ETH movement
        chain_link_price_feed: "0x5c9C449BbC9a6075A2c061dF312a35fd1E05fF22", // weETH/ETH
        chainlink_aggregator: "0x4df36f726d8059d881294166db52c1d13e976fe7",
    });

    tokens.push(Erc20Token {
        name: "Rocket Pool Protocol",
        symbol: "RPL",
        decimals: 18,
        address: "0xD33526068D116cE69F19A9ee46F0bd304F21A51f",
        liquidation_bonus: 0,
        liquidation_threshold: 0,
        chain_link_price_feed: "0x4E155eD98aFE9034b7A5962f6C84c86d869daA9d",
        chainlink_aggregator: "0x5df960959de45a2ba9dc11e6fd6f77107f43256c",
    });

    tokens.push(Erc20Token {
        name: "Frax Share",
        symbol: "FXS",
        decimals: 18,
        address: "0x3432B6A60D23Ca0dFCa7761B7ab56459D9C964D0",
        liquidation_bonus: 11000,
        liquidation_threshold: 4200,
        chain_link_price_feed: "0x6Ebc52C8C1089be9eB3945C4350B68B8E4C2233f",
        chainlink_aggregator: "0x9d78092775dfe715dfe1b0d71ac1a4d6e3652559",
    });

    tokens.push(Erc20Token {
        name: "StargateToken",
        symbol: "STG",
        decimals: 18,
        address: "0xAf5191B0De278C7286d6C7CC6ab6BB8A73bA2Cd6",
        liquidation_bonus: 11000,
        liquidation_threshold: 3700,
        chain_link_price_feed: "0x7A9f34a0Aa917D438e9b6E630067062B7F8f6f3d",
        chainlink_aggregator: "0x73455b8acd6d205544cbc034a6f6cab58c56ef47",
    });

    tokens.push(Erc20Token {
        name: "Kyber Network Crystal v2",
        symbol: "KNC",
        decimals: 18,
        address: "0xdeFA4e8a7bcBA345F687a2f1456F5Edd9CE97202",
        liquidation_bonus: 11000,
        liquidation_threshold: 3700,
        chain_link_price_feed: "0xf8fF43E991A81e6eC886a3D281A2C6cC19aE70Fc",
        chainlink_aggregator: "0xbc60258f775683ea28048030806ad3a80c4a33ae",
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

    // copy hashmap with chain link price feed as index
    for token in &tokens {
        token_hash.insert(
            token.chain_link_price_feed.to_string().to_lowercase(),
            *token,
        );
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
