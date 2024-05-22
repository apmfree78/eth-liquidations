use crate::data::erc20::{Convert, TOKEN_DATA};
use crate::exchanges::aave_v3::user_data::AaveToken;
use async_trait::async_trait;
use bigdecimal::{BigDecimal, FromPrimitive, Zero};
use ethers::providers::{Provider, Ws};
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

#[async_trait]
pub trait UserAccountData {
    async fn get_health_factor_from_oracle(
        &self,
        client: &Arc<Provider<Ws>>,
    ) -> Result<BigDecimal, Box<dyn std::error::Error>>;

    async fn get_health_factor_with_uniswap_v3(
        &self,
        client: &Arc<Provider<Ws>>,
    ) -> Result<BigDecimal, Box<dyn std::error::Error>>;

    async fn get_list_of_user_tokens(&self) -> Result<Vec<AaveToken>, Box<dyn std::error::Error>>;
}

#[async_trait]
impl UserAccountData for AaveUser {
    async fn get_health_factor_from_oracle(
        &self,
        client: &Arc<Provider<Ws>>,
    ) -> Result<BigDecimal, Box<dyn std::error::Error>> {
        let bps_factor = BigDecimal::from_u64(10_u64.pow(4)).unwrap();

        let mut total_debt_usd = BigDecimal::zero();
        let mut liquidation_threshold_collateral_sum = BigDecimal::zero();

        for r in &self.reserves {
            let token = TOKEN_DATA.get(&*r.reserve.symbol).unwrap();

            // 1. get token price USD
            let token_price_usd = token.get_token_oracle_price(&client).await?;

            // 2. get get current total debt in USD
            // *************************************
            let current_total_debt = BigDecimal::from_str(&*r.current_total_debt)?;
            // *************************************

            if current_total_debt > BigDecimal::zero() {
                let current_total_debt_usd = &current_total_debt * &token_price_usd;

                // 3. add current total debt to total debt
                total_debt_usd += &current_total_debt_usd;
            }

            if r.reserve.usage_as_collateral_enabled {
                // 4. get atoken balance in USD
                // *************************************
                let current_atoken_balance =
                    BigDecimal::from_str(&*r.current_atoken_balance).unwrap();
                // *************************************

                if current_atoken_balance > BigDecimal::zero() {
                    let current_atoken_usd = current_atoken_balance * &token_price_usd;

                    // 5. update liquidity threshold colleral sum
                    let liquidation_threshold =
                        BigDecimal::from_str(&*r.reserve.reserve_liquidation_threshold).unwrap();
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

    async fn get_health_factor_with_uniswap_v3(
        &self,
        client: &Arc<Provider<Ws>>,
    ) -> Result<BigDecimal, Box<dyn std::error::Error>> {
        let bps_factor = BigDecimal::from_u64(10_u64.pow(4)).unwrap();

        let mut total_debt_eth = BigDecimal::zero();
        let mut liquidation_threshold_collateral_sum = BigDecimal::zero();

        for r in &self.reserves {
            let token = TOKEN_DATA.get(&*r.reserve.symbol).unwrap();

            // 1. get token price from uniswap
            let token_price_eth = token.get_token_price_in_("WETH", &client).await?;

            // 2. get get current total debt in USD
            // *************************************
            let current_total_debt = BigDecimal::from_str(&*r.current_total_debt)?;
            // *************************************

            if current_total_debt > BigDecimal::zero() {
                let current_total_debt_eth = &current_total_debt * &token_price_eth;

                // 3. add current total debt to total debt
                total_debt_eth += &current_total_debt_eth;
            }

            if r.reserve.usage_as_collateral_enabled {
                // 4. get atoken balance in USD
                // *************************************
                let current_atoken_balance =
                    BigDecimal::from_str(&*r.current_atoken_balance).unwrap();
                // *************************************

                if current_atoken_balance > BigDecimal::zero() {
                    let current_atoken_eth = current_atoken_balance * &token_price_eth;

                    // 5. update liquidity threshold colleral sum
                    let liquidation_threshold =
                        BigDecimal::from_str(&*r.reserve.reserve_liquidation_threshold).unwrap();
                    // *************************************

                    liquidation_threshold_collateral_sum +=
                        current_atoken_eth * &liquidation_threshold / &bps_factor;
                }
            }
        }
        // println!("total debt {}", total_debt_eth);
        let mut health_factor = BigDecimal::zero();
        if total_debt_eth > BigDecimal::zero() {
            health_factor = liquidation_threshold_collateral_sum / total_debt_eth;
        }
        Ok(health_factor)
    }

    async fn get_list_of_user_tokens(&self) -> Result<Vec<AaveToken>, Box<dyn std::error::Error>> {
        let mut user_token_list: Vec<AaveToken> = Vec::new();

        for r in &self.reserves {
            let token = TOKEN_DATA.get(&*r.reserve.symbol).unwrap();
            // println!("getting price of {} in usd", token.symbol);
            // let token_price_eth = token.get_token_price_in_("USDC", &client).await?;

            let current_total_debt = BigDecimal::from_str(&*r.current_total_debt)?;
            let current_atoken_balance = BigDecimal::from_str(&*r.current_atoken_balance).unwrap();
            let reserve_liquidation_threshold =
                BigDecimal::from_str(&*r.reserve.reserve_liquidation_threshold).unwrap();
            let reserve_liquidation_bonus =
                BigDecimal::from_str(&*r.reserve.reserve_liquidation_bonus).unwrap();
            let usage_as_collateral_enabled = r.reserve.usage_as_collateral_enabled;
            // get debt, colladeral, liquidation threshold, bonus, and usage colladeral boolean
            user_token_list.push(AaveToken {
                token: *token,
                current_total_debt,
                usage_as_collateral_enabled,
                current_atoken_balance,
                reserve_liquidation_threshold,
                reserve_liquidation_bonus,
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
