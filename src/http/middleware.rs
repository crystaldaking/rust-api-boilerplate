use axum::{
    extract::State,
    http::{self, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::{auth, http::AppState};

/*pub async fn require_jwt<B>(
    State(state): State<AppState>,
    req: http::Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let cfg = match state.jwt_config.as_ref() {
        Some(cfg) => cfg,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    auth::validate_token(token, cfg).map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(next.run(req).await)
}*/
