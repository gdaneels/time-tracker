use crate::tt::entry;

pub trait Database {
    fn new() -> rusqlite::Result<Self>
    where
        Self: Sized;
    fn add(&self, entry: &entry::Entry) -> Result<(), rusqlite::Error>;
    fn update(&self, entry: &entry::Entry) -> Result<(), rusqlite::Error>;
    fn current(&self) -> Result<entry::Entry, rusqlite::Error>;
    fn all(&self) -> Result<Vec<entry::Entry>, rusqlite::Error>;
}
