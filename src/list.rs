use crate::log_entry::LogEntry;
use chrono::{Local, NaiveDate};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn list_logs(date: Option<String>, range: Option<i64>, tags: Vec<String>) {
    let file = File::open("action_log.json").expect("Unable to open the log file");
    let reader = BufReader::new(file);

    println!("Date\t\tTime\t\tContent\t\tTags");
    println!("----\t\t----\t\t-------\t\t----");

    for line in reader.lines() {
        let log_entry: LogEntry =
            serde_json::from_str(&line.expect("Unable to read the log entry"))
                .expect("Unable to deserialize the log entry");

        if let Some(d) = &date {
            let filter_date =
                NaiveDate::parse_from_str(d, "%Y-%m-%d").expect("Invalid date format");
            if log_entry.timestamp.date_naive() != filter_date {
                continue;
            }
        }

        if let Some(r) = range {
            let log_date = log_entry.timestamp.date_naive();
            let now = Local::now().date_naive();
            if (now - log_date).num_days() > r {
                continue;
            }
        }

        if !tags.is_empty() && !tags.iter().all(|t| log_entry.tags.contains(t)) {
            continue;
        }

        println!(
            "{}\t{}\t{}\t{}",
            log_entry.timestamp.format("%Y-%m-%d"),
            log_entry.timestamp.format("%H:%M:%S"),
            log_entry.content,
            log_entry.tags.join(", ")
        );
    }
}