use crate::log_entry::LogEntry;
use chrono::Local;
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::Write;

pub fn log_action(content: String, tags: Vec<String>) {
    let unique_tags: Vec<String> = tags
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    let log_entry = LogEntry {
        timestamp: Local::now(),
        content,
        tags: unique_tags,
    };

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("action_log.json")
        .expect("Unable to open or create the log file");

    let serialized = serde_json::to_string(&log_entry).expect("Unable to serialize the log entry");
    writeln!(file, "{}", serialized).expect("Unable to write to the log file");

    println!("Log entry added successfully!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_log_action() {
        let content = "Test content".to_string();
        let tags = vec!["tag1".to_string(), "tag2".to_string()];

        log_action(content, tags);

        let file_content = fs::read_to_string("action_log.json").unwrap();
        let log_entries: Vec<LogEntry> = file_content
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();

        assert_eq!(log_entries.len(), 1);
        assert_eq!(log_entries[0].content, "Test content");
        assert_eq!(log_entries[0].tags, vec!["tag1", "tag2"]);

        fs::remove_file("action_log.json").unwrap();
    }
}
