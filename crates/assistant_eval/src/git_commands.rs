use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::{
    fs,
    path::Path,
};
use tempfile::TempDir;
use util::command::new_smol_command;
use walkdir::WalkDir;

#[derive(Debug, Deserialize)]
pub struct SetupConfig {
    #[serde(rename = "base.sha")]
    pub base_sha: String,
}

pub async fn run_git_command(repo_path: &Path, args: Vec<&str>) -> Result<String> {
    let output = new_smol_command("git")
        .current_dir(repo_path)
        .args(args.clone())
        .output()
        .await?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    } else {
        Err(anyhow!(
            "Git command failed: {} with status: {}",
            args.join(" "),
            output.status
        ))
    }
}

pub async fn read_base_sha(framework_path: &Path) -> Result<String> {
    let setup_path = framework_path.join("setup.json");
    let setup_content = smol::unblock(move || std::fs::read_to_string(&setup_path)).await?;
    let setup_config: SetupConfig = serde_json_lenient::from_str_lenient(&setup_content)?;
    Ok(setup_config.base_sha)
}

pub async fn checkout_repo(repo_path: &Path, commit_sha: &str) -> Result<()> {
    run_git_command(repo_path, vec!["checkout", commit_sha]).await?;
    Ok(())
}

pub async fn run_git(repo_path: &Path, args: &[&str]) -> Result<String> {
    run_git_command(repo_path, args.to_vec()).await
}

pub async fn query_git(repo_path: &Path, args: &[&str]) -> Result<String> {
    run_git_command(repo_path, args.to_vec()).await
}

pub async fn setup_temp_repo(exercise_path: &Path, _base_sha: &str) -> Result<TempDir> {
    let temp_dir = TempDir::new()?;

    // Copy the exercise files to the temp directory, excluding .docs and .meta
    for entry in WalkDir::new(exercise_path).min_depth(0).max_depth(10) {
        let entry = entry?;
        let source_path = entry.path();

        // Skip .docs and .meta directories completely
        if source_path.starts_with(exercise_path.join(".docs"))
            || source_path.starts_with(exercise_path.join(".meta"))
        {
            continue;
        }

        if source_path.is_file() {
            let relative_path = source_path.strip_prefix(exercise_path)?;
            let dest_path = temp_dir.path().join(relative_path);

            // Make sure parent directories exist
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::copy(source_path, dest_path)?;
        }
    }

    // Initialize git repo in the temp directory
    run_git_command(temp_dir.path(), vec!["init"]).await?;
    run_git_command(temp_dir.path(), vec!["add", "."]).await?;
    run_git_command(temp_dir.path(), vec!["commit", "-m", "Initial commit"]).await?;

    println!("Created temp repo without .docs and .meta directories");

    Ok(temp_dir)
}
