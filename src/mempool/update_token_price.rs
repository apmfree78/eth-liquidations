use crate::data::erc20::{Convert, Erc20Token};
use crate::data::token_price_hash::set_saved_token_price;
use eyre::Result;
use std::sync::Arc;

use ethers::providers::{Provider, Ws};

pub async fn update_token_price_for_(
    token: &Erc20Token,
    client: &Arc<Provider<Ws>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let token_price = token.get_token_price_in_("USDC", client).await?;
    set_saved_token_price(token.address, token_price).await?;
    Ok(())
}
