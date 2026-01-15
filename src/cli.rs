use crate::config::Config;
use clap::{App, Arg, SubCommand};

use crate::list;
use crate::logger;
use crate::markdown;
use crate::tags;

pub fn run(config: &Config) {
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
                )
                .arg(
                    Arg::with_name("search")
                        .short("s")
                        .long("search")
                        .value_name("KEYWORD")
                        .help("Filter logs by keyword")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Delete log entries by ID")
                .arg(
                    Arg::with_name("id")
                        .help("The IDs of the log entries to delete (space separated)")
                        .required(true)
                        .multiple(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("edit")
                .about("Edit a log entry by ID")
                .arg(
                    Arg::with_name("id")
                        .help("The ID of the log entry to edit")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("content")
                        .help("The new content of the log entry")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::with_name("tags")
                        .short("t")
                        .long("tags")
                        .value_name("TAGS")
                        .help("Comma-separated list of new tags (overwrites existing tags)")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("tags")
                .about("List all used tags and their usage counts"),
        )
        .subcommand(
            SubCommand::with_name("mdt")
                .about("output log entries in markdown table format")
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
                )
                .arg(
                    Arg::with_name("search")
                        .short("s")
                        .long("search")
                        .value_name("KEYWORD")
                        .help("Filter logs by keyword")
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
            logger::log_action(&config, content, tags);
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
            let search = sub_matches.value_of("search").map(|s| s.to_string());
            list::list_logs(&config, date, range, tags, search);
        }
        ("delete", Some(sub_matches)) => {
            let ids_result: Result<Vec<usize>, _> = sub_matches
                .values_of("id")
                .unwrap()
                .map(|id_str| id_str.parse::<usize>())
                .collect();

            match ids_result {
                Ok(ids) => logger::delete_logs(&config, ids),
                Err(_) => eprintln!("Invalid ID found. Please provide numeric IDs."),
            }
        }
        ("edit", Some(sub_matches)) => {
            let id_str = sub_matches.value_of("id").unwrap();
            let content = sub_matches.value_of("content").unwrap().to_string();
            let tags = sub_matches.value_of("tags").map(|t| {
                t.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            });

            match id_str.parse::<usize>() {
                Ok(id) => logger::update_log(&config, id, content, tags),
                Err(_) => eprintln!("Invalid ID: {}", id_str),
            }
        }
        ("tags", Some(_)) => {
            tags::list_tags(&config);
        }
        ("mdt", Some(sub_matches)) => {
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
            let search = sub_matches.value_of("search").map(|s| s.to_string());
            markdown::output_markdown_table(&config, date, range, tags, search);
        }
        _ => {
            println!("No subcommand was used");
        }
    }
}
