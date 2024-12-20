use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Error;
use log::info; // Logging macro
use reqwest::Client;
use serde::Serialize;
use serde_json::from_str;
use serde_json::json;
use serde_json::to_string_pretty;
use std::env;
use std::sync::{Arc, Mutex};
use std::vec;
use tokio::time::{sleep, Duration};

use solvers::api::routes::solve::dto::auction::to_domain;
use solvers::api::routes::solve::dto::solution::from_domain;
use solvers::domain::solver::Naive;
use solvers_dto::auction::Auction;
use solvers_dto::notification::Notification;
use solvers_dto::solution::Solutions;
use structures::edge::Edge;

mod solve;
mod structures;
mod utils;

use solve::given::fill_with_given_liquidity;
use solve::onchain::fill_with_uniswap_liquidity;
use structures::gas_price::{GasOracleResponse, GasPrices};
use structures::pools::{Pool, Pools};
use utils::parse::json_to_struct;
use utils::token_swap::get_direct_swap_pool;
use utils::token_swap::FetchPoolError;

#[derive(Serialize, Debug)]
struct VisualizationResult {
    solutions: Solutions,
    graph_edges: Vec<Edge>,
}

async fn write_to_firebase(path: &str, data: &str) -> Result<(), reqwest::Error> {
    let firebase_url = "https://solverevlos-c795c-default-rtdb.firebaseio.com/"; // Replace with your Firebase URL
    let client = Client::new();

    let url = format!("{}/{}.json", firebase_url, path);
    client.put(&url).body(data.to_string()).send().await?;
    Ok(())
}

async fn get_hello() -> impl Responder {
    let message = "Hello, World!";
    info!("Received hello request: {}", message);

    // Write the message to Firebase
    if let Err(e) = write_to_firebase("logs/hello", message).await {
        info!("Failed to write to Firebase: {}", e);
    }

    HttpResponse::Ok().json(json!({"message": message}))
}

async fn post_solve(body: web::Bytes) -> impl Responder {
    let body_str = String::from_utf8_lossy(&body);
    info!("Received solve request: {}", body_str);

    // Write the raw body to Firebase
    if let Err(e) = write_to_firebase("logs/solve", &body_str).await {
        info!("Failed to write to Firebase: {}", e);
    }

    let auction: Auction = match from_str(&body_str) {
        Ok(auction) => auction,
        Err(_) => {
            return HttpResponse::BadRequest().json(json!({"error": "Invalid input"}));
        }
    };

    let auction_domain = match to_domain(&auction) {
        Ok(value) => value,
        Err(err) => {
            log::info!("invalid auction");
            return HttpResponse::BadRequest().json(json!({ "error": err.message }));
        }
    };
    // let deadline = auction_domain.deadline.clone();
    // let groups = naive::group_by_token_pair(&auction_domain);
    // for (token_pair, group) in groups.iter() {
    //     (context_a, context_b) = split_into_contexts(group);
    // }

    // let naivesolver = Naive;
    // let solutions = naivesolver.solve(auction_domain).await;
    // let solutionsdto = from_domain(&solutions);

    // if (auction.liquidity.len() == 0) {}
    // ADD AVI FUNCTION HERE
    let (solution, graph_edges) = fill_with_given_liquidity(auction);

    let solutionsdto = Solutions {
        solutions: vec![solution],
    };

    println!("Solutions: {:#?}", solutionsdto);
    HttpResponse::Ok().json(solutionsdto)
}

async fn post_visualization(body: web::Bytes) -> impl Responder {
    let body_str = String::from_utf8_lossy(&body);
    info!("Received solve request: {}", body_str);

    // Write the raw body to Firebase
    if let Err(e) = write_to_firebase("logs/solve", &body_str).await {
        info!("Failed to write to Firebase: {}", e);
    }

    let auction: Auction = match from_str(&body_str) {
        Ok(auction) => auction,
        Err(_) => {
            return HttpResponse::BadRequest().json(json!({"error": "Invalid input"}));
        }
    };

    // ADD AVI FUNCTION HERE

    let (solution, graph_edges) = fill_with_given_liquidity(auction);

    let solutionsdto = Solutions {
        solutions: vec![solution],
    };
    println!("Solution: {:#?}", solutionsdto);

    let visualizationdto = VisualizationResult {
        solutions: solutionsdto,
        graph_edges: graph_edges,
    };

    HttpResponse::Ok().json(visualizationdto)
}

