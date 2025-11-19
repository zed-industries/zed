use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::Path;

pub mod after_release;
pub mod cherry_pick;
pub mod compare_perf;
pub mod danger;
pub mod nix_build;
pub mod release_nightly;
pub mod run_bundling;

pub mod release;
pub mod run_agent_evals;
pub mod run_tests;
pub mod runners;
pub mod steps;
pub mod vars;

#[derive(Parser)]
pub struct GenerateWorkflowArgs {}

pub fn run_workflows(_: GenerateWorkflowArgs) -> Result<()> {
    steps::set_repo_owner("zed-industries");

    let dir = Path::new(".github/workflows");

    let workflows = vec![
        ("danger.yml", danger::danger()),
        ("run_bundling.yml", run_bundling::run_bundling()),
        ("release_nightly.yml", release_nightly::release_nightly()),
        ("run_tests.yml", run_tests::run_tests()),
        ("release.yml", release::release()),
        ("cherry_pick.yml", cherry_pick::cherry_pick()),
        ("compare_perf.yml", compare_perf::compare_perf()),
        ("run_unit_evals.yml", run_agent_evals::run_unit_evals()),
        (
            "run_cron_unit_evals.yml",
            run_agent_evals::run_cron_unit_evals(),
        ),
        ("run_agent_evals.yml", run_agent_evals::run_agent_evals()),
        ("after_release.yml", after_release::after_release()),
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
