use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub exp: usize,  // unix timestamp
}

#[derive(Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub exp_minutes: u64,
}

impl JwtConfig {
    pub fn from_env() -> Option<Self> {
        let secret = std::env::var("JWT_SECRET").ok()?;
        let exp_minutes = std::env::var("JWT_EXP_MINUTES")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(60);

        Some(Self {
            secret,
            exp_minutes,
        })
    }
}

pub fn create_token(user_id: &str, cfg: &JwtConfig) -> Result<String> {
    let exp = (SystemTime::now() + Duration::from_secs(cfg.exp_minutes * 60))
        .duration_since(UNIX_EPOCH)?
        .as_secs() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp,
    };

    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(cfg.secret.as_bytes()),
    )?;

    Ok(token)
}

pub fn validate_token(token: &str, cfg: &JwtConfig) -> Result<TokenData<Claims>> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(cfg.secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(data)
}
