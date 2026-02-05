use sea_orm::ExprTrait;

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn new() -> Self {
        dotenvy::dotenv_override().unwrap();
        let mut missing_envs: Vec<String> = Vec::new();

        let database_url_key = "DATABASE_URL";
        let database_url = std::env::var(database_url_key).unwrap_or_else(|_| {
            missing_envs.push(database_url_key.to_string());
            "".to_string()
        });

        if missing_envs.len() > 0 {
            panic!(
                "Error: You should set all prerequisites environment variables:\n\t- {}",
                missing_envs.join("\n\t- ")
            )
        }

        return Self { database_url };
    }
}
