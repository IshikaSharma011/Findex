use anyhow::Result;
use colored::Colorize;
use humansize::{format_size, DECIMAL};
use std::time::Instant;

use crate::database::Database;
use crate::models::{SearchResult, SearchType};

/// Execute a keyword search and return a `SearchResult`
pub fn search_keyword(db: &Database, keyword: &str, limit: usize) -> Result<SearchResult> {
    let start = Instant::now();
    let entries = db.search_by_keyword(keyword, limit)?;
    let elapsed_ms = start.elapsed().as_millis();

    Ok(SearchResult {
        entries,
        query: keyword.to_string(),
        search_type: SearchType::Keyword,
        elapsed_ms,
    })
}

/// Execute an extension search and return a `SearchResult`
pub fn search_extension(db: &Database, ext: &str, limit: usize) -> Result<SearchResult> {
    let start = Instant::now();
    let entries = db.search_by_extension(ext, limit)?;
    let elapsed_ms = start.elapsed().as_millis();

    Ok(SearchResult {
        entries,
        query: ext.to_string(),
        search_type: SearchType::Extension,
        elapsed_ms,
    })
}

/// Print search results in a rich human-readable table
pub fn print_results(result: &SearchResult) {
    let count = result.entries.len();

    // ── Header ─────────────────────────────────────────────────────────────
    println!();
    println!(
        "{}",
        format!(
            " 🔍  {} search for '{}' — {} result{} ({} ms)",
            result.search_type,
            result.query,
            count,
            if count == 1 { "" } else { "s" },
            result.elapsed_ms,
        )
        .bold()
        .white()
        .on_bright_black()
    );
    println!();

    if result.entries.is_empty() {
        println!("  {}  No matches found.", "✗".red());
        println!();
        return;
    }

    // ── Column header ──────────────────────────────────────────────────────
    println!(
        "  {:<5}  {:<32}  {:<8}  {:<12}  {}",
        "#".dimmed(),
        "FILENAME".dimmed(),
        "EXT".dimmed(),
        "SIZE".dimmed(),
        "PATH".dimmed(),
    );
    println!("  {}", "─".repeat(100).dimmed());

    // ── Rows ───────────────────────────────────────────────────────────────
    for (idx, entry) in result.entries.iter().enumerate() {
        let idx_str = format!("{:>4}.", idx + 1);
        let filename = truncate(&entry.filename, 32);
        let ext = if entry.extension.is_empty() {
            "—".dimmed().to_string()
        } else {
            format!(".{}", entry.extension).cyan().to_string()
        };
        let size = format_size(entry.size_bytes, DECIMAL);
        let path = shrink_path(&entry.absolute_path, 60);

        let row = format!(
            "  {:<5}  {:<32}  {:<8}  {:>12}  {}",
            idx_str.dimmed(),
            filename.bold(),
            ext,
            size.yellow().to_string(),
            path.dimmed(),
        );

        if idx % 2 == 0 {
            println!("{}", row);
        } else {
            println!("{}", row);
        }
    }

    println!("  {}", "─".repeat(100).dimmed());
    println!(
        "  {} {} file{} matched",
        "→".green(),
        count.to_string().green().bold(),
        if count == 1 { "" } else { "s" }
    );
    println!();
}

/// Print search results as compact JSON (delegated to serde)
pub fn print_results_json(result: &SearchResult) -> Result<()> {
    let out = serde_json::to_string_pretty(&result.entries)?;
    println!("{}", out);
    Ok(())
}

/// Truncate a string to `max_chars`, appending "…" if needed
fn truncate(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        format!("{}…", &s[..max_chars.saturating_sub(1)])
    }
}

/// Shorten a path by replacing the home directory with '~'
fn shrink_path(path: &str, max_len: usize) -> String {
    let home = dirs::home_dir()
        .and_then(|h| h.to_str().map(|s| s.to_string()))
        .unwrap_or_default();

    let shortened = if !home.is_empty() && path.starts_with(&home) {
        format!("~{}", &path[home.len()..])
    } else {
        path.to_string()
    };

    if shortened.len() <= max_len {
        shortened
    } else {
        let keep = max_len.saturating_sub(4);
        format!("…{}", &shortened[shortened.len().saturating_sub(keep)..])
    }
}
