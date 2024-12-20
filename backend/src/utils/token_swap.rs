use reqwest::Client;
use serde_json::Error as SerdeError;
use thiserror::Error;
use crate::structures::pools::{Pools, Pool};

/* 
    Useful Addresses:

    let weth = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2";
    let cow = "0xdef1ca1fb7fbcdc777520aa7f396b4e015f497ab";
    let sup = "0xe53EC727dbDEB9E2d5456c3be40cFF031AB40A55";
    let sui = "0x8d7Cfe0C1045Ed9D3e65F9746F270c9c1f65262a";
    let usdt = "0xdAC17F958D2ee523a2206206994597C13D831ec7";
    let usdc = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";
*/

#[derive(Error, Debug)]
pub enum FetchPoolError {
    #[error("Network request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Failed to parse JSON: {0}")]
    JsonError(#[from] SerdeError),
    #[error("No matching pools found")]
    NoMatchingPools,
    #[error("HTTP error: {0}")]
    HttpError(String),
}

/*
    Example usage:

    match get_direct_swap_pool(tokenA, tokenB).await {
        Ok(pool) => println!("Found pool: {:?}", pool),
        Err(e) => eprintln!("Error: {}", e),
    }
*/
pub async fn get_direct_swap_pool(token_from: &str, token_to: &str) -> Result<Pool, FetchPoolError> {
    let client = Client::new();

    // Build the URL by inserting `token_from` into the endpoint
    let url = format!(
        "https://api.geckoterminal.com/api/v2/networks/eth/tokens/{}/pools",
        token_from
    );

    // Make the GET request
    let response = client.get(&url).send().await;

    println!("here");

    // Handle potential network errors
    let response = match response {
        Ok(resp) => resp,
        Err(err) => {
            eprintln!("Network error occurred: {}", err);
            return Err(FetchPoolError::RequestError(err));
        }
    };

    println!("here2");

    // Check if the response status is a success
    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_else(|_| "Could not read response body".to_string());
        eprintln!("Request failed with status: {}", status);
        eprintln!("Response body: {}", text);
        return Err(FetchPoolError::HttpError(format!("Status: {}, Body: {}", status, text)));
    }

    println!("here3");

    // Deserialize the response into `Pools`
    let pools: Pools = response.json().await?;

    println!("kms");

    // Filter pools where the `quote_token` ID matches `token_to`
    let matching_pools: Vec<Pool> = pools.data.into_iter()
        .filter(|pool| {
            let base_token_id = &pool.relationships.base_token.data.id;
            let quote_token_id = &pool.relationships.quote_token.data.id;
            println!("{}", base_token_id);
            println!("{}", quote_token_id);
            println!("{}", &format!("eth_{}", token_to));
            base_token_id.eq_ignore_ascii_case(&format!("eth_{}", token_to)) ||
            quote_token_id.eq_ignore_ascii_case(&format!("eth_{}", token_to))
        })
        .collect();

    println!("here4");

    // Find the pool with the highest `base_token_price_quote_token`
    let highest_price_pool = matching_pools.into_iter().max_by(|a, b| {
        let a_base_token_id = &a.relationships.base_token.data.id;
        let a_forward = if a_base_token_id.eq_ignore_ascii_case(&format!("eth_{}", token_from)) {&a.attributes.base_token_price_quote_token} else {&a.attributes.quote_token_price_base_token};

        let b_base_token_id = &b.relationships.base_token.data.id;
        let b_forward = if b_base_token_id.eq_ignore_ascii_case(&format!("eth_{}", token_from)) {&b.attributes.base_token_price_quote_token} else {&b.attributes.quote_token_price_base_token};
        if a_forward > b_forward {
            std::cmp::Ordering::Greater
        } else if a_forward < b_forward {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    });

    println!("here5");

    // Return the result or an error if no matching pool is found
    highest_price_pool.ok_or(FetchPoolError::NoMatchingPools)
}

/*
    Example usage:

    match get_intermediate_swap_pools(WETH/USDT, tokenA, tokenB).await {
        Ok((pool1, pool2)) => {
            println!("First Pool (token_from <-> WETH/USDT):\n{:#?}", pool1);
            println!("Second Pool (WETH/USDT <-> token_to):\n{:#?}", pool2);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
*/
pub async fn get_intermediate_swap_pools(token_mid: &str, token_from: &str, token_to: &str) -> Result<(Pool, Pool), FetchPoolError> {
    // WETH address
    // First call: token_from -> token_mid
    let pool1 = get_direct_swap_pool(token_from, token_mid).await?;

    // Second call: token_mid -> token_to
    let pool2 = get_direct_swap_pool(token_to, token_mid).await?;

    // Return both pools
    Ok((pool1, pool2))
}