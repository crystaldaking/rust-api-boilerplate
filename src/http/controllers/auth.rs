use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    auth::create_token,
    http::AppState,
    models::user::User,
    services::user_service::UserService,
};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        Self {
            id: u.id.to_string(),
            email: u.email,
            created_at: u.created_at.to_rfc3339(),
        }
    }
}

// POST /auth/register
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>, ) -> Result<(StatusCode, Json<UserResponse>), StatusCode> {
    let user = UserService::register(&state.db_pool, &payload.email, &payload.password)
        .await
        .map_err(|err| {
            if err.to_string() == "user_already_exists" {
                StatusCode::CONFLICT
            } else {
                eprintln!("register error: {err:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;

    Ok((StatusCode::CREATED, Json(user.into())))
}

// POST /auth/login
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>, ) -> Result<Json<LoginResponse>, StatusCode> {
    let user = UserService::authenticate(&state.db_pool, &payload.email, &payload.password)
        .await
        .map_err(|err| {
            eprintln!("login error: {err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let jwt_cfg = state.jwt_config.as_ref().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    let token = create_token(&user.id.to_string(), jwt_cfg)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_resp: UserResponse = user.into();

    Ok(Json(LoginResponse {
        token,
        user: user_resp,
    }))
}
