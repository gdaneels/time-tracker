use chrono::prelude::*;
use std::fmt;

// implement a debug trait for Entry

#[derive(Debug)]
pub struct Entry {
    pub(super) start_timestamp: DateTime<Local>,
    pub(super) stop_timestamp: Option<DateTime<Local>>,
    pub(super) topic: String,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(stop_timestamp) = self.stop_timestamp {
            write!(f, "Entry: {} - {} - {}", self.start_timestamp, stop_timestamp, self.topic)
        } else {
            write!(f, "Entry: {} - ... - {}", self.start_timestamp, self.topic)
        }
    }
}

impl Default for Entry {
    fn default() -> Self {
        Entry {
            start_timestamp: Local::now(),
            stop_timestamp: None,
            topic: String::from("no topic")
        }
    }
}

impl Entry {
    pub fn new(topic: String) -> Self {
        Entry {
            start_timestamp: Local::now(),
            stop_timestamp: None,
            topic,
        }
    }
}
