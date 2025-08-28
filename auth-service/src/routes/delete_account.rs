use axum::{extract::State, http, response::IntoResponse, Json};
use serde::{Deserialize};

use crate::{app_state::AppState, domain::{AuthAPIError, Email, Password, UserStoreError}};

pub async fn delete_account(
    State(state): State<AppState>,
    Json(request): Json<DeleteAccountRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(request.password)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;
    
    let mut user_store = state.user_store.write().await;
    
    match user_store.delete_user(&email, &password).await {
        Ok(_) => {
            Ok((http::StatusCode::NO_CONTENT, ()))
        },
        Err(UserStoreError::UnexpectedError) => Err(AuthAPIError::UnexpectedError),
        Err(UserStoreError::UserNotFound) => Err(AuthAPIError::InvalidCredentials),
        Err(UserStoreError::InvalidCredentials) => Err(AuthAPIError::InvalidCredentials),
        Err(_) => Err(AuthAPIError::UnexpectedError),
    }
}

#[derive(Deserialize)]
pub struct DeleteAccountRequest {
    pub email: String,
    pub password: String,
}
