use super::events::{AaveEvent, AaveUserAction, AaveUserEvent, LiquidationEvent};
use super::user_structs::{AaveToken, AaveUserData};
use crate::data::erc20::{u256_to_big_decimal, TOKEN_DATA};
use crate::utils::type_conversion::address_to_string;
use bigdecimal::BigDecimal;
use core::panic;

pub enum TokenToRemove {
    TokenToRemove(String),
    None,
}

pub fn get_user_action_from_event(event: Box<dyn AaveEvent>) -> AaveUserAction {
    let token_address = event.get_reserve();
    let token_address = address_to_string(token_address);

    let token = TOKEN_DATA
        .get(token_address.trim())
        .unwrap_or_else(|| panic!("No token found for address: {}", token_address));
    let amount = event.get_amount();
    let amount = u256_to_big_decimal(&amount);

    AaveUserAction {
        user_event: event.get_type(),
        user_address: event.get_user(),
        token: *token,
        amount_transferred: amount,
        use_a_tokens: event.get_use_a_tokens(),
    }
}

pub trait Update {
    fn update(
        &mut self,
        aave_action: &AaveUserAction,
    ) -> Result<TokenToRemove, Box<dyn std::error::Error>>;

    fn liquidate(&mut self, event: LiquidationEvent) -> Result<(), Box<dyn std::error::Error>>;
}

impl Update for AaveUserData {
    fn liquidate(&mut self, event: LiquidationEvent) -> Result<(), Box<dyn std::error::Error>> {
        for token in self.tokens.iter_mut() {
            println!("checking token {}", token.token.symbol);
            if token.token.address.to_lowercase() == event.get_collateral_token_address() {
                // update collateral
                println!("collateral token {}", token.token.symbol);

                token.current_atoken_balance =
                    &token.current_atoken_balance - &event.get_collateral_liquidated();
            } else if token.token.address.to_lowercase() == event.get_debt_token_address() {
                // update debt
                println!("debt token {}", token.token.symbol);
                token.current_total_debt =
                    &token.current_total_debt - &event.get_amount_debt_reduced();
            }
        }

        Ok(())
    }

    fn update(
        &mut self,
        aave_action: &AaveUserAction,
    ) -> Result<TokenToRemove, Box<dyn std::error::Error>> {
        let token_address = aave_action.token.address.to_lowercase();
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
                if let Some(token) = TOKEN_DATA.get(&token_address) {
                    self.tokens.push(AaveToken {
                        token: *token,
                        current_total_debt: aave_action.amount_transferred.clone(),
                        usage_as_collateral_enabled: false,
                        current_atoken_balance: BigDecimal::from(0),
                        reserve_liquidation_threshold: BigDecimal::from(
                            token.liquidation_threshold,
                        ),
                        reserve_liquidation_bonus: BigDecimal::from(token.liquidation_bonus),
                    });
                } else {
                    panic!(
                        "token address provided is invalid or doesn't exist! => {}",
                        token_address
                    );
                }
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
                if let Some(token) = TOKEN_DATA.get(&token_address) {
                    self.tokens.push(AaveToken {
                        token: *token,
                        current_total_debt: BigDecimal::from(0),
                        usage_as_collateral_enabled: false,
                        current_atoken_balance: aave_action.amount_transferred.clone(),
                        reserve_liquidation_threshold: BigDecimal::from(
                            token.liquidation_threshold,
                        ),
                        reserve_liquidation_bonus: BigDecimal::from(token.liquidation_bonus),
                    });
                } else {
                    panic!(
                        "token address provided is invalid or doesn't exist! => {}",
                        token_address
                    );
                }
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
