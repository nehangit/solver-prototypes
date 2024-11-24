use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde_json::json;
use log::info; // Logging macro

#[derive(serde::Deserialize, Debug)]
struct NotifyData {
    auctionId: String,
    solutionId: u64,
    kind: String,
    // Additional metadata is handled dynamically due to `additionalProperties`
    #[serde(flatten)]
    additional_metadata: serde_json::Value,
}

async fn get_hello() -> impl Responder {
    HttpResponse::Ok().json(json!({"message": "Hello, World!"}))
}

async fn post_data(item: web::Json<MyData>) -> impl Responder {
    HttpResponse::Ok().json({
        format!("Solving : {} {}", item.name, item.age)
    })
}

async fn post_notify(item: web::Json<NotifyData>) -> impl Responder {
    info!("Received notify request: {:?}", item);
    HttpResponse::Ok().json(json!({"status": "ok"}))
}

#[derive(serde::Deserialize)]
struct MyData {
    name: String,
    age: u8,
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