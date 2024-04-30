use crate::abi::{aave_oracle::AAVE_ORACLE, aave_v3_pool::AAVE_V3_POOL};
use crate::get_aave_users::{get_aave_v3_users, UserAccountData};
use async_trait::async_trait;
use bigdecimal::{BigDecimal, FromPrimitive, Zero};
use ethers::providers::{Provider, Ws};
use ethers::{abi::Address, core::types::U256};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use uniswap_sdk_core::entities::token::{Token, TokenMeta};
use uniswap_v3_sdk::{
    constants::{FeeAmount, FACTORY_ADDRESS},
    extensions::{fraction_to_big_decimal, get_pool},
};

pub enum PricingSource {
    AaveOracle,
    UniswapV3,
}
#[derive(Clone, Debug)]
pub struct AaveToken {
    pub token: Erc20Token,
    pub current_total_debt: BigDecimal,
    pub token_price_eth: BigDecimal,
    pub usage_as_collateral_enabled: bool,
    pub current_atoken_balance: BigDecimal,
    pub reserve_liquidation_threshold: BigDecimal,
    pub reserve_liquidation_bonus: BigDecimal,
}

#[derive(Clone, Debug)]
pub struct AaveUserData {
    pub id: Address,
    pub total_debt: BigDecimal,
    pub colladeral_times_liquidation_factor: BigDecimal,
    pub tokens: Vec<AaveToken>,
    pub health_factor: BigDecimal,
}

