use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use language::Point;
use std::path::{Path, PathBuf};
use std::str::FromStr;

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

        let point = Point::new(line.saturating_sub(1), column.saturating_sub(1));

        Ok(CursorPosition { path, point })
    }
}

fn get_context(worktree: &Path, cursor: &CursorPosition) -> Result<()> {
    println!("Getting context for:");
    println!("  Worktree: {}", worktree.display());
    println!("  File: {}", cursor.path.display());
    println!(
        "  Position: line {}, column {}",
        cursor.point.row + 1,
        cursor.point.column + 1
    );

    // TODO: Implement actual context retrieval logic
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::GetContext { worktree, cursor } => {
            get_context(&worktree, &cursor)?;
        }
    }

    Ok(())
}
