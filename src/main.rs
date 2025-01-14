use clap::{Parser, Subcommand};
use tt::TimeTracker;

#[derive(Parser, Debug)]
struct InputEntry {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Start {topic: Option<String>},
    Stop {topic: Option<String>},
}

fn main() -> Result<(), rusqlite::Error> {
    let tt = TimeTracker::new()?;
    let args = InputEntry::parse();
    match &args.command {
        Command::Start { topic } => {
            println!("Starting timer for topic: {:?}", topic);
            tt.start(topic.clone().unwrap());
        },
        Command::Stop { .. } => {
            tt.stop();
        },
    }

    let timetracker = tt::TimeTracker::new();

    Ok(())
}

// TODO
// - make project structure: make lib and runnable binary
// - make entry struct
// - make tt struct
