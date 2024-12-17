use {
    solvers_dto::auction::{Auction, Liquidity},
    solvers_dto::solution::{Solution, Trade, Fulfillment, Interaction, LiquidityInteraction, Call},
    web3::types::{H160, U256},
    std::collections::HashMap,
    crate::structures::pools::{Pool, Pools},
    crate::utils::token_swap::get_direct_swap_pool,
};

use std::str::FromStr;
use bigdecimal::{BigDecimal, ToPrimitive, FromPrimitive, Signed, Zero};

/// Converts a BigDecimal to U256 by **truncating** any fractional part.
///
/// Panics if the number is negative or too large for U256.
fn bigdecimal_to_u256(value: &BigDecimal) -> U256 {
    // Ensure it's non-negative
    if value.is_negative() {
        panic!("Cannot convert negative BigDecimal to U256");
    }

    // Truncate the decimal part by converting to string without fractional digits
    let truncated = value.with_scale(0); // floor for positive values

    // Convert string representation to U256
    U256::from_dec_str(&truncated.to_string()).expect("Value too large for U256")
}

fn calculate_reserves(pool: &Pool) -> (BigDecimal, BigDecimal) {
    // For example, assume these attributes are already BigDecimal fields.
    let reserve_in_usd = &pool.attributes.reserve_in_usd;
    let base_token_price_usd = &pool.attributes.base_token_price_usd;
    let base_token_price_quote_token = &pool.attributes.base_token_price_quote_token;
    let quote_token_price_usd = &pool.attributes.quote_token_price_usd;

    // x = reserve_in_usd / (base_token_price_usd + base_token_price_quote_token * quote_token_price_usd)
    let denominator = base_token_price_usd + (base_token_price_quote_token * quote_token_price_usd);
    let x = reserve_in_usd / &denominator;

    // y = base_token_price_quote_token * x
    let y = base_token_price_quote_token * &x;

    let bDx = BigDecimal::from_f64(x).unwrap_or(BigDecimal::zero());
    let bDy = BigDecimal::from_f64(y).unwrap_or(BigDecimal::zero());
    (bDx, bDy)
}

