use std::sync::Arc;

use auth_service::{
    domain::{Email, MockUserStore, Password, User, UserStoreError},
    routes::SignupResponse,
    ErrorResponse,
};
use tokio::sync::RwLock;

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "password": "passowrd123",
            "requires2FA": true,
        }),
        serde_json::json!({
            "email": random_email,
            "password": "passowrd123",
        }),
        serde_json::json!({
            "email": random_email,
            "requires2FA": true,
        }),
        serde_json::json!({
            "email": random_email,
        }),
        serde_json::json!({
            "requires2FA": true,
        }),
        serde_json::json!({
            "password": "passowrd123",
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "failed for input: {:?}",
            test_case,
        )
    }
}

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let body = serde_json::json!({
        "email": random_email,
        "password": "passowrd123",
        "requires2FA": true,
    });

    let response = app.post_signup(&body).await;
    assert_eq!(
        response.status().as_u16(),
        201,
        "failed for input: {:?}",
        body
    );

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };

    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response,
    );
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;
    let input = [
        serde_json::json!({
            "email": "invalid_email",
            "password": "passowrd123",
            "requires2FA": true,
        }),
        serde_json::json!({
            "email": get_random_email(),
            "password": "1",
            "requires2FA": true,
        }),
        serde_json::json!({
            "email": "",
            "password": "1",
            "requires2FA": true,
        }),
    ];

    for i in input.iter() {
        let response = app.post_signup(&i).await;
        assert_eq!(response.status().as_u16(), 400, "Failed for input: {:?}", i);

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned(),
        );
    }
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    let body = serde_json::json!({
        "email": random_email,
        "password": "passowrd123",
        "requires2FA": true,
    });

    app.post_signup(&body).await;
    let response = app.post_signup(&body).await;
    assert_eq!(response.status().as_u16(), 409,);
    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists".to_owned(),
    );
}

#[tokio::test]
async fn should_return_500_when_db_layer_get_user_returns_unexpectd_error() {
    let mut mock_user_store = MockUserStore::new();
    let random_email = get_random_email();

    let expected_email = Email::parse(random_email.clone()).unwrap();
    mock_user_store
        .expect_get_user()
        .withf(move |email| email == &expected_email)
        .once()
        .returning(|_email| Box::pin(async { Err(UserStoreError::UnexpectedError) }));

    let user_store: Arc<RwLock<MockUserStore>> = Arc::new(RwLock::new(mock_user_store));
    let app = TestApp::with_user_store(user_store).await;

    let body = serde_json::json!({
        "email": random_email,
        "password": "passowrd123",
        "requires2FA": true,
    });

    let response = app.post_signup(&body).await;
    assert_eq!(response.status().as_u16(), 500,);
    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Unexpected error".to_owned(),
    );
}

#[tokio::test]
async fn should_return_500_when_db_layer_add_user_returns_unexpectd_error() {
    let mut mock_user_store = MockUserStore::new();
    let random_email = get_random_email();
    let password = "password123";
    let expected_email = Email::parse(random_email.clone()).unwrap();
    let expected_email_for_get = expected_email.clone();
    mock_user_store
        .expect_get_user()
        .withf(move |email| email == &expected_email_for_get)
        .once()
        .returning(|_email| Box::pin(async { Err(UserStoreError::UnexpectedError) }));

    let user = User::new(expected_email.clone(), Password::parse(password.to_string()).unwrap(), true);
    mock_user_store
        .expect_add_user()
        .withf(move |u| u.email == user.email && u.password == u.password && u.require_2fa == user.require_2fa)
        .once()
        .returning(|_u| Box::pin(async { Err(UserStoreError::UnexpectedError) }));

    let user_store: Arc<RwLock<MockUserStore>> = Arc::new(RwLock::new(mock_user_store));
    let app = TestApp::with_user_store(user_store).await;

    let body = serde_json::json!({
        "email": random_email,
        "password": password,
        "requires2FA": true,
    });

    let response = app.post_signup(&body).await;
    assert_eq!(response.status().as_u16(), 500,);
    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Unexpected error".to_owned(),
    );
}
