use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::Path;

mod danger;
mod nix;
mod run_bundling;

mod runners;
mod steps;
mod vars;

#[derive(Parser)]
pub struct GenerateWorkflowArgs {}

pub fn run_workflows(_: GenerateWorkflowArgs) -> Result<()> {
    let dir = Path::new(".github/workflows");

    let workflows = vec![
        ("danger.yml", danger::danger()),
        ("nix.yml", nix::nix()),
        ("run_bundling.yml", run_bundling::run_bundling()),
    ];
    fs::create_dir_all(dir)
        .with_context(|| format!("Failed to create directory: {}", dir.display()))?;

    for (filename, workflow) in workflows {
        let content = workflow
            .to_string()
            .map_err(|e| anyhow::anyhow!("{}: {:?}", filename, e))?;
        let content = format!("# generated `cargo xtask workflows`. Do not edit.\n{content}");
        let file_path = dir.join(filename);
        fs::write(&file_path, content)?;
    }

    Ok(())
}
