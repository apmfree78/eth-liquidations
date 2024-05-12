use super::events::{AaveEvent, AaveUserAction, AaveUserEvent};
use super::user_data::{AaveToken, AaveUserData};
use crate::data::erc20::{address_to_string, u256_to_big_decimal, TOKEN_DATA};
use bigdecimal::BigDecimal;
use core::panic;

pub fn get_user_action_from_event(event: Box<dyn AaveEvent>) -> AaveUserAction {
    let token_address = event.get_reserve();
    let token_address = address_to_string(token_address);
    let token;
    if let Some(_token) = TOKEN_DATA.get(token_address.trim()) {
        token = _token;
    } else {
        panic!("No token found for address: {}", token_address)
    }
    let amount = event.get_amount();
    let amount = u256_to_big_decimal(&amount);

    return AaveUserAction {
        user_event: event.get_type(),
        user_address: event.get_user(),
        token: *token,
        amount_transferred: amount,
        use_a_tokens: event.get_use_a_tokens(),
    };
}

pub trait Update {
    fn update(&mut self, aave_action: &AaveUserAction) -> Result<(), Box<dyn std::error::Error>>;
}

impl Update for AaveUserData {
    fn update(&mut self, aave_action: &AaveUserAction) -> Result<(), Box<dyn std::error::Error>> {
        let token_address = &aave_action.token.address;
        match aave_action.user_event {
            AaveUserEvent::WithDraw => {
                for token in &mut self.tokens {
                    if token.token.address == *token_address {
                        // update
                        token.current_atoken_balance -= aave_action.amount_transferred.clone();
                        return Ok(());
                    };
                }
            }
            AaveUserEvent::Borrow => {
                // find token in aave user data
                for token in &mut self.tokens {
                    if token.token.address == *token_address {
                        // update
                        token.current_total_debt += aave_action.amount_transferred.clone();
                        return Ok(());
                    };
                }

                // token does not exist , add it
                let token = TOKEN_DATA.get(*token_address).unwrap();
                self.tokens.push(AaveToken {
                    token: *token,
                    current_total_debt: aave_action.amount_transferred.clone(),
                    usage_as_collateral_enabled: false,
                    current_atoken_balance: BigDecimal::from(0),
                    reserve_liquidation_threshold: BigDecimal::from(token.liquidation_threshold),
                    reserve_liquidation_bonus: BigDecimal::from(token.liquidation_bonus),
                })
            }
            AaveUserEvent::Repay => {
                // find token in aave user data
                for token in &mut self.tokens {
                    if token.token.address == *token_address {
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
                        return Ok(());
                    };
                }
            }
            AaveUserEvent::Supply => {
                for token in &mut self.tokens {
                    if token.token.address == *token_address {
                        // update
                        token.current_atoken_balance += aave_action.amount_transferred.clone();
                        return Ok(());
                    };
                }
                // token does not exist , add it
                let token = TOKEN_DATA.get(*token_address).unwrap();
                self.tokens.push(AaveToken {
                    token: *token,
                    current_total_debt: BigDecimal::from(0),
                    usage_as_collateral_enabled: false,
                    current_atoken_balance: aave_action.amount_transferred.clone(),
                    reserve_liquidation_threshold: BigDecimal::from(token.liquidation_threshold),
                    reserve_liquidation_bonus: BigDecimal::from(token.liquidation_bonus),
                })
            }
            AaveUserEvent::ReserveUsedAsCollateralEnabled => {
                // find token in aave user data
                for token in &mut self.tokens {
                    if token.token.address == *token_address {
                        // update
                        token.usage_as_collateral_enabled = true;
                        return Ok(());
                    };
                }
            }
            AaveUserEvent::ReserveUsedAsCollateralDisabled => {
                // find token in aave user data
                for token in &mut self.tokens {
                    if token.token.address == *token_address {
                        // update
                        token.usage_as_collateral_enabled = false;
                        return Ok(());
                    };
                }
            }
            _ => {}
        }
        Ok(())
    }
}
