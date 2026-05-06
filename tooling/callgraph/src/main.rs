use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

/// Static analysis tool that detects blocking calls in async Rust code.
///
/// Walks the AST of Rust source files, identifies async functions,
/// and checks for calls to known-blocking operations like std::fs::*,
/// std::thread::sleep, Mutex::lock, etc.
#[derive(Parser, Debug)]
#[command(name = "zed-callgraph", version)]
struct Cli {
    /// Package(s) to analyze (e.g., `-p editor`). Omit to analyze the whole workspace.
    #[arg(short, long)]
    package: Vec<String>,

    /// Path to the Cargo workspace root. Defaults to current directory.
    #[arg(long, default_value = ".")]
    manifest_path: PathBuf,

    /// Include pedantic checks (e.g., parking_lot::Mutex::lock).
    #[arg(long)]
    pedantic: bool,

    /// Output format: "human" or "json".
    #[arg(long, default_value = "human")]
    output: String,

    /// Include test code in the analysis. By default, `#[test]` functions,
    /// `#[cfg(test)]` modules, and files under `tests/` directories are skipped.
    #[arg(long)]
    include_tests: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let blocklist = callgraph::blocklist::Blocklist::load(cli.pedantic)?;
    let source_files =
        callgraph::analyzer::discover_source_files(&cli.manifest_path, &cli.package)?;

    let mut total_warnings = 0;
    for source_file in &source_files {
        let analysis =
            callgraph::analyzer::analyze_file(source_file, &blocklist, cli.include_tests)?;
        callgraph::diagnostics::emit_file_warnings(
            &analysis.warnings,
            &analysis.source,
            &analysis.path.to_string_lossy(),
            &cli.output,
        );
        total_warnings += analysis.warnings.len();
    }

    if total_warnings > 0 {
        eprintln!("\nFound {total_warnings} blocking call(s) in async contexts.");
        std::process::exit(1);
    }

    Ok(())
}