#[async_trait]
pub trait Generate {
    async fn get_users(
        client: &Arc<Provider<Ws>>,
    ) -> Result<Vec<AaveUserData>, Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait HealthFactor {
    async fn get_health_factor_from_(
        &self,
        source_for_pricing: PricingSource,
        client: &Arc<Provider<Ws>>,
    ) -> Result<BigDecimal, Box<dyn std::error::Error>>;

    async fn get_and_update_health_factor_with_(
        &mut self,
        source_for_pricing: PricingSource,
        client: &Arc<Provider<Ws>>,
    ) -> Result<BigDecimal, Box<dyn std::error::Error>>;
}

#[async_trait]
impl Generate for AaveUserData {
    async fn get_users(
        client: &Arc<Provider<Ws>>,
    ) -> Result<Vec<AaveUserData>, Box<dyn std::error::Error>> {
        println!("connecting to aave_v3_pool");
        let aave_v3_pool = AAVE_V3_POOL::new(*AAVE_V3_POOL_ADDRESS, client.clone());

        // store all data that we need for user
        let mut aave_user_data: Vec<AaveUserData> = Vec::new();

        let aave_users = get_aave_v3_users().await?;
        println!("got aave_v3 users");
        let bps_factor = BigDecimal::from_u64(10_u64.pow(4)).unwrap();
        let standard_scale = BigDecimal::from_u64(10_u64.pow(18)).unwrap();
        println!("found { } users from aave v3 graphql", aave_users.len());
        for user in &aave_users {
            let user_id: Address = user.id.parse()?;
            // println!("getting user account data from aave_v3_pool");
            let (
                total_collateral_base,
                total_debt_base,
                _available_borrows_base,
                current_liquidation_threshold,
                _ltv,
                health_factor,
            ) = aave_v3_pool.get_user_account_data(user_id).call().await?;

            let total_debt = u256_to_big_decimal(&total_debt_base);
            let total_collateral = u256_to_big_decimal(&total_collateral_base);
            let liquidation_threshold = u256_to_big_decimal(&current_liquidation_threshold);
            let colladeral_times_liquidation_factor =
                &liquidation_threshold * &total_collateral / &bps_factor;
            let health_factor = u256_to_big_decimal(&health_factor) / &standard_scale;

            // println!("getting list of user tokens");
            // this is list of tokens that user is either using as colladeral or borrowing
            let user_tokens = user.get_list_of_user_tokens(&client).await?;

            let aave_user = AaveUserData {
                id: user_id,
                total_debt: total_debt.clone(),
                colladeral_times_liquidation_factor,
                health_factor: health_factor.clone(),
                tokens: user_tokens,
            };

            // validate user data - 10% of graphql data for aave users is not accurate
            let aave_user_health_factor = aave_user.health_factor.clone();
            let aave_user_calculated_health_factor = aave_user
                .get_health_factor_from_(PricingSource::AaveOracle, &client)
                .await?;
            let lower_bound = BigDecimal::from_str("0.95")? * &aave_user_health_factor;
            let upper_bound = BigDecimal::from_str("1.05")? * &aave_user_health_factor;

            if aave_user_calculated_health_factor > lower_bound
                && aave_user_calculated_health_factor < upper_bound
            {
                // save data to AvveUserData
                aave_user_data.push(aave_user);
            }
        }

        println!("{} valid users saved", aave_user_data.len());
        Ok(aave_user_data)
    }
}

#[async_trait]
impl HealthFactor for AaveUserData {
    async fn get_health_factor_from_(
        &self,
        source_for_pricing: PricingSource,
        client: &Arc<Provider<Ws>>,
    ) -> Result<BigDecimal, Box<dyn std::error::Error>> {
        let bps_factor = BigDecimal::from_u64(10_u64.pow(4)).unwrap();

        let mut total_debt_usd = BigDecimal::zero();
        let mut liquidation_threshold_collateral_sum = BigDecimal::zero();

        for r in &self.tokens {
            let token = TOKEN_DATA.get(&*r.token.symbol).unwrap();

            // 1. get token price USD
            let token_price_usd = match source_for_pricing {
                PricingSource::UniswapV3 => token.get_token_price_in_("USDC", &client).await?,
                PricingSource::AaveOracle => token.get_token_oracle_price(&client).await?,
            };

            let token_decimal_factor =
                BigDecimal::from_u64(10_u64.pow(token.decimals.into())).unwrap();

            // 2. get get current total debt in USD
            // *************************************
            let current_total_debt = r.current_total_debt.clone();
            // *************************************

            if current_total_debt > BigDecimal::zero() {
                let current_total_debt_usd =
                    &current_total_debt * &token_price_usd / &token_decimal_factor;

                // 3. add current total debt to total debt
                total_debt_usd += &current_total_debt_usd;
            }

            if r.usage_as_collateral_enabled {
                // 4. get atoken balance in USD
                // *************************************
                let current_atoken_balance = r.current_atoken_balance.clone();
                // *************************************

                if current_atoken_balance > BigDecimal::zero() {
                    let current_atoken_usd =
                        &current_atoken_balance * &token_price_usd / &token_decimal_factor;

                    // 5. update liquidity threshold colleral sum
                    let liquidation_threshold = r.reserve_liquidation_threshold.clone();
                    // *************************************

                    liquidation_threshold_collateral_sum +=
                        current_atoken_usd * &liquidation_threshold / &bps_factor;
                }
            }
        }

        // println!("total debt {}", total_debt_usd);
        let mut health_factor = BigDecimal::zero();
        if total_debt_usd > BigDecimal::zero() {
            health_factor = liquidation_threshold_collateral_sum / total_debt_usd;
        }
        Ok(health_factor)
    }

