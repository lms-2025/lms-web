use crate::configs::app_config::Config;
use once_cell::sync::Lazy;

// Define the global CONFIG variable
pub static CONFIG: Lazy<Config> = Lazy::new(Config::from_env);