async fn post_notify(body: web::Bytes) -> impl Responder {
    let body_str = String::from_utf8_lossy(&body);
    info!("Received notify request: {}", body_str);

    // Write the raw body to Firebase
    if let Err(e) = write_to_firebase("logs/notify", &body_str).await {
        info!("Failed to write to Firebase: {}", e);
    }

    let notif: Notification = match from_str(&body_str) {
        Ok(notif) => notif,
        Err(_) => {
            return HttpResponse::BadRequest().json(json!({"error": "Invalid input"}));
        }
    };

    HttpResponse::Ok().json(json!({"status": "ok"}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .app_data(web::PayloadConfig::default().limit(50 * 1024 * 1024))
            .route("/", web::get().to(get_hello))
            .route("/notify", web::post().to(post_notify))
            .route("/solve", web::post().to(post_solve))
            .route("/visualize", web::post().to(post_visualization))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}

// #[tokio::main]
// async fn main() {
//     // Replace with your Etherscan API key
//     let api_key = "BQAT6JY5DAY3MJF8FY7PMHKACRPXM19JB1".to_string();

//     // Shared gas_price variable
//     let gas_price = Arc::new(Mutex::new(None));

//     // Clone the Arc to move into the task
//     let gas_price_clone = gas_price.clone();

//     // Spawn the background task
//     tokio::spawn(async move {
//         update_gas_price(gas_price_clone, api_key).await;
//     });

//     sleep(Duration::from_millis(1000)).await;

//     let auction: Auction = json_to_struct("data/real_sample.json");

//     let gas_price_lock = gas_price.lock().unwrap();
//     if let Some(gas_prices) = &*gas_price_lock {
//         let solution = fill_with_uniswap_liquidity(auction, gas_prices.clone()).await;
//         println!("{:#?}", solution.0);

//         let sol_json = serde_json::to_string_pretty(&solution.0).unwrap();
//         println!("{}", sol_json);

//         let edge_json = serde_json::to_string_pretty(&solution.1).unwrap();
//         println!("{}", edge_json);
//     } else {
//         println!("Gas price not available yet");
//     }
// }

/// Fetches gas prices from Etherscan
async fn fetch_gas_prices(api_key: &str) -> Result<GasPrices, Error> {
    let url = format!(
        "https://api.etherscan.io/api?module=gastracker&action=gasoracle&apikey={}",
        api_key
    );

    let response = reqwest::get(&url).await?;
    let gas_oracle: GasOracleResponse = response.json().await?;

    if gas_oracle.status == "1" {
        Ok(gas_oracle.result)
    } else {
        Err(anyhow::anyhow!("could not find gas prices"))
    }
}

/// Background task to update the `gas_price` variable
async fn update_gas_price(gas_price: Arc<Mutex<Option<GasPrices>>>, api_key: String) {
    loop {
        match fetch_gas_prices(&api_key).await {
            Ok(new_gas_prices) => {
                let mut gas_price_lock = gas_price.lock().unwrap();
                *gas_price_lock = Some(new_gas_prices);
                println!("Updated gas price: {:?}", gas_price_lock.as_ref().unwrap());
            }
            Err(err) => {
                println!("Failed to fetch gas prices: {}", err);
            }
        }

        // Sleep for 0.3 seconds before the next fetch
        sleep(Duration::from_millis(300)).await;
    }
}

pub async fn populate_liquidity_for_auction(
    auction: &mut Auction,
    network: &str,
    dex: &str,
) -> Result<(), FetchPoolError> {
    for order in &auction.orders {
        let pools =
            fetch_pools(network, dex, Some("base_token,quote_token"), Some(1), None).await?;

        // Filter pools that directly connect token_from to token_to.
        // The pools from GeckoTerminal have token IDs like "eth_0x..."
        // We'll assume token_from and token_to are given as "0x..." and need to match that format.
        let token_from_id = format!("eth_{}", order.sell_token.to_lowercase());
        let token_to_id = format!("eth_{}", order.buy_token.to_lowercase());

        let direct_pools: Vec<&Pool> = pools
            .data
            .iter()
            .filter(|p| {
                let base_token_id = &p.relationships.base_token.data.id.to_lowercase();
                let quote_token_id = &p.relationships.quote_token.data.id.to_lowercase();

                // Check if (base_token == token_from && quote_token == token_to)
                // OR (quote_token == token_from && base_token == token_to)
                (base_token_id == token_from_id && quote_token_id == token_to_id)
                    || (base_token_id == token_to_id && quote_token_id == token_from_id)
            })
            .collect();

        // Convert these filtered pools to Liquidity variants
        let liquidity_variants: Vec<Liquidity> = direct_pools
            .into_iter()
            .filter_map(|p| convert_pool_to_liquidity(p))
            .collect();

        // Filter only ConstantProduct pools
        let constant_product_pools: Vec<Liquidity> = liquidity_variants
            .into_iter()
            .filter(|l| matches!(l, Liquidity::ConstantProduct(_)))
            .collect();

        // Add these constant product pools to the auction's liquidity
        auction.liquidity.extend(constant_product_pools);
    }

    Ok(())
}
