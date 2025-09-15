use std::{error::Error};

use axum::{http::{self}, response::{IntoResponse, Response}, routing::{delete, post}, serve::Serve, Json, Router};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tower_http::{cors::CorsLayer, services::ServeDir};

pub mod domain;
pub mod routes;
pub mod services;
pub mod app_state;
pub mod utils;

use app_state::AppState;
use domain::AuthAPIError;
use serde::{Deserialize, Serialize};

use crate::utils::ALLOWED_ORIGINS;

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let allowed_origins = ALLOWED_ORIGINS.clone();

        let cors = CorsLayer::new()
            .allow_methods([http::Method::GET, http::Method::POST])
            // Allow cookies to be included in requests
            .allow_credentials(true)
            .allow_origin(allowed_origins);

        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(routes::signup))
            .route("/delete-account", delete(routes::delete_account))
            .route("/login", post(routes::login))
            .route("/logout", post(routes::logout))
            .route("/verify-2fa", post(routes::verify_2fa))
            .route("/verify-token", post(routes::verify_token))
            // .route("/refresh-token")
            .with_state(app_state)
            .layer(cors);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (http::StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (http::StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::IncorrectCredentials => (http::StatusCode::UNAUTHORIZED, "Incorrect credentials"),
            AuthAPIError::InvalidToken => (http::StatusCode::UNAUTHORIZED, "Invalid auth token"),
            AuthAPIError::MissingToken => (http::StatusCode::BAD_REQUEST, "Missing auth token"),
            AuthAPIError::UnexpectedError => (http::StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error"),
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });

        (status, body).into_response()
    }
}

pub async fn get_postgres_pool(url: &str) -> Result<PgPool, sqlx::Error> {
    // Create a new PostgreSQL connection pool
    PgPoolOptions::new().max_connections(5).connect(url).await
}