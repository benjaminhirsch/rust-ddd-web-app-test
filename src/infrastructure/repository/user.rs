use async_trait::async_trait;
use sqlx::{Error, Pool, Postgres};
use uuid::Uuid;
use crate::domain::repository::user::UserRepositoryTrait;

use crate::User;

pub struct UserRepository {
    pub connection: Pool<Postgres>
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {

    fn get_by_id(&self, _: Uuid) -> Result<User, String> {
        panic!("dsfdsf")
    }

    async fn get_all(&self) -> Result<Vec<User>, Error>{
        let query = sqlx::query_as::<_, User>("select * from users");
        query.fetch_all(&self.connection).await
    }
}