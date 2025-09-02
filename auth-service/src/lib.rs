use std::{error::Error};

use axum::{http::{self}, response::{IntoResponse, Response}, routing::{delete, post}, serve::Serve, Json, Router};
use tower_http::{cors::CorsLayer, services::ServeDir};

pub mod domain;
pub mod routes;
pub mod services;
pub mod app_state;
pub mod utils;

use app_state::AppState;
use domain::AuthAPIError;
use serde::{Deserialize, Serialize};

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let allowed_origins = [
            "http://localhost:8000".parse()?,
            "http://67.205.181.209:8000".parse()?,
            "http://67.205.181.209:80".parse()?,
        ];

        let cors = CorsLayer::new()
            // Allow GET and POST requests
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
            AuthAPIError::UnexpectedError => (http::StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error"),
            AuthAPIError::IncorrectCredentials => (http::StatusCode::UNAUTHORIZED, "Unauthorized"),
            AuthAPIError::InvalidToken => (http::StatusCode::UNAUTHORIZED, "Unauthorized"),
            AuthAPIError::MissingToken => (http::StatusCode::BAD_REQUEST, "Missing token"),
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });

        (status, body).into_response()
    }
}