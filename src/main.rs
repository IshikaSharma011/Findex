mod cli;
mod database;
mod models;
mod scanner;
mod search;

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use humansize::{format_size, DECIMAL};
use indicatif::{ProgressBar, ProgressStyle};

use cli::{Cli, Commands};
use database::{db_file_size, resolve_db_path, Database};
use scanner::{scan_directory, ScanConfig};
use search::{print_results, print_results_json, search_extension, search_keyword};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!();
        eprintln!("  {} {}", "error:".red().bold(), e);
        for cause in e.chain().skip(1) {
            eprintln!("  {} {}", "caused by:".dimmed(), cause);
        }
        eprintln!();
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();
    let db_path = resolve_db_path(&cli.db);

    match cli.command {
        Commands::Build {
            directory,
            clean,
            threads,
        } => cmd_build(db_path, directory, clean, threads).await,

        Commands::Search { keyword, limit, json } => {
            cmd_search(db_path, keyword, limit, json)
        }

        Commands::Ext {
            extension,
            limit,
            json,
        } => cmd_ext(db_path, extension, limit, json),

        Commands::Stats { top, json } => cmd_stats(db_path, top, json),
    }
}

// ── Commands ─────────────────────────────────────────────────────────────────

/// Build command: scan a directory and persist metadata into SQLite
async fn cmd_build(db_path: String, directory: String, clean: bool, threads: usize) -> Result<()> {
    print_banner();
    println!(
        "  {} {}",
        "Building index from:".bold(),
        directory.cyan()
    );
    println!("  {} {}", "Database:".bold(), db_path.cyan());
    println!(
        "  {} {}",
        "Threads:".bold(),
        threads.to_string().cyan()
    );
    println!();

    let mut db = Database::open(&db_path).context("Failed to open database")?;

    if clean {
        println!("  {} Clearing existing index…", "⚑".yellow());
        db.clear().context("Failed to clear database")?;
        println!("  {} Index cleared.", "✓".green());
        println!();
    }

    // ── Scan ──────────────────────────────────────────────────────────────
    let config = ScanConfig {
        root: directory.clone(),
        threads,
    };

    println!("  {} Scanning filesystem…", "⟳".cyan());
    let (entries, scan_stats) = scan_directory(config).await?;

    println!();
    println!(
        "  {} Found {} files in {} directories ({} ms)",
        "✓".green(),
        scan_stats.files_found.to_string().bold(),
        scan_stats.dirs_visited.to_string().bold(),
        scan_stats.elapsed_ms,
    );
    if scan_stats.files_skipped > 0 {
        println!(
            "  {} Skipped {} inaccessible/symlink entries",
            "⚠".yellow(),
            scan_stats.files_skipped
        );
    }
    println!();

    // ── Insert ────────────────────────────────────────────────────────────
    if entries.is_empty() {
        println!("  {} No files to index.", "⚑".yellow());
        return Ok(());
    }

    let total = entries.len();
    let pb = ProgressBar::new(total as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "  {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
        )
        .unwrap()
        .progress_chars("█▉▊▋▌▍▎▏  "),
    );

    // Insert in chunks for progress reporting
    const CHUNK: usize = 1000;
    let mut total_inserted = 0usize;

    for chunk in entries.chunks(CHUNK) {
        let inserted = db
            .insert_batch(chunk)
            .context("Failed to insert batch into database")?;
        total_inserted += inserted;
        pb.inc(chunk.len() as u64);
    }

    pb.finish_and_clear();

    let db_size = db_file_size(&db_path);
    println!(
        "  {} Indexed {} files  (DB size: {})",
        "✓".green(),
        total_inserted.to_string().green().bold(),
        format_size(db_size, DECIMAL).yellow(),
    );
    println!();
    println!("  Run {}  to explore the index.", "`indexer stats`".bold().cyan());
    println!();

    Ok(())
}

/// Search command: keyword search across filenames
fn cmd_search(db_path: String, keyword: String, limit: usize, json: bool) -> Result<()> {
    let db = open_db_or_bail(&db_path)?;
    let result = search_keyword(&db, &keyword, limit)?;

    if json {
        print_results_json(&result)?;
    } else {
        print_results(&result);
    }
    Ok(())
}

/// Ext command: filter by file extension
fn cmd_ext(db_path: String, extension: String, limit: usize, json: bool) -> Result<()> {
    let db = open_db_or_bail(&db_path)?;
    let result = search_extension(&db, &extension, limit)?;

    if json {
        print_results_json(&result)?;
    } else {
        print_results(&result);
    }
    Ok(())
}

