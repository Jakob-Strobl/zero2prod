use std::net::TcpListener;

// tokio::test is the equivalent of tokio::main + test attribute
// You can expand the code generated with
// `cargo expand --test health_check`
#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    // setup reqwest to make http requests
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let body = "name=John%20Doe&email=john_doe%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=john_doe%40gmail.com", "missing the name"),
        ("", "missing both the name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // additional custom error message on test assertion failing
            "The API did not fail with 400 bad request when the payload was {}.",
            error_message
        )
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to an available port");
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("Failed to bind to address");

    // non-binding let because the server is spawned as a background task
    // We don't need the handle in scope, so we don't bind it to a variable name.
    // tokio handles cleanup when the runtime is dropped
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
