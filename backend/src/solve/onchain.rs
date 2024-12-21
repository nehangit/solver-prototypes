use {
    crate::structures::edge::Edge,
    crate::structures::gas_price::{GasOracleResponse, GasPrices},
    crate::structures::pools::{Pool, Pools},
    crate::utils::token_swap::get_direct_swap_pool,
    solvers_dto::auction::{Auction, Liquidity},
    solvers_dto::solution::{
        Call, Fulfillment, Interaction, LiquidityInteraction, Solution, Trade,
    },
    std::collections::HashMap,
    web3::types::{H160, U256},
};

use bigdecimal::{BigDecimal, FromPrimitive, Signed, ToPrimitive, Zero};
use std::str::FromStr;

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

    (x, y)
}

/*
    Solves the auction using the best price UniSwap pool
*/
pub async fn fill_with_uniswap_liquidity(
    auction: Auction,
    gas_prices: &GasPrices,
) -> (Solution, Vec<Edge>) {
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
    // Initialize vector of directed edges (from, to)
    let mut graph_edges: Vec<Edge> = Vec::new();

    for order in &auction.orders {
        let mut fulfilled = false;

        let str_sell_token = format!("{:#x}", &order.sell_token);
        let str_buy_token = format!("{:#x}", &order.buy_token);

        // Try to get pool from a direct swap
        if let Ok(best_pool) = get_direct_swap_pool(&str_sell_token, &str_buy_token).await {
            // We'll interpret pool.attributes.* as BigDecimal
            // If they're not BigDecimal, parse them here:
            let base_token_price_quote_token =
                best_pool.attributes.base_token_price_quote_token.clone();
            let quote_token_price_base_token =
                best_pool.attributes.quote_token_price_base_token.clone();
            let base_token_price_usd = best_pool.attributes.base_token_price_usd.clone();
            let quote_token_price_usd = best_pool.attributes.quote_token_price_usd.clone();

            // If buyToken is the baseToken
            if best_pool
                .relationships
                .base_token
                .data
                .id
                .eq_ignore_ascii_case(&format!("eth_{}", &order.buy_token))
            {
                // Convert U256 sell_amount/buy_amount into BigDecimal
                // let order_buy_amount = BigDecimal::from(order.buy_amount.as_u128());
                let u128_val_buy = order.buy_amount.as_u128();
                let order_buy_amount = BigDecimal::from_str(&u128_val_buy.to_string()).unwrap();

                let u128_val_sell = order.sell_amount.as_u128();
                let order_sell_amount = BigDecimal::from_str(&u128_val_sell.to_string()).unwrap();

                // Condition: buy_amount * price_quote_token <= sell_amount ?
                let lhs = &order_buy_amount * &base_token_price_quote_token;
                let rhs = &order_sell_amount;

                let max_buy_amt = if &lhs <= rhs {
                    // We pick the full buy_amount
                    order.buy_amount.as_u128()
                } else {
                    // We pick sell_amount * quote_token_price_base_token
                    // (both must end as BigDecimal before converting to u128)
                    let u128_val_calc = order.sell_amount.as_u128();
                    let calc = BigDecimal::from_str(&u128_val_calc.to_string()).unwrap()
                        * &quote_token_price_base_token;
                    calc.to_u128().unwrap_or(u128::MAX)
                };

                // Similarly compute max_sell_amt = max_buy_amt * base_token_price_quote_token
                // But we do it in BigDecimal first:
                let u128_val_max_buy = max_buy_amt;
                let max_buy_amt_dec = BigDecimal::from_str(&u128_val_max_buy.to_string()).unwrap();
                let max_sell_amt_dec = &max_buy_amt_dec * &base_token_price_quote_token;

                // Calculate reserves in BigDecimal
                let (buy_liquidity, sell_liquidity) = calculate_reserves(&best_pool);
                // buy_liquidity and sell_liquidity are BigDecimal

                // Check if there's enough liquidity
                if buy_liquidity >= max_buy_amt_dec && sell_liquidity >= max_sell_amt_dec {
                    solution
                        .prices
                        .entry(H160::from_str(&str_buy_token).unwrap_or(H160::zero()))
                        .or_insert(bigdecimal_to_u256(
                            &best_pool.attributes.base_token_price_usd,
                        ));
                    solution
                        .prices
                        .entry(H160::from_str(&str_sell_token).unwrap_or(H160::zero()))
                        .or_insert(bigdecimal_to_u256(
                            &best_pool.attributes.quote_token_price_usd,
                        ));

                    // Fee = abs( (max_buy_amt * base_token_price_usd) - (max_sell_amt * quote_token_price_usd ) )
                    let fee_calc = (&max_buy_amt_dec * &base_token_price_usd)
                        - (&max_sell_amt_dec * &quote_token_price_usd);
                    let expected_fee_dec =
                        fee_calc.abs() / (&max_buy_amt_dec * &quote_token_price_usd);

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
                    cumulative_gas += bigdecimal_to_u256(
                        &(BigDecimal::from(75000) * gas_prices.ProposeGasPrice.clone()),
                    ); // Assume gas used to be around 75000

                    // Interaction amounts
                    let input_amount_u256 = bigdecimal_to_u256(&max_buy_amt_dec);
                    let output_amount_u256 = executed_amount_u256;

                    let interaction = Interaction::Liquidity(LiquidityInteraction {
                        internalize: false,
                        id: best_pool.id.clone(),
                        input_token: order.sell_token,
                        output_token: order.buy_token,
                        input_amount: input_amount_u256,
                        output_amount: output_amount_u256,
                    });

                    solution.interactions.push(interaction);

                    graph_edges.push(Edge {
                        from: h160_to_string(&order.owner),
                        to: best_pool.id.clone(),
                        amount: input_amount_u256,
                        token: order.sell_token,
                    });

                    // if 'receiver' is set (i.e. owner wants to redirect their trade) there should be an edge (pool, receiver)
                    // otherwise make an edge (pool, owner address)
                    if let Some(receiver) = order.receiver {
                        graph_edges.push(Edge {
                            from: best_pool.id.clone(),
                            to: h160_to_string(&receiver),
                            amount: output_amount_u256,
                            token: order.buy_token,
                        });
                    } else {
                        graph_edges.push(Edge {
                            from: best_pool.id.clone(),
                            to: h160_to_string(&order.owner),
                            amount: output_amount_u256,
                            token: order.buy_token,
                        });
                    }

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
            } else {
                // if buyToken is quoteToken
                // Convert U256 sell_amount/buy_amount into BigDecimal

                let u128_val_buy = order.buy_amount.as_u128();
                let order_buy_amount = BigDecimal::from_str(&u128_val_buy.to_string()).unwrap();

                let u128_val_sell = order.sell_amount.as_u128();
                let order_sell_amount = BigDecimal::from_str(&u128_val_sell.to_string()).unwrap();



                // Condition: buy_amount * price_quote_token <= sell_amount ?
                let lhs = &order_buy_amount * &quote_token_price_base_token;
                let rhs = &order_sell_amount;

                let max_buy_amt = if &lhs <= rhs {
                    // We pick the full buy_amount
                    order.buy_amount.as_u128()
                } else {
                    // We pick sell_amount * quote_token_price_base_token
                    // (both must end as BigDecimal before converting to u128)

                    let u128_val_calc = order.sell_amount.as_u128();
                    let calc = BigDecimal::from_str(&u128_val_calc.to_string()).unwrap()
                        * &quote_token_price_base_token;
                    calc.to_u128().unwrap_or(u128::MAX)
                };

                // Similarly compute max_sell_amt = max_buy_amt * base_token_price_quote_token
                // But we do it in BigDecimal first:
                let u128_val_max_buy = max_buy_amt;
                let max_buy_amt_dec = BigDecimal::from_str(&u128_val_max_buy.to_string()).unwrap();
                let max_sell_amt_dec = &max_buy_amt_dec * &quote_token_price_base_token;

                // Calculate reserves in BigDecimal
                let (buy_liquidity, sell_liquidity) = calculate_reserves(&best_pool);
                // buy_liquidity and sell_liquidity are BigDecimal

                // Check if there's enough liquidity
                if buy_liquidity >= max_buy_amt_dec && sell_liquidity >= max_sell_amt_dec {
                    solution
                        .prices
                        .entry(H160::from_str(&str_buy_token).unwrap_or(H160::zero()))
                        .or_insert(bigdecimal_to_u256(
                            &best_pool.attributes.quote_token_price_usd,
                        ));
                    solution
                        .prices
                        .entry(H160::from_str(&str_sell_token).unwrap_or(H160::zero()))
                        .or_insert(bigdecimal_to_u256(
                            &best_pool.attributes.base_token_price_usd,
                        ));

                    // Fee = abs( (max_buy_amt * base_token_price_usd) - (max_sell_amt * quote_token_price_usd ) )
                    let fee_calc = (&max_buy_amt_dec * &quote_token_price_usd)
                        - (&max_sell_amt_dec * &base_token_price_usd);
                    let expected_fee_dec =
                        fee_calc.abs() / (&max_buy_amt_dec * &quote_token_price_usd);

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
                    cumulative_gas += bigdecimal_to_u256(
                        &(BigDecimal::from(75000) * gas_prices.ProposeGasPrice.clone()),
                    ); // Assume gas used to be 75000

                    // Interaction amounts
                    let input_amount_u256 = bigdecimal_to_u256(&max_buy_amt_dec);
                    let output_amount_u256 = executed_amount_u256;

                    let interaction = Interaction::Liquidity(LiquidityInteraction {
                        internalize: false,
                        id: best_pool.id.clone(),
                        input_token: order.sell_token,
                        output_token: order.buy_token,
                        input_amount: input_amount_u256,
                        output_amount: output_amount_u256,
                    });

                    solution.interactions.push(interaction);

                    graph_edges.push(Edge {
                        from: h160_to_string(&order.owner),
                        to: best_pool.id.clone(),
                        amount: input_amount_u256,
                        token: order.sell_token,
                    });

                    // if 'receiver' is set (i.e. owner wants to redirect their trade) there should be an edge (pool, receiver)
                    // otherwise make an edge (pool, owner address)
                    if let Some(receiver) = order.receiver {
                        graph_edges.push(Edge {
                            from: best_pool.id.clone(),
                            to: h160_to_string(&receiver),
                            amount: output_amount_u256,
                            token: order.buy_token,
                        });
                    } else {
                        graph_edges.push(Edge {
                            from: best_pool.id.clone(),
                            to: h160_to_string(&order.owner),
                            amount: output_amount_u256,
                            token: order.buy_token,
                        });
                    }

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
            let hex_uid: String = order
                .uid
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect();
            println!("Could not find direct swap for order '0x{}'", hex_uid);
        }

        if !fulfilled {
            let hex_uid: String = order
                .uid
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect();
            println!("Order '0x{}' not filled", hex_uid);
        }
    }

    // Set gas
    solution.gas = Some(cumulative_gas.as_u64());
    (solution, graph_edges)
}

fn h160_to_string(address: &H160) -> String {
    format!("0x{:x}", address)
}
