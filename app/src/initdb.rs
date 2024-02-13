use anyhow::{anyhow, Context};
use data::sqlite_repository::SqliteRepository;
use domain::{Password, Repository};
use mimir::application::get_connection_pool;
use mimir::configuration::Settings;
use secrecy::Secret;
use sqlx::migrate::MigrateDatabase;
use sqlx::Sqlite;
use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let admin_pass = Secret::new(
        std::env::args()
            .collect::<Vec<_>>()
            .get(1)
            .ok_or(anyhow!("USAGE: initdb ADMIN_PASSWORD"))?
            .to_string(),
    );
    let config = Settings::read_from_file().context("Failed to read configuration.")?;
    create_database_only(config, admin_pass).await?;
    println!("Initialized a new database file");
    Ok(())
}

async fn create_database_only(config: Settings, admin_pass: Secret<String>) -> anyhow::Result<()> {
    if Sqlite::database_exists(&config.database.url).await? {
        println!("Warning: database file exists, re-creating");
        Sqlite::drop_database(&config.database.url).await?;
    }
    let hashed = Password::parse(admin_pass)?.hash_password()?;
    let pool = get_connection_pool(&config.database, true).await?;

    sqlx::migrate::Migrator::new(Path::new("./migrations"))
        .await
        .unwrap()
        .run(&pool)
        .await
        .context("Failed to apply migrations")?;

    SqliteRepository::new(pool)
        .update_admin_password(&hashed)
        .await?;
    Ok(())
}
