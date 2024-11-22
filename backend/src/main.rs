use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde_json::json;
async fn get_hello() -> impl Responder {
    HttpResponse::Ok().json(json!({"message": "Hello, World!"}))
}

async fn post_data(item: web::Json<MyData>) -> impl Responder {
    HttpResponse::Ok().json({
        format!("Solving : {} {}", item.name, item.age)
    })
}

#[derive(serde::Deserialize)]
struct MyData {
    name: String,
    age: u8,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_hello))     
            .route("/solve", web::post().to(post_data)) 
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}