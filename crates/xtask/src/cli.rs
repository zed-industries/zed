use clap::{Parser, Subcommand};
use std::path::PathBuf;
/// Common utilities for Zed developers.
// For more information, see [matklad's repository README](https://github.com/matklad/cargo-xtask/)
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Command to run.
#[derive(Subcommand)]
pub enum Commands {
    /// Builds theme types for interop with Typescript.
    BuildThemeTypes {
        #[clap(short, long, default_value = "schemas")]
        out_dir: PathBuf,
        #[clap(short, long, default_value = "theme.json")]
        file_name: PathBuf,
    },
}