/// Stats command: show aggregate statistics about the index
fn cmd_stats(db_path: String, top: usize, json: bool) -> Result<()> {
    let db = open_db_or_bail(&db_path)?;
    let stats = db.get_stats(top)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&stats)?);
        return Ok(());
    }

    print_banner();
    println!("  {}", "Index Statistics".bold().underline());
    println!();

    // ── Summary ───────────────────────────────────────────────────────────
    println!("  {}", "Summary".bold());
    println!(
        "  {:.<30} {}",
        "Total files ",
        stats.total_files.to_string().green().bold()
    );
    println!(
        "  {:.<30} {}",
        "Total size ",
        format_size(stats.total_size_bytes, DECIMAL).yellow().bold()
    );
    println!(
        "  {:.<30} {}",
        "Unique extensions ",
        stats.unique_extensions.to_string().cyan().bold()
    );
    if let Some(ts) = stats.last_indexed {
        println!(
            "  {:.<30} {}",
            "Last indexed ",
            ts.format("%Y-%m-%d %H:%M:%S UTC").to_string().dimmed()
        );
    }
    println!(
        "  {:.<30} {}",
        "Database path ",
        stats.index_path.dimmed()
    );
    let db_size = db_file_size(&stats.index_path);
    println!(
        "  {:.<30} {}",
        "Database size ",
        format_size(db_size, DECIMAL).dimmed()
    );
    println!();

    // ── Top extensions ────────────────────────────────────────────────────
    if !stats.top_extensions.is_empty() {
        println!("  {}", format!("Top {} Extensions by Count", top).bold());
        println!(
            "  {:<5}  {:<12}  {:>10}  {:>14}",
            "#".dimmed(),
            "EXT".dimmed(),
            "FILES".dimmed(),
            "TOTAL SIZE".dimmed(),
        );
        println!("  {}", "─".repeat(52).dimmed());

        let max_count = stats.top_extensions.first().map(|e| e.count).unwrap_or(1);

        for (i, ext) in stats.top_extensions.iter().enumerate() {
            let bar_len = ((ext.count as f64 / max_count as f64) * 20.0) as usize;
            let bar = "█".repeat(bar_len);
            let ext_label = if ext.extension.is_empty() {
                "(none)".to_string()
            } else {
                format!(".{}", ext.extension)
            };

            println!(
                "  {:>4}.  {:<12}  {:>10}  {:>14}  {}",
                (i + 1).to_string().dimmed(),
                ext_label.cyan(),
                ext.count.to_string().bold(),
                format_size(ext.total_size_bytes, DECIMAL).yellow(),
                bar.green(),
            );
        }
        println!();
    }

    // ── Largest files ─────────────────────────────────────────────────────
    if !stats.largest_files.is_empty() {
        println!("  {}", "Top 5 Largest Files".bold());
        println!(
            "  {:<5}  {:<30}  {:>12}  {}",
            "#".dimmed(),
            "FILENAME".dimmed(),
            "SIZE".dimmed(),
            "PATH".dimmed(),
        );
        println!("  {}", "─".repeat(90).dimmed());

        for (i, f) in stats.largest_files.iter().enumerate() {
            println!(
                "  {:>4}.  {:<30}  {:>12}  {}",
                (i + 1).to_string().dimmed(),
                truncate_str(&f.filename, 30).bold(),
                format_size(f.size_bytes, DECIMAL).yellow().bold(),
                shrink_home(&f.absolute_path, 50).dimmed(),
            );
        }
        println!();
    }

    Ok(())
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn open_db_or_bail(db_path: &str) -> Result<Database> {
    let db = Database::open(db_path).context("Failed to open database")?;
    if db.is_empty()? {
        eprintln!(
            "\n  {} Index is empty. Run {} first.\n",
            "⚠".yellow(),
            "`indexer build <directory>`".bold().cyan()
        );
        std::process::exit(2);
    }
    Ok(db)
}

fn print_banner() {
    println!();
    println!(
        "{}",
        "  ╔══════════════════════════════════╗"
            .bold()
            .bright_cyan()
    );
    println!(
        "{}",
        "  ║   📁  File System Indexer v1.0  ║"
            .bold()
            .bright_cyan()
    );
    println!(
        "{}",
        "  ╚══════════════════════════════════╝"
            .bold()
            .bright_cyan()
    );
    println!();
}

fn truncate_str(s: &str, n: usize) -> String {
    if s.len() <= n {
        s.to_string()
    } else {
        format!("{}…", &s[..n.saturating_sub(1)])
    }
}

fn shrink_home(path: &str, max: usize) -> String {
    let home = dirs::home_dir()
        .and_then(|h| h.to_str().map(|s| s.to_string()))
        .unwrap_or_default();
    let s = if !home.is_empty() && path.starts_with(&home) {
        format!("~{}", &path[home.len()..])
    } else {
        path.to_string()
    };
    if s.len() > max {
        format!("…{}", &s[s.len().saturating_sub(max)..])
    } else {
        s
    }
}
