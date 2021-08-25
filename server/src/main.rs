extern crate dotenv;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use tokio_compat_02::FutureExt;

start_server().compat().await;

mod auth;
mod config;
mod errors;
mod handlers;
mod entities;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let admin_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let api_url = std::env::var("API_DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = config::migrate_and_config_db(&admin_url, &api_url)
        .await
        .expect("Error migrating database");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(pool.clone())
            .configure(handlers::init)
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}
