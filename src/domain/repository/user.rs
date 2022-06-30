use async_trait::async_trait;
use sqlx::Error;

use crate::domain::entity::user::User;

#[async_trait]
pub trait UserRepositoryTrait {
    async fn get_by_id(&self, id: String) -> Result<User, Error>;
    async fn get_all(&self) -> Result<Vec<User>, Error>;
}
