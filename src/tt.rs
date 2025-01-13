//! # tt module
//!
//! This module contains the basic building blocks for the time tracker library crate.

mod entry;

/// Contains the time tracker struct.
pub struct TimeTracker {
    // database: TimeTrackerDatabase,
    latest_entry: Option<entry::Entry>,
}

impl TimeTracker {
    /// Creates a new instance of the Time Tracker.
    pub fn new() -> Self {
        TimeTracker {
            latest_entry: None,
        }
    }

    pub fn start(&self, topic: String) {
        let entry = entry::Entry::new(topic);
        println!("Starting timer for topic {:?} at time {:?}.", entry.topic, entry.timestamp);
    }

    pub fn stop(&self) {
        if let Some(entry) = &self.latest_entry {
            println!("Stop working on topic: {:?}", entry.topic);
        } else {
            println!("Not working on anything currently...");
        }
    }
}
