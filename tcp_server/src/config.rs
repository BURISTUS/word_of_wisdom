use config::ConfigError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Application {
    pub application_config: ApplicationConfig,
    pub settings_config: SettingsConfig,
}

#[derive(Debug, Deserialize)]
pub struct ApplicationConfig {
    pub port: i32,
    pub host: String,
}

#[derive(Debug, Deserialize)]
pub struct SettingsConfig {
    pub max_connections: i32,
    pub request_timeout: u64,
    pub target_difficulty: usize,
}

pub fn get_config() -> Result<Application, ConfigError> {
    let mut path = config::Config::default();
    path.merge(config::File::with_name("config"))?;
    path.try_into()
}
