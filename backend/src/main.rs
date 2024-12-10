use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use log::info;
use naive::split_into_contexts;

use serde_json::json;
use serde_json::from_str;

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
    log::info!("Received hello  request: {:?}", "Hello, World!");
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

async fn post_notify(body: web::Bytes) -> impl Responder {
    let body_str = String::from_utf8_lossy(&body);
    info!("Received notify request: {}", body_str);

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
            .route("/", web::get().to(get_hello))
            .route("/notify", web::post().to(post_notify))
            .route("/solve", web::post().to(post_solve))
    })
    .bind(format!("0.0.0.0:{}",port))?
    .run()
    .await
}
