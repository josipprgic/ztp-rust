use std::net::TcpListener;
use secrecy::ExposeSecret;
use ztp_rust::{configuration::{must_load_configuration, DatabaseSettings}, email_client::EmailClient, startup::run, telemetry::{get_subscriber, init_sub}};
use sqlx::PgPool;
use std::sync::Once;

pub struct TestApp {
    pub addr: String, 
    pub db_pool: PgPool
}

static ONCE_LOG_INIT: Once = Once::new();

pub async fn spawn_app() -> TestApp {
    init_logging();

    let config = must_load_configuration();
    let pool = init_db(config.database).await;

    let ec = EmailClient::new("http://localhost:8081/send_email".to_string(), 
                              "tester@test.com".to_string().try_into().unwrap());

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    // Start server 
    let server = run(listener, pool.clone(), ec).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    
    TestApp {
        addr: format!("http://127.0.0.1:{}", port),
        db_pool: pool,
    }
}

pub async fn send_request(client: &reqwest::Client, body: &'static str, address: &str) -> reqwest::Response {
    client
        .post(&format!{"{}/subscriptions", &address})
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to exec reqwest")
}

fn init_logging() {
    ONCE_LOG_INIT.call_once(|| {
        if std::env::var("TEST_LOG").is_ok() {
            let sub = get_subscriber("ztp_test".to_string(), "debug".into(), std::io::stdout);
            init_sub(sub);
        } else {
            let sub = get_subscriber("ztp_test".to_string(), "debug".into(), std::io::sink);
            init_sub(sub);
        }
    });
}

async fn init_db(config: DatabaseSettings) -> PgPool {
    let pool = PgPool::connect(&config.connection_string().expose_secret())
        .await
        .expect("Failed to connect to the database");
    
    sqlx::query!("delete from subscriptions")
        .execute(&pool)
        .await
        .expect("Failed to retrieve saved record");

    pool
}
