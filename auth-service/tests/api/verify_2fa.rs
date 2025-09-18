use auth_service::{
    domain::{Email, LoginAttemptId, TwoFACode},
    utils::JWT_COOKIE_NAME,
};
use test_helpers::api_test;
use uuid::Uuid;

use crate::helpers::{TestApp, get_random_email};

#[api_test]
async fn should_return_422_if_malformed_input() {
    let random_email = get_random_email();

    let body = serde_json::json!({
        "email": random_email,
        // "loginAttemptId": "",
        "2FACode": "",
    });
    let response = app.post_verify_2fa(&body).await;

    assert_eq!(response.status().as_u16(), 422);
}

#[api_test]
async fn should_return_400_if_invalid_input() {
    let random_email = get_random_email();

    let body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": "0b704039-b847-4cfb-9f41-20fdacfbfb2a",
        "2FACode": "invalid_fa_cdoe",
    });

    let response = app.post_verify_2fa(&body).await;

    assert_eq!(response.status().as_u16(), 400);
}

#[api_test]
async fn should_return_401_if_incorrect_credentials() {
    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true,
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    let email = Email::parse(random_email.to_string()).unwrap();
    let login_attempt_id = LoginAttemptId::parse(Uuid::new_v4().to_string()).unwrap();
    let code = TwoFACode::parse("123456".to_string()).unwrap();

    {
        let mut two_fa_code_store = app.two_fa_code_store.write().await;
        if two_fa_code_store
            .add_code(email, login_attempt_id.clone(), code)
            .await
            .is_err()
        {
            panic!("test failed");
        };
    }

    let verify_2fa_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id.as_ref(),
        "2FACode": "000000",
    });
    let response = app.post_verify_2fa(&verify_2fa_body).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[api_test]
async fn should_return_401_if_old_code() {
    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true,
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    let email = Email::parse(random_email.to_string()).unwrap();
    let login_attempt_id = LoginAttemptId::parse(Uuid::new_v4().to_string()).unwrap();
    let old_code = TwoFACode::parse("000000".to_string()).unwrap();
    let new_code = TwoFACode::parse("123456".to_string()).unwrap();

    let mut two_fa_code_store = app.two_fa_code_store.write().await;
    if two_fa_code_store
        .add_code(email.clone(), login_attempt_id.clone(), old_code)
        .await
        .is_err()
    {
        panic!("test failed");
    };
    drop(two_fa_code_store);
    let mut two_fa_code_store = app.two_fa_code_store.write().await;
    if two_fa_code_store
        .add_code(email, login_attempt_id.clone(), new_code)
        .await
        .is_err()
    {
        panic!("test failed");
    };
    drop(two_fa_code_store);

    let verify_2fa_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id.as_ref(),
        "2FACode": "000000",
    });
    let response = app.post_verify_2fa(&verify_2fa_body).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[api_test]
async fn should_return_200_if_correct_code() {
    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true,
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    let email = Email::parse(random_email.to_string()).unwrap();
    let login_attempt_id = LoginAttemptId::parse(Uuid::new_v4().to_string()).unwrap();
    let code = TwoFACode::parse("000000".to_string()).unwrap();

    let mut two_fa_code_store = app.two_fa_code_store.write().await;
    if two_fa_code_store
        .add_code(email, login_attempt_id.clone(), code)
        .await
        .is_err()
    {
        panic!("test failed");
    };
    drop(two_fa_code_store);

    let verify_2fa_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id.as_ref(),
        "2FACode": "000000",
    });
    let response = app.post_verify_2fa(&verify_2fa_body).await;

    assert_eq!(response.status().as_u16(), 200);
    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

#[api_test]
async fn should_return_401_if_same_code_twice() {
    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true,
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    let email = Email::parse(random_email.to_string()).unwrap();
    let login_attempt_id = LoginAttemptId::parse(Uuid::new_v4().to_string()).unwrap();
    let code = TwoFACode::parse("000000".to_string()).unwrap();

    let mut two_fa_code_store = app.two_fa_code_store.write().await;
    if two_fa_code_store
        .add_code(email, login_attempt_id.clone(), code)
        .await
        .is_err()
    {
        panic!("test failed");
    };
    drop(two_fa_code_store);

    let verify_2fa_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id.as_ref(),
        "2FACode": "000000",
    });
    let response = app.post_verify_2fa(&verify_2fa_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let verify_2fa_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id.as_ref(),
        "2FACode": "000000",
    });
    let response = app.post_verify_2fa(&verify_2fa_body).await;

    assert_eq!(response.status().as_u16(), 401);
}
