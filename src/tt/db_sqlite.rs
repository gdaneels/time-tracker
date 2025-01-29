use super::db;
use super::entry::Entry;
use rusqlite::Connection;

impl From<rusqlite::Error> for db::DatabaseError {
    fn from(err: rusqlite::Error) -> Self {
        db::DatabaseError::DatabaseMisc(err.to_string())
    }
}

pub struct DatabaseSqlite {
    connection: Connection,
}

impl DatabaseSqlite {
    fn create_table(&self) -> rusqlite::Result<()> {
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
}

impl db::Database for DatabaseSqlite {
    fn new() -> db::Result<Self> {
        let connection = Connection::open("./tt.db")?;
        let db_sqlite = DatabaseSqlite { connection };
        db_sqlite.create_table()?;
        Ok(db_sqlite)
    }

    fn add(&self, entry: &Entry) -> db::Result<()> {
        self.connection.execute(
            "INSERT INTO tt (start_timestamp, stop_timestamp, topic) VALUES (?1, NULL, ?2)",
            (entry.start_timestamp, &entry.topic),
        )?;
        Ok(())
    }

    fn update(&self, entry: &Entry) -> db::Result<()> {
        match self.connection.execute(
            "UPDATE tt SET stop_timestamp = ?1, topic =?2 WHERE start_timestamp = ?3",
            (entry.stop_timestamp, &entry.topic, entry.start_timestamp),
        ) {
            Ok(updated) => println!("updated {:?} rows", updated),
            Err(err) => println!("updating went wrong {:?}", err),
        }
        Ok(())
    }

    fn current(&self) -> db::Result<Entry> {
        self.connection.query_row("SELECT start_timestamp, stop_timestamp, topic FROM (SELECT * FROM tt ORDER BY start_timestamp DESC LIMIT 1) WHERE stop_timestamp IS NULL",
            [],
            |row| {
                Ok(Entry {
                    start_timestamp: row.get(0)?,
                    stop_timestamp: row.get(1)?,
                    topic: row.get(2)?,
                })
            },
        ).map_err(Into::into)
        // can't use ? here because it will unpack Ok(Entry) to Entry
    }

    fn latest(&self) -> db::Result<Entry> {
        self.connection.query_row(
            "SELECT start_timestamp, stop_timestamp, topic FROM tt ORDER BY start_timestamp DESC LIMIT 1",
            [],
            |row| {
                Ok(Entry {
                    start_timestamp: row.get(0)?,
                    stop_timestamp: row.get(1)?,
                    topic: row.get(2)?,
                })
            },
        ).map_err(Into::into)
    }

    /// Get all entries from the database.
    fn all(&self) -> db::Result<Vec<Entry>> {
        let mut statement = self
            .connection
            .prepare("SELECT start_timestamp, stop_timestamp, topic FROM tt")?;
        let entries = statement.query_map([], |row| {
            Ok(Entry {
                start_timestamp: row.get(0)?,
                stop_timestamp: row.get(1)?,
                topic: row.get(2)?,
            })
        })?;
        let mut result = Vec::new();
        for entry in entries {
            result.push(entry?);
        }
        if result.len() == 0 {
            return Err(db::DatabaseError::NoEntries);
        }
        Ok(result)
    }
}
