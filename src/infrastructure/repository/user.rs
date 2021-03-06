use crate::domain::repository::user::UserRepositoryTrait;
use async_trait::async_trait;
use color_eyre::Result;
use sqlx::{Error, Pool, Postgres};

use crate::User;

pub struct UserRepository<'p> {
    connection: &'p Pool<Postgres>,
}
impl<'p> UserRepository<'p> {
    pub fn new(connection: &'p Pool<Postgres>) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl<'p> UserRepositoryTrait for UserRepository<'p> {
    async fn get_by_id(&self, uuid: String) -> Result<User, Error> {
        sqlx::query_as::<_, User>("select * from users where id::text = $1")
            .bind(uuid)
            .fetch_one(self.connection)
            .await
    }

    async fn get_all(&self) -> Result<Vec<User>, Error> {
        sqlx::query_as::<_, User>("select * from users")
            .fetch_all(self.connection)
            .await
    }
}
