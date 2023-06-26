// tests/api/change_password.rs

// dependencies
use crate::helpers::{assert_is_redirect_to, spawn_app};
use uuid::Uuid;

#[tokio::test]
async fn you_must_be_logged_in_to_see_the_change_password_form() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_change_password().await;

    // Assert
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn you_must_be_logged_in_to_change_your_password() {
    // Arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();

    // Act
    let response = app
        .post_change_password(&serde_json::json!({
        "current_password": Uuid::new_v4().to_string(),
        "new_password": &new_password,
        "new_password_check": &new_password,
        }))
        .await;

    // Assert
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn new_password_fields_must_match() {
    // Arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let another_new_password = Uuid::new_v4().to_string();

    // Act - Part 1 - Login
    app.post_login(&serde_json::json!({
    "username": &app.test_user.username,
    "password": &app.test_user.password
    }))
    .await;

    // Act - Part 2 - Try to change password
    let response = app
        .post_change_password(&serde_json::json!({
        "current_password": &app.test_user.password,
        "new_password": &new_password,
        "new_password_check": &another_new_password,
        }))
        .await;
    assert_is_redirect_to(&response, "/admin/password");

    // Act - Part 3 - Follow the redirect
    let html_page = app.get_change_password_html().await;
    assert!(html_page
        .contains("You entered two different new passwords - the field values must match."));
}

#[tokio::test]
async fn current_password_must_be_valid() {
    // Arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let wrong_password = Uuid::new_v4().to_string();

    // Act - Part 1 -Login
    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password
    }))
    .await;

    // Act - Part 2 - Try to change password
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &wrong_password,
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;

    // Assert
    assert_is_redirect_to(&response, "/admin/password");

    // Act - Part 3 - Follow the redirect
    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains("The current password is incorrect."));
}

#[tokio::test]
async fn new_password_must_be_correct_length() {
    // Arrange
    let app = spawn_app().await;

    let test_cases = vec![
        "99ds09a",
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdefd"
    ];

    for case in test_cases {
        // Act - Part 1 - Login
        app.post_login(&serde_json::json!({
            "username": &app.test_user.username,
            "password": &app.test_user.password,
        }))
        .await;

        // Act - Part 2 - Try to change password
        let response = app
            .post_change_password(&serde_json::json!({
                "current_password": &app.test_user.password,
                "new_password": &case,
                "new_password_check": &case,
            }))
            .await;

        // Assert - Part 2
        assert_is_redirect_to(&response, "/admin/password");

        // Act - Part 3 - Follow the redirect
        let html_page = app.get_change_password_html().await;

        // Assert - Part 3
        assert!(
            html_page.contains("The new password should be between 12 and 128 characters long.")
        );
    }
}

#[tokio::test]
async fn changing_password_works() {
    // Arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();

    // Act - Part 1 - Login
    let login_body = serde_json::json!({
    "username": &app.test_user.username,
    "password": &app.test_user.password
    });
    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/admin/dashboard");

    // Act - Part 2 - Change password
    let response = app
        .post_change_password(&serde_json::json!({
        "current_password": &app.test_user.password,
        "new_password": &new_password,
        "new_password_check": &new_password,
        }))
        .await;
    assert_is_redirect_to(&response, "/admin/password");

    // Act - Part 3 - Follow the redirect
    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains("Your password has been changed."));

    // Act - Part 4 - Logout
    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login");

    // Act - Part 5 - Follow the redirect
    let html_page = app.get_login_html().await;
    assert!(html_page.contains("You have successfully logged out."));

    // Act - Part 6 - Login using the new password
    let login_body = serde_json::json!({
    "username": &app.test_user.username,
    "password": &new_password
    });
    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/admin/dashboard");
}
