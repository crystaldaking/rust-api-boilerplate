use axum::{Json, extract::State, http::StatusCode};
use redis::Client as RedisClient;
use serde::Serialize;
use sqlx::PgPool;
use tracing::error;

use crate::http::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
    services: ServicesStatus,
}

#[derive(Serialize)]
pub struct ServicesStatus {
    postgres: &'static str,
    redis: &'static str,
}

pub async fn health_check(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<HealthResponse>), StatusCode> {
    let postgres_ok = check_postgres(&state.db_pool).await;
    let redis_ok = check_redis(&state.redis_client).await;

    let overall_status = if postgres_ok && redis_ok {
        "ok"
    } else {
        "degraded"
    };

    let status_code = if overall_status == "ok" {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    Ok((
        status_code,
        Json(HealthResponse {
            status: overall_status,
            services: ServicesStatus {
                postgres: if postgres_ok { "ok" } else { "error" },
                redis: if redis_ok { "ok" } else { "error" },
            },
        }),
    ))
}

async fn check_postgres(pool: &PgPool) -> bool {
    match sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(pool)
        .await
    {
        Ok(_) => true,
        Err(err) => {
            error!(%err, "Database health check failed");
            false
        }
    }
}

async fn check_redis(client: &RedisClient) -> bool {
    match client.get_multiplexed_async_connection().await {
        Ok(mut conn) => match redis::cmd("PING").query_async::<String>(&mut conn).await {
            Ok(_) => true,
            Err(err) => {
                error!(%err, "Redis PING failed");
                false
            }
        },
        Err(err) => {
            error!(%err, "Redis connection failed");
            false
        }
    }
}