    async fn get_and_update_health_factor_with_(
        &mut self,
        source_for_pricing: PricingSource,
        client: &Arc<Provider<Ws>>,
    ) -> Result<BigDecimal, Box<dyn std::error::Error>> {
        // obtain latest health factor
        let health_factor = self
            .get_health_factor_from_(source_for_pricing, &client)
            .await?;

        self.health_factor = health_factor.clone();

        Ok(health_factor)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Erc20Token {
    pub name: &'static str,
    pub symbol: &'static str,
    pub decimals: u8,
    pub address: &'static str,
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
            || self.symbol == "LDO"
            || self.symbol == "rETH"
            || self.symbol == "LUSD"
            || self.symbol == "weETH"
            || self.symbol == "wstETH"
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

    // async fn get_token_price_in_eth(
    //     &self,
    //     client: &Arc<Provider<Ws>>,
    // ) -> Result<BigDecimal, Box<dyn std::error::Error>> {
    //     if self.symbol == "WETH" {
    //         return Ok(BigDecimal::from(1));
    //     };

    //     // put in exception for GHO - calculate price is USDC and convert to ETH
    //     let weth_token: &Erc20Token = TOKEN_DATA.get("WETH").unwrap();
    //     let weth_token = weth_token.get_token(1).await?;

    //     let token = self.get_token(1).await?;

    //     let pool = get_pool(
    //         1,
    //         FACTORY_ADDRESS,
    //         token.meta.address,
    //         weth_token.meta.address,
    //         FeeAmount::MEDIUM,
    //         client.clone(),
    //         None,
    //     )
    //     .await?;

    //     let token_price_in_eth = pool.token0_price();
    //     let token_price_in_eth = fraction_to_big_decimal(&token_price_in_eth);
    //     let token_price_in_eth = convert_uniswap_to_bigdecimal(token_price_in_eth);

    //     Ok(token_price_in_eth)
    // }
}

fn convert_uniswap_to_bigdecimal(
    uniswap_bd: uniswap_sdk_core::prelude::BigDecimal,
) -> bigdecimal::BigDecimal {
    let as_string = uniswap_bd.to_string(); // Convert Uniswap BigDecimal to String
    bigdecimal::BigDecimal::from_str(&as_string).expect("Invalid BigDecimal format")
    // Convert String to bigdecimal BigDecimal
}

pub static WETH_ADDRESS: Lazy<Address> = Lazy::new(|| {
    "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
        .parse()
        .expect("Invalid address")
});
pub static AAVE_ORACLE_ADDRESS: Lazy<Address> = Lazy::new(|| {
    "0x54586bE62E3c3580375aE3723C145253060Ca0C2"
        .parse()
        .expect("Invalid address")
});
pub static AAVE_V3_POOL_ADDRESS: Lazy<Address> = Lazy::new(|| {
    "0x87870Bca3F3fD6335C3F4ce8392D69350B4fA4E2"
        .parse()
        .expect("Invalid address")
});

pub static TOKEN_DATA: Lazy<HashMap<&'static str, Erc20Token>> = Lazy::new(|| {
    let mut tokens = Vec::<Erc20Token>::new();

    tokens.push(Erc20Token {
        name: "Wrapped Ether",
        symbol: "WETH",
        decimals: 18,
        address: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
    });

    tokens.push(Erc20Token {
        name: "Wrapped liquid staked Ether 2.0",
        symbol: "wstETH",
        decimals: 18,
        address: "0x7f39C581F595B53c5cb19bD0b3f8dA6c935E2Ca0",
    });

    tokens.push(Erc20Token {
        name: "USD Coin",
        symbol: "USDC",
        decimals: 6,
        address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    });

    tokens.push(Erc20Token {
        name: "Dai Stablecoin",
        symbol: "DAI",
        decimals: 18,
        address: "0x6B175474E89094C44Da98b954EedeAC495271d0F",
    });

    tokens.push(Erc20Token {
        name: "ChainLink Token",
        symbol: "LINK",
        decimals: 18,
        address: "0x514910771AF9Ca656af840dff83E8264EcF986CA",
    });

    tokens.push(Erc20Token {
        name: "Tether USD",
        symbol: "USDT",
        decimals: 6,
        address: "0xdac17f958d2ee523a2206206994597c13d831ec7",
    });

    tokens.push(Erc20Token {
        name: "Wrapped BTC",
        symbol: "WBTC",
        decimals: 8,
        address: "0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599",
    });

    tokens.push(Erc20Token {
        name: "Rocket Pool ETH",
        symbol: "rETH",
        decimals: 18,
        address: "0xae78736Cd615f374D3085123A210448E74Fc6393",
    });

    tokens.push(Erc20Token {
        name: "Maker",
        symbol: "MKR",
        decimals: 18,
        address: "0x9f8F72aA9304c8B593d555F12eF6589cC3A579A2",
    });

    tokens.push(Erc20Token {
        name: "Coinbase Wrapped Staked ETH",
        symbol: "cbETH",
        decimals: 18,
        address: "0xBe9895146f7AF43049ca1c1AE358B0541Ea49704",
    });

    tokens.push(Erc20Token {
        name: "PayPal USD",
        symbol: "PYUSD",
        decimals: 6,
        address: "0x6c3ea9036406852006290770BEdFcAbA0e23A0e8",
    });

    tokens.push(Erc20Token {
        name: "Lido DAO Token",
        symbol: "LDO",
        decimals: 18,
        address: "0x5A98FcBEA516Cf06857215779Fd812CA3beF1B32",
    });

    tokens.push(Erc20Token {
        name: "Uniswap",
        symbol: "UNI",
        decimals: 18,
        address: "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984",
    });

    tokens.push(Erc20Token {
        name: "Synthetix Network Token",
        symbol: "SNX",
        decimals: 18,
        address: "0xC011a73ee8576Fb46F5E1c5751cA3B9Fe0af2a6F",
    });

    tokens.push(Erc20Token {
        name: "1INCH Token",
        symbol: "1INCH",
        decimals: 18,
        address: "0x111111111117dC0aa78b770fA6A738034120C302",
    });

    tokens.push(Erc20Token {
        name: "LUSD Stablecoin",
        symbol: "LUSD",
        decimals: 18,
        address: "0x5f98805A4E8be255a32880FDeC7F6728C6568bA0",
    });

    tokens.push(Erc20Token {
        name: "Ethereum Name Service",
        symbol: "ENS",
        decimals: 18,
        address: "0xC18360217D8F7Ab5e7c516566761Ea12Ce7F9D72",
    });

    tokens.push(Erc20Token {
        name: "Balancer",
        symbol: "BAL",
        decimals: 18,
        address: "0xba100000625a3754423978a60c9317c58a424e3D",
    });

    tokens.push(Erc20Token {
        name: "Curve DAO Token",
        symbol: "CRV",
        decimals: 18,
        address: "0xD533a949740bb3306d119CC777fa900bA034cd52",
    });

    tokens.push(Erc20Token {
        name: "Frax",
        symbol: "FRAX",
        decimals: 18,
        address: "0x853d955aCEf822Db058eb8505911ED77F175b99e",
    });

    tokens.push(Erc20Token {
        name: "Curve.Fi USD Stablecoin",
        symbol: "crvUSD",
        decimals: 18,
        address: "0xf939E0A03FB07F59A73314E73794Be0E57ac1b4E",
    });

    tokens.push(Erc20Token {
        name: "Gho Token",
        symbol: "GHO",
        decimals: 18,
        address: "0x40D16FC0246aD3160Ccc09B8D0D3A2cD28aE6C2f",
    });

    tokens.push(Erc20Token {
        name: "Aave Token",
        symbol: "AAVE",
        decimals: 18,
        address: "0x7Fc66500c84A76Ad7e9c93437bFc5Ac33E2DDaE9",
    });

    tokens.push(Erc20Token {
        name: "Savings Dai",
        symbol: "sDAI",
        decimals: 18,
        address: "0x83F20F44975D03b1b09e64809B757c47f942BEeA",
    });

    tokens.push(Erc20Token {
        name: "Wrapped eETH",
        symbol: "weETH",
        decimals: 18,
        address: "0xCd5fE23C85820F7B72D0926FC9b05b43E359b7ee",
    });

    let mut token_hash = HashMap::new();

    // create hashmap with token symbol as index
    for token in &tokens {
        token_hash.insert(token.symbol, *token);
    }

    // copy hashmap with token address as index
    for token in &tokens {
        token_hash.insert(token.address, *token);
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
