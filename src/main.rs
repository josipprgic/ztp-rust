use std::net::TcpListener;
use secrecy::ExposeSecret;
use ztp_rust::{configuration, startup::run, telemetry::{get_subscriber, init_sub}};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let sub = get_subscriber("ztp_rust".to_string(), "info".to_string(), std::io::stdout);
    init_sub(sub);

    let config = configuration::must_load_configuration();
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    let conn = PgPool::connect(&config.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to database");

    run(listener, conn)?.await
}
