use ethers::abi::Address;
use futures::lock::Mutex;
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::sync::Arc;

pub static USERS_TO_TRACK: Lazy<Arc<Mutex<HashSet<Address>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashSet::<Address>::new())));

pub async fn get_tracked_users() -> Result<HashSet<Address>, Box<dyn std::error::Error>> {
    let users_hashset = Arc::clone(&USERS_TO_TRACK);
    let users = users_hashset.lock().await;

    Ok(users.clone())
}

pub async fn add_tracked_users(
    users_to_add: Vec<Address>,
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
