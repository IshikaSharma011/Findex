use clap::{Parser, Subcommand};

/// A production-quality File System Indexer — scan, store, and search file metadata at blazing speed.
#[derive(Parser, Debug)]
#[command(
    name = "indexer",
    version = "1.0.0",
    author = "FS Indexer",
    about = "Recursively indexes filesystem metadata into SQLite for fast searching",
    long_about = None,
    propagate_version = true,
)]
pub struct Cli {
    /// Path to the SQLite database file
    #[arg(
        long,
        global = true,
        default_value = "index.db",
        help = "Path to SQLite database file"
    )]
    pub db: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Recursively scan a directory and index all file metadata into SQLite
    #[command(name = "build")]
    Build {
        /// Directory to scan and index
        #[arg(value_name = "DIRECTORY", help = "Root directory to index")]
        directory: String,

        /// Clear existing index before building
        #[arg(long, short = 'c', help = "Wipe the existing index before scanning")]
        clean: bool,

        /// Number of parallel threads for scanning
        #[arg(
            long,
            short = 'j',
            default_value = "4",
            help = "Number of worker threads"
        )]
        threads: usize,
    },

    /// Search the index by filename keyword
    #[command(name = "search")]
    Search {
        /// Keyword to search for in filenames
        #[arg(value_name = "KEYWORD", help = "Search keyword (matches filename)")]
        keyword: String,

        /// Maximum number of results to display
        #[arg(
            long,
            short = 'n',
            default_value = "20",
            help = "Maximum results to show"
        )]
        limit: usize,

        /// Output results as JSON
        #[arg(long, help = "Output results as JSON")]
        json: bool,
    },

    /// Search the index by file extension
    #[command(name = "ext")]
    Ext {
        /// File extension to filter by (without leading dot)
        #[arg(value_name = "EXTENSION", help = "Extension to search (e.g., rs, py, txt)")]
        extension: String,

        /// Maximum number of results to display
        #[arg(
            long,
            short = 'n',
            default_value = "20",
            help = "Maximum results to show"
        )]
        limit: usize,

        /// Output results as JSON
        #[arg(long, help = "Output results as JSON")]
        json: bool,
    },

    /// Display indexing statistics and summary
    #[command(name = "stats")]
    Stats {
        /// Show top N extensions
        #[arg(long, default_value = "10", help = "Number of top extensions to show")]
        top: usize,

        /// Output stats as JSON
        #[arg(long, help = "Output stats as JSON")]
        json: bool,
    },
}
