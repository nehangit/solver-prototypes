use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use log::info; // Logging macro
use serde_json::json;
use solvers_dto::auction::Auction;
use solvers_dto::notification::Notification;

mod structures;
mod utils;
mod token_swap;

use structures::pools::{Pool, Pools};
use utils::parse::json_to_struct;
use token_swap::get_highest_price_pool;

#[derive(serde::Deserialize, Debug)]
struct NotifyData {
    auction_id: String,
    solution_id: u64,
    kind: String,
    // Additional metadata is handled dynamically due to `additionalProperties`
    #[serde(flatten)]
    additional_metadata: serde_json::Value,
}

async fn get_hello() -> impl Responder {
    HttpResponse::Ok().json(json!({"message": "Hello, World!"}))
}

async fn post_data(auction: web::Json<Auction>) -> impl Responder {
    HttpResponse::Ok().json({ format!("Solving : {}", auction.id.unwrap_or_default()) })
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
            .route("/solve", web::post().to(post_data))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
