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
            Err(err) => {
                eprintln!("** Warning loading configuration: {} **", err);
                config::Config::default()
            }
        },
        None => {
            eprintln!("** Warning invalid configuration file path **");
            config::Config::default()
        }
    };

    cli::run(&config);
}
