use axum::{extract::State, http, response::IntoResponse, Json};
use serde::{Deserialize};

use crate::{app_state::AppState, domain::{AuthAPIError, Email, Password}};

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(request.password)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;
    
    let user_store = &state.user_store.read().await;

    if user_store.validate_user(&email, &password).await.is_err() {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    let result = user_store.get_user(&email).await;
    if result.is_err() {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    let _user = result.unwrap();

    Ok(http::StatusCode::OK)
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}