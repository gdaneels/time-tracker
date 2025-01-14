use rusqlite::{Connection, Result};
use super::entry::Entry;

pub struct DatabaseSqlite {
    connection: Connection,
}

impl DatabaseSqlite {
    pub fn new() -> rusqlite::Result<Self> {
        let connection = Connection::open_in_memory()?;
        let db_sqlite = DatabaseSqlite { connection };
        db_sqlite.create_table()?;
        Ok(db_sqlite)
    }

    pub fn create_table(&self) -> rusqlite::Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS tt (
                id INTEGER PRIMARY KEY,
                timestamp DATE NOT NULL,
                topic TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    // pub fn add(&self, entry: Entry) -> rusqlite::Result<()> {
    //     self.connection.execute(
    //         "INSERT INTO tt (timestamp, topic) VALUES (?1, ?2)",
    //         (entry.timestamp, entry.topic),
    //     )?;
    //     Ok(())
    // }
}
