use crate::abi::aave_oracle::AAVE_ORACLE;
use crate::crypto_data::{
    Erc20Token, AAVE_ORACLE_ADDRESS, AAVE_V3_POOL_ADDRESS, TOKEN_DATA, WETH_ADDRESS,
};
use bigdecimal::{BigDecimal, FromPrimitive, Zero};
use core::panic;
use ethers::providers::{Provider, Ws};
use ethers::{abi::Address, core::types::U256};
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use serde_json;
use std::str::FromStr;
use std::sync::Arc;
#[derive(Debug, Serialize, Deserialize)]
pub struct AaveUser {
    pub id: String,
    #[serde(rename = "borrowedReservesCount")]
    borrowed_reserves_count: i64,
    reserves: Vec<UserReserve>,
}

pub trait UserAccountData {
    async fn get_health_factor_from_oracle(
        &self,
        client: &Arc<Provider<Ws>>,
    ) -> Result<BigDecimal, Box<dyn std::error::Error>>;
    fn get_list_of_user_tokens(&self) -> Result<Vec<Erc20Token>, Box<dyn std::error::Error>>;
}

impl UserAccountData for AaveUser {
    async fn get_health_factor_from_oracle(
        &self,
        client: &Arc<Provider<Ws>>,
    ) -> Result<BigDecimal, Box<dyn std::error::Error>> {
        let aave_oracle = AAVE_ORACLE::new(*AAVE_ORACLE_ADDRESS, client.clone());
        let bps_factor = BigDecimal::from_u64(10_u64.pow(4)).unwrap();
        // let usd_decimals = BigDecimal::from_u64(10_u64.pow(8)).unwrap();

        let mut total_debt_usd = BigDecimal::zero();
        let mut liquidation_threshold_collateral_sum = BigDecimal::zero();

        for r in &self.reserves {
            let token_address: Address;
            let token_decimals: u8;
            match TOKEN_DATA.get(&*r.reserve.symbol) {
                Some(token) => {
                    token_address = token.address.parse()?;
                    token_decimals = token.decimals;
                }
                None => panic!("No value found for {}", r.reserve.symbol),
            }
            // 1. get token price from AAve Oracle
            let token_price_usd_u256: U256 =
                aave_oracle.get_asset_price(token_address).call().await?;
            let token_price_usd: BigDecimal =
                BigDecimal::from_str(&(token_price_usd_u256.to_string())).unwrap();
            let token_decimal_factor =
                BigDecimal::from_u64(10_u64.pow(token_decimals.into())).unwrap();

            // 2. get get current total debt in USD
            let current_total_debt = BigDecimal::from_str(&*r.current_total_debt)?;

            if current_total_debt > BigDecimal::zero() {
                let current_total_debt_usd =
                    &current_total_debt * &token_price_usd / &token_decimal_factor;

                // 3. add current total debt to total debt
                total_debt_usd += &current_total_debt_usd;
            }

            if r.reserve.usage_as_collateral_enabled {
                // 4. get atoken balance in USD
                let current_atoken_balance =
                    BigDecimal::from_str(&*r.current_atoken_balance).unwrap();

                if current_atoken_balance > BigDecimal::zero() {
                    let current_atoken_usd =
                        current_atoken_balance * &token_price_usd / &token_decimal_factor;

                    // 5. update liquidity threshold colleral sum
                    let liquidation_threshold =
                        BigDecimal::from_str(&*r.reserve.reserve_liquidation_threshold).unwrap();
                    liquidation_threshold_collateral_sum +=
                        current_atoken_usd * &liquidation_threshold / &bps_factor;
                }
            }
        }
        println!("total debt {}", total_debt_usd);
        let mut health_factor = BigDecimal::zero();
        if total_debt_usd > BigDecimal::zero() {
            health_factor = liquidation_threshold_collateral_sum / total_debt_usd;
        }
        Ok(health_factor)
    }

    fn get_list_of_user_tokens(&self) -> Result<Vec<Erc20Token>, Box<dyn std::error::Error>> {
        let mut user_token_list: Vec<Erc20Token> = Vec::new();

        for r in &self.reserves {
            let token_address: &str;
            let token_decimals: u8;
            let token_name: &str;
            let token_symbol: &str;
            match TOKEN_DATA.get(&*r.reserve.symbol) {
                Some(token) => {
                    token_address = token.address;
                    token_decimals = token.decimals;
                    token_name = token.name;
                    token_symbol = token.symbol;
                }
                None => panic!("No value found for {}", r.reserve.symbol),
            }
            user_token_list.push(Erc20Token {
                name: token_name,
                symbol: token_symbol,
                decimals: token_decimals,
                address: token_address,
            })
        }
        Ok(user_token_list)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserReserve {
    #[serde(rename = "currentATokenBalance")]
    current_atoken_balance: String,
    #[serde(rename = "currentStableDebt")]
    current_stable_debt: String,
    #[serde(rename = "currentVariableDebt")]
    current_variable_debt: String,
    #[serde(rename = "currentTotalDebt")]
    current_total_debt: String,
    reserve: Reserve,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reserve {
    id: String,
    name: String,
    symbol: String,
    #[serde(rename = "usageAsCollateralEnabled")]
    usage_as_collateral_enabled: bool,
    #[serde(rename = "reserveLiquidationThreshold")]
    reserve_liquidation_threshold: String,
    #[serde(rename = "reserveLiquidationBonus")]
    reserve_liquidation_bonus: String,
    price: Price,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Price {
    #[serde(rename = "priceInEth")]
    price_in_eth: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    users: Vec<AaveUser>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    data: Option<Data>,
    errors: Option<Vec<serde_json::Value>>,
}

pub async fn get_aave_v3_users() -> Result<Vec<AaveUser>, Box<dyn std::error::Error>> {
    let query = r#"
    {
     users(where: {borrowedReservesCount_gt: 0}) {
        id
        borrowedReservesCount
        reserves {
          currentATokenBalance
          currentStableDebt
          currentVariableDebt
          currentTotalDebt
          reserve {
            id
            name
            symbol
            usageAsCollateralEnabled
            reserveLiquidationThreshold
            reserveLiquidationBonus
            price {
              priceInEth
            }
          }
        }
      }
    }"#;

    let client = Client::new();
    let response = client
        .post("https://api.thegraph.com/subgraphs/name/aave/protocol-v3")
        .header(header::CONTENT_TYPE, "application/json")
        .json(&serde_json::json!({ "query": query }))
        .send()
        .await?;

    let raw_response = response.text().await?;
    // println!("Raw JSON response: {}", raw_response);

    let response: Response = serde_json::from_str(&raw_response)?;

    if let Some(errors) = response.errors {
        for error in errors {
            println!("Error: {:?}", error);
        }
        return Err("GraphQL errors occurred".into()); // Convert the static string error into a Box<dyn Error>
    }

    match response.data {
        Some(data) => {
            if data.users.is_empty() {
                println!("no user object found");
                return Ok(vec![]);
            } else {
                println!("user object found");
                return Ok(data.users);
            }
        }
        None => {
            println!("Data field not found in the response.");
            return Ok(vec![]);
        }
    }
}
