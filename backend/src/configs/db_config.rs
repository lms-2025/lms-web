use super::global_config::CONFIG;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub name: String,
}

impl DatabaseConfig {
    pub fn new() -> Self {
        let url: String = CONFIG.database_url.clone();
        let database_name: String = CONFIG.database_name.clone();
        DatabaseConfig {
            url,
            name: database_name,
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self::new()
    }
}

pub enum DatabaseCollection {
    Users,
    Posts,
    Comments,
}
