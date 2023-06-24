mod routes;

use actix_web::{get, web, App, HttpServer, Responder};

#[get("/health")]
async fn greet() -> impl Responder {
    format!("Health is good ! ")
}

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
