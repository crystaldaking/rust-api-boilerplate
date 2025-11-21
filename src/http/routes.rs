use axum::{
    routing::get,
    Router,
};

use crate::http::{controllers, AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(controllers::health::health_check))
}