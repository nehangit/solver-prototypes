use primitive_types::U256;
use solvers::domain::{auction::Auction, order::Side}; // Domain object
use std::collections::HashMap;
use web3::types::Address;

#[derive(Debug)]
pub struct Group<'a> {
    orders: Vec<&'a solvers::domain::order::Order>,
    liquidity: &'a solvers::domain::liquidity::Liquidity,
    pool: &'a solvers::domain::liquidity::constant_product::Pool,
}
pub type Groups<'a> = HashMap<solvers::domain::liquidity::TokenPair, Group<'a>>;

///
/// Helper function that groups an auction by token pairs, where each group contains:
/// 1. All orders associated with the token pair.
/// 2. The "deepest" constant product pool (i.e., the pool with the most liquidity) for the pair,
///    as indicated by the highest `K` value in Uniswap V2-style constant product pools.
/// See above data structures for implementations.
///
pub fn group_by_token_pair(auction: &Auction) -> Groups {
    // Create a mutable collection to store groups by token pairs.
    let mut groups = Groups::new();

    // Iterate over all liquidity pools in the auction.
    for liquidity in &auction.liquidity {
        // Check if the current liquidity is a constant product pool.
        // If not, skip this iteration.
        let pool = match &liquidity.state {
            solvers::domain::liquidity::State::ConstantProduct(pool) => pool,
            _ => continue,
        };

        // Find existing or create a new entry for the token pair in the groups.
        groups
            .entry(pool.tokens()) // Use the token pair of the pool as the key.
            .and_modify(|group| {
                // If an entry already exists, compare the pool's `K` value with the
                // current group's pool. Update the group if the current pool has more liquidity (larger K).
                if group.pool.k() < pool.k() {
                    group.liquidity = liquidity; // Update the liquidity reference.
                    group.pool = pool; // Update to the deeper pool.
                }
            })
            .or_insert_with(|| Group {
                // If no entry exists, create a new group with the current pool.
                orders: Vec::new(), // Start with an empty list of orders.
                liquidity,          // Reference the current liquidity.
                pool,               // Use the current pool as the group's pool.
            });
    }

    // Iterate over all orders in the auction.
    for order in &auction.orders {
        // Skip invalid orders where the sell or buy amount is zero.
        // These could cause errors in the solver algorithm.
        if order.sell.amount.is_zero() || order.buy.amount.is_zero() {
            continue;
        }

        // Try to create a token pair from the order's sell and buy tokens.
        // If the tokens are invalid (e.g., identical), skip this order.
        let tokens =
            match solvers::domain::liquidity::TokenPair::new(order.sell.token, order.buy.token) {
                Some(value) => value,
                None => continue,
            };

        // Add the order to the corresponding group for the token pair.
        groups
            .entry(tokens) // Locate the group by the token pair.
            .and_modify(|group| group.orders.push(order)); // Append the order to the group's orders.
    }

    // Remove any groups that do not contain orders.
    groups.retain(|_, group| !group.orders.is_empty());

    // Return the groups structure.
    groups
}

#[derive(Debug, Clone)]
struct TokenContext {
    address: Address,
    reserve: U256,
    buy_volume: U256,
    sell_volume: U256,
}

// Split orders into contexts: takes a limit order array with all having the same pair of tokens,
// and consolidate the buy and sell contexts (amount being bought and sold, and chosen pool reserve amount)

fn split_into_contexts(group: &Group) -> (TokenContext, TokenContext) {
    let pool = group.pool;
    let orders = &group.orders;
    let mut contexts = HashMap::new();

    // Fetch the reserves as a tuple
    let (reserve_a, reserve_b) = pool.reserves.get();
    let token_a = reserve_a.token.0;
    let token_b = reserve_b.token.0;

    contexts.insert(
        token_a,
        TokenContext {
            address: reserve_a.token.0,
            reserve: reserve_a.amount,
            buy_volume: U256::zero(),
            sell_volume: U256::zero(),
        },
    );
    contexts.insert(
        token_b,
        TokenContext {
            address: reserve_b.token.0,
            reserve: reserve_b.amount,
            buy_volume: U256::zero(),
            sell_volume: U256::zero(),
        },
    );

    // Process each order to consolidate buy and sell volumes into the appropriate contexts
    for order in orders {
        // Update the context for the buy token
        if let Some(buy_context) = contexts.get_mut(&order.buy.token.0) {
            if (order.side == Side::Buy) {
                buy_context.buy_volume += order.buy.amount;
            }
        } else {
            panic!("Buy token in order not found contexts object");
        }

        // Update the context for the sell token
        if let Some(sell_context) = contexts.get_mut(&order.sell.token.0) {
            if (order.side == Side::Sell) {
                sell_context.sell_volume += order.sell.amount;
            }
        } else {
            panic!("Sell token in order not found in contexts object");
        }
    }
    assert_eq!(contexts.len(), 2, "Orders contain more than two tokens");
    let mut contexts = contexts.drain().map(|(_, v)| v);
    (contexts.next().unwrap(), contexts.next().unwrap())
}
