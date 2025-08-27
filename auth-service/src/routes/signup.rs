use axum::{extract::State, http, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::domain::{AuthAPIError, Email, Password};
use crate::{app_state::AppState, domain::User};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(request.password)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;
    
    let user = User::new(email.clone(), password, request.requires_2fa);
    let mut user_store = state.user_store.write().await;
    
    match user_store.get_user(&email).await {
        Ok(_) => return Err(AuthAPIError::UserAlreadyExists),
        Err(crate::domain::UserStoreError::UnexpectedError) => return Err(AuthAPIError::UnexpectedError),
        Err(_) => (),
    };
         
    match user_store.add_user(user).await {
        Ok(_) => {
            let response = Json(SignupResponse{
                message: "User created successfully!".to_string(),
            });
            Ok((http::StatusCode::CREATED, response))
        },
        Err(crate::domain::UserStoreError::UserAlreadyExists) => Err(AuthAPIError::UserAlreadyExists),
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