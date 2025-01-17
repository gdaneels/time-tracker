//! # tt module
//!
//! This module contains the basic building blocks for the time tracker library crate.

mod entry;
mod db_sqlite;

/// Contains the time tracker struct.
pub struct TimeTracker {
    // database: TimeTrackerDatabase,
    latest_entry: Option<entry::Entry>,
    database: db_sqlite::DatabaseSqlite,
}

impl TimeTracker {
    /// Creates a new instance of the Time Tracker.
    pub fn new() -> rusqlite::Result<Self> {
        let database = db_sqlite::DatabaseSqlite::new()?;
        let timetracker = TimeTracker {
            latest_entry: None,
            database,
        };
        Ok(timetracker)
    }

    pub fn start(&self, topic: String) {
        let entry = entry::Entry::new(topic);
        self.database.add(&entry);
        println!("Starting timer for topic {:?} at time {:?}.", entry.topic, entry.timestamp);
    }

    pub fn stop(&self) {
        if let Some(entry) = &self.latest_entry {
            println!("Stop working on topic: {:?}", entry.topic);
        } else {
            println!("Not working on anything currently...");
        }
    }
    
    /// Show the current tracked topic.
    pub fn show_current(&self) {
        if let Ok(entry) = self.database.current() {
            println!("Current topic: {}", entry);
        } else {
            println!("No current topic.");
        }
    }
}
