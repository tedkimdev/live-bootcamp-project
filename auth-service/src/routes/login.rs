use axum::{http, response::IntoResponse};

pub async fn login() -> impl IntoResponse {
    http::StatusCode::OK.into_response()
}
