pub mod configuration;
pub mod infrastructure;
pub mod telemetry;

#[cfg(feature = "mysql")]
pub use sqlx::mysql::{MySqlPool as DbPool, MySqlRow as DbRow};

#[cfg(feature = "mysql")]
pub async fn connect(db_url: &str) -> Result<DbPool, sqlx::Error> {
    DbPool::connect(db_url).await
}
#[cfg(feature = "sqlite")]
pub use sqlx::sqlite::{SqlitePool as DbPool, SqliteRow as DbRow};

#[cfg(feature = "sqlite")]
pub async fn connect(db_url: &str) -> Result<DbPool, sqlx::Error> {
    DbPool::connect(db_url).await
}
