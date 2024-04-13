use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use ztp_rust::{
    configuration, email_client,
    startup::run,
    telemetry::{get_subscriber, init_sub},
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let sub = get_subscriber("ztp_rust".to_string(), "info".to_string(), std::io::stdout);
    init_sub(sub);

    let config = configuration::must_load_configuration();
    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address)?;

    let conn = PgPool::connect(&config.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to database");

    let sender = config
        .email
        .sender()
        .expect("Failed to parse sender email from config");
    let emailc = email_client::EmailClient::new(config.email.address, sender);

    run(listener, conn, emailc)?.await
}
