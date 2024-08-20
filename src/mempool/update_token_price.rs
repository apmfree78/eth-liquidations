use crate::data::erc20::{Convert, Erc20Token};
use crate::data::token_data_hash::get_token_connected_to_eth;
use crate::data::token_price_hash::set_saved_token_price;
use eyre::Result;
use log::info;
use std::sync::Arc;

use ethers::providers::{Provider, Ws};

pub async fn update_token_price_for_(
    token_to_update: &Erc20Token,
    client: &Arc<Provider<Ws>>,
) -> Result<(), Box<dyn std::error::Error>> {
    if token_to_update.symbol == "WETH" {
        let tokens_with_price_connected_to_eth = get_token_connected_to_eth().await?;
        // when ETH price changes must update prices for all these tokens
        for token in tokens_with_price_connected_to_eth.values() {
            info!("price updated for {} => {}", token.name, token.symbol);
            let original_token_price = token.get_saved_price_from_token_price_hash().await?;

            // use UNISWAP to get real time price for token
            let token_price = token.get_token_price_in_("USDC", client).await?;
            set_saved_token_price(&token.address, token_price).await?;
            let update_token_price = token.get_saved_price_from_token_price_hash().await?;
            info!(
                "price updated for {} => {}",
                token_to_update.name, token_to_update.symbol
            );
            info!(
                "changed from {} => {}",
                original_token_price.with_scale(3),
                update_token_price.with_scale(3)
            );
        }
    } else {
        let original_token_price = token_to_update
            .get_saved_price_from_token_price_hash()
            .await?;

        // use UNISWAP to get real time price for token
        let token_price = token_to_update.get_token_price_in_("USDC", client).await?;
        set_saved_token_price(&token_to_update.address, token_price).await?;
        let update_token_price = token_to_update
            .get_saved_price_from_token_price_hash()
            .await?;
        info!(
            "price updated for {} => {}",
            token_to_update.name, token_to_update.symbol
        );

        info!(
            "changed from {} => {}",
            original_token_price.with_scale(3),
            update_token_price.with_scale(3)
        );
    }
    Ok(())
}
