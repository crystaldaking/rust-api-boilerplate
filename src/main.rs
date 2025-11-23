mod auth;
mod config;
mod db;
mod http;
mod redis_client;
mod models;
mod repositories;
mod services;

use anyhow::Result;
use config::AppConfig;
use db::{init_db_pool, run_migrations};
use redis_client::init_redis_client;
use auth::JwtConfig;

use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("FATAL ERROR: {err:?}");
    }
}

async fn run() -> Result<()> {
    println!(">>> Starting rust-api boilerplate inside binary");

    dotenvy::dotenv().ok();

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .init();

    let cfg = AppConfig::from_env()?;
    println!(">>> Config loaded, binding to {}", cfg.addr());

    let db_pool = connect_db_with_retry(&cfg.database_url).await?;
    println!(">>> Connected to Postgres");

    run_migrations(&db_pool).await?;
    println!(">>> Migrations applied");

    let redis_client = init_redis_client(&cfg.redis_url)?;
    println!(">>> Connected to Redis");

    let jwt_config = JwtConfig::from_env();
    let state = http::AppState::new(db_pool, redis_client, jwt_config);
    let app = http::create_app(state);

    let addr = cfg.addr();
    let listener = TcpListener::bind(&addr).await?;
    println!(">>> Listening on http://{}", addr);
    tracing::info!("Listening on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

/// Простой retry, чтобы пережить момент, когда postgres ещё не поднялся
async fn connect_db_with_retry(database_url: &str) -> Result<sqlx::PgPool> {
    let mut last_err = None;

    for attempt in 1..=10 {
        match init_db_pool(database_url).await {
            Ok(pool) => return Ok(pool),
            Err(e) => {
                println!("Postgres not ready (attempt {attempt}/10): {e}");
                last_err = Some(e);
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        }
    }

    Err(last_err.unwrap().into())
}
