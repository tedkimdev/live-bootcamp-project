use axum::{http, response::IntoResponse};

pub async fn signup() -> impl IntoResponse {
    http::StatusCode::OK.into_response()
}
