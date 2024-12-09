use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use log::info;
use naive::split_into_contexts;
// Logging macro
use serde_json::json;

use solvers::api::routes::solve::dto::auction::to_domain;
use solvers::domain::solver::Naive;
use solvers_dto::auction::Auction;
use solvers_dto::notification::Notification;
// use solvers_dto::solution::Solution;
use solvers::api::routes::solve::dto::solution::from_domain;

mod naive;
mod structures;
mod token_swap;
mod utils;

use structures::pools::{Pool, Pools};
use token_swap::get_direct_swap_pool;
use token_swap::get_intermediate_swap_pools;
use utils::parse::json_to_struct;

async fn get_hello() -> impl Responder {
    HttpResponse::Ok().json(json!({"message": "Hello, World!"}))
}

async fn post_solve(auction: web::Json<Auction>) -> impl Responder {
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
    let naivesolver = Naive;
    let solutions = naivesolver.solve(auction_domain).await;
    let solutionsdto = from_domain(&solutions);
    HttpResponse::Ok().json(solutionsdto)
    // HttpResponse::Ok().json(json!({
    //     "message": format!("Solving auction {}", auction.id.unwrap_or_default())
    // }))
}

async fn post_notify(notif: web::Json<Notification>) -> impl Responder {
    info!("Received notify request: {:?}", notif);
    HttpResponse::Ok().json(json!({"status": "ok"}))
}

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
