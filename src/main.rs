use std::net::TcpListener;
use ztp_rust::{configuration::{self}, startup::run};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = configuration::must_load_configuration();

    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    let conn = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to database");
    run(listener, conn)?.await
}
