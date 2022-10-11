pub mod configuration;
pub mod infrastructure;
pub mod telemetry;

#[cfg(feature = "sqlite")]
pub use sqlx::sqlite::{SqlitePool as DbPool, SqliteRow as DbRow};

#[cfg(feature = "sqlite")]
pub async fn connect(db_url: &str) -> Result<DbPool, sqlx::Error> {
    DbPool::connect(db_url).await
}
