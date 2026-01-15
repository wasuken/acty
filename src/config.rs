use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

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
        let log_file = dirs::data_local_dir()
            .map(|path| path.join("acty").join("action_log.json"))
            .unwrap_or_else(|| PathBuf::from("action_log.json"));

        Config {
            log_file: log_file.to_string_lossy().into_owned(),
        }
    }
}
