use crate::domain::entity::user::User;
use crate::domain::repository::user::UserRepositoryTrait;
use crate::infrastructure::repository::user::UserRepository;
use sqlx::postgres::PgPoolOptions;

mod domain;
mod infrastructure;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://app:app@localhost/app?sslmode=disable")
        .await?;

    let user_repo = UserRepository { connection: pool };
    println!("{:?}", user_repo.get_all().await?);

    Ok(())
}
