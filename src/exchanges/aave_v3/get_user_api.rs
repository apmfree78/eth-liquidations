use super::user_structs::{SampleSize, BPS_FACTOR};
use crate::data::{erc20::Erc20Token, token_data_hash::save_erc20_token};
use crate::exchanges::aave_v3::user_structs::AaveToken;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use ethers::providers::{Provider, Ws};
use log::{debug, error, info, warn};
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::Arc;
use std::{env, str::FromStr};

#[derive(Debug, Serialize, Deserialize)]
pub struct AaveUser {
    pub id: String,
    #[serde(rename = "borrowedReservesCount")]
    borrowed_reserves_count: i64,
    reserves: Vec<UserReserve>,
}

#[async_trait]
pub trait UserAccountData {
    async fn get_list_of_user_tokens(
        &self,
        client: &Arc<Provider<Ws>>,
    ) -> Result<(Vec<AaveToken>, f64, f64, f64)>;
}

#[async_trait]
impl UserAccountData for AaveUser {
    async fn get_list_of_user_tokens(
        &self,
        client: &Arc<Provider<Ws>>,
    ) -> Result<(Vec<AaveToken>, f64, f64, f64)> {
        let bps_factor = BigDecimal::from_f64(BPS_FACTOR).unwrap();
        let mut user_token_list: Vec<AaveToken> = Vec::new();

        let mut total_debt = BigDecimal::from(0);
        let mut collateral_times_liquidation_factor = BigDecimal::from(0);

        for r in &self.reserves {
            let Reserve {
                name,
                id,
                symbol,
                decimals,
                reserve_liquidation_bonus,
                reserve_liquidation_threshold,
                usage_as_collateral_enabled,
            } = r.reserve.clone();

            let token_address = extract_first_address(&id).unwrap().to_string();

            let token = Erc20Token {
                name,
                symbol,
                decimals,
                address: token_address,
                liquidation_bonus: u16::from_str(&reserve_liquidation_bonus).unwrap(),
                liquidation_threshold: u16::from_str(&reserve_liquidation_threshold).unwrap(),
                ..Default::default()
            };

            //*******************************************************************************
            // SAVE TOKEN TO GLOBAL STATE
            save_erc20_token(&token, client).await?;

            //*******************************************************************************

            let decimal_factor = BigDecimal::from_u64(10_u64.pow(decimals.into())).unwrap();
            let current_total_debt =
                BigDecimal::from_str(&r.current_total_debt).unwrap() / &decimal_factor;
            let current_atoken_balance =
                &BigDecimal::from_str(&r.current_atoken_balance).unwrap() / &decimal_factor;
            let reserve_liquidation_threshold =
                BigDecimal::from_str(&r.reserve.reserve_liquidation_threshold).unwrap();
            let reserve_liquidation_bonus =
                BigDecimal::from_str(&r.reserve.reserve_liquidation_bonus).unwrap();
            // let usage_as_collateral_enabled = r.reserve.usage_as_collateral_enabled;

            // update meta data
            total_debt += &current_total_debt;
            if usage_as_collateral_enabled && current_atoken_balance > BigDecimal::from(0) {
                collateral_times_liquidation_factor +=
                    &current_atoken_balance / &bps_factor * &reserve_liquidation_threshold
            }

            // get debt, colladeral, liquidation threshold, bonus, and usage colladeral boolean
            user_token_list.push(AaveToken {
                token,
                current_total_debt: current_total_debt.to_f64().unwrap(),
                usage_as_collateral_enabled,
                current_atoken_balance: current_atoken_balance.to_f64().unwrap(),
                reserve_liquidation_threshold: reserve_liquidation_threshold.to_f64().unwrap()
                    / BPS_FACTOR as f64,
                reserve_liquidation_bonus: reserve_liquidation_bonus.to_f64().unwrap()
                    / BPS_FACTOR as f64,
            })
        }
        // calculate health factor
        let health_factor = &collateral_times_liquidation_factor / &total_debt;
        Ok((
            user_token_list,
            total_debt.to_f64().unwrap(),
            collateral_times_liquidation_factor.to_f64().unwrap(),
            health_factor.to_f64().unwrap(),
        ))
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reserve {
    id: String,
    name: String,
    symbol: String,
    decimals: u8,
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

pub async fn get_aave_v3_users() -> Result<Vec<AaveUser>> {
    let (thegraph_url, query) = get_graphql_url_and_query(SampleSize::SmallBatch);

    let client = Client::new();
    let response = client
        .post(&thegraph_url)
        .header(header::CONTENT_TYPE, "application/json")
        .json(&serde_json::json!({ "query": &query }))
        .send()
        .await?;

    let raw_response = response.text().await?;

    let response: Response = serde_json::from_str(&raw_response)?;

    if let Some(errors) = response.errors {
        for error in errors {
            error!("Error: {:?}", error);
        }
        return Err(anyhow!("GraphQL errors occurred"));
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

pub async fn get_all_aave_v3_users() -> Result<Vec<AaveUser>> {
    let client = Client::new();
    let mut all_users = Vec::<AaveUser>::new();
    let mut id_cursor = "0".to_string();

    let (thegraph_url, query) = get_graphql_url_and_query(SampleSize::All);

    loop {
        let response = client
            .post(&thegraph_url)
            .header(header::CONTENT_TYPE, "application/json")
            .json(&serde_json::json!({
            "query": &query,
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
            return Err(anyhow!("GraphQL errors occurred"));
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
                    id_cursor.clone_from(&last_element.id);
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

pub fn get_graphql_url_and_query(sample_size: SampleSize) -> (String, String) {
    let api_key = env::var("THEGRAPH_API_KEY").expect("API_KEY not found in .env file");
    let root_url = env::var("THEGRAPH_ROOT_URL").expect("THEGRAPH_ROOT_URL not found in .env file");
    let subgraph_id = env::var("PROTOCOL_V3_SUBGRAPH_ID")
        .expect("PROTOCOL_V3_SUBGRAPH_ID not found in .env file");
    let thegraph_url = format!("{}/api/{}/subgraphs/id/{}", root_url, api_key, subgraph_id);

    // TODO - add decimals will use chainlink contract methods get price feed and aggregator address
    let query = match sample_size {
        SampleSize::All => {
            r#"
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
            decimals
            usageAsCollateralEnabled
            reserveLiquidationThreshold
            reserveLiquidationBonus
            price {
              priceInEth
            }
          }
        }
      }
    }"#
        }
        SampleSize::SmallBatch => {
            r#"
    { 
     users(first: 1000, where: {borrowedReservesCount_gt: 0}) {
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
            decimals
            usageAsCollateralEnabled
            reserveLiquidationThreshold
            reserveLiquidationBonus
            price {
              priceInEth
            }
          }
        }
      }
    }"#
        }
    };

    (thegraph_url.to_string(), query.to_string())
}

fn extract_first_address(addresses: &str) -> Option<&str> {
    if addresses.len() >= 42 {
        // Check if string is at least as long as one Ethereum address
        Some(&addresses[..42]) // Extract the first 42 characters, which should be the first address
    } else {
        None // Return None if string is too short
    }
}
