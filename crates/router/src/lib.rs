mod routes;

use actix_web::{get, App, HttpServer, Responder};

#[get("/health")]
async fn greet() -> impl Responder {
    "Health is good !".to_string()
}

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

pub fn sample_func() -> i32 {
    21
}