/*
    Solves the auction using the best price UniSwap pool
*/
pub async fn fill_with_uniswap_liquidity(auction: Auction) -> Solution {
    // Initialize solution
    let mut solution = Solution {
        id: auction.id.unwrap_or_default() as u64,
        prices: HashMap::new(),
        trades: Vec::new(),
        pre_interactions: Vec::new(),
        interactions: Vec::new(),
        post_interactions: Vec::new(),
        gas: None,
    };

    let mut cumulative_gas: U256 = U256::zero();

    for order in &auction.orders {
        let mut fulfilled = false;

        let str_sell_token = format!("{:#x}", &order.sell_token);
        let str_buy_token = format!("{:#x}", &order.buy_token);

        // Try to get pool from a direct swap
        if let Ok(best_pool) = get_direct_swap_pool(&str_sell_token, &str_buy_token).await {
            // We'll interpret pool.attributes.* as BigDecimal
            // If they're not BigDecimal, parse them here:
            let base_token_price_quote_token = BigDecimal::from_f64(best_pool.attributes.base_token_price_quote_token)
                .unwrap_or(BigDecimal::zero());
            let quote_token_price_base_token = BigDecimal::from_f64(best_pool.attributes.quote_token_price_base_token)
                .unwrap_or(BigDecimal::zero());
            let base_token_price_usd = BigDecimal::from_f64(best_pool.attributes.base_token_price_usd)
                .unwrap_or(BigDecimal::zero());
            let quote_token_price_usd = BigDecimal::from_f64(best_pool.attributes.quote_token_price_usd)
                .unwrap_or(BigDecimal::zero());

            // If buyToken is the baseToken
            if best_pool.relationships.base_token.data.id.eq_ignore_ascii_case(&format!("eth_{}", &order.buy_token))
            {
                // Convert U256 sell_amount/buy_amount into BigDecimal
                let order_buy_amount = BigDecimal::from(order.buy_amount.as_u128());
                let order_sell_amount = BigDecimal::from(order.sell_amount.as_u128());

                // Condition: buy_amount * price_quote_token <= sell_amount ?
                let lhs = &order_buy_amount * &base_token_price_quote_token;
                let rhs = &order_sell_amount;

                let max_buy_amt = if &lhs <= rhs {
                    // We pick the full buy_amount
                    order.buy_amount.as_u128()
                } else {
                    // We pick sell_amount * quote_token_price_base_token
                    // (both must end as BigDecimal before converting to u128)
                    let calc = BigDecimal::from(order.sell_amount.as_u128()) * &quote_token_price_base_token;
                    calc.to_u128().unwrap_or(u128::MAX)
                };

                // Similarly compute max_sell_amt = max_buy_amt * base_token_price_quote_token
                // But we do it in BigDecimal first:
                let max_buy_amt_dec = BigDecimal::from(max_buy_amt);
                let max_sell_amt_dec = &max_buy_amt_dec * &base_token_price_quote_token;

                // Calculate reserves in BigDecimal
                let (buy_liquidity, sell_liquidity) = calculate_reserves(&best_pool); 
                // buy_liquidity and sell_liquidity are BigDecimal

                // Check if there's enough liquidity
                if buy_liquidity >= max_buy_amt_dec && sell_liquidity >= max_sell_amt_dec {
                    // Fee = abs( (max_buy_amt * base_token_price_usd) - (max_sell_amt * quote_token_price_usd ) )
                    let fee_calc = (&max_buy_amt_dec * &base_token_price_usd)
                        - (&max_sell_amt_dec * &quote_token_price_usd);
                    let expected_fee_dec = fee_calc.abs();

                    // Convert results to U256
                    let executed_amount_u256 = bigdecimal_to_u256(&max_sell_amt_dec);
                    let fee_u256 = Some(bigdecimal_to_u256(&expected_fee_dec));

                    // Build trade
                    let trade = Trade::Fulfillment(Fulfillment {
                        order: order.uid,
                        executed_amount: executed_amount_u256,
                        fee: fee_u256,
                    });

                    solution.trades.push(trade);
                    cumulative_gas += U256::one(); // Stub for gas cost

                    // Interaction amounts
                    let input_amount_u256 = bigdecimal_to_u256(&max_buy_amt_dec);
                    let output_amount_u256 = executed_amount_u256;

                    let interaction = Interaction::Liquidity(LiquidityInteraction {
                        internalize: false,
                        id: best_pool.id,
                        input_token: order.sell_token,
                        output_token: order.buy_token,
                        input_amount: input_amount_u256,
                        output_amount: output_amount_u256,
                    });

                    solution.interactions.push(interaction);

                    // Copy over pre_interactions
                    for pre_interaction in &order.pre_interactions {
                        solution.pre_interactions.push(Call {
                            target: pre_interaction.target,
                            value: pre_interaction.value,
                            calldata: pre_interaction.call_data.clone(),
                        });
                    }

                    // Copy over post_interactions
                    for post_interaction in &order.post_interactions {
                        solution.post_interactions.push(Call {
                            target: post_interaction.target,
                            value: post_interaction.value,
                            calldata: post_interaction.call_data.clone(),
                        });
                    }

                    fulfilled = true;
                }
            }
        } else {
            let hex_uid: String = order.uid.iter().map(|byte| format!("{:02x}", byte)).collect();
            println!("Could not find direct swap for order '0x{}'", hex_uid);
        }

        if !fulfilled {
            let hex_uid: String = order.uid.iter().map(|byte| format!("{:02x}", byte)).collect();
            println!("Order '0x{}' not filled", hex_uid);
            continue;
        }
    }

    // Set gas
    solution.gas = Some(cumulative_gas.as_u64());
    solution
}