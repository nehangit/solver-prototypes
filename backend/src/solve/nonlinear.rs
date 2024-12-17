use {
    solvers_dto::auction::{Auction, Token},
    web3::types::{H160, U256},
    std::collections::HashMap,
    std::convert::TryInto,
};

/*
    To be completed when or if previous auction results for CoW come back
    up (currently 403 Forbidden)
*/
pub fn get_clearing_prices(auction: &Auction) -> HashMap<H160, f64> {
    let token_index = enumerate_tokens(&auction.tokens);

    let (t_b, t_s) = get_buy_sell_matrices(&token_index, auction);

    // previous settling price (pull from https://api.cow.fi/mainnet/api/v1/solver_competition/latest)
    let p_old: Vec<U256> = Vec::new();

    // Fill with lambda values lambda_j = 1 / (n * (p_old)_j)
    let lambda: Vec<f64> = p_old
        .iter()
        .filter_map(|p| {
            let p_usize: usize = (*p).try_into().unwrap();
            Some(1.0 / (auction.tokens.len() * p_usize) as f64)
        })
        .collect();

    // Fluctuation parameter (default to 1, but should be changed accordingly)
    let delta: f64 = 1.0;

    // lower and upper bounds for p_j
    // (p_lb)_j = (p_old)_j / (1 + delta)
    // (p_ub)_j = (p_old)_j * (1 + delta)
    let p_lb: Vec<f64> = p_old
        .iter()
        .filter_map(|p| {
            let p_usize: usize = (*p).try_into().unwrap();
            Some(p_usize as f64 / (1.0 + delta))
        })
        .collect();
    let p_ub: Vec<f64> = p_old
        .iter()
        .filter_map(|p| {
            let p_usize: usize = (*p).try_into().unwrap();
            Some(p_usize as f64 * (1.0 + delta))
        })
        .collect();

    // Optimize clearing prices using nonlinear programming
    return HashMap::new();
}

fn enumerate_tokens(tokens: &HashMap<H160, Token>) -> HashMap<H160, i32> {
    let mut token_index = HashMap::new();
    let mut counter = 0;

    for key in tokens.keys() {
        token_index.insert(*key, counter);
        counter += 1;
    }

    token_index
}

fn get_buy_sell_matrices(token_index: &HashMap<H160, i32>, auction: &Auction) -> (Vec<Vec<bool>>, Vec<Vec<bool>>) {
    let mut t_b: Vec<Vec<bool>> = vec![vec![false; auction.orders.len()]; auction.tokens.len()];
    let mut t_s: Vec<Vec<bool>> = vec![vec![false; auction.orders.len()]; auction.tokens.len()];

    for (order_index, order) in auction.orders.iter().enumerate() {
        if let Some(&buy_index) = token_index.get(&order.buy_token) {
            t_b[buy_index as usize][order_index] = true; // Mark token as bought
        }

        if let Some(&sell_index) = token_index.get(&order.sell_token) {
            t_s[sell_index as usize][order_index] = true; // Mark token as sold
        }
    }

    (t_b, t_s)
}