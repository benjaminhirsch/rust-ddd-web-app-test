use crate::domain::entity::user::User;
use crate::domain::repository::user::UserRepositoryTrait;
use crate::infrastructure::handler::home::index;
use crate::infrastructure::repository::user::UserRepository;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{middleware::Logger, web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;

mod domain;
mod infrastructure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://app:app@localhost/app?sslmode=disable")
        .await;

    let user_repo1 = UserRepository::new(pool.as_ref().unwrap());
    let user_repo2 = UserRepository::new(pool.as_ref().unwrap());

    println!("{:?}", user_repo1.get_all().await);
    println!("{:?}", user_repo2.get_all().await);
    println!(
        "{:?}",
        user_repo1
            .get_by_id("4fee8e7a-f840-11ec-b939-0242ac120002".to_string())
            .await
    );
    println!(
        "{:?}",
        user_repo2
            .get_by_id("4fee8e7a-f840-11ec-b939-0242ac120002".to_string())
            .await
    );

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .build(),
            )
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.01", 8080))?
    .run()
    .await
}
