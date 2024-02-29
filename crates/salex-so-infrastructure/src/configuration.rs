// #[cfg(feature = "mysql")]
// use sqlx::mysql::MySqlConnectOptions;
// #[cfg(feature = "sqlite")]
// use sqlx::sqlite::MySqlConnectOptions as DbConnectOptions;
// use sqlx::ConnectOptions;

// #[derive(serde::Deserialize)]
// pub struct Settings {
//     pub database: DatabaseSettings,
//     pub application: ApplicationSettings,
// }

// #[derive(serde::Deserialize)]
// pub struct ApplicationSettings {
//     pub port: u16,
//     pub host: String,
// }

// #[derive(serde::Deserialize, Clone)]
// pub struct DatabaseSettings {
//     pub username: String,
//     pub password: Secret<String>,
//     #[serde(deserialize_with = "deserialize_number_from_string")]
//     pub port: u16,
//     pub host: String,
//     pub database_name: String,
//     pub require_ssl: bool,
// }

// impl DatabaseSettings {
//     pub fn without_db(&self) -> MySqlConnectOptions {
//         MySqlConnectOptions::new()
//             .host(&self.host)
//             .username(&self.username)
//             .password(&self.password.expose_secret())
//             .port(self.port)
//             .charset("utf8mb4")
//         // .collation("utf8mb4")
//     }

//     pub fn with_db(&self) -> MySqlConnectOptions {
//         self.without_db().database(&self.database_name)
//     }

//     pub fn connection_string_without_db(&self) -> Secret<String> {
//         Secret::new(format!(
//             "mysql://{}:{}@{}:{}",
//             self.username,
//             self.password.expose_secret(),
//             self.host,
//             self.port,
//         ))
//     }
// }

// pub fn get_configuration() -> Result<Settings, config::ConfigError> {
//     use config::{Config, File};

//     let base_path = std::env::current_dir().expect("Failed to determine current directory");
//     let configuration_directory = base_path.join("configuration");

//     // Detect the running environment
//     let environment: Environment = std::env::var("APP_ENVIRONMENT")
//         .unwrap_or_else(|_| "local".into())
//         .try_into()
//         .expect("Failed to parse APP_ENVIRONMENT");

//     let settings = Config::builder()
//         // Read the "default" configuration file
//         .add_source(File::with_name(configuration_directory.join("base").as_os_str().to_str().expect("")))
//         // Layer on the environment-specific values.
//         .add_source(
//             File::with_name(configuration_directory.join(environment.as_str()).as_os_str().to_str().expect("")).required(true),
//         )
//         .add_source(config::Environment::with_prefix("app").separator("__"))
//         .build()?;
//     settings.try_deserialize()
// }

// pub enum Environment {
//     Local,
//     Production,
// }

// impl Environment {
//     pub fn as_str(&self) -> &'static str {
//         match self {
//             Environment::Local => "local",
//             Environment::Production => "production",
//         }
//     }
// }

// impl TryFrom<String> for Environment {
//     type Error = String;

//     fn try_from(s: String) -> Result<Self, Self::Error> {
//         match s.to_lowercase().as_str() {
//             "local" => Ok(Self::Local),
//             "production" => Ok(Self::Production),
//             other => Err(format!(
//                 "{} is not a supported environment. Use either `local` or `production`.",
//                 other
//             )),
//         }
//     }
// }
