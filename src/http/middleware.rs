use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::Header;
use uuid::Uuid;

use crate::{auth, http::AppState};

pub async fn require_jwt(State(state): State<AppState>, mut request: Request, next: Next)
                         -> Result<Response, StatusCode> {
    let auth_header = request.headers().get(header::AUTHORIZATION)
        .and_then(|h|h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth_header.split_once("Bearer ").ok_or(StatusCode::UNAUTHORIZED)?.1;

    let jwt_cfg = state.jwt_config.as_ref().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let data = auth::validate_token(token, jwt_cfg).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user_id = Uuid::parse_str(&data.claims.sub).map_err(|_| StatusCode::UNAUTHORIZED)?;

    request.extensions_mut().insert(user_id);

    Ok(next.run(request).await)
}