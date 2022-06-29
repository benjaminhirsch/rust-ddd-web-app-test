use uuid::Uuid;
use async_trait::async_trait;
use sqlx::Error;

use crate::domain::entity::user::User;

#[async_trait]
pub trait UserRepositoryTrait {
    fn get_by_id(&self, id: Uuid) -> Result<User, String>;
    async fn get_all(&self) -> Result<Vec<User>, Error>;
}
