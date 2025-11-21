mod routes;
mod controllers;
mod middleware;

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
    Router::new()
        .merge(routes::routes())
        .with_state(state)
}