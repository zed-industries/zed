mod cli;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use schemars::schema_for;
use theme::Theme;

fn build_themes(mut out_dir: PathBuf, file_name: PathBuf) -> Result<()> {
    let theme = schema_for!(Theme);
    let output = serde_json::to_string_pretty(&theme)?;

    std::fs::create_dir(&out_dir)?;

    let mut file_path = out_dir;
    out_dir.push(file_name);

    std::fs::write(file_path, output)?;

    Ok(())
}

fn main() -> Result<()> {
    let args = cli::Cli::parse();
    match args.command {
        cli::Commands::BuildThemeTypes { out_dir, file_name } => build_themes(out_dir, file_name),
    }
}
