use axum::{extract::State, http, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::domain::AuthAPIError;
use crate::{app_state::AppState, domain::User};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = request.email;
    let password = request.password;

    if email.is_empty() || !email.contains('@') || password.len() < 8 {
        return Err(AuthAPIError::InvalidCredentials);
    }
    
    let user = User::new(email.clone(), password, request.requires_2fa);
    
    let mut user_store = state.user_store.write().await;
    
    match user_store.get_user(&email).await {
        Ok(_) => return Err(AuthAPIError::UserAlreadyExists),
        Err(_) => println!("@"),
    };

    match user_store.add_user(user).await {
        Ok(_) => {
            let response = Json(SignupResponse{
                message: "User created successfully!".to_string(),
            });
            Ok((http::StatusCode::CREATED, response))
        },
        Err(_) => {
            Err(AuthAPIError::UnexpectedError)
        }
    }
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}