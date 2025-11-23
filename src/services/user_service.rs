use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::PgPool;

use crate::models::user::User;
use crate::repositories::user_repository::UserRepository;

pub struct UserService;

impl UserService {
    pub fn hash_password(plain: &str) -> Result<String> {
        let hashed = hash(plain, DEFAULT_COST)?;
        Ok(hashed)
    }

    pub fn verify_password(plain: &str, hashed: &str) -> Result<bool> {
        let ok = verify(plain, hashed)?;
        Ok(ok)
    }

    pub async fn register(pool: &PgPool, email: &str, password: &str) -> Result<User> {
        if let Some(_) = UserRepository::find_by_email(pool, email).await? {
            anyhow::bail!("user_already_exists");
        }

        let password_hash = Self::hash_password(password)?;
        let user = UserRepository::create(pool, email, &password_hash).await?;
        Ok(user)
    }

    pub async fn authenticate(pool: &PgPool, email: &str, password: &str) -> Result<Option<User>> {
        if let Some(user) = UserRepository::find_by_email(pool, email).await? {
            let ok = Self::verify_password(password, &user.password_hash)?;
            if ok {
                return Ok(Some(user));
            }
        }
        Ok(None)
    }
}
