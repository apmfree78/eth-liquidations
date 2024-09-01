use crate::abi::wsteth::WST_ETH;
use crate::data::erc20::{u256_to_big_decimal, Convert, Erc20Token};
use crate::data::token_data_hash::{
    get_token_data, get_tokens_priced_in_btc, get_tokens_priced_in_eth,
};
use crate::data::token_price_hash::{get_saved_token_price, set_saved_token_price};
use bigdecimal::BigDecimal;
use ethers::providers::{Provider, Ws};
use ethers::types::{Address, U256};
use eyre::Result;
use log::{debug, info};
use std::sync::Arc;

pub async fn update_token_price_for_(
    token_to_update: &Erc20Token,
    new_token_price: BigDecimal,
    client: &Arc<Provider<Ws>>,
) -> Result<(), Box<dyn std::error::Error>> {
    if token_to_update.symbol == "WETH" {
        let tokens_with_price_priced_in_eth = get_tokens_priced_in_eth().await?;
        let current_eth_price = get_saved_token_price(token_to_update.address.clone()).await?;

        // when ETH price changes must update prices for all these tokens
        for token in tokens_with_price_priced_in_eth.values() {
            if token.symbol == "WETH" {
                continue;
            }

            info!("price updated for {} => {}", token.name, token.symbol);
            let original_token_price = token.get_saved_price_from_token_price_hash().await?;

            // use UNISWAP to get real time price for token
            let priced_in_eth_price = get_saved_token_price(token.address.clone()).await?;
            let new_priced_in_eth_price =
                &priced_in_eth_price * &new_token_price / &current_eth_price; // adjust price based on new price of ETH

            set_saved_token_price(&token.address, new_priced_in_eth_price).await?;
            let update_token_price = token.get_saved_price_from_token_price_hash().await?;
            info!(
                "changed from {} => {}",
                original_token_price.with_scale(3),
                update_token_price.with_scale(3)
            );
        }

        // now set new price of ETH (in this case WETH which is 1:1 with ETH price)
        set_saved_token_price(&token_to_update.address, new_token_price.clone()).await?;
        info!(
            "price updated for {} => {}",
            token_to_update.name, token_to_update.symbol
        );
        info!(
            "changed from {} => {}",
            current_eth_price.with_scale(3),
            new_token_price.with_scale(3)
        );
    } else if token_to_update.symbol == "BTC" {
        let tokens_with_price_priced_in_btc = get_tokens_priced_in_btc().await?;
        let current_btc_price = get_saved_token_price(token_to_update.address.clone()).await?;

        // when ETH price changes must update prices for all these tokens
        for token in tokens_with_price_priced_in_btc.values() {
            info!("price updated for {} => {}", token.name, token.symbol);
            let original_token_price = token.get_saved_price_from_token_price_hash().await?;

            // use UNISWAP to get real time price for token
            let priced_inken_price = get_saved_token_price(token.address.clone()).await?;
            let new_priced_inken_price =
                &priced_inken_price * &new_token_price / &current_btc_price; // adjust price based on new price of ETH

            set_saved_token_price(&token.address, new_priced_inken_price).await?;
            let update_token_price = token.get_saved_price_from_token_price_hash().await?;
            info!("price updated for {} => {}", token.name, token.symbol);
            info!(
                "changed from {} => {}",
                original_token_price.with_scale(3),
                update_token_price.with_scale(3)
            );
        }

        // now set new price of BTC
        set_saved_token_price(&token_to_update.address, new_token_price.clone()).await?;
        info!(
            "price updated for {} => {}",
            token_to_update.name, token_to_update.symbol
        );
        info!(
            "changed from {} => {}",
            current_btc_price.with_scale(3),
            new_token_price.with_scale(3)
        );
    } else {
        let original_token_price = token_to_update
            .get_saved_price_from_token_price_hash()
            .await?;

        set_saved_token_price(&token_to_update.address, new_token_price).await?;
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

        // EDGE CASE FOR wstETH price, which is linked to stETH price
        if token_to_update.symbol == "stETH" {
            debug!("updating wstETH price");
            let unit_scale = U256::from(10u64.pow(18));

            let token_data = get_token_data().await?;
            let wsteth_token = token_data.get("wstETH").unwrap();
            let wsteth_address: Address = wsteth_token.address.parse()?;
            let wsteth_contract = WST_ETH::new(wsteth_address, client.clone());

            // calculate exchange rate
            let st_to_wst_exchange_rate = wsteth_contract
                .get_wst_eth_by_st_eth(unit_scale)
                .call()
                .await?;

            debug!("exchange rate for st to wst => {}", st_to_wst_exchange_rate);

            let st_to_wst_exchange_rate =
                u256_to_big_decimal(&st_to_wst_exchange_rate) / BigDecimal::from(10u64.pow(18));

            let original_wsteth_token_price =
                wsteth_token.get_saved_price_from_token_price_hash().await?;

            // wstETH price is stETH price TIMES exchange rate
            let new_wseth_token_price = &update_token_price * &st_to_wst_exchange_rate;

            // update price !
            set_saved_token_price(&wsteth_token.address, new_wseth_token_price.clone()).await?;

            info!(
                "wsteth price changed from {} => {}",
                original_wsteth_token_price.with_scale(3),
                new_wseth_token_price.with_scale(3)
            );
        }
    }
    Ok(())
}
