use std::sync::Arc;

use auth_service::{
    domain::{Email, MockUserStore, Password, User, UserStoreError},
    ErrorResponse,
};
use test_helpers::api_test;
use tokio::sync::RwLock;

use crate::helpers::{get_random_email, TestApp};

#[api_test]
async fn should_return_422_if_malformed_input() {
    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "email": random_email,
        }),
        serde_json::json!({
            "password": "passowrd123",
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.delete_account(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "failed for input: {:?}",
            test_case,
        )
    }
}

#[api_test]
async fn should_return_400_if_invalid_input() {
    let input = [
        serde_json::json!({
            "email": "invalid_email",
            "password": "passowrd123",
        }),
        serde_json::json!({
            "email": get_random_email(),
            "password": "1",
        }),
    ];

    for i in input.iter() {
        let response = app.delete_account(&i).await;
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


#[api_test]
async fn should_return_400_if_user_not_found() {
    let input = [
        serde_json::json!({
            "email": "dev.ted.kim@gmail.com",
            "password": "passowrd123",
        }),
    ];

    for i in input.iter() {
        let response = app.delete_account(&i).await;
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

#[api_test]
async fn should_return_204_if_success() {
    let random_email = get_random_email();
    let body = serde_json::json!({
        "email": random_email,
        "password": "passowrd123",
        "requires2FA": true,
    });

    app.post_signup(&body).await;

    let delete_account_body = serde_json::json!({
        "email": random_email,
        "password": "passowrd123",
    });
    let response = app.delete_account(&delete_account_body).await;
    assert_eq!(response.status().as_u16(), 204);
}

#[tokio::test]
async fn should_return_500_when_db_layer_add_user_returns_unexpectd_error() {
    let mut mock_user_store = MockUserStore::new();
    let random_email = get_random_email();
    let password = "password123";
    let expected_email = Email::parse(random_email.clone()).unwrap();
    
    let user = User::new(expected_email.clone(), Password::parse(password.to_string()).unwrap(), true);
    mock_user_store
        .expect_delete_user()
        .withf(move |email, password| *email == user.email && *password == user.password)
        .once()
        .returning(|_email, _password| Box::pin(async { Err(UserStoreError::UnexpectedError) }));

    let user_store: Arc<RwLock<MockUserStore>> = Arc::new(RwLock::new(mock_user_store));
    let app = TestApp::with_user_store(user_store).await;

    let body = serde_json::json!({
        "email": random_email,
        "password": password,
    });

    let response = app.delete_account(&body).await;
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
