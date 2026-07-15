use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::mpsc;
use walkdir::WalkDir;

use crate::models::FileEntry;

/// Scanning statistics collected during a build run
#[derive(Debug, Default)]
pub struct ScanStats {
    pub files_found: u64,
    pub dirs_visited: u64,
    pub files_skipped: u64,
    pub bytes_total: u64,
    pub elapsed_ms: u128,
}

/// Configuration for the scanner
pub struct ScanConfig {
    pub root: String,
    pub threads: usize,
}

/// Scan `config.root` recursively, skipping hidden entries, and stream
/// batches of `FileEntry` values through the returned channel.
///
/// Returns `(rx, stats_handle)` — consume `rx` while the scan runs, then
/// await `stats_handle` for final statistics.
pub async fn scan_directory(config: ScanConfig) -> Result<(Vec<FileEntry>, ScanStats)> {
    let root = std::fs::canonicalize(&config.root)
        .with_context(|| format!("Cannot access directory '{}'", config.root))?;

    if !root.is_dir() {
        anyhow::bail!("'{}' is not a directory", root.display());
    }

    // Progress spinner
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::with_template("{spinner:.cyan} [{elapsed_precise}] {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));
    spinner.set_message("Starting scan…");

    let start = std::time::Instant::now();

    let files_found = Arc::new(AtomicU64::new(0));
    let dirs_visited = Arc::new(AtomicU64::new(0));
    let files_skipped = Arc::new(AtomicU64::new(0));
    let bytes_total = Arc::new(AtomicU64::new(0));

    let ff = Arc::clone(&files_found);
    let dv = Arc::clone(&dirs_visited);
    let fs = Arc::clone(&files_skipped);
    let bt = Arc::clone(&bytes_total);
    let spinner_clone = spinner.clone();
    let root_clone = root.clone();

    // Use a channel for backpressure-friendly streaming
    let (tx, mut rx) = mpsc::channel::<Vec<FileEntry>>(64);
    const BATCH_SIZE: usize = 512;

    // Spawn the blocking walk on a dedicated thread pool
    let walk_handle = tokio::task::spawn_blocking(move || {
        let mut batch: Vec<FileEntry> = Vec::with_capacity(BATCH_SIZE);

        for result in WalkDir::new(&root_clone)
            .follow_links(false)
            .same_file_system(false)
            .into_iter()
            .filter_entry(|e| !is_hidden(e))
        {
            let entry = match result {
                Ok(e) => e,
                Err(_) => {
                    fs.fetch_add(1, Ordering::Relaxed);
                    continue;
                }
            };

            let file_type = entry.file_type();

            if file_type.is_dir() {
                dv.fetch_add(1, Ordering::Relaxed);
                spinner_clone.set_message(format!(
                    "Scanning  {}",
                    entry.path().display()
                ));
                continue;
            }

            if !file_type.is_file() {
                fs.fetch_add(1, Ordering::Relaxed);
                continue;
            }

            match build_entry(entry.path()) {
                Ok(file_entry) => {
                    bt.fetch_add(file_entry.size_bytes, Ordering::Relaxed);
                    ff.fetch_add(1, Ordering::Relaxed);
                    batch.push(file_entry);
                    if batch.len() >= BATCH_SIZE {
                        let send_batch = std::mem::replace(&mut batch, Vec::with_capacity(BATCH_SIZE));
                        // Best-effort send; if channel closed we stop
                        if tx.blocking_send(send_batch).is_err() {
                            break;
                        }
                    }
                }
                Err(_) => {
                    fs.fetch_add(1, Ordering::Relaxed);
                }
            }
        }

        // Flush remaining
        if !batch.is_empty() {
            let _ = tx.blocking_send(batch);
        }
    });

    // Collect all batches
    let mut all_entries: Vec<FileEntry> = Vec::new();
    while let Some(batch) = rx.recv().await {
        all_entries.extend(batch);
    }

    walk_handle.await.context("Scanner thread panicked")?;

    let elapsed_ms = start.elapsed().as_millis();
    spinner.finish_and_clear();

    Ok((
        all_entries,
        ScanStats {
            files_found: files_found.load(Ordering::Relaxed),
            dirs_visited: dirs_visited.load(Ordering::Relaxed),
            files_skipped: files_skipped.load(Ordering::Relaxed),
            bytes_total: bytes_total.load(Ordering::Relaxed),
            elapsed_ms,
        },
    ))
}

/// Build a `FileEntry` from a path using only `std::fs` metadata (no extra syscalls)
fn build_entry(path: &Path) -> Result<FileEntry> {
    let metadata = path
        .metadata()
        .with_context(|| format!("Cannot stat '{}'", path.display()))?;

    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string();

    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let absolute_path = path
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Non-UTF-8 path"))?
        .to_string();

    let size_bytes = metadata.len();

    let last_modified = metadata
        .modified()
        .unwrap_or(SystemTime::UNIX_EPOCH);
    let last_modified: DateTime<Utc> = last_modified.into();

    Ok(FileEntry::new(
        filename,
        extension,
        absolute_path,
        size_bytes,
        last_modified,
    ))
}

/// Returns `true` when a directory entry is hidden (starts with `.`)
fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}
