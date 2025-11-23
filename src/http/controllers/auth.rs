use axum::{
    extract::{Extension, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    auth::create_token,
    http::AppState,
    models::user::User,
    repositories::user_repository::UserRepository,
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

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
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
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<UserResponse>), (StatusCode, Json<ErrorResponse>)> {
    let user = UserService::register(&state.db_pool, &payload.email, &payload.password)
        .await
        .map_err(|err| {
            let msg = err.to_string();
            eprintln!("[register] error: {msg}");

            if msg == "user_already_exists" {
                (
                    StatusCode::CONFLICT,
                    Json(ErrorResponse {
                        error: "user_already_exists".into(),
                    }),
                )
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "internal_error".into(),
                    }),
                )
            }
        })?;

    Ok((StatusCode::CREATED, Json(user.into())))
}

// POST /auth/login
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<ErrorResponse>)> {
    let user = UserService::authenticate(&state.db_pool, &payload.email, &payload.password)
        .await
        .map_err(|err| {
            eprintln!("[login] error: {err:?}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "internal_error".into(),
                }),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "invalid_credentials".into(),
                }),
            )
        })?;

    let jwt_cfg = state.jwt_config.as_ref().ok_or_else(|| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "jwt_not_configured".into(),
            }),
        )
    })?;

    let token = create_token(&user.id.to_string(), jwt_cfg).map_err(|err| {
        eprintln!("[login] token error: {err:?}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "token_error".into(),
            }),
        )
    })?;

    let user_resp: UserResponse = user.into();

    Ok(Json(LoginResponse {
        token,
        user: user_resp,
    }))
}

// GET /auth/me
pub async fn me(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<UserResponse>, (StatusCode, Json<ErrorResponse>)> {
    let user = UserRepository::find_by_id(&state.db_pool, user_id)
        .await
        .map_err(|err| {
            eprintln!("[me] db error: {err:?}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "internal_error".into(),
                }),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "user_not_found".into(),
                }),
            )
        })?;

    Ok(Json(user.into()))
}
