use clap::{App, Arg, SubCommand};

use crate::list;
use crate::logger;

pub fn run() {
    let matches = App::new("Action Logger")
        .version("0.1.0")
        .author("Your Name")
        .about("A simple action logging tool")
        .subcommand(
            SubCommand::with_name("log")
                .about("Log a new action")
                .arg(
                    Arg::with_name("content")
                        .help("The content of the log entry")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("tags")
                        .short("t")
                        .long("tags")
                        .value_name("TAGS")
                        .help("Comma-separated list of tags")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("List log entries")
                .arg(
                    Arg::with_name("date")
                        .short("d")
                        .long("date")
                        .value_name("DATE")
                        .help("Filter logs by date (YYYY-MM-DD)")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("range")
                        .short("r")
                        .long("range")
                        .value_name("DAYS")
                        .help("Filter logs by date range (in days)")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("tags")
                        .short("t")
                        .long("tags")
                        .value_name("TAGS")
                        .help("Filter logs by tags (comma-separated)")
                        .takes_value(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("log", Some(sub_matches)) => {
            let content = sub_matches.value_of("content").unwrap().to_string();
            let tags: Vec<String> = sub_matches
                .value_of("tags")
                .unwrap_or("")
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            logger::log_action(content, tags);
        }
        ("list", Some(sub_matches)) => {
            let date = sub_matches.value_of("date").map(|d| d.to_string());
            let range = sub_matches
                .value_of("range")
                .map(|r| r.parse::<i64>().expect("Invalid range"));
            let tags: Vec<String> = sub_matches
                .value_of("tags")
                .unwrap_or("")
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            list::list_logs(date, range, tags);
        }
        _ => {
            println!("No subcommand was used");
        }
    }
}
