use axum::{http, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{domain::AuthAPIError, utils::validate_token};

pub async fn verify_token(
    Json(request): Json<VerifyTokenRequest>
) -> Result<impl IntoResponse, AuthAPIError> {
    if validate_token(&request.token).await.is_err() {
        return Err(AuthAPIError::InvalidToken);
    }

    Ok(http::StatusCode::OK)
}

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String
}