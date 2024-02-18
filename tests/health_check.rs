use std::net::TcpListener;
use ztp_rust::{configuration::must_load_configuration, startup::run};
use sqlx::{PgPool, Connection};

pub struct TestApp {
    pub addr: String, 
    pub db_pool: PgPool
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let config = must_load_configuration();
    let pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to the database");
    
    sqlx::query!("delete from subscriptions")
        .execute(&pool)
        .await
        .expect("Failed to retrieve saved record");


    let server = run(listener, pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // We return the application address to the caller!
    TestApp {
        addr: format!("http://127.0.0.1:{}", port),
        db_pool: pool,
    }
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/health_check", &app.addr))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_data() {
    let app_address = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = send_request(&client, body, app_address.addr.as_str()).await;

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("select email, name from subscriptions")
        .fetch_one(&app_address.db_pool)
        .await
        .expect("Failed to retrieve saved record");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

async fn send_request(client: &reqwest::Client, body: &'static str, address: &str) -> reqwest::Response {
    client
        .post(&format!{"{}/subscriptions", &address})
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to exec reqwest")
}

#[tokio::test]
async fn subscribe_returns_400_for_invalid_data() {
    let app_address = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing name"),
        ("", "missing bothe name and email")
    ];

    for (invalid_body, error_case) in test_cases {
        let response = send_request(&client, invalid_body, app_address.addr.as_str()).await;

        assert_eq!(400, response.status(), "Failed to return 400 on invalid request params: {}", error_case);
    }
}
