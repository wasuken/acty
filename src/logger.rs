use crate::config::Config;
use crate::log_entry::LogEntry;
use chrono::Local;
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::Write;

pub fn log_action(config: &Config, content: String, tags: Vec<String>) {
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

    let path = std::path::Path::new(&config.log_file);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("Unable to create log directory");
    }

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(config.log_file.to_string())
        .expect("Unable to open or create the log file");

    let serialized = serde_json::to_string(&log_entry).expect("Unable to serialize the log entry");
    writeln!(file, "{}", serialized).expect("Unable to write to the log file");

    println!("Log entry added successfully!");
}

pub fn delete_log(config: &Config, index: usize) {
    let path = std::path::Path::new(&config.log_file);
    if !path.exists() {
        eprintln!("Log file not found.");
        return;
    }

    let contents = std::fs::read_to_string(path).expect("Unable to read log file");
    let mut lines: Vec<&str> = contents.lines().collect();

    if index == 0 || index > lines.len() {
        eprintln!("Invalid ID: {}. Use 'list' command to see available IDs.", index);
        return;
    }

    lines.remove(index - 1);

    let mut file = std::fs::File::create(path).expect("Unable to open log file for writing");
    for line in lines {
        writeln!(file, "{}", line).expect("Unable to write to log file");
    }

    println!("Log entry {} deleted successfully.", index);
}

pub fn update_log(config: &Config, index: usize, new_content: String, new_tags: Option<Vec<String>>) {
    let path = std::path::Path::new(&config.log_file);
    if !path.exists() {
        eprintln!("Log file not found.");
        return;
    }

    let contents = std::fs::read_to_string(path).expect("Unable to read log file");
    let lines: Vec<&str> = contents.lines().collect();

    if index == 0 || index > lines.len() {
        eprintln!("Invalid ID: {}. Use 'list' command to see available IDs.", index);
        return;
    }

    let mut log_entry: LogEntry = serde_json::from_str(lines[index - 1]).expect("Unable to deserialize log entry");
    
    log_entry.content = new_content;
    
    if let Some(tags) = new_tags {
        let unique_tags: Vec<String> = tags
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        log_entry.tags = unique_tags;
    }

    let serialized = serde_json::to_string(&log_entry).expect("Unable to serialize updated log entry");
    // We need to own the strings to put them back into the vector if we want to write them all at once
    // Alternatively, we can just write the file directly from the iteration logic, but let's stick to the vector approach for simplicity in replacing.
    // However, lines is Vec<&str>, so we can't replace a &str with a String easily without reconstructing the whole buffer or using a different approach.
    // Let's read, modify the specific entry in a list of Strings, and write back.
    
    let mut lines_owned: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    lines_owned[index - 1] = serialized;

    let mut file = std::fs::File::create(path).expect("Unable to open log file for writing");
    for line in lines_owned {
        writeln!(file, "{}", line).expect("Unable to write to log file");
    }

    println!("Log entry {} updated successfully.", index);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_log_action() {
        let content = "Test content".to_string();
        let tags = vec!["tag1".to_string(), "tag2".to_string()];
        let test_json_path = "action_log_test.json";
        let config = Config {
            log_file: test_json_path.to_string(),
        };

        log_action(&config, content, tags);

        let file_content = fs::read_to_string(test_json_path).unwrap();
        let log_entries: Vec<LogEntry> = file_content
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();

        assert_eq!(log_entries.len(), 1);
        assert_eq!(log_entries[0].content, "Test content");
        assert!(log_entries[0].tags.contains(&"tag1".to_string()));
        assert!(log_entries[0].tags.contains(&"tag2".to_string()));

        fs::remove_file(test_json_path).unwrap();
    }

    #[test]
    fn test_delete_log() {
        let test_json_path = "action_log_delete_test.json";
        let config = Config {
            log_file: test_json_path.to_string(),
        };

        // Ensure clean state
        if std::path::Path::new(test_json_path).exists() {
            fs::remove_file(test_json_path).unwrap();
        }

        log_action(&config, "Entry 1".to_string(), vec![]);
        log_action(&config, "Entry 2".to_string(), vec![]);
        log_action(&config, "Entry 3".to_string(), vec![]);

        // Delete the second entry (index 2)
        delete_log(&config, 2);

        let file_content = fs::read_to_string(test_json_path).unwrap();
        let log_entries: Vec<LogEntry> = file_content
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();

        assert_eq!(log_entries.len(), 2);
        assert_eq!(log_entries[0].content, "Entry 1");
        assert_eq!(log_entries[1].content, "Entry 3");

        // Try deleting invalid index
        delete_log(&config, 99);
        let file_content_after_invalid = fs::read_to_string(test_json_path).unwrap();
        assert_eq!(file_content, file_content_after_invalid);

        fs::remove_file(test_json_path).unwrap();
    }

    #[test]
    fn test_update_log() {
        let test_json_path = "action_log_update_test.json";
        let config = Config {
            log_file: test_json_path.to_string(),
        };

        // Ensure clean state
        if std::path::Path::new(test_json_path).exists() {
            fs::remove_file(test_json_path).unwrap();
        }

        log_action(&config, "Old Content".to_string(), vec!["old_tag".to_string()]);

        // Update content and tags
        update_log(
            &config, 
            1, 
            "New Content".to_string(), 
            Some(vec!["new_tag1".to_string(), "new_tag2".to_string()])
        );

        let file_content = fs::read_to_string(test_json_path).unwrap();
        let log_entries: Vec<LogEntry> = file_content
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();

        assert_eq!(log_entries.len(), 1);
        assert_eq!(log_entries[0].content, "New Content");
        assert!(log_entries[0].tags.contains(&"new_tag1".to_string()));
        assert!(log_entries[0].tags.contains(&"new_tag2".to_string()));
        assert!(!log_entries[0].tags.contains(&"old_tag".to_string()));

        // Update only content
        update_log(&config, 1, "New Content 2".to_string(), None);
        let file_content_2 = fs::read_to_string(test_json_path).unwrap();
        let log_entries_2: Vec<LogEntry> = file_content_2
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        
        assert_eq!(log_entries_2[0].content, "New Content 2");
        // Tags should remain unchanged from previous update
        assert!(log_entries_2[0].tags.contains(&"new_tag1".to_string()));

        fs::remove_file(test_json_path).unwrap();
    }
}
