use axum::{http, response::IntoResponse};

pub async fn verify_2fa() -> impl IntoResponse {
    http::StatusCode::OK.into_response()
}
