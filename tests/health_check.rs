use std::net::TcpListener;
use ztp_rust::run;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // We return the application address to the caller!
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = send_request(&client, body, app_address.as_str()).await;

    assert_eq!(200, response.status().as_u16());
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
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing name"),
        ("", "missing bothe name and email")
    ];

    for (invalid_body, error_case) in test_cases {
        let response = send_request(&client, invalid_body, app_address.as_str()).await;

        assert_eq!(400, response.status(), "Failed to return 400 on invalid request params: {}", error_case);
    }
}
