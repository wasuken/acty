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

pub fn delete_logs(config: &Config, ids: Vec<usize>) {
    let path = std::path::Path::new(&config.log_file);
    if !path.exists() {
        eprintln!("Log file not found.");
        return;
    }

    let contents = std::fs::read_to_string(path).expect("Unable to read log file");
    let mut lines: Vec<&str> = contents.lines().collect();

    // Sort IDs in descending order to prevent shifting indices from affecting subsequent deletions
    let mut sorted_ids = ids;
    sorted_ids.sort_by(|a, b| b.cmp(a));
    sorted_ids.dedup(); // Remove duplicates

    let mut deleted_count = 0;
    for index in sorted_ids {
        if index == 0 || index > lines.len() {
            eprintln!("Skipping invalid ID: {}", index);
            continue;
        }
        lines.remove(index - 1);
        deleted_count += 1;
    }

    let mut file = std::fs::File::create(path).expect("Unable to open log file for writing");
    for line in lines {
        writeln!(file, "{}", line).expect("Unable to write to log file");
    }

    println!("{} log entry(ies) deleted successfully.", deleted_count);
}

pub fn get_log_count(config: &Config) -> usize {
    let path = std::path::Path::new(&config.log_file);
    if !path.exists() {
        return 0;
    }
    let contents = std::fs::read_to_string(path).unwrap_or_default();
    contents.lines().count()
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

pub fn copy_log(config: &Config, index: usize, new_content: Option<String>) {
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

    let original_entry: LogEntry = serde_json::from_str(lines[index - 1]).expect("Unable to deserialize log entry");
    
    let log_entry = LogEntry {
        timestamp: Local::now(),
        content: new_content.unwrap_or(original_entry.content),
        tags: original_entry.tags,
    };

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(config.log_file.to_string())
        .expect("Unable to open or create the log file");

    let serialized = serde_json::to_string(&log_entry).expect("Unable to serialize the log entry");
    writeln!(file, "{}", serialized).expect("Unable to write to the log file");

    println!("Log entry {} copied to new entry successfully!", index);
}

pub fn archive_logs(config: &Config, days: i64) {
    let path = std::path::Path::new(&config.log_file);
    if !path.exists() {
        println!("No logs to archive.");
        return;
    }

    let contents = std::fs::read_to_string(path).expect("Unable to read log file");
    let mut keep_logs: Vec<String> = Vec::new();
    let mut archive_logs: Vec<String> = Vec::new();

    let cutoff_date = Local::now().date_naive() - chrono::Duration::days(days);

    for line in contents.lines() {
        if let Ok(entry) = serde_json::from_str::<LogEntry>(line) {
            if entry.timestamp.date_naive() < cutoff_date {
                archive_logs.push(line.to_string());
            } else {
                keep_logs.push(line.to_string());
            }
        }
    }

    if archive_logs.is_empty() {
        println!("No logs found older than {} days.", days);
        return;
    }

    // Append to archive file
    let archive_path = path.with_file_name("archive.json");
    let mut archive_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(archive_path)
        .expect("Unable to open archive file");

    for line in &archive_logs {
        writeln!(archive_file, "{}", line).expect("Unable to write to archive file");
    }

    // Overwrite main log file
    let mut main_file = std::fs::File::create(path).expect("Unable to open log file for writing");
    for line in &keep_logs {
        writeln!(main_file, "{}", line).expect("Unable to write to log file");
    }

    println!("Archived {} logs older than {} days.", archive_logs.len(), days);
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
    fn test_delete_logs() {
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
        log_action(&config, "Entry 4".to_string(), vec![]);

        // Delete entries 2 and 4
        delete_logs(&config, vec![2, 4]);

        let file_content = fs::read_to_string(test_json_path).unwrap();
        let log_entries: Vec<LogEntry> = file_content
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();

        assert_eq!(log_entries.len(), 2);
        assert_eq!(log_entries[0].content, "Entry 1");
        assert_eq!(log_entries[1].content, "Entry 3");

        // Try deleting invalid index
        delete_logs(&config, vec![99]);
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

    #[test]
    fn test_copy_log() {
        let test_json_path = "action_log_copy_test.json";
        let config = Config {
            log_file: test_json_path.to_string(),
        };

        if std::path::Path::new(test_json_path).exists() {
            fs::remove_file(test_json_path).unwrap();
        }

        log_action(&config, "Original Content".to_string(), vec!["tag1".to_string()]);

        // Copy with same content
        copy_log(&config, 1, None);

        let contents = fs::read_to_string(test_json_path).unwrap();
        let entries: Vec<LogEntry> = contents.lines().map(|l| serde_json::from_str(l).unwrap()).collect();

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[1].content, "Original Content");
        assert_eq!(entries[1].tags, vec!["tag1".to_string()]);
        assert!(entries[1].timestamp > entries[0].timestamp);

        // Copy with new content
        copy_log(&config, 1, Some("New Content".to_string()));
        
        let contents_2 = fs::read_to_string(test_json_path).unwrap();
        let entries_2: Vec<LogEntry> = contents_2.lines().map(|l| serde_json::from_str(l).unwrap()).collect();

        assert_eq!(entries_2.len(), 3);
        assert_eq!(entries_2[2].content, "New Content");
        assert_eq!(entries_2[2].tags, vec!["tag1".to_string()]); // Tags preserved

        fs::remove_file(test_json_path).unwrap();
    }

    #[test]
    fn test_archive_logs() {
        let test_dir = std::env::temp_dir().join("acty_test_archive");
        std::fs::create_dir_all(&test_dir).unwrap();
        let log_path = test_dir.join("action_log.json");
        let archive_path = test_dir.join("archive.json");
        
        let config = Config {
            log_file: log_path.to_string_lossy().to_string(),
        };

        // Create logs manually
        let old_date = Local::now() - chrono::Duration::days(10);
        let new_date = Local::now();

        let old_entry = LogEntry {
            timestamp: old_date,
            content: "Old Log".to_string(),
            tags: vec![],
        };
        let new_entry = LogEntry {
            timestamp: new_date,
            content: "New Log".to_string(),
            tags: vec![],
        };

        {
            let mut file = std::fs::File::create(&log_path).unwrap();
            writeln!(file, "{}", serde_json::to_string(&old_entry).unwrap()).unwrap();
            writeln!(file, "{}", serde_json::to_string(&new_entry).unwrap()).unwrap();
        }

        // Archive logs older than 7 days
        archive_logs(&config, 7);

        // Check main log file (should only have new entry)
        let main_contents = std::fs::read_to_string(&log_path).unwrap();
        let main_lines: Vec<&str> = main_contents.lines().collect();
        assert_eq!(main_lines.len(), 1);
        assert!(main_lines[0].contains("New Log"));

        // Check archive file (should have old entry)
        let archive_contents = std::fs::read_to_string(&archive_path).unwrap();
        let archive_lines: Vec<&str> = archive_contents.lines().collect();
        assert_eq!(archive_lines.len(), 1);
        assert!(archive_lines[0].contains("Old Log"));

        std::fs::remove_dir_all(test_dir).unwrap();
    }
}
