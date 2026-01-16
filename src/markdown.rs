use crate::config::Config;
use crate::log_entry::LogEntry;
use crate::util::{should_include_log, sort_tags};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn output_markdown_table(
    config: &Config,
    date: Option<String>,
    range: Option<i64>,
    tags: Vec<String>,
    search: Option<String>,
    use_archive: bool,
) {
    let path = if use_archive {
        std::path::Path::new(&config.log_file).with_file_name("archive.json")
    } else {
        std::path::PathBuf::from(&config.log_file)
    };

    let file = match File::open(&path) {
        Ok(f) => f,
        Err(_) => {
            println!("No logs found.");
            return;
        }
    };
    let reader = BufReader::new(file);

    println!("| Date       | Time     | Tags          | Content |");
    println!("|------------|----------|---------------|---------|");

    for line in reader.lines() {
        let log_entry: LogEntry =
            serde_json::from_str(&line.expect("Unable to read the log entry"))
                .expect("Unable to deserialize the log entry");

        if !should_include_log(&log_entry, &date, &range, &tags, &search) {
            continue;
        }

        let sorted_tags = sort_tags(log_entry.tags.clone());

        println!(
            "| {} | {} | {} | {} |",
            log_entry.timestamp.format("%Y-%m-%d"),
            log_entry.timestamp.format("%H:%M:%S"),
            sorted_tags.join(", "),
            log_entry.content.replace("|", "\\|"),
        );
    }
}
