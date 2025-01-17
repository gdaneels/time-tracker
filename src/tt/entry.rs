use chrono::prelude::*;

// implement a debug trait for Entry

pub struct Entry {
    pub(super) timestamp: DateTime<Local>,
    pub(super) topic: String,
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
