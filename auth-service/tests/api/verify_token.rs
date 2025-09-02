use auth_service::{domain::Email, utils::generate_auth_cookie};

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let body = serde_json::json!({
        "": "",
    });
    let response = app.post_verify_token(&body).await;

    assert_eq!(
        response.status().as_u16(),
        422,
    );
}

#[tokio::test]
async fn should_return_200_valid_token() {
    let app = TestApp::new().await;

    let random_email = Email::parse(get_random_email()).unwrap();
    let cookie = generate_auth_cookie(&random_email).unwrap();
    
    let body = serde_json::json!({
        "token": cookie.value().to_string(),
    });
    let response = app.post_verify_token(&body).await;

    assert_eq!(
        response.status().as_u16(),
        200,
    );
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    let body = serde_json::json!({
        "token": "invalid_token",
    });
    let response = app.post_verify_token(&body).await;

    assert_eq!(
        response.status().as_u16(),
        401,
    );
}