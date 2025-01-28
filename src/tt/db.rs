use crate::tt::entry;
use std::fmt;

#[derive(PartialEq, Debug)]
enum DatabaseError {
    NoEntries,
    DatabaseSpecific,
}

impl fmt::Display for DatabaseError {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            DatabaseError::NoEntries => "No entries in the database.",
            DatabaseError::DatabaseSpecific => "Database specific error occured.",
        };
        f.write_str(description)
    }
}

// todo: implement trait From to automatically generate errors when using ?

pub trait Database: Sized {
    fn new() -> rusqlite::Result<Self>
    where
        Self: Sized;
    fn add(&self, entry: &entry::Entry) -> Result<(), rusqlite::Error>;
    // update the entry based on the start_timestamp
    fn update(&self, entry: &entry::Entry) -> Result<(), rusqlite::Error>;
    // returns current ongoing entry, meaning it has not been stopped yet
    fn current(&self) -> Result<entry::Entry, rusqlite::Error>;
    // returns the latest entry, meaning the one with the latest start_timestamp
    fn latest(&self) -> Result<entry::Entry, rusqlite::Error>;
    fn all(&self) -> Result<Vec<entry::Entry>, rusqlite::Error>;
}
