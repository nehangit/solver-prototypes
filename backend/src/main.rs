use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use futures::StreamExt;
use log::info;
use serde_json::from_str;
use serde_json::json;
use solvers_dto::auction::Auction;
use solvers_dto::notification::Notification;
use solvers_dto::solution::Solutions;
use std::env;
mod utils;

async fn get_hello() -> impl Responder {
    log::info!("Received hello  request: {:?}", "Hello, World!");
    HttpResponse::Ok().json(json!({"message": "Hello, World!"}))
}

async fn post_data(mut payload: web::Payload) -> impl Responder {
    let mut body = web::BytesMut::new();

    // Read the body chunk by chunk
    while let Some(chunk) = payload.next().await {
        match chunk {
            Ok(data) => body.extend_from_slice(&data),
            Err(err) => {
                log::error!("Error while reading payload: {:?}", err);
                return HttpResponse::BadRequest().finish();
            }
        }
    }

    // Attempt to convert the body into a valid UTF-8 string
    let body_str = match std::str::from_utf8(&body) {
        Ok(valid_str) => valid_str, // Body is valid UTF-8
        Err(_) => {
            log::error!("Received invalid UTF-8 in request body");
            return HttpResponse::BadRequest().json(json!({"error": "Invalid UTF-8 input"}));
        }
    };

    // Attempt to deserialize the body into an Auction
    let auction: Auction = match serde_json::from_str(body_str) {
        Ok(auction) => auction,
        Err(err) => {
            log::error!("Failed to parse JSON: {}", err);
            return HttpResponse::BadRequest().json(json!({"error": "Invalid JSON input"}));
        }
    };

    // Log the deserialized Auction object
    log::info!("Auction object: {:?}", auction);

    let empty_solution = Solutions { solutions: vec![] };

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
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
