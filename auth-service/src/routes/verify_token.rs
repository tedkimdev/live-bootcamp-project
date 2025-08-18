use axum::{http, response::IntoResponse};

pub async fn verify_token() -> impl IntoResponse {
    http::StatusCode::OK.into_response()
}
