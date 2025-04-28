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
    #[arg(short, long, default_value = "out")]
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
        let current_dir = std::env::current_dir()
            .expect("Failed to get current directory");
        current_dir
            .parent()
            .expect("Failed to get parent directory")
            .join("gpui")
    });
    
    // Determine output directory - make it relative to the gpui_site crate directory
    let output_dir = if args.output_dir.is_absolute() {
        args.output_dir
    } else {
        // Find path to gpui_site crate directory regardless of where we're running from
        let workspace_root = std::env::current_dir().expect("Failed to get current directory");
        let gpui_site_dir = workspace_root.join("crates").join("gpui_site");
        
        // Create the output path relative to the gpui_site directory
        gpui_site_dir.join(&args.output_dir)
    };
    
    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&output_dir)
        .with_context(|| format!("Failed to create output directory: {}", output_dir.display()))?;
    
    println!("Output directory: {}", output_dir.display());
    
    println!("Generating gpui site from {} to {}", 
        gpui_dir.display(), 
        output_dir.display());
    
    // Generate the site
    generator::generate_site(&gpui_dir, &output_dir)?;
    
    println!("Site generation complete!");
    Ok(())
}