use actix_web::web::Data;
use sea_orm::{DatabaseConnection, SqlxPostgresConnector};
use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;

use crate::errors::ApiError;

pub type Pool = Data<DatabaseConnection>;

pub async fn migrate_and_config_db(admin_url: &str, api_url: &str) -> Result<Pool, ApiError> {
    info!("Migrating database...");
    let migrations: Migrator = sqlx::migrate!();
    let admin_pool = PgPoolOptions::new().connect(admin_url).await?;
    migrations.run(&admin_pool).await?;
    admin_pool.close().await;
    let api_pool: DatabaseConnection = SqlxPostgresConnector::connect(api_url).await?;
    Ok(Data::new(api_pool))
}
