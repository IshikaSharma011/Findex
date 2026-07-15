<<<<<<< HEAD
<div align="center">

# 🗂️ Findex

**A blazing-fast filesystem indexer and search CLI, built in Rust.**

Scan once. Search instantly. No cloud, no daemon, no dependencies.

[![Rust](https://img.shields.io/badge/built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![SQLite](https://img.shields.io/badge/storage-SQLite-blue?style=flat-square&logo=sqlite)](https://sqlite.org/)
[![Platform](https://img.shields.io/badge/platform-Windows-0078d4?style=flat-square&logo=windows)](https://www.microsoft.com/windows)
[![License: MIT](https://img.shields.io/badge/license-MIT-green?style=flat-square)](LICENSE)

</div>

---

**Findex** recursively walks your directories, stores every file's metadata into a local SQLite database, and lets you search across thousands of files by name or extension in under a millisecond — entirely offline, with a single binary.

```powershell
.\target\release\indexer.exe build C:\Users\YourName\Projects   # index a folder
.\target\release\indexer.exe search config                       # find files by name
.\target\release\indexer.exe ext rs                              # filter by extension
.\target\release\indexer.exe stats                               # explore your filesystem
=======
<<<<<<< HEAD
# 1. Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Unzip and build
unzip fs-indexer.zip
cd fs-indexer
cargo build --release       # ~2-3 min first time (compiles SQLite)

# 3. Use it
./target/release/indexer build ~/Documents --db myindex.db
./target/release/indexer stats --db myindex.db
./target/release/indexer search config --db myindex.db
./target/release/indexer ext rs --db myindex.db



# 📁 File System Indexer CLI

A production-quality CLI tool written in **Rust** that recursively scans directories, indexes file metadata into **SQLite**, and lets you search and analyze your filesystem at blazing speed.

---

## Tech Stack

| Crate       | Purpose                          |
|-------------|----------------------------------|
| `tokio`     | Async runtime                    |
| `walkdir`   | Recursive directory traversal    |
| `rusqlite`  | SQLite (bundled, no install needed) |
| `clap`      | CLI argument parsing             |
| `serde`     | JSON serialization               |
| `anyhow`    | Error handling                   |
| `colored`   | Terminal colors                  |
| `indicatif` | Progress bars & spinners         |
| `humansize` | Human-readable file sizes        |
| `chrono`    | Timestamps                       |

---

## Project Structure

```
fs-indexer/
├── Cargo.toml
└── src/
    ├── main.rs       ← Entry point, command dispatch, output rendering
    ├── cli.rs        ← Clap CLI definitions (Commands, flags)
    ├── models.rs     ← FileEntry, IndexStats, SearchResult structs
    ├── database.rs   ← SQLite layer (open, insert batch, search, stats)
    ├── scanner.rs    ← Async recursive directory walk (skips hidden)
    └── search.rs     ← Search logic + result formatting
>>>>>>> 6191237 (Add File System Indexer project)
```

---

<<<<<<< HEAD
## ✨ Features

- **Instant search** — queries hit a local SQLite index, not your disk
- **Extension filter** — `ext py` lists every Python file you've indexed
- **Rich stats** — total size, top extensions with bar chart, 5 largest files
- **Hidden-folder aware** — automatically skips `.git`, `.cache`, etc.
- **Upsert on re-scan** — re-run `build` to update changed files, nothing is lost
- **JSON output** — pipe-friendly with `--json` on every command
- **Zero runtime dependencies** — SQLite is bundled, one `.exe` to copy

---

## 🖥️ Prerequisites (Windows)

### 1. Install Rust

Go to **https://rustup.rs** → download and run `rustup-init.exe`

In the installer, press **1** (standard installation) and wait for it to finish (~2 min).

**Close and reopen VS Code after install**, then verify in the terminal:

```powershell
rustc --version
cargo --version
```

### 2. Install Visual Studio C++ Build Tools

Rust needs the MSVC linker. Go to:
**https://visualstudio.microsoft.com/visual-cpp-build-tools/**

Install the **"Desktop development with C++"** workload.

### 3. Install the VS Code extension

Open Extensions (`Ctrl+Shift+X`) → search **rust-analyzer** → Install.

---

## 📦 Installation

```powershell
# 1. Clone the repo
git clone https://github.com/yourusername/findex.git
cd findex

# 2. Build (first time takes ~3-5 min, then fast)
cargo build --release

# 3. Binary is ready at
.\target\release\indexer.exe
```

You'll see this when it's done:
```
Finished release [optimized] target(s) in 3m 12s
=======
## Build from Source

### Prerequisites
- Rust 1.70+ → https://rustup.rs

```bash
# Clone / extract the project
cd fs-indexer

# Debug build (fast compile, slower binary)
cargo build

# Release build (optimized — recommended)
cargo build --release

# Binary location
./target/release/indexer
>>>>>>> 6191237 (Add File System Indexer project)
```

---

<<<<<<< HEAD
## 🚀 Usage

> **Tip:** Open the VS Code terminal with **Ctrl + `** (backtick key, top-left of keyboard)

---

### `build` — Index a directory

```powershell
# Index Documents
.\target\release\indexer.exe build C:\Users\YourName\Documents

# Use a custom database file
.\target\release\indexer.exe build C:\Users\YourName\Projects --db C:\myindex.db

# Wipe and re-scan from scratch
.\target\release\indexer.exe build C:\Users\YourName\Projects --clean

# Use more CPU threads
.\target\release\indexer.exe build C:\Users\YourName\Projects --threads 8
```

**Output:**
=======
## Commands & Usage

### 1. `build` — Index a directory

```bash
./target/release/indexer build <DIRECTORY> [OPTIONS]

# Examples
./target/release/indexer build ~/Documents
./target/release/indexer build /usr/lib --db myindex.db
./target/release/indexer build ~/Projects --clean      # wipe before re-scan
./target/release/indexer build ~/Projects --threads 8  # parallel threads
```

**Expected output:**
>>>>>>> 6191237 (Add File System Indexer project)
```
  ╔══════════════════════════════════╗
  ║   📁  File System Indexer v1.0  ║
  ╚══════════════════════════════════╝

<<<<<<< HEAD
  Building index from: C:\Users\YourName\Projects
=======
  Building index from: ~/Projects
>>>>>>> 6191237 (Add File System Indexer project)
  Database: index.db
  Threads: 4

  ⟳ Scanning filesystem…

  ✓ Found 1,842 files in 203 directories (38 ms)

<<<<<<< HEAD
  ⠸ [00:00:02] [████████████████████░░░░░░░░░░░░░░░░░░░░] 1492/1842 (00:00:01)
=======
  ⠸ [00:00:02] [████████████████████████░░░░░░░░░░░░░░░░] 1492/1842 (00:00:01)
>>>>>>> 6191237 (Add File System Indexer project)

  ✓ Indexed 1,842 files  (DB size: 512.00 kB)

  Run `indexer stats`  to explore the index.
```

---

<<<<<<< HEAD
### `search` — Find files by keyword

```powershell
# Search by filename
.\target\release\indexer.exe search config

# Show more results
.\target\release\indexer.exe search config --limit 50

# JSON output
.\target\release\indexer.exe search readme --json

# Use a specific database
.\target\release\indexer.exe search main --db C:\myindex.db
```

**Output:**
```
 🔍  keyword search for 'config' — 4 results (0 ms)

  #      FILENAME                          EXT       SIZE          PATH
  ────────────────────────────────────────────────────────────────────────────────────────────────────
     1.  config.toml                       .toml          2.10 kB  C:\Users\YourName\app\config.toml
     2.  config.yaml                       .yaml          1.40 kB  C:\Users\YourName\api\config.yaml
     3.  config.json                       .json          3.80 kB  C:\Users\YourName\web\config.json
     4.  database_config.py                .py            1.10 kB  C:\Users\YourName\db\database_config.py
  ────────────────────────────────────────────────────────────────────────────────────────────────────
  → 4 files matched
=======
### 2. `search` — Keyword search in filenames

```bash
./target/release/indexer search <KEYWORD> [OPTIONS]

# Examples
./target/release/indexer search main
./target/release/indexer search config --limit 50
./target/release/indexer search readme --json      # JSON output
```

**Expected output:**
```
 🔍  keyword search for 'main' — 3 results (0 ms)

  #      FILENAME                          EXT       SIZE          PATH
  ────────────────────────────────────────────────────────────────────────────────────────────────────
     1.  main.rs                           .rs           10.99 kB  ~/fs-indexer/src/main.rs
     2.  main.py                           .py            2.34 kB  ~/myproject/main.py
     3.  main.go                           .go            5.10 kB  ~/goproj/main.go
  ────────────────────────────────────────────────────────────────────────────────────────────────────
  → 3 files matched
>>>>>>> 6191237 (Add File System Indexer project)
```

---

<<<<<<< HEAD
### `ext` — Filter by file extension

```powershell
# Find all Rust files
.\target\release\indexer.exe ext rs

# Find all Python files
.\target\release\indexer.exe ext py --limit 100

# Find all JSON files
.\target\release\indexer.exe ext json

# Find all text files
.\target\release\indexer.exe ext txt

# JSON output
.\target\release\indexer.exe ext rs --json
```

**Output:**
=======
### 3. `ext` — Filter by file extension

```bash
./target/release/indexer ext <EXTENSION> [OPTIONS]

# Examples
./target/release/indexer ext rs
./target/release/indexer ext py --limit 100
./target/release/indexer ext json --json
./target/release/indexer ext txt --db myindex.db
```

**Expected output:**
>>>>>>> 6191237 (Add File System Indexer project)
```
 🔍  extension search for 'rs' — 6 results (0 ms)

  #      FILENAME                          EXT       SIZE          PATH
  ────────────────────────────────────────────────────────────────────────────────────────────────────
<<<<<<< HEAD
     1.  cli.rs                            .rs            2.89 kB  C:\Users\YourName\findex\src\cli.rs
     2.  database.rs                       .rs           10.60 kB  C:\Users\YourName\findex\src\database.rs
     3.  main.rs                           .rs           10.99 kB  C:\Users\YourName\findex\src\main.rs
     4.  models.rs                         .rs            2.12 kB  C:\Users\YourName\findex\src\models.rs
     5.  scanner.rs                        .rs            6.11 kB  C:\Users\YourName\findex\src\scanner.rs
     6.  search.rs                         .rs            4.70 kB  C:\Users\YourName\findex\src\search.rs
=======
     1.  cli.rs                            .rs            2.89 kB  ~/fs-indexer/src/cli.rs
     2.  database.rs                       .rs           10.60 kB  ~/fs-indexer/src/database.rs
     3.  main.rs                           .rs           10.99 kB  ~/fs-indexer/src/main.rs
     4.  models.rs                         .rs            2.12 kB  ~/fs-indexer/src/models.rs
     5.  scanner.rs                        .rs            6.11 kB  ~/fs-indexer/src/scanner.rs
     6.  search.rs                         .rs            4.70 kB  ~/fs-indexer/src/search.rs
>>>>>>> 6191237 (Add File System Indexer project)
  ────────────────────────────────────────────────────────────────────────────────────────────────────
  → 6 files matched
```

---

<<<<<<< HEAD
### `stats` — Explore index statistics

```powershell
# Full stats report
.\target\release\indexer.exe stats

# Show top 15 extensions
.\target\release\indexer.exe stats --top 15

# JSON output
.\target\release\indexer.exe stats --json

# Stats from a specific database
.\target\release\indexer.exe stats --db C:\myindex.db
```

**Output:**
```
=======
### 4. `stats` — Show index statistics

```bash
./target/release/indexer stats [OPTIONS]

# Examples
./target/release/indexer stats
./target/release/indexer stats --top 15
./target/release/indexer stats --json
```

**Expected output:**
```
  ╔══════════════════════════════════╗
  ║   📁  File System Indexer v1.0  ║
  ╚══════════════════════════════════╝

>>>>>>> 6191237 (Add File System Indexer project)
  Index Statistics

  Summary
  Total files .................. 1,842
  Total size ................... 248.30 MB
  Unique extensions ............ 37
  Last indexed ................. 2026-07-01 05:08:54 UTC
  Database path ................ index.db
  Database size ................ 512.00 kB

  Top 10 Extensions by Count
<<<<<<< HEAD
  #      EXT          FILES      TOTAL SIZE
  ──────────────────────────────────────────────────
     1.  .rs            312       18.40 MB  ████████████████████
     2.  .toml           84        1.20 MB  █████
     3.  .md             71        3.80 MB  ████
     4.  .json           60        9.10 MB  ███
     5.  .py             45        2.30 MB  ██

  Top 5 Largest Files
  #      FILENAME                     SIZE         PATH
  ──────────────────────────────────────────────────────────────────────────
     1.  archive.bin             102.40 MB  C:\Users\YourName\data\archive.bin
     2.  database_dump.sql        48.20 MB  C:\Users\YourName\backups\dump.sql
=======
  #      EXT                FILES      TOTAL SIZE
  ────────────────────────────────────────────────────
     1.  .rs                  312        18.40 MB  ████████████████████
     2.  .toml                 84         1.20 MB  █████
     3.  .md                   71         3.80 MB  ████
     4.  .json                 60         9.10 MB  ███
     5.  .py                   45         2.30 MB  ██
     ...

  Top 5 Largest Files
  #      FILENAME                                SIZE  PATH
  ──────────────────────────────────────────────────────────────────────
     1.  some_large_file.bin              102.40 MB  ~/data/...
     2.  database_dump.sql                 48.20 MB  ~/backups/...
     ...
>>>>>>> 6191237 (Add File System Indexer project)
```

---

<<<<<<< HEAD
## ⚙️ Options reference

### Global

| Flag | Default | Description |
|------|---------|-------------|
| `--db <PATH>` | `index.db` | Path to the SQLite database file |
| `-h, --help` | — | Print help |
| `-V, --version` | — | Print version |

### Per-command

| Command | Flag | Default | Description |
|---------|------|---------|-------------|
| `build` | `--clean, -c` | false | Wipe index before scanning |
| `build` | `--threads, -j` | 4 | Worker threads |
| `search` | `--limit, -n` | 20 | Max results to show |
| `search` | `--json` | false | Output as JSON |
| `ext` | `--limit, -n` | 20 | Max results to show |
| `ext` | `--json` | false | Output as JSON |
| `stats` | `--top` | 10 | Top N extensions to show |
| `stats` | `--json` | false | Output as JSON |

---

## 💡 Pro tip — use `cargo run` during development

While working on the project, skip typing the full `.exe` path by using:

```powershell
cargo run --release -- build C:\Users\YourName\Documents
cargo run --release -- search config
cargo run --release -- ext rs
cargo run --release -- stats
```

The `--` separates Cargo's own flags from your program's arguments.

---

## 🏗️ Project structure

```
findex/
├── Cargo.toml
└── src/
    ├── main.rs       — Entry point, command dispatch, output rendering
    ├── cli.rs        — Clap CLI definitions (all commands and flags)
    ├── models.rs     — Core structs: FileEntry, IndexStats, SearchResult
    ├── scanner.rs    — Async recursive directory walk (skips hidden dirs)
    ├── database.rs   — SQLite layer: open, batch upsert, search, stats queries
    └── search.rs     — Search logic and colored table renderer
=======
## Global Options

```
--db <PATH>    Path to the SQLite database file (default: index.db)
-h, --help     Print help
-V, --version  Print version
```

## Per-command Options

| Command  | Flag           | Default | Description                    |
|----------|----------------|---------|--------------------------------|
| `build`  | `--clean, -c`  | false   | Wipe index before scanning     |
| `build`  | `--threads, -j`| 4       | Worker threads                 |
| `search` | `--limit, -n`  | 20      | Max results                    |
| `search` | `--json`       | false   | Output as JSON                 |
| `ext`    | `--limit, -n`  | 20      | Max results                    |
| `ext`    | `--json`       | false   | Output as JSON                 |
| `stats`  | `--top`        | 10      | Number of top extensions shown |
| `stats`  | `--json`       | false   | Output as JSON                 |

---

## Quick Start (copy-paste)

```bash
# Build the binary
cargo build --release

# Index your home directory
./target/release/indexer build ~ --db ~/myindex.db

# Find all Rust source files
./target/release/indexer ext rs --db ~/myindex.db

# Search for config files
./target/release/indexer search config --db ~/myindex.db --limit 50

# Show statistics
./target/release/indexer stats --db ~/myindex.db

# Re-index after changes (clean rebuild)
./target/release/indexer build ~ --db ~/myindex.db --clean
>>>>>>> 6191237 (Add File System Indexer project)
```

---

<<<<<<< HEAD
## 🔧 Tech stack

| Crate | Purpose |
|-------|---------|
| [`tokio`](https://tokio.rs) | Async runtime for concurrent scan + insert |
| [`walkdir`](https://docs.rs/walkdir) | Recursive directory traversal |
| [`rusqlite`](https://docs.rs/rusqlite) | SQLite (bundled — no system install needed) |
| [`clap`](https://docs.rs/clap) | CLI argument parsing and help generation |
| [`serde`](https://serde.rs) | JSON serialization for `--json` output |
| [`anyhow`](https://docs.rs/anyhow) | Ergonomic error handling |
| [`indicatif`](https://docs.rs/indicatif) | Progress bars and spinners |
| [`colored`](https://docs.rs/colored) | Terminal color output |
| [`humansize`](https://docs.rs/humansize) | Human-readable file sizes |
| [`chrono`](https://docs.rs/chrono) | Timestamps |

---

## 🐛 Troubleshooting

**`'cargo' is not recognized`**
Close and reopen VS Code after installing Rust. It adds itself to PATH but VS Code needs a restart to pick it up.

**`linking with link.exe failed`**
Install the Visual Studio C++ Build Tools: https://visualstudio.microsoft.com/visual-cpp-build-tools/
Select the **"Desktop development with C++"** workload.

**`can't find the project / wrong folder`**
Make sure the VS Code terminal is inside the `findex` folder. Run:
```powershell
cd C:\path\to\findex
```

**`Couldn't open database`**
The folder in your `--db` path must exist first. If you use `--db C:\data\myindex.db`, create the `C:\data\` folder manually before running.

---

## 📄 License

MIT — see [LICENSE](LICENSE).
=======
## Features

- ✅ Recursive directory scanning via `walkdir`
- ✅ Hidden directories/files automatically skipped (`.git`, `.cache`, etc.)
- ✅ Stores: filename, extension, absolute path, size, last modified, indexed timestamp
- ✅ SQLite with WAL mode + indices for fast queries
- ✅ Upsert on conflict — re-running `build` updates changed files
- ✅ Batch inserts (512 files/tx) for maximum throughput
- ✅ Progress bar during indexing, spinner during scan
- ✅ JSON output mode for all commands (`--json`)
- ✅ Human-readable file sizes everywhere
- ✅ `~` expansion in database path
=======
# File-Indexer
>>>>>>> 74e7c149ad87d690981ffaa6120ab52410e7530a
>>>>>>> 6191237 (Add File System Indexer project)
