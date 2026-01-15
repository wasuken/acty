use crate::config::Config;
use crate::log_entry::LogEntry;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn list_tags(config: &Config) {
    let tag_counts = get_tag_counts(config);

    if tag_counts.is_empty() {
        println!("No tags found.");
        return;
    }

    // Sort by count (descending), then by name (ascending)
    let mut sorted_tags: Vec<(&String, &usize)> = tag_counts.iter().collect();
    sorted_tags.sort_by(|a, b| {
        let count_cmp = b.1.cmp(a.1); // Descending count
        if count_cmp == std::cmp::Ordering::Equal {
            a.0.cmp(b.0) // Ascending name
        } else {
            count_cmp
        }
    });

    println!("{:<20} {}", "TAG", "COUNT");
    println!("{:<20} {}", "---", "-----");

    for (tag, count) in sorted_tags {
        println!("{:<20} {}", tag, count);
    }
}

fn get_tag_counts(config: &Config) -> HashMap<String, usize> {
    let path = std::path::Path::new(&config.log_file);
    if !path.exists() {
        return HashMap::new();
    }

    let file = File::open(path).expect("Unable to open the log file");
    let reader = BufReader::new(file);
    let mut tag_counts: HashMap<String, usize> = HashMap::new();

    for line in reader.lines() {
        if let Ok(l) = line {
            if let Ok(entry) = serde_json::from_str::<LogEntry>(&l) {
                for tag in entry.tags {
                    *tag_counts.entry(tag).or_insert(0) += 1;
                }
            }
        }
    }
    tag_counts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logger::log_action;
    use std::fs;

    #[test]
    fn test_tag_counting() {
        let test_json_path = "action_log_tags_test.json";
        let config = Config {
            log_file: test_json_path.to_string(),
        };

        if std::path::Path::new(test_json_path).exists() {
            fs::remove_file(test_json_path).unwrap();
        }

        log_action(&config, "Log 1".to_string(), vec!["work".to_string(), "urgent".to_string()]);
        log_action(&config, "Log 2".to_string(), vec!["work".to_string(), "meeting".to_string()]);
        log_action(&config, "Log 3".to_string(), vec!["rest".to_string()]);

        let counts = get_tag_counts(&config);
        
        assert_eq!(*counts.get("work").unwrap(), 2);
        assert_eq!(*counts.get("urgent").unwrap(), 1);
        assert_eq!(*counts.get("meeting").unwrap(), 1);
        assert_eq!(*counts.get("rest").unwrap(), 1);
        assert!(counts.get("unknown").is_none());

        fs::remove_file(test_json_path).unwrap();
    }
}
