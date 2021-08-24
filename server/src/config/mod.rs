use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, PgPool};

pub type Pool = PgPool;

pub async fn migrate_and_config_db(url: &str) -> Result<Pool, Error> {
    info!("Migrating database...");
    let migrations: Migrator = sqlx::migrate!();
    let pool = PgPoolOptions::new().connect(url).await?;
    migrations.run(&pool).await?;
    Ok(pool)
}
