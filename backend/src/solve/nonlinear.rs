use {
    solvers_dto::auction::{Auction, Token},
    web3::types::{H160, U256},
    crate::structures::competition_data::CompetitionData,
    std::collections::HashMap,
    std::convert::TryInto,
    std::error::Error,
    serde_json::Value,
};

/*
    WIP nonlinear programming solver.
*/
pub async fn get_clearing_prices(auction: &Auction) -> Result<HashMap<H160, f64>, Box<dyn Error>> {
    let token_index = enumerate_tokens(&auction.tokens);

    let (t_b, t_s) = get_buy_sell_matrices(&token_index, auction);

    // previous settling price (pull from https://api.cow.fi/mainnet/api/v1/solver_competition/latest)
    let url = "https://api.cow.fi/mainnet/api/v1/solver_competition/latest"; 
    let response = reqwest::get(url).await?.text().await?;

    // Parse the JSON response
    let latest_auction: CompetitionData = serde_json::from_str(&response).expect("Parsing failed");
    
    // initialize old prices to reference prices in case some tokens weren't used in the previous auction
    let mut p_old: Vec<U256> = auction.tokens
        .iter()
        .filter_map(|(token_address, token_data)| token_data.reference_price)
        .collect();
    
    // iterate over all solutions to find the winner's settlement prices
    for solution in latest_auction.solutions.iter() {
        if (solution.is_winner) {
            for (token, price) in &solution.clearing_prices {
                if (token_index.contains_key(&token)) {
                    p_old[token_index[&token] as usize] = *price;
                }
            }
        }
    }

    println!("{:#?}", p_old);

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
    return Ok(HashMap::new());
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