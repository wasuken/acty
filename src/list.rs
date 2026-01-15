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

    println!("ID\tTime\t\tGap\t\tTags\t\tContent");
    println!("--\t----\t\t---\t\t----\t\t-------");

    let mut previous_time: Option<chrono::DateTime<chrono::Local>> = None;
    let mut total_duration_seconds: i64 = 0;

    for (index, line) in reader.lines().enumerate() {
        let log_entry: LogEntry =
            serde_json::from_str(&line.expect("Unable to read the log entry"))
                .expect("Unable to deserialize the log entry");

        if !should_include_log(&log_entry, &date, &range, &tags, &search) {
            continue;
        }

        let gap_str = match previous_time {
            Some(prev) => {
                let duration = log_entry.timestamp - prev;
                let seconds = duration.num_seconds();
                total_duration_seconds += seconds;
                
                if seconds < 60 {
                    format!("{}s", seconds)
                } else if seconds < 3600 {
                    format!("{}m", seconds / 60)
                } else {
                    let hours = seconds / 3600;
                    let minutes = (seconds % 3600) / 60;
                    format!("{}h {}m", hours, minutes)
                }
            }
            None => "-".to_string(),
        };

        let sorted_tags = sort_tags(log_entry.tags.clone());

        println!(
            "{}\t{}\t{}\t{}\t{}",
            index + 1,
            log_entry.timestamp.format("%Y-%m-%d %H:%M"),
            gap_str,
            sorted_tags.join(", "),
            log_entry.content,
        );

        previous_time = Some(log_entry.timestamp);
    }

    if total_duration_seconds > 0 {
        let hours = total_duration_seconds / 3600;
        let minutes = (total_duration_seconds % 3600) / 60;
        let seconds = total_duration_seconds % 60;
        println!("\nTotal Duration: {}h {}m {}s", hours, minutes, seconds);
    }
}
