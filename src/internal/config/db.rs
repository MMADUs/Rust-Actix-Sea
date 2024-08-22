use std::time::Duration;
use dotenv::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

use migration::{Migrator, MigratorTrait};

pub async fn connect_to_db() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in env");

    let mut opt = ConnectOptions::new(database_url);

    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let db_conn = Database::connect(opt).await?;

    Migrator::up(&db_conn, None).await?;

    Ok(db_conn)
}