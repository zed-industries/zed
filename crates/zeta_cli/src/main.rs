mod headless;

use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use gpui::{App, AppContext, Application};
use language::Buffer;
use language::Point;
use reqwest_client::ReqwestClient;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::str::FromStr;
use std::sync::Arc;
use zeta::build_predict_edits_body_for_cli;

#[derive(Parser, Debug)]
#[command(name = "zeta")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    GetContext {
        #[arg(long)]
        worktree: PathBuf,
        #[arg(long)]
        cursor: CursorPosition,
    },
}

#[derive(Debug, Clone)]
struct CursorPosition {
    path: PathBuf,
    point: Point,
}

impl FromStr for CursorPosition {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 3 {
            return Err(anyhow!(
                "Invalid cursor format. Expected 'file.rs:line:column', got '{}'",
                s
            ));
        }

        let path = PathBuf::from(parts[0]);
        let line: u32 = parts[1]
            .parse()
            .map_err(|_| anyhow!("Invalid line number: '{}'", parts[1]))?;
        let column: u32 = parts[2]
            .parse()
            .map_err(|_| anyhow!("Invalid column number: '{}'", parts[2]))?;

        // Convert from 1-based to 0-based indexing
        let point = Point::new(line.saturating_sub(1), column.saturating_sub(1));

        Ok(CursorPosition { path, point })
    }
}

fn get_context(worktree: &Path, cursor: &CursorPosition, cx: &mut App) -> Result<()> {
    // Resolve the full path of the file
    let file_path = if cursor.path.is_absolute() {
        cursor.path.clone()
    } else {
        worktree.join(&cursor.path)
    };

    // Read the file content
    let content = std::fs::read_to_string(&file_path)
        .map_err(|e| anyhow!("Failed to read file {}: {}", file_path.display(), e))?;

    // Create a buffer from the file content
    let buffer = cx.new(|cx| Buffer::local(content, cx));
    let snapshot = buffer.read(cx).snapshot();

    // Build the PredictEditsBody using the shared function
    let body = build_predict_edits_body_for_cli(
        cursor.point,
        &cursor.path,
        &snapshot,
        false, // can_collect_data = false for CLI usage
    )?;

    // Output as pretty-printed JSON
    let json = serde_json::to_string_pretty(&body)?;
    println!("{}", json);

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let http_client = Arc::new(ReqwestClient::new());
    let app = Application::headless().with_http_client(http_client);

    match args.command {
        Commands::GetContext { worktree, cursor } => {
            app.run(move |cx| {
                headless::init(cx);
                if let Err(e) = get_context(&worktree, &cursor, cx) {
                    eprintln!("Failed to get context: {}", e);
                    exit(1);
                }
                cx.quit();
            });
        }
    }

    Ok(())
}
