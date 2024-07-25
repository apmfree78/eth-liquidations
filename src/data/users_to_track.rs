use futures::lock::Mutex;
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::sync::Arc;

use crate::exchanges::aave_v3::user_structs::LiquidationCandidate;

pub static USERS_TO_TRACK: Lazy<Arc<Mutex<HashSet<LiquidationCandidate>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashSet::<LiquidationCandidate>::new())));

pub async fn get_tracked_users() -> Result<HashSet<LiquidationCandidate>, Box<dyn std::error::Error>>
{
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
