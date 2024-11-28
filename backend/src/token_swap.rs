use reqwest::Client;
use serde_json::Error as SerdeError;
use thiserror::Error;
use crate::structures::pools::{Pools, Pool};

#[derive(Error, Debug)]
pub enum FetchPoolError {
    #[error("Network request failed")]
    RequestError(#[from] reqwest::Error),
    #[error("Failed to parse JSON")]
    JsonError(#[from] SerdeError),
    #[error("No matching pools found")]
    NoMatchingPools,
}

pub async fn get_highest_price_pool(token_from: &str, token_to: &str) -> Result<Pool, FetchPoolError> {
    let client = Client::new();

    // Build the URL by inserting `token_from` into the endpoint
    let url = format!(
        "https://api.geckoterminal.com/api/v2/networks/eth/tokens/{}/pools",
        token_from
    );

    // Make the GET request
    let response = client.get(&url)
        .send()
        .await?
        .error_for_status()?; // Ensure the response status is OK

    // Deserialize the response into `Pools`
    let pools: Pools = response.json().await?;

    // Filter pools where the `quote_token` ID matches `token_to`
    let matching_pools: Vec<Pool> = pools.data.into_iter()
        .filter(|pool| pool.relationships.quote_token.data.id == format!{"eth_{}", token_to})
        .collect();

    // Find the pool with the highest `base_token_price_quote_token`
    let highest_price_pool = matching_pools.into_iter()
        .max_by(|a, b| a.attributes.base_token_price_quote_token
            .partial_cmp(&b.attributes.base_token_price_quote_token)
            .unwrap_or(std::cmp::Ordering::Equal));

    // Return the result or an error if no matching pool is found
    highest_price_pool.ok_or(FetchPoolError::NoMatchingPools)
}
