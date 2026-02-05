use std::env;
use validator::Validate;

#[derive(Debug, Validate)]
pub struct Config {
    #[validate(url(message = "DATABASE_URL is not a valid URL"))]
    pub database_url: String,
    #[validate(length(min = 1, message = "HOST cannot be empty"))]
    pub host: String,
    #[validate(range(min = 1, max = 65535, message = "PORT must be between 1 and 65535"))]
    pub port: u16,
}

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
        };

        config.validate().expect("Invalid configuration");
        config
    }
}
