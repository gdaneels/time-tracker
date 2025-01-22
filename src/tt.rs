//! # tt module
//!
//! This module contains the basic building blocks for the time tracker library crate.

mod db;
mod db_sqlite;
mod entry;

/// Contains the time tracker struct.
pub struct TimeTracker<T: db::Database> {
    // database: TimeTrackerDatabase,
    latest_entry: Option<entry::Entry>,
    database: T,
}

impl<T> TimeTracker<T>
where
    T: db::Database,
{
    /// Creates a new instance of the Time Tracker.
    pub fn new() -> rusqlite::Result<Self>
    {
        let database = T::new()?;
        let timetracker = TimeTracker {
            latest_entry: None,
            database,
        };
        Ok(timetracker)
    }

    pub fn start(&self, topic: String) {
        let entry = entry::Entry::new(topic);
        match self.database.add(&entry) {
            Ok(_) => {
                println!(
                    "Starting timer for topic {:?} at time {:?}.",
                    entry.topic, entry.start_timestamp
                );
            }
            Err(error) => println!("Error adding entry {:?} with error {:?}", entry, error),
        }
    }

    pub fn stop(&self) {
        if let Some(entry) = &self.latest_entry {
            // entry.stop();
            // self.database.update(&self.latest_entry);
            println!("Stop working on topic: {:?}", entry.topic);
        } else {
            println!("Not working on anything currently...");
        }
    }

    /// Show the current tracked topic.
    pub fn print_current(&self) {
        if let Ok(entry) = self.database.current() {
            println!("Current topic: {}", entry);
        } else {
            println!("No current topic.");
        }
    }

    pub fn print_all(&self) {
        if let Ok(entries) = self.database.all() {
            for entry in entries {
                println!("{}", entry);
            }
        } else {
            println!("No entries found.");
        }
    }
}
