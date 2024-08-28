use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn an_error_flash_message_is_set_on_failure() {
    // arrange
    let app = spawn_app().await;

    // act
    let response = app
        .post_login(&serde_json::json!({
            "username": "random-username",
            "password": "random-password"
        }))
        .await;

    // assert
    assert_is_redirect_to(&response, "/login");

    // act
    let html_page = app.get_login_html().await;

    // assert
    assert!(html_page.contains(r#"<p><i>Authentication failed</i></p>"#));

    // act (reload login page)
    let html_page = app.get_login_html().await;

    // assert
    assert!(!html_page.contains(r#"<p><i>Authentication failed</i></p>"#));
}
