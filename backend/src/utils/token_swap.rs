use crate::structures::pools::{Pool, Pools};
use reqwest::Client;
use serde_json::Error as SerdeError;
use thiserror::Error;
use solvers_dto::auction::{ConstantProductPool, ConstantProductReserve, Liquidity};
use std::collections::HashMap;
use bigdecimal::{BigDecimal};
use web3::types::{H160, U256};


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


pub async fn get_direct_swap_pool(token_from: &str, token_to: &str) -> Result<Pool, FetchPoolError> {
    let client = Client::new();

    // Build the URL by inserting `token_from` into the endpoint
    let url = format!(
        "https://api.geckoterminal.com/api/v2/networks/eth/tokens/{}/pools",
        token_from
    );

    // Make the GET request
    let response = client.get(&url).send().await.map_err(FetchPoolError::RequestError)?;

    // Check if the response status is a success
    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_else(|_| "Could not read response body".to_string());
        eprintln!("Request failed with status: {}", status);
        eprintln!("Response body: {}", text);
        return Err(FetchPoolError::HttpError(format!("Status: {}, Body: {}", status, text)));
    }

    // Read the entire response as text
    let text = response.text().await.unwrap_or_else(|_| "Could not read response body".to_string());
    println!("Raw JSON was: {}", text);
    // Parse the JSON from the text
    let pools: Pools = match serde_json::from_str(&text) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to deserialize response: {:?}", e);
            eprintln!("Raw JSON was: {}", text);
            return Err(FetchPoolError::JsonError(e));
        }
    };

    // Filter pools where the `quote_token` ID matches `token_to`
    let matching_pools: Vec<Pool> = pools.data.into_iter()
        .filter(|pool| {
            let base_token_id = &pool.relationships.base_token.data.id;
            let quote_token_id = &pool.relationships.quote_token.data.id;
            let pool_type = &pool.relationships.dex.data.id.to_lowercase();

            // println!("{}", base_token_id);
            // println!("{}", quote_token_id);
            // println!("{}", &format!("eth_{}", token_to));
            // base_token_id.eq_ignore_ascii_case(&format!("eth_{}", token_to)) ||
            quote_token_id.eq_ignore_ascii_case(&format!("eth_{}", token_to)) && pool_type.eq_ignore_ascii_case("uniswap_v2")
        })
        .collect();

    // Find the pool with the highest `base_token_price_quote_token`
    let highest_price_pool = matching_pools.into_iter().max_by(|a, b| {
        let a_base_token_id = &a.relationships.base_token.data.id;
        let a_forward = if a_base_token_id.eq_ignore_ascii_case(&format!("eth_{}", token_from)) {
            &a.attributes.base_token_price_quote_token
        } else {
            &a.attributes.quote_token_price_base_token
        };

        let b_base_token_id = &b.relationships.base_token.data.id;
        let b_forward = if b_base_token_id.eq_ignore_ascii_case(&format!("eth_{}", token_from)) {
            &b.attributes.base_token_price_quote_token
        } else {
            &b.attributes.quote_token_price_base_token
        };

        a_forward.partial_cmp(b_forward).unwrap_or(std::cmp::Ordering::Equal)
    });

    println!("here5");
    // println!("{:?}", highest_price_pool);
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


pub fn convert_pool_to_liquidity(pool: &Pool) -> Option<Liquidity> {
    let pool_address = pool.attributes.address;

    let base_token_addr = parse_token_id(&pool.relationships.base_token.data.id)?;
    let quote_token_addr = parse_token_id(&pool.relationships.quote_token.data.id)?;

    // Assume Uniswap v2 fee is 0.3%
    let place_fee = BigDecimal::from(0);

    // Extract values
    let reserve_in_usd = &pool.attributes.reserve_in_usd;
    let base_token_price_usd = &pool.attributes.base_token_price_usd;
    let quote_token_price_usd = &pool.attributes.quote_token_price_usd;

    // Half of the reserve_in_usd
    let half = reserve_in_usd / BigDecimal::from(2);
    // base_reserve = half / base_token_price_usd
    let base_reserve = &half / base_token_price_usd;
    // quote_reserve = half / quote_token_price_usd
    let quote_reserve = &half / quote_token_price_usd;

    // Assume 18 decimals for scaling
    let scale = bigdecimal::BigDecimal::from(10u64.pow(18));
    let base_scaled = &base_reserve * &scale;
    let quote_scaled = &quote_reserve * &scale;

    let base_u256 = decimal_to_u256(&base_scaled);

    let quote_u256 = decimal_to_u256(&quote_scaled);

    let mut tokens_map = HashMap::new();
    tokens_map.insert(base_token_addr, ConstantProductReserve { balance: base_u256 });
    tokens_map.insert(quote_token_addr, ConstantProductReserve { balance: quote_u256 });

    Some(Liquidity::ConstantProduct(ConstantProductPool {
        id: pool.id.clone(),
        address: pool_address,
        router: H160::zero(),
        gas_estimate: U256::from(0),
        tokens: tokens_map,
        fee: place_fee,
    }))
}

fn parse_token_id(token_id: &str) -> Option<H160> {
    let stripped = token_id.strip_prefix("eth_")?;
    parse_h160(stripped)
}

fn parse_h160(s: &str) -> Option<H160> {
    let hex_str = s.strip_prefix("0x").unwrap_or(s);
    let bytes = hex::decode(hex_str).ok()?;
    if bytes.len() != 20 {
        return None;
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(&bytes);
    Some(H160(arr))
}

fn decimal_to_u256(d: &BigDecimal) -> U256 {
    // Create a truncated version with no fractional digits.
    // `with_scale(0)` reduces the scale to zero, effectively truncating towards zero.
    let truncated = d.with_scale(0);

    // Convert to string
    let s = truncated.to_string();

    // If the value is negative, we cannot represent it in U256.
    // Decide how to handle this case. Here, we return zero.
    if s.starts_with('-') {
        return U256::zero();
    }

    // Attempt to parse as U256
    U256::from_dec_str(&s).unwrap_or(U256::zero())
}