use crate::data::erc20::TOKEN_DATA;
use crate::exchanges::aave_v3::user_structs::AaveToken;
use async_trait::async_trait;
use bigdecimal::BigDecimal;
use core::panic;
use log::{debug, error, info, warn};
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use serde_json;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct AaveUser {
    pub id: String,
    #[serde(rename = "borrowedReservesCount")]
    borrowed_reserves_count: i64,
    reserves: Vec<UserReserve>,
}

#[async_trait]
pub trait UserAccountData {
    async fn get_list_of_user_tokens(&self) -> Result<Vec<AaveToken>, Box<dyn std::error::Error>>;
}

#[async_trait]
impl UserAccountData for AaveUser {
    async fn get_list_of_user_tokens(&self) -> Result<Vec<AaveToken>, Box<dyn std::error::Error>> {
        let mut user_token_list: Vec<AaveToken> = Vec::new();

        for r in &self.reserves {
            let token = TOKEN_DATA.get(&*r.reserve.symbol).unwrap_or_else(|| {
                panic!(
                    "could not find token with symbol, and name: {} => {} ",
                    r.reserve.symbol, r.reserve.name
                )
            });

            let current_total_debt = BigDecimal::from_str(&r.current_total_debt)?;
            let current_atoken_balance = BigDecimal::from_str(&r.current_atoken_balance).unwrap();
            let reserve_liquidation_threshold =
                BigDecimal::from_str(&r.reserve.reserve_liquidation_threshold).unwrap();
            let reserve_liquidation_bonus =
                BigDecimal::from_str(&r.reserve.reserve_liquidation_bonus).unwrap();
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
     users(first: 300, where: {borrowedReservesCount_gt: 0}) {
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

    let response: Response = serde_json::from_str(&raw_response)?;

    if let Some(errors) = response.errors {
        for error in errors {
            error!("Error: {:?}", error);
        }
        return Err("GraphQL errors occurred".into()); // Convert the static string error into a Box<dyn Error>
    }

    match response.data {
        Some(data) => {
            if data.users.is_empty() {
                warn!("no user object found");
                Ok(vec![])
            } else {
                debug!("user object found");
                Ok(data.users)
            }
        }
        None => {
            error!("Data field not found in the response.");
            Ok(vec![])
        }
    }
}

pub async fn get_all_aave_v3_users() -> Result<Vec<AaveUser>, Box<dyn std::error::Error>> {
    let query = r#"
    query GetUsers($lastID: String) {
     users(first: 1000, where: { borrowedReservesCount_gt: 0, id_gt: $lastID }) {
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
    let mut all_users = Vec::<AaveUser>::new();
    let mut id_cursor = "0".to_string();

    loop {
        let response = client
            .post("https://api.thegraph.com/subgraphs/name/aave/protocol-v3")
            .header(header::CONTENT_TYPE, "application/json")
            .json(&serde_json::json!({
            "query": query,
            "variables": {
                "lastID": id_cursor,
            }
            }))
            .send()
            .await?;

        let raw_response = response.text().await?;

        let response: Response = serde_json::from_str(&raw_response)?;

        if let Some(errors) = response.errors {
            for error in errors {
                error!("Error: {:?}", error);
            }
            return Err("GraphQL errors occurred".into());
        }

        match response.data {
            Some(mut data) => {
                if data.users.is_empty() {
                    debug!("no more users found");
                    break;
                } else {
                    info!("{} users found", data.users.len());

                    // find last element of array to index users
                    let last_element = data.users.last().unwrap();
                    id_cursor = last_element.id.clone();
                    all_users.append(&mut data.users);
                }
            }
            None => {
                error!("Data field not found in the response.");
                break;
            }
        };
    }

    Ok(all_users)
}
