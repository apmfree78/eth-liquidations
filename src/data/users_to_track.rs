use futures::lock::Mutex;
use once_cell::sync::Lazy;
use std::sync::Arc;

// TODO - change Vec to Vec!!
use crate::exchanges::aave_v3::user_structs::LiquidationCandidate;

pub static USERS_TO_TRACK: Lazy<Arc<Mutex<Vec<LiquidationCandidate>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Vec::<LiquidationCandidate>::new())));

pub async fn get_tracked_users() -> Result<Vec<LiquidationCandidate>, Box<dyn std::error::Error>> {
    let users_hashset = Arc::clone(&USERS_TO_TRACK);
    let users = users_hashset.lock().await;

    Ok(users.clone())
}

pub async fn add_tracked_users(
    users_to_add: Vec<LiquidationCandidate>,
) -> Result<(), Box<dyn std::error::Error>> {
    let users_hashset = Arc::clone(&USERS_TO_TRACK);
    let mut users = users_hashset.lock().await;
    users.extend(users_to_add);
    Ok(())
}

pub async fn reset_tracked_users() -> Result<(), Box<dyn std::error::Error>> {
    let users_hashset = Arc::clone(&USERS_TO_TRACK);
    let mut users = users_hashset.lock().await;

    users.clear();
    Ok(())
}
