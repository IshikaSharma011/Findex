use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a single indexed file entry stored in SQLite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub id: Option<i64>,
    pub filename: String,
    pub extension: String,
    pub absolute_path: String,
    pub size_bytes: u64,
    pub last_modified: DateTime<Utc>,
    pub indexed_at: DateTime<Utc>,
}

impl FileEntry {
    pub fn new(
        filename: String,
        extension: String,
        absolute_path: String,
        size_bytes: u64,
        last_modified: DateTime<Utc>,
    ) -> Self {
        Self {
            id: None,
            filename,
            extension,
            absolute_path,
            size_bytes,
            last_modified,
            indexed_at: Utc::now(),
        }
    }
}

/// Statistics about the current index
#[derive(Debug, Serialize, Deserialize)]
pub struct IndexStats {
    pub total_files: u64,
    pub total_size_bytes: u64,
    pub unique_extensions: u64,
    pub top_extensions: Vec<ExtensionStat>,
    pub largest_files: Vec<FileSizeStat>,
    pub last_indexed: Option<DateTime<Utc>>,
    pub index_path: String,
}

/// Per-extension statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionStat {
    pub extension: String,
    pub count: u64,
    pub total_size_bytes: u64,
}

/// File size entry for top-N largest files
#[derive(Debug, Serialize, Deserialize)]
pub struct FileSizeStat {
    pub filename: String,
    pub absolute_path: String,
    pub size_bytes: u64,
}

/// Results from a search query
#[derive(Debug)]
pub struct SearchResult {
    pub entries: Vec<FileEntry>,
    pub query: String,
    pub search_type: SearchType,
    pub elapsed_ms: u128,
}

/// Type of search performed
#[derive(Debug, Clone)]
pub enum SearchType {
    Keyword,
    Extension,
}

impl std::fmt::Display for SearchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchType::Keyword => write!(f, "keyword"),
            SearchType::Extension => write!(f, "extension"),
        }
    }
}
