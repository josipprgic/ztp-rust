use crate::helpers::{spawn_app, send_request};

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

#[tokio::test]
async fn subscribe_returns_400_for_missing_data() {
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

#[tokio::test]
async fn subscribe_returns_400_for_invalid_data() {
    let app_address = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=%20%20%20", "email=test@testete.com"),
        ("name=dobro", "email=ursula%40_le_guin%40gmail.com"),
        ("name=%20%20", "email=some%40%40gmail.com")
    ];

    for (invalid_body, error_case) in test_cases {
        let response = send_request(&client, invalid_body, app_address.addr.as_str()).await;

        assert_eq!(400, response.status(), "Failed to return 400 on invalid request params: {}", error_case);
    }
}
