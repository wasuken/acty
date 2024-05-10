use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LogEntry {
    #[serde(with = "local_date_time")]
    pub timestamp: DateTime<Local>,
    pub content: String,
    pub tags: Vec<String>,
}

mod local_date_time {
    use chrono::{DateTime, Local};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(datetime: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = datetime.to_rfc3339();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_rfc3339(&s)
            .map(|datetime| datetime.with_timezone(&Local))
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_deserialize() {
        let now = Local::now();
        let log_entry = LogEntry {
            timestamp: now,
            content: "Test content".to_string(),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
        };

        let serialized = serde_json::to_string(&log_entry).unwrap();
        let deserialized: LogEntry = serde_json::from_str(&serialized).unwrap();

        assert_eq!(log_entry.content, deserialized.content);
        assert_eq!(log_entry.tags, deserialized.tags);
        assert_eq!(log_entry.timestamp, deserialized.timestamp);
    }
}
