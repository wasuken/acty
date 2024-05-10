use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub log_file: String,
}

impl Config {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(file_path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            log_file: "action_log.json".to_string(),
        }
    }
}
