use anyhow::{Context, Result};
use chrono::{DateTime, TimeZone, Utc};
use rusqlite::{params, Connection};
use std::path::Path;

use crate::models::{ExtensionStat, FileEntry, FileSizeStat, IndexStats};

/// Database version for schema migrations
const DB_VERSION: i64 = 1;

/// Manages all SQLite interactions for the file index
pub struct Database {
    conn: Connection,
    pub path: String,
}

impl Database {
    /// Open (or create) the SQLite database at the given path and run migrations
    pub fn open(path: &str) -> Result<Self> {
        let conn = Connection::open(path)
            .with_context(|| format!("Failed to open database at '{}'", path))?;

        // Performance tuning pragmas
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA cache_size = -32000;
             PRAGMA temp_store = MEMORY;
             PRAGMA mmap_size = 268435456;",
        )
        .context("Failed to set SQLite pragmas")?;

        let db = Self {
            conn,
            path: path.to_string(),
        };

        db.run_migrations()
            .context("Failed to run database migrations")?;

        Ok(db)
    }

    /// Run schema migrations idempotently
    fn run_migrations(&self) -> Result<()> {
        // Create version tracking table
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS schema_version (
                version INTEGER NOT NULL
            );",
        )?;

        let current_version: i64 = self
            .conn
            .query_row(
                "SELECT COALESCE(MAX(version), 0) FROM schema_version",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        if current_version < DB_VERSION {
            self.apply_migration_v1()?;
            self.conn.execute(
                "INSERT INTO schema_version (version) VALUES (?1)",
                params![DB_VERSION],
            )?;
        }

        Ok(())
    }

    /// Initial schema: files table + indices
    fn apply_migration_v1(&self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS files (
                id            INTEGER PRIMARY KEY AUTOINCREMENT,
                filename      TEXT    NOT NULL,
                extension     TEXT    NOT NULL DEFAULT '',
                absolute_path TEXT    NOT NULL UNIQUE,
                size_bytes    INTEGER NOT NULL DEFAULT 0,
                last_modified INTEGER NOT NULL,
                indexed_at    INTEGER NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_files_filename  ON files (filename COLLATE NOCASE);
            CREATE INDEX IF NOT EXISTS idx_files_extension ON files (extension COLLATE NOCASE);
            CREATE INDEX IF NOT EXISTS idx_files_size      ON files (size_bytes DESC);
            CREATE INDEX IF NOT EXISTS idx_files_indexed   ON files (indexed_at DESC);
            ",
        )
        .context("Failed to create files table / indices")
    }

    /// Wipe all indexed data (keeps schema)
    pub fn clear(&self) -> Result<()> {
        self.conn
            .execute_batch("DELETE FROM files;")
            .context("Failed to clear files table")
    }

    /// Insert a batch of file entries using a single transaction for maximum throughput
    pub fn insert_batch(&mut self, entries: &[FileEntry]) -> Result<usize> {
        if entries.is_empty() {
            return Ok(0);
        }

        let tx = self
            .conn
            .transaction()
            .context("Failed to begin transaction")?;

        let mut inserted = 0usize;

        {
            let mut stmt = tx.prepare_cached(
                "INSERT INTO files
                    (filename, extension, absolute_path, size_bytes, last_modified, indexed_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                 ON CONFLICT(absolute_path) DO UPDATE SET
                    filename      = excluded.filename,
                    extension     = excluded.extension,
                    size_bytes    = excluded.size_bytes,
                    last_modified = excluded.last_modified,
                    indexed_at    = excluded.indexed_at",
            )?;

            for entry in entries {
                stmt.execute(params![
                    entry.filename,
                    entry.extension,
                    entry.absolute_path,
                    entry.size_bytes as i64,
                    entry.last_modified.timestamp(),
                    entry.indexed_at.timestamp(),
                ])?;
                inserted += 1;
            }
        }

        tx.commit().context("Failed to commit transaction")?;
        Ok(inserted)
    }

    /// Search files by keyword (matches against filename, case-insensitive)
    pub fn search_by_keyword(&self, keyword: &str, limit: usize) -> Result<Vec<FileEntry>> {
        let pattern = format!("%{}%", keyword);
        let mut stmt = self.conn.prepare(
            "SELECT id, filename, extension, absolute_path, size_bytes, last_modified, indexed_at
             FROM files
             WHERE filename LIKE ?1
             ORDER BY filename ASC
             LIMIT ?2",
        )?;

        let entries = stmt
            .query_map(params![pattern, limit as i64], Self::row_to_entry)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .context("Failed to query by keyword")?;

        Ok(entries)
    }

    /// Search files by extension (case-insensitive, strip leading dot if present)
    pub fn search_by_extension(&self, ext: &str, limit: usize) -> Result<Vec<FileEntry>> {
        let clean_ext = ext.trim_start_matches('.').to_lowercase();
        let mut stmt = self.conn.prepare(
            "SELECT id, filename, extension, absolute_path, size_bytes, last_modified, indexed_at
             FROM files
             WHERE LOWER(extension) = ?1
             ORDER BY filename ASC
             LIMIT ?2",
        )?;

        let entries = stmt
            .query_map(params![clean_ext, limit as i64], Self::row_to_entry)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .context("Failed to query by extension")?;

        Ok(entries)
    }

    /// Gather aggregate statistics about the current index
    pub fn get_stats(&self, top_n: usize) -> Result<IndexStats> {
        // Totals
        let (total_files, total_size_bytes): (u64, u64) = self
            .conn
            .query_row(
                "SELECT COUNT(*), COALESCE(SUM(size_bytes), 0) FROM files",
                [],
                |row| Ok((row.get::<_, i64>(0)? as u64, row.get::<_, i64>(1)? as u64)),
            )
            .context("Failed to query totals")?;

        let unique_extensions: u64 = self
            .conn
            .query_row(
                "SELECT COUNT(DISTINCT extension) FROM files",
                [],
                |row| row.get::<_, i64>(0),
            )
            .map(|v| v as u64)
            .context("Failed to query unique extensions")?;

        // Top N extensions by file count
        let mut ext_stmt = self.conn.prepare(
            "SELECT extension, COUNT(*) as cnt, COALESCE(SUM(size_bytes), 0)
             FROM files
             GROUP BY extension
             ORDER BY cnt DESC
             LIMIT ?1",
        )?;
        let top_extensions = ext_stmt
            .query_map(params![top_n as i64], |row| {
                Ok(ExtensionStat {
                    extension: row.get(0)?,
                    count: row.get::<_, i64>(1)? as u64,
                    total_size_bytes: row.get::<_, i64>(2)? as u64,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()
            .context("Failed to query top extensions")?;

        // Top 5 largest files
        let mut large_stmt = self.conn.prepare(
            "SELECT filename, absolute_path, size_bytes
             FROM files
             ORDER BY size_bytes DESC
             LIMIT 5",
        )?;
        let largest_files = large_stmt
            .query_map([], |row| {
                Ok(FileSizeStat {
                    filename: row.get(0)?,
                    absolute_path: row.get(1)?,
                    size_bytes: row.get::<_, i64>(2)? as u64,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()
            .context("Failed to query largest files")?;

        // Last indexed timestamp
        let last_indexed: Option<DateTime<Utc>> = self
            .conn
            .query_row(
                "SELECT MAX(indexed_at) FROM files",
                [],
                |row| row.get::<_, Option<i64>>(0),
            )
            .ok()
            .flatten()
            .map(|ts| Utc.timestamp_opt(ts, 0).single())
            .flatten();

        Ok(IndexStats {
            total_files,
            total_size_bytes,
            unique_extensions,
            top_extensions,
            largest_files,
            last_indexed,
            index_path: self.path.clone(),
        })
    }

    /// Map a SQLite row to a FileEntry
    fn row_to_entry(row: &rusqlite::Row<'_>) -> rusqlite::Result<FileEntry> {
        let last_modified_ts: i64 = row.get(5)?;
        let indexed_at_ts: i64 = row.get(6)?;

        Ok(FileEntry {
            id: Some(row.get(0)?),
            filename: row.get(1)?,
            extension: row.get(2)?,
            absolute_path: row.get(3)?,
            size_bytes: row.get::<_, i64>(4)? as u64,
            last_modified: Utc
                .timestamp_opt(last_modified_ts, 0)
                .single()
                .unwrap_or_default(),
            indexed_at: Utc
                .timestamp_opt(indexed_at_ts, 0)
                .single()
                .unwrap_or_default(),
        })
    }

    /// Check whether the database has any indexed files
    pub fn is_empty(&self) -> Result<bool> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM files", [], |row| row.get(0))?;
        Ok(count == 0)
    }
}

/// Resolve the path to the database, expanding '~' if needed
pub fn resolve_db_path(path: &str) -> String {
    if path.starts_with('~') {
        if let Some(home) = dirs::home_dir() {
            return home
                .join(&path[2..])
                .to_string_lossy()
                .into_owned();
        }
    }
    path.to_string()
}

/// Return the size of the database file in bytes, if it exists
pub fn db_file_size(path: &str) -> u64 {
    Path::new(path)
        .metadata()
        .map(|m| m.len())
        .unwrap_or(0)
}
