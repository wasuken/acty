use crate::log_entry::LogEntry;
use chrono::{Local, NaiveDate};

pub fn sort_tags(mut tags: Vec<String>) -> Vec<String> {
    tags.sort_by(|a, b| {
        let len_cmp = a.len().cmp(&b.len());
        if len_cmp == std::cmp::Ordering::Equal {
            a.cmp(b)
        } else {
            len_cmp
        }
    });
    tags
}

pub fn should_include_log(
    log_entry: &LogEntry,
    date: &Option<String>,
    range: &Option<i64>,
    tags: &[String],
    search: &Option<String>,
) -> bool {
    if let Some(d) = date {
        let filter_date = NaiveDate::parse_from_str(d, "%Y-%m-%d").expect("Invalid date format");
        if log_entry.timestamp.date_naive() != filter_date {
            return false;
        }
    }

    if let Some(r) = range {
        let log_date = log_entry.timestamp.date_naive();
        let now = Local::now().date_naive();
        if (now - log_date).num_days() > *r {
            return false;
        }
    }

    if !tags.is_empty() && !tags.iter().all(|t| log_entry.tags.contains(t)) {
        return false;
    }

    if let Some(s) = search {
        let s_lower = s.to_lowercase();
        let content_match = log_entry.content.to_lowercase().contains(&s_lower);
        let tag_match = log_entry.tags.iter().any(|t| t.to_lowercase().contains(&s_lower));
        
        if !content_match && !tag_match {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::log_entry::LogEntry;
    use chrono::Local;

    #[test]
    fn test_should_include_log_search() {
        let entry = LogEntry {
            timestamp: Local::now(),
            content: "Meeting with the team".to_string(),
            tags: vec!["work".to_string(), "urgent".to_string()],
        };

        // Match content (case insensitive)
        assert!(should_include_log(&entry, &None, &None, &[], &Some("meeting".to_string())));
        
        // Match tag (should pass after update)
        assert!(should_include_log(&entry, &None, &None, &[], &Some("urgent".to_string())));

        // No match
        assert!(!should_include_log(&entry, &None, &None, &[], &Some("lunch".to_string())));

        // Match partial content
        assert!(should_include_log(&entry, &None, &None, &[], &Some("team".to_string())));
    }
}
