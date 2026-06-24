use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppConfig {
    pub app_name: String,
    pub port: u16,
    pub database_url: String,
    #[serde(default)]
    pub debug: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigValidationError {
    EmptyAppName,
    EmptyDatabaseUrl,
    InvalidPort,
}

impl AppConfig {
    pub fn validate(&self) -> Result<(), ConfigValidationError> {
        if self.app_name.trim().is_empty() {
            return Err(ConfigValidationError::EmptyAppName);
        }

        if self.database_url.trim().is_empty() {
            return Err(ConfigValidationError::EmptyDatabaseUrl);
        }

        if self.port == 0 {
            return Err(ConfigValidationError::InvalidPort);
        }

        Ok(())
    }
}

pub fn load_config(input: &str) -> Result<AppConfig, serde_json::Error> {
    serde_json::from_str(input)
}
