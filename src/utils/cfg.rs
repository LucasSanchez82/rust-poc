use std::{env, path::PathBuf, sync::OnceLock};
use tracing::Level;
use validator::Validate;

#[derive(Debug, Validate)]
pub struct Config {
    #[validate(url(message = "DATABASE_URL is not a valid URL"))]
    pub database_url: String,
    #[validate(length(min = 1, message = "HOST cannot be empty"))]
    pub host: String,
    #[validate(range(min = 1, max = 65535, message = "PORT must be between 1 and 65535"))]
    pub port: u16,
    pub log_level: tracing::Level,
    #[validate(skip)]
    pub dockerfile_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
static CONFIG: OnceLock<Config> = OnceLock::new();

impl Config {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();

        let config = Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT must be a valid number"),
            log_level: Self::get_log_level(),
            dockerfile_path: PathBuf::from(
                env::var("DOCKERFILE_PATH")
                    .unwrap_or("./dockerfiles/".to_owned())
                    .as_str(),
            ),
        };

        config.validate().expect("Invalid configuration");
        config
    }

    pub fn get_log_level() -> Level {
        match env::var("LOG_LEVEL")
            .unwrap_or("info".to_owned())
            .to_lowercase()
            .as_str()
        {
            "error" => Level::ERROR,
            "info" => Level::INFO,
            "warn" => Level::WARN,
            "debug" => Level::DEBUG,
            "trace" => Level::TRACE,
            _ => Level::INFO,
        }
    }

    pub fn instance() -> &'static Config {
        CONFIG.get_or_init(Config::new)
    }
}
