use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &app.address)) // get health_check
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success()); // OK
    assert_eq!(Some(0), response.content_length()); // no body
}
