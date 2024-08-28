use crate::helpers::{assert_is_redirect_to, spawn_app};
use uuid::Uuid;

#[tokio::test]
async fn you_must_be_logged_in_to_see_the_change_password_form() {
    // arrange
    let app = spawn_app().await;

    // act
    let response = app.get_change_password().await;

    // assert
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn you_must_be_logged_in_to_change_your_password() {
    // arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();

    // act
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": Uuid::new_v4().to_string(),
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;

    // assert
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn new_password_fields_must_match() {
    // arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let another_new_password = Uuid::new_v4().to_string();

    // act (login)
    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    }))
    .await;
    // act (change password)
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &app.test_user.password,
            "new_password": &new_password,
            "new_password_check": &another_new_password,
        }))
        .await;

    // assert
    assert_is_redirect_to(&response, "/admin/password");

    // act (follow redirect)
    let html_page = app.get_change_password_html().await;

    // assert
    assert!(html_page.contains(
        "<p><i>You entered two different new passwords - \
         the field values must match.</i></p>",
    ));
}

#[tokio::test]
async fn current_password_must_be_valid() {
    // arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let wrong_password = Uuid::new_v4().to_string();

    // act (login)
    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    }))
    .await;

    // act (change password)
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &wrong_password,
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;

    // assert
    assert_is_redirect_to(&response, "/admin/password");

    // act (follow redirect)
    let html_page = app.get_change_password_html().await;

    // assert
    assert!(html_page.contains("<p><i>The current password is incorrect.</i></p>",));
}

#[tokio::test]
async fn logout_clears_session_state() {
    // arrange
    let app = spawn_app().await;

    // act (login)
    let response = app
        .post_login(&serde_json::json!({
            "username": &app.test_user.username,
            "password": &app.test_user.password,
        }))
        .await;
    assert_is_redirect_to(&response, "/admin/dashboard");

    // act (follow redirect)
    let html_page = app.get_admin_dashboard_html().await;
    assert!(html_page.contains(&format!("Welcome {}!", app.test_user.username)));

    // act (logout)
    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login");

    // act (follow redirect)
    let html_page = app.get_login_html().await;
    assert!(html_page.contains(r#"<p><i>You have successfully logged out.</i></p>"#));

    // act (load admin dashboard)
    let response = app.get_admin_dashboard().await;
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn changing_password_works() {
    // arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();

    // act (login)
    let response = app
        .post_login(&serde_json::json!({
            "username": &app.test_user.username,
            "password": &app.test_user.password,
        }))
        .await;
    assert_is_redirect_to(&response, "/admin/dashboard");

    // act (change password)
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &app.test_user.password,
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;
    assert_is_redirect_to(&response, "/admin/password");

    // act (follow redirect)
    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains("<p><i>Your password has been changed.</i></p>"));

    // act (logout)
    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login");

    // act (follow redirect)
    let html_page = app.get_login_html().await;
    assert!(html_page.contains(r#"<p><i>You have successfully logged out.</i></p>"#));

    // act (login with new password)
    let response = app
        .post_login(&serde_json::json!({
            "username": &app.test_user.username,
            "password": &new_password,
        }))
        .await;
    assert_is_redirect_to(&response, "/admin/dashboard");
}
