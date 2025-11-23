use axum::{
    routing::get,
    Router,
};
use axum::routing::post;
use crate::http::{controllers, AppState};

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(controllers::health::health_check))
        .route("/auth/register", post(controllers::auth::register))
        .route("/auth/login", post(controllers::auth::login))
}

pub fn protected_routes() -> Router<AppState> {
    Router::new().route("/auth/me", get(controllers::auth::me))
}