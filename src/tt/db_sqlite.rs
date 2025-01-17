use rusqlite::{Connection, Result, ToSql};
use super::entry::Entry;

pub struct DatabaseSqlite {
    connection: Connection,
}

impl DatabaseSqlite {
    pub fn new() -> rusqlite::Result<Self> {
        let connection = Connection::open("./tt.db")?;
        let db_sqlite = DatabaseSqlite { connection };
        db_sqlite.create_table()?;
        Ok(db_sqlite)
    }

    pub fn create_table(&self) -> rusqlite::Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS tt (
                id INTEGER PRIMARY KEY,
                timestamp TEXT NOT NULL,
                topic TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn add(&self, entry: &Entry) -> rusqlite::Result<()> {
        let timestamp = entry.timestamp.format("%Y-%m-%d %H:%M:%S").to_string();
        self.connection.execute(
            "INSERT INTO tt (timestamp, topic) VALUES (?1, ?2)",
            (entry.timestamp, &entry.topic),
        )?;
        Ok(())
    }

    pub fn current(&self) -> rusqlite::Result<Entry> {
        self.connection.query_row("SELECT * FROM tt ORDER BY timestamp DESC LIMIT 1", [], |row| {
            Ok(Entry {
                timestamp: row.get(1)?,
                topic: row.get(2)?,
            })
        })
    }
}
