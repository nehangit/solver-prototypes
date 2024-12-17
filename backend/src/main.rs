use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use log::info; // Logging macro
use serde_json::json;
use serde_json::to_string_pretty;

// use solvers::api::routes::solve::dto::auction::to_domain;
use solvers_dto::auction::Auction;
use solvers_dto::notification::Notification;
use solvers_dto::solution::Solution;
// use solvers::api::routes::solve::dto::solution::from_domain;

mod structures;
mod utils;
mod solve;

use structures::pools::{Pool, Pools};
use utils::token_swap::get_direct_swap_pool;
use utils::token_swap::get_intermediate_swap_pools;
use utils::parse::json_to_struct;
use solve::given::fill_with_given_liquidity;

async fn get_hello() -> impl Responder {
    HttpResponse::Ok().json(json!({"message": "Hello, World!"}))
}

/*
async fn post_solve(auction: web::Json<Auction>) -> impl Responder {
    let auction_domain = match to_domain(&auction) {
        Ok(value) => value,
        Err(err) => {
            log::info!("invalid auction");
            return HttpResponse::BadRequest().json(json!({ "error": err.message }));
        }
    };
    let deadline = auction_domain.deadline.clone();
    let groups = naive::group_by_token_pair(&auction_domain);
    HttpResponse::Ok().json(json!({
        "message": format!("Solving auction {}", auction.id.unwrap_or_default())
    }))
}
*/

// temporary post_solve while to_domain can't be imported
async fn post_solve(auction: web::Json<Auction>) -> impl Responder {
    HttpResponse::Ok().json({ format!("Solving : {}", auction.id.unwrap_or_default()) })
}

async fn post_notify(notif: web::Json<Notification>) -> impl Responder {
    info!("Received notify request: {:?}", notif);
    HttpResponse::Ok().json(json!({"status": "ok"}))
}

/*
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_hello))
            .route("/notify", web::post().to(post_notify))
            .route("/solve", web::post().to(post_solve))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
*/

fn main() {
    let auction: Auction = json_to_struct("data/onchain_liquidity.json");
    let solution = fill_with_given_liquidity(auction);
    println!("{:#?}", solution);

    let json = serde_json::to_string_pretty(&solution).unwrap();
    println!("{}", json);
}
