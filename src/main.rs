use clap::{Parser, Subcommand};
use tt::TimeTracker;
use tt::db_sqlite;

#[derive(Parser, Debug)]
struct InputEntry {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Current,
    Latest,
    All,
    Start { topic: Option<String> },
    Stop { topic: Option<String> },
}

fn main() -> Result<(), tt::DatabaseError> {
    let mut tt: TimeTracker<db_sqlite::DatabaseSqlite> = TimeTracker::new()?;
    let args = InputEntry::parse();
    match &args.command {
        Command::Start { topic } => {
            println!("Starting timer for topic: {:?}", topic);
            tt.start(topic.clone().unwrap());
        }
        Command::Stop { .. } => {
            tt.stop();
        }
        Command::Latest => {
            tt.print_latest();
        }
        Command::Current => {
            tt.print_current();
        }
        Command::All => {
            tt.print_all();
        }
    }

    Ok(())
}

// TODO
// - make project structure: make lib and runnable binary
// - make entry struct
// - make tt struct
