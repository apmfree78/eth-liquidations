use super::events::{AaveEvent, AaveUserAction, AaveUserEvent, LiquidationEvent};
use super::user_structs::{AaveToken, AaveUserData};
use crate::data::erc20::u256_to_big_decimal;
use crate::data::token_data_hash::{get_and_save_erc20_by_token_address, get_token_data};
use crate::utils::type_conversion::address_to_string;
use anyhow::Result;
use async_trait::async_trait;
use bigdecimal::BigDecimal;
use ethers::providers::{Provider, Ws};
use log::debug;
use std::sync::Arc;

pub enum TokenToRemove {
    TokenToRemove(String),
    None,
}

pub async fn get_user_action_from_event(
    event: Box<dyn AaveEvent>,
    client: &Arc<Provider<Ws>>,
) -> Result<AaveUserAction> {
    let token_address = event.get_reserve();
    let token_address = address_to_string(token_address);
    let token_data = get_token_data().await?;

    // token does not exist , add it
    let token = match token_data.get(&token_address) {
        Some(token) => token.clone(),
        None => {
            let token = get_and_save_erc20_by_token_address(&token_address, client).await?;
            token.clone()
        }
    };

    let amount = event.get_amount();
    let amount = u256_to_big_decimal(&amount);

    Ok(AaveUserAction {
        user_event: event.get_type(),
        user_address: event.get_user(),
        token: token.clone(),
        amount_transferred: amount,
        use_a_tokens: event.get_use_a_tokens(),
    })
}

#[async_trait]
pub trait Update {
    async fn update(
        &mut self,
        aave_action: &AaveUserAction,
        client: &Arc<Provider<Ws>>,
    ) -> Result<TokenToRemove>;

    fn liquidate(&mut self, event: LiquidationEvent) -> Result<()>;
}

#[async_trait]
impl Update for AaveUserData {
    fn liquidate(&mut self, event: LiquidationEvent) -> Result<()> {
        for token in self.tokens.iter_mut() {
            debug!("checking token {}", token.token.symbol);
            if token.token.address.to_lowercase() == event.get_collateral_token_address() {
                // update collateral
                debug!("collateral token {}", token.token.symbol);

                token.current_atoken_balance =
                    &token.current_atoken_balance - &event.get_collateral_liquidated();
            } else if token.token.address.to_lowercase() == event.get_debt_token_address() {
                // update debt
                debug!("debt token {}", token.token.symbol);
                token.current_total_debt =
                    &token.current_total_debt - &event.get_amount_debt_reduced();
            }
        }

        Ok(())
    }

    async fn update(
        &mut self,
        aave_action: &AaveUserAction,
        client: &Arc<Provider<Ws>>,
    ) -> Result<TokenToRemove> {
        let token_address = aave_action.token.address.to_lowercase();
        let token_data = get_token_data().await?;
        let mut token_index: Option<usize> = None;
        match aave_action.user_event {
            AaveUserEvent::WithDraw => {
                for (index, token) in self.tokens.iter_mut().enumerate() {
                    if token.token.address.to_lowercase() == token_address {
                        // update
                        token.current_atoken_balance -= aave_action.amount_transferred.clone();

                        // if token has no debt or a token balance then remove
                        if token.current_total_debt == BigDecimal::from(0)
                            && token.current_atoken_balance == BigDecimal::from(0)
                        {
                            token_index = Some(index);
                            break;
                        } else {
                            return Ok(TokenToRemove::None);
                        }
                    };
                }

                // if  no  debt or a token balance then  remove token
                if let Some(index) = token_index {
                    self.tokens.remove(index);
                    return Ok(TokenToRemove::TokenToRemove(token_address));
                }
                return Ok(TokenToRemove::None);
            }
            AaveUserEvent::Borrow => {
                // find token in aave user data
                for token in &mut self.tokens {
                    if token.token.address.to_lowercase() == token_address {
                        // update
                        token.current_total_debt += aave_action.amount_transferred.clone();
                        return Ok(TokenToRemove::None);
                    };
                }

                // token does not exist , add it
                let new_token = match token_data.get(&token_address) {
                    Some(token) => token.clone(),
                    None => {
                        let token =
                            get_and_save_erc20_by_token_address(&token_address, client).await?;
                        token.clone()
                    }
                };

                self.tokens.push(AaveToken {
                    token: new_token.clone(),
                    current_total_debt: aave_action.amount_transferred.clone(),
                    usage_as_collateral_enabled: false,
                    current_atoken_balance: BigDecimal::from(0),
                    reserve_liquidation_threshold: BigDecimal::from(
                        new_token.liquidation_threshold,
                    ),
                    reserve_liquidation_bonus: BigDecimal::from(new_token.liquidation_bonus),
                });
            }
            AaveUserEvent::Repay => {
                // find token in aave user data
                for (index, token) in self.tokens.iter_mut().enumerate() {
                    if token.token.address.to_lowercase() == token_address {
                        // update
                        token.current_total_debt -= aave_action.amount_transferred.clone();

                        // if using a token to pay back debt , subtract debt from a token balance
                        if aave_action.use_a_tokens {
                            if token.current_atoken_balance > aave_action.amount_transferred.clone()
                            {
                                token.current_atoken_balance -=
                                    aave_action.amount_transferred.clone();
                            } else {
                                token.current_atoken_balance = BigDecimal::from(0)
                            }
                        }

                        // if token has no debt or a token balance then remove
                        if token.current_total_debt == BigDecimal::from(0)
                            && token.current_atoken_balance == BigDecimal::from(0)
                        {
                            token_index = Some(index);
                            break;
                        } else {
                            return Ok(TokenToRemove::None);
                        }
                    };
                }

                // if  no  debt or a token balance then  remove token
                if let Some(index) = token_index {
                    self.tokens.remove(index);
                    return Ok(TokenToRemove::TokenToRemove(token_address));
                }
                return Ok(TokenToRemove::None);
            }
            AaveUserEvent::Supply => {
                for token in &mut self.tokens {
                    if token.token.address.to_lowercase() == token_address {
                        // update
                        token.current_atoken_balance += aave_action.amount_transferred.clone();
                        return Ok(TokenToRemove::None);
                    };
                }

                // token does not exist , add it
                let new_token = match token_data.get(&token_address) {
                    Some(token) => token.clone(),
                    None => {
                        let token =
                            get_and_save_erc20_by_token_address(&token_address, client).await?;
                        token.clone()
                    }
                };

                self.tokens.push(AaveToken {
                    token: new_token.clone(),
                    current_total_debt: BigDecimal::from(0),
                    usage_as_collateral_enabled: false,
                    current_atoken_balance: aave_action.amount_transferred.clone(),
                    reserve_liquidation_threshold: BigDecimal::from(
                        new_token.liquidation_threshold,
                    ),
                    reserve_liquidation_bonus: BigDecimal::from(new_token.liquidation_bonus),
                });
            }
            AaveUserEvent::ReserveUsedAsCollateralEnabled => {
                // find token in aave user data
                for token in &mut self.tokens {
                    if token.token.address.to_lowercase() == token_address {
                        // update
                        token.usage_as_collateral_enabled = true;
                        return Ok(TokenToRemove::None);
                    };
                }
            }
            AaveUserEvent::ReserveUsedAsCollateralDisabled => {
                // find token in aave user data
                for token in &mut self.tokens {
                    if token.token.address.to_lowercase() == token_address {
                        // update
                        token.usage_as_collateral_enabled = false;
                        return Ok(TokenToRemove::None);
                    };
                }
            }
            _ => {}
        }
        Ok(TokenToRemove::None)
    }
}
