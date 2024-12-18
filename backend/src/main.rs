use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use log::info;
use reqwest::Client;
use serde_json::json;
use serde_json::from_str;
use solvers_dto::auction::Auction;
use solvers_dto::notification::Notification;
use solvers_dto::solution::Solutions;
use std::env;
mod utils;

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

async fn post_data(body: web::Bytes) -> impl Responder {
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

    let empty_solution = Solutions {
        solutions: vec![],
    };

    HttpResponse::Ok().json(empty_solution)
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
            .route("/solve", web::post().to(post_data))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
