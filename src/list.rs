use crate::config::Config;
use crate::log_entry::LogEntry;
use crate::util::{should_include_log, sort_tags};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn list_logs(
    config: &Config,
    date: Option<String>,
    range: Option<i64>,
    tags: Vec<String>,
    search: Option<String>,
) {
    let file = File::open(config.log_file.to_string()).expect("Unable to open the log file");
    let reader = BufReader::new(file);

    println!("ID\tDate\t\tTime\t\tTags");
    println!("--\t----\t\t----\t\t-------\t\t----");

    for (index, line) in reader.lines().enumerate() {
        let log_entry: LogEntry =
            serde_json::from_str(&line.expect("Unable to read the log entry"))
                .expect("Unable to deserialize the log entry");

        if !should_include_log(&log_entry, &date, &range, &tags, &search) {
            continue;
        }

        let sorted_tags = sort_tags(log_entry.tags.clone());

        println!(
            "{}\t{}\t{}\t{}\nContent> {}",
            index + 1,
            log_entry.timestamp.format("%Y-%m-%d"),
            log_entry.timestamp.format("%H:%M:%S"),
            sorted_tags.join(", "),
            log_entry.content,
        );
    }
}
