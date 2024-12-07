use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use log::info; 
use serde_json::json;
use serde_json::from_str;
use solvers_dto::auction::Auction;
use solvers_dto::notification::Notification;
use solvers_dto::solution::Solutions;
use std::env;
mod utils;


async fn get_hello() -> impl Responder {
    log::info!("Received hello  request: {:?}", "Hello, World!");
    HttpResponse::Ok().json(json!({"message": "Hello, World!"}))
}

async fn post_data(body: web::Bytes) -> impl Responder {
    let body_str = String::from_utf8_lossy(&body);
    info!("Received solve request: {}", body_str);

    let auction: Auction = match from_str(&body_str) {
        Ok(auction) => auction,
        Err(_) => {
            return HttpResponse::BadRequest().json(json!({"error": "Invalid input"}));
        }
    };

    let empty_solution = Solutions {
        solutions: vec![],
    };

    HttpResponse::Ok().json(empty_solution)
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
            .route("/solve", web::post().to(post_data))
    })
    .bind(format!("0.0.0.0:{}",port))?
    .run()
    .await
}
