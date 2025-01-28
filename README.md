## A command-line time tracking application, written in Rust. ##

You can add a new entry as such:
```
cargo run -- start [topic-name]
```
It is impossible to start working on a new topic (i.e., starting a new topic) while the previous one is not stopped.

You can stop the current topic by running:
```
cargo run -- stop
```

You can view the current ongoing topic. This will only show a result when the latest topic is *not* stopped (i.e. the *current* topic).
```
cargo run -- current
```

You can view the latest added topic (can be stopped already, but doesn't have to):
```
cargo run -- latest
```

View all database entries:
```
cargo run -- all
```
