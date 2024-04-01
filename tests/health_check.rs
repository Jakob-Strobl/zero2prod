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
