use router::configs::settings::Settings;
use router::core::errors::{ApplicationError, ApplicationResult};

#[actix_web::main]
async fn main() -> ApplicationResult<()> {
    let conf = Settings::new().expect("Unable to construct application configuration");
    println!("Application started: {:?}", conf);

    #[allow(clippy::unwrap_used)]
    let server = router::start_server(conf).await.unwrap();
    let _ = server.await;

    Err(ApplicationError::from(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Server shut down",
    )))
}
