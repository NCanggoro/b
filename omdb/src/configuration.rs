use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use secrecy::{Secret, ExposeSecret};
use sqlx::postgres::PgConnectOptions;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: AppSettings
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub db_name: String,
    pub with_ssl: bool
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Failed to read current directory");
    let configuration_directory = base_path.join("configuration");
    
    settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");
    
    settings.merge(
        config::File::from(configuration_directory.join(environment.as_str())).required(true)
    )?;

    settings.merge(config::Environment::with_prefix("app").separator("__"))?;

    settings.try_into()
}

pub enum Environment {
    Local,
    Production
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production"
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not supported environment.Use local or production",
                other
            ))
        }
    }
}

impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.db_name)
    }
}
