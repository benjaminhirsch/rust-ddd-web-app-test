use sqlx::types::time::{PrimitiveDateTime};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime
}