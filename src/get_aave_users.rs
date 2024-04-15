use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use serde_json;
use tokio;
#[derive(Debug, Serialize, Deserialize)]
pub struct AaveUser {
    pub id: String,
    #[serde(rename = "borrowedReservesCount")]
    borrowed_reserves_count: i64,
    reserves: Vec<UserReserve>,
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
