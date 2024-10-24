# Time Tracker CLI
> A simple command-line tool for tracking time spent on tasks, built using Rust.

# Prerequisites
- Rust installed on your machine

# Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/eatPizza311/time-tracker.git
   cd time-tracker
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
   
> You can use `cargo run` instead

# Usage
- Start tracking time:
  ```bash
  cargo run -- --db-dir db.json --lockfile track.lock start
  ```
  > Don't forget the `--` between cargo and command!
- Stop tracking time:
  ```bash
  cargo run -- --db-dir db.json --lockfile track.lock stop
  ```
- Get the report:
  ```bash
  cargo run -- --db-dir db.json --lockfile track.lock report
  ```
You can specify the location of a flat-file database and a lockfile.

![截圖 2024-10-24 晚上10 38 39](https://github.com/user-attachments/assets/df8a68de-d640-4de8-b6fc-fbe2ca615655)


You can also run without any flags:

![圖片](https://github.com/user-attachments/assets/7c877827-a50f-41e6-a770-1e2804f7291b)

The db and lockfile will be stored in the track directory at the following path with default name:
| Platform  | db file (records.json) | lockfile (track.lock) |
| ------------- | ------------- | ------------- |
| Linux  | `$XDG_DATA_HOME` or `$HOME`/.local/share  | `$XDG_CACHE_HOME` or `$HOME`/.cache |
| macOS  | `$HOME`/Library/Application Support  | `$HOME`/Library/Caches  |
| Windows  | `{FOLDERID_RoamingAppData}`  | `{FOLDERID_LocalAppData}`  |
