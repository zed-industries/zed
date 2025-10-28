use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::Path;

mod danger;
mod nix_build;
// mod release;
mod run_bundling;
// mod run_tests;

mod runners;
mod steps;
mod vars;

#[derive(Parser)]
pub struct GenerateWorkflowArgs {}

pub fn run_workflows(_: GenerateWorkflowArgs) -> Result<()> {
    let dir = Path::new(".github/workflows");

    let workflows = vec![
        ("danger.yml", danger::danger()),
        ("nix_build.yml", nix_build::nix_build()),
        ("run_bundling.yml", run_bundling::run_bundling()),
        // ("run_tests.yml", run_tests::run_tests()),
        // ("release.yml", release::release()),
    ];
    fs::create_dir_all(dir)
        .with_context(|| format!("Failed to create directory: {}", dir.display()))?;

    for (filename, workflow) in workflows {
        let content = workflow
            .to_string()
            .map_err(|e| anyhow::anyhow!("{}: {:?}", filename, e))?;
        let content = format!(
            "# Generated from xtask::workflows::{}\n# Rebuild with `cargo xtask workflows`.\n{}",
            workflow.name.unwrap(),
            content
        );
        let file_path = dir.join(filename);
        fs::write(&file_path, content)?;
    }

    Ok(())
}
