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
                start_timestamp TEXT NOT NULL,
                stop_timestamp TEXT,
                topic TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn add(&self, entry: &Entry) -> rusqlite::Result<()> {
        let start_timestamp = entry.start_timestamp.format("%Y-%m-%d %H:%M:%S").to_string();
        self.connection.execute(
            "INSERT INTO tt (start_timestamp, stop_timestamp, topic) VALUES (?1, NULL, ?2)",
            (entry.start_timestamp, &entry.topic),
        )?;
        Ok(())
    }

    pub fn current(&self) -> rusqlite::Result<Entry> {
        self.connection.query_row("SELECT start_timestamp, topic FROM tt ORDER BY start_timestamp DESC LIMIT 1", [], |row| {
            println!("row: {:?}", row);
            Ok(Entry {
                start_timestamp: row.get(0)?,
                topic: row.get(1)?,
                ..Default::default()
            })
        })
    }
}
