use solvers::domain::auction::Auction; // Domain object
use std::collections::HashMap;

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
