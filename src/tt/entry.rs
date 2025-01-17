use chrono::prelude::*;
use std::fmt;

// implement a debug trait for Entry

#[derive(Debug)]
pub struct Entry {
    pub(super) timestamp: DateTime<Local>,
    pub(super) topic: String,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Entry: {} - {}", self.timestamp, self.topic)
    }
}

impl Default for Entry {
    fn default() -> Self {
        Entry {
            timestamp: Local::now(),
            topic: String::from("no topic")
        }
    }
}

impl Entry {
    pub fn new(topic: String) -> Self {
        Entry {
            timestamp: Local::now(),
            topic,
        }
    }
}
