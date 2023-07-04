pub mod configs;
pub mod core;
pub mod routes;

use actix_web::{dev::Server, get, App, HttpServer, Responder};
use configs::settings::Settings;

#[get("/health")]
async fn greet() -> impl Responder {
    "Health is good !".to_string()
}

pub async fn start_server(conf: Settings) -> std::io::Result<Server> {
    let server = HttpServer::new(|| App::new().service(greet))
        .bind((conf.server.base_url, conf.server.port))?
        .run();
    Ok(server)
}

pub fn sample_func() -> i32 {
    21
}
