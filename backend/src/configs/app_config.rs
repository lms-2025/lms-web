use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub email: String,
    pub email_password: String,
    pub database_url: String,
    pub database_name: String,
    pub jwt_secret: String,
    pub port: String,
    pub rust_log: String,
    pub redis_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        Config {
            email: env::var("EMAIL").expect("EMAIL environment variable is not set"),
            email_password: env::var("EMAIL_PASSWORD")
                .expect("EMAIL_PASSWORD environment variable is not set"),
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL environment variable is not set"),
            database_name: env::var("DATABASE_NAME")
                .expect("DATABASE_NAME environment variable is not set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET environment variable is not set"),
            port: env::var("PORT").unwrap_or_else(|_| "8080".to_string()),
            rust_log: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
            redis_url: env::var("REDIS_URL").expect("REDIS_URL environment variable is not set"),
        }
    }
}
