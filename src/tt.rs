//! # tt module
//!
//! This module contains the basic building blocks for the time tracker library crate.

mod db;
pub use db::DatabaseError;
pub mod db_sqlite;
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
    pub fn new() -> Result<Self, DatabaseError> {
        let database = T::new()?;
        let latest = match database.latest() {
            Ok(entry) => Some(entry),
            Err(_) => None,
        };
        let timetracker = TimeTracker {
            latest_entry: latest,
            database,
        };
        Ok(timetracker)
    }

    pub fn start(&mut self, topic: String) {
        if self.latest_entry.is_some()
            && self.latest_entry.as_ref().unwrap().stop_timestamp.is_none()
        {
            println!(
                "You can not start a new entry before stopping the previous one: {:?}",
                self.latest_entry
            );
            return;
        }

        // println!("stop_timestamp: {:?}", self.latest_entry.as_ref().unwrap().stop_timestamp);

        let entry = entry::Entry::new(topic);
        match self.database.add(&entry) {
            Ok(_) => {
                println!(
                    "Starting timer for topic {:?} at time {:?}.",
                    entry.topic, entry.start_timestamp
                );
                self.latest_entry = Some(entry);
            }
            Err(error) => println!("Error adding entry {:?} with error {:?}", entry, error),
        }
    }

    pub fn stop(&mut self) {
        if let Some(ref mut entry) = self.latest_entry {
            println!("Stopping this entry: {:?}", entry);
            entry.stop();
            if self.database.update(entry).is_err() {
                // todo: should rollback stop timestamp of latest entry if this fails
                println!(
                    "Could not update latest entry with new stop timestamp: {:?}",
                    entry
                );
            } else {
                println!("Stop working on topic: {:?}", entry);
            }
        } else {
            println!("Not working on anything currently...");
        }
    }

    /// Show the current tracked topic.
    pub fn print_current(&self) {
        if let Ok(entry) = self.database.current() {
            println!("Current topic: {}", entry);
        } else {
            println!("No current ongoing topic.");
        }
    }

    /// Show the current tracked topic.
    pub fn print_latest(&self) {
        if let Some(entry) = &self.latest_entry {
            println!("Latest topic: {}", entry);
        } else {
            println!("No topics added yet.");
        }
    }

    pub fn print_all(&self) {
        match self.database.all() {
            Ok(entries) => {
                for entry in entries {
                    println!("{}", entry);
                }
            }
            Err(err) => {
                println!("No entries found ({}).", err);
            }
        }
    }
}
