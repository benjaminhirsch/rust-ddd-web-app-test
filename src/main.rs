use crate::config::Config;
use crate::domain::entity::user::User;
use crate::infrastructure::handler::home::index;
use crate::infrastructure::repository::user::UserRepository;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{middleware::Logger, web, App, HttpServer};
use color_eyre::Result;
use sqlx::postgres::PgPoolOptions;
use tracing::{info, instrument};

mod config;
mod domain;
mod infrastructure;

#[actix_web::main]
#[instrument]
async fn main() -> Result<()> {
    let config = Config::from_env().expect("Server configuration");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(
            config
                .get_string("database_url")
                .expect("Missing database url")
                .as_str(),
        )
        .await;

    let _ = UserRepository::new(pool.as_ref().unwrap());

    let host = config.get_string("host").expect("Missing host name");
    let port = config.get_int("port").expect("Missing port");

    info!("Starting server at http://{}:{}/", &host, &port);

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
    .bind(format!("{}:{}", host, port))?
    .run()
    .await?;
    Ok(())
}
