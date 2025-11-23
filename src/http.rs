mod routes;
mod controllers;
mod middleware;
use axum::{middleware as axum_middleware};

use axum::Router;
use redis::Client as RedisClient;
use sqlx::PgPool;

use crate::auth::JwtConfig;

#[derive(Clone)]
pub struct AppState{
    pub db_pool: PgPool,
    pub redis_client: RedisClient,
    pub jwt_config: Option<JwtConfig>
}

impl AppState {
    pub fn new(db_pool: PgPool, redis_client: RedisClient, jwt_config: Option<JwtConfig>) -> Self {
        Self {
            db_pool,
            redis_client,
            jwt_config
        }
    }
}

pub fn create_app(state: AppState) -> Router {
    let public = routes::public_routes();
    let protected = routes::protected_routes().route_layer(
        axum_middleware::from_fn_with_state(state.clone(), middleware::require_jwt)
    );

    public.merge(protected).with_state(state)
}