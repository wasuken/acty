mod cli;
mod config;
mod list;
mod log_entry;
mod logger;

fn main() {
    let config = match config::Config::from_file("config.toml") {
        Ok(config) => config,
        Err(err) => {
            eprintln!("** Warning loading configuration: {} **", err);
            config::Config::default()
        }
    };

    cli::run(&config);
}
