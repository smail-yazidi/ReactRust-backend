use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use actix_cors::Cors;

#[derive(Serialize)]
struct Message {
    msg: String,
}

#[get("/api/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(Message {
        msg: "مرحباً من Rust API!".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    HttpServer::new(|| {
        let cors = Cors::permissive(); // يسمح لكل المواقع
        App::new()
            .wrap(cors)
            .service(hello)
    })
    .bind(("0.0.0.0", port.parse().unwrap()))?
    .run()
    .await
}

