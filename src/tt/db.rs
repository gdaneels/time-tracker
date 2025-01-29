use crate::tt::entry;
use std::fmt;

#[derive(PartialEq)]
pub enum DatabaseError {
    NoEntries,
    DatabaseMisc(String),
}

pub type Result<T> = std::result::Result<T, DatabaseError>;

impl fmt::Display for DatabaseError {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            DatabaseError::NoEntries => "No entries in the database.".to_string(),
            DatabaseError::DatabaseMisc(err) => format!("Database specific error occured: {}", err),
        };
        f.write_str(&description)
    }
}

impl fmt::Debug for DatabaseError {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

// todo: implement trait From to automatically generate errors when using ?

pub trait Database: Sized {
    fn new() -> Result<Self>
    where
        Self: Sized;
    fn add(&self, entry: &entry::Entry) -> Result<()>;
    // update the entry based on the start_timestamp
    fn update(&self, entry: &entry::Entry) -> Result<()>;
    // returns current ongoing entry, meaning it has not been stopped yet
    fn current(&self) -> Result<entry::Entry>;
    // returns the latest entry, meaning the one with the latest start_timestamp
    fn latest(&self) -> Result<entry::Entry>;
    fn all(&self) -> Result<Vec<entry::Entry>>;
}
