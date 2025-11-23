use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::User;

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_email(pool: &PgPool, email: &str) -> sqlx::Result<Option<User>> {
        sqlx::query_as!(User, "SELECT id, email, password_hash, created_at FROM users WHERE email = $1", email)
            .fetch_optional(pool)
            .await
    }

    pub async fn create(pool: &PgPool, email: &str, password_hash: &str) -> sqlx::Result<User> {
        sqlx::query_as!(User, "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id, email, password_hash, created_at", email, password_hash)
            .fetch_one(pool)
            .await
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> sqlx::Result<Option<User>> {
       sqlx::query_as!(User, "SELECT id, email, password_hash, created_at FROM users WHERE id = $1", id)
           .fetch_optional(pool)
           .await
    }
}