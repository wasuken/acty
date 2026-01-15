use dirs::home_dir;
use std::path::PathBuf;
mod cli;
mod config;
mod list;
mod log_entry;
mod logger;
mod markdown;
mod tags;
mod util;

fn main() {
    let config_path = match home_dir() {
        Some(home) => home.join(".config/acty/config.toml"),
        None => {
            eprintln!("** Warning: Unable to determine home directory **");
            PathBuf::from("config.toml")
        }
    };
    let config = match config_path.to_str() {
        Some(path) => match config::Config::from_file(path) {
            Ok(config) => config,
            Err(_) => {
                // If file not found or error, use default without noise.
                // Assuming most users won't have a config file initially.
                config::Config::default()
            }
        },
        None => config::Config::default(),
    };

    cli::run(&config);
}
