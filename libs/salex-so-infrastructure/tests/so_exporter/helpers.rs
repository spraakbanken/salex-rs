use once_cell::sync::Lazy;
use salex_so::application::{exporter::SoExporter, repositories::SoOrdRepository};
use salex_so_infrastructure::{
    connect,
    infrastructure::{queries::SqlGetBForms, repositories::SqlSoOrdRepository},
    telemetry::{get_subscriber, init_subscriber},
    DbPool,
};
use std::error::Error;
use std::sync::Arc;

pub struct TestApp {
    pub pool: DbPool,
    pub so_exporter: SoExporter,
    pub so_ord_repo: Arc<dyn SoOrdRepository>,
}

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

impl TestApp {
    pub async fn try_new() -> Result<Self, Box<dyn Error>> {
        Lazy::force(&TRACING);
        // let configuration = get_configuration().expect("Failed to read configuration.");
        // configuration.database.database_name = uuid::Uuid::new_v4().to_string();
        let pool = configure_database("sqlite::memory:").await;

        let so_ord_repo = Arc::new(SqlSoOrdRepository::new(pool.clone()));
        let get_bforms = Arc::new(SqlGetBForms::new(pool.clone()));
        let so_exporter = SoExporter::new(so_ord_repo.clone(), get_bforms);
        Ok(Self {
            pool,
            so_exporter,
            so_ord_repo,
        })
    }
}

async fn configure_database(config: &str) -> DbPool {
    // use sqlx::migrate::MigrateDatabase;

    // Create database
    // let mut connection = PgConnection::connect_with(&config.without_db())
    //     .await
    //     .expect("Failed to connect");
    // connection
    //     .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
    //     .await
    //     .expect("Failed to create database");

    // Migrate database
    let pool = connect(config).await.expect("Failed to connect to Mariadb");
    // sqlx::migrate!("./migrations")
    //     .run(&pool)
    //     .await
    //     .expect("Failed to run migrations!");
    pool
}
