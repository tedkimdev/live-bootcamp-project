use axum::{http, response::IntoResponse};

pub async fn logout() -> impl IntoResponse {
    http::StatusCode::OK.into_response()
}
