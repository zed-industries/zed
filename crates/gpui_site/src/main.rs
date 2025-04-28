use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

mod templates;
mod markdown;
mod examples;
mod generator;

/// Static site generator for gpui documentation
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output directory for the generated site
    #[arg(short, long, default_value = "site")]
    output_dir: PathBuf,

    /// gpui crate directory
    #[arg(short, long)]
    gpui_dir: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Determine gpui directory
    let gpui_dir = args.gpui_dir.unwrap_or_else(|| {
        // Default to the sibling directory if not specified
        std::env::current_dir()
            .expect("Failed to get current directory")
            .parent()
            .expect("Failed to get parent directory")
            .join("gpui")
    });
    
    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&args.output_dir)
        .with_context(|| format!("Failed to create output directory: {}", args.output_dir.display()))?;
    
    println!("Generating gpui site from {} to {}", 
        gpui_dir.display(), 
        args.output_dir.display());
    
    // Generate the site
    generator::generate_site(&gpui_dir, &args.output_dir)?;
    
    println!("Site generation complete!");
    Ok()
}