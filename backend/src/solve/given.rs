use {
    solvers_dto::auction::{Auction, Liquidity},
    solvers_dto::solution::{Solution, Trade, Fulfillment, Interaction, LiquidityInteraction, Call},
    web3::types::{H160, U256},
    bigdecimal::ToPrimitive,
    std::collections::HashMap,
};

/*
    Solves the auction using the given liquidity from the auction
*/
pub fn fill_with_given_liquidity(auction: Auction) -> Solution {
    // Step 1: Get clearing prices for tokens
    let clearing_prices = get_clearing_prices(&auction);
    println!("{:#?}", clearing_prices);

    // Step 2: Initialize solution structure
    let mut solution = Solution {
        id: auction.id.unwrap_or_default() as u64,
        prices: clearing_prices
            .iter()
            .map(|(token, price)| (*token, U256::from(price.trunc() as u64)))
            .collect(),
        trades: Vec::new(),
        pre_interactions: Vec::new(),
        interactions: Vec::new(),
        post_interactions: Vec::new(),
        gas: None,
    };

    let mut cumulative_gas: U256 = U256::zero();

    // Step 3: Iterate through orders and match against available liquidity
    for order in &auction.orders {
        let sell_price = clearing_prices
            .get(&order.sell_token)
            .copied()
            .unwrap_or_default();
        let buy_price = clearing_prices
            .get(&order.buy_token)
            .copied()
            .unwrap_or_default();

        if sell_price == 0.0 || buy_price == 0.0 {
            continue; // Skip orders with missing prices
        }

        let clearing_sell_amount = order.sell_amount.as_u128() as f64 / sell_price;
        let clearing_buy_amount = order.buy_amount.as_u128() as f64 / buy_price;

        // Match order with liquidity
        let mut fulfilled = false;
        for liquidity in &auction.liquidity {
            match liquidity {
                Liquidity::ConstantProduct(pool) => {
                    let sell_balance = pool
                        .tokens
                        .get(&order.sell_token)
                        .map(|reserve| reserve.balance)
                        .unwrap_or_default()
                        .as_u128() as f64;

                    let buy_balance = pool
                        .tokens
                        .get(&order.buy_token)
                        .map(|reserve| reserve.balance)
                        .unwrap_or_default()
                        .as_u128() as f64;

                    // Ensure the pool has enough liquidity to fulfill the order
                    if sell_balance >= clearing_sell_amount && buy_balance >= clearing_buy_amount {
                        // Create trade
                        let trade = Trade::Fulfillment(Fulfillment {
                            order: order.uid,
                            executed_amount: order.sell_amount, // Fulfill the full sell amount
                            fee: Some(U256::from_dec_str(&(pool.fee.to_f64().unwrap() * (order.sell_amount.low_u64() as f64)).to_string()).expect("")),
                        });

                        solution.trades.push(trade);
                        cumulative_gas += pool.gas_estimate;

                        // Add interaction to draw liquidity from this pool
                        let interaction = Interaction::Liquidity(LiquidityInteraction {
                            internalize: false,
                            id: pool.id.clone(),
                            input_token: order.sell_token,
                            output_token: order.buy_token,
                            input_amount: order.sell_amount,
                            output_amount: order.buy_amount,
                        });

                        solution.interactions.push(interaction);

                        // Add pre- and post-interactions
                        for pre_interaction in &order.pre_interactions {
                            solution.pre_interactions.push(Call {
                                target: pre_interaction.target,
                                value: pre_interaction.value,
                                calldata: pre_interaction.call_data.clone(),
                            });
                        }

                        for post_interaction in &order.post_interactions {
                            solution.post_interactions.push(Call {
                                target: post_interaction.target,
                                value: post_interaction.value,
                                calldata: post_interaction.call_data.clone(),
                            });
                        }

                        fulfilled = true;
                        break;
                    }
                }
                _ => continue, // Skip other liquidity types for now
            }
        }

        if !fulfilled {
            // Order couldn't be fulfilled, skip
            let hex_uid: String = order.uid.iter().map(|byte| format!("{:02x}", byte)).collect();
            println!("Order '0x{}' not filled", hex_uid);
            continue;
        }
    }

    // Set the gas for the solution
    solution.gas = Some(cumulative_gas.as_u64());

    solution
}

pub fn get_clearing_prices(auction: &Auction) -> HashMap<H160, f64> {
    // WETH address according to token_swap.rs
    let weth_address: H160 = "c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
        .parse()
        .expect("Invalid WETH address");

    let mut clearing_prices: HashMap<H160, f64> = HashMap::new();
    clearing_prices.insert(weth_address, 1.0);

    // Temporary storage for token prices derived from pools
    let mut token_prices: HashMap<H160, Vec<f64>> = HashMap::new();

    // Iterate over all liquidity sources
    for liquidity in &auction.liquidity {
        // Consider only constant product pools for now
        if let Liquidity::ConstantProduct(pool) = liquidity {
            // Ensure the pool contains WETH
            if pool.tokens.contains_key(&weth_address) {
                // Fetch WETH balance
                let weth_reserve = pool.tokens[&weth_address].balance;

                for (token_address, token_reserve) in &pool.tokens {
                    if *token_address != weth_address {
                        // Compute the price of the token in terms of WETH
                        let price_in_weth = (weth_reserve.as_u128() as f64)
                            / (token_reserve.balance.as_u128() as f64);

                        // Store this price in the token_prices map
                        token_prices
                            .entry(*token_address)
                            .or_insert_with(Vec::new)
                            .push(price_in_weth);
                    }
                }
            }
        }
    }

    // Compute the average price for each token and update clearing_prices
    for (token_address, prices) in token_prices {
        let average_price = prices.iter().sum::<f64>() / prices.len() as f64;
        clearing_prices.insert(token_address, average_price);
    }

    clearing_prices
}
