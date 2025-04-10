use anyhow::{Result, anyhow};
use serde::Deserialize;
use std::{fs, path::Path};
use tempfile::TempDir;
use util::command::new_smol_command;
use walkdir::WalkDir;

#[derive(Debug, Deserialize)]
pub struct SetupConfig {
    #[serde(rename = "base.sha")]
    pub base_sha: String,
}

#[derive(Debug, Deserialize)]
pub struct RepoInfo {
    pub remote_url: String,
    pub head_sha: String,
}

pub async fn run_git(repo_path: &Path, args: &[&str]) -> Result<String> {
    let output = new_smol_command("git")
        .current_dir(repo_path)
        .args(args)
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

pub async fn read_repo_info(exercise_path: &Path) -> Result<RepoInfo> {
    let repo_info_path = exercise_path.join(".meta").join("repo_info.json");
    println!("Reading repo info from: {}", repo_info_path.display());
    let repo_info_content = smol::unblock(move || std::fs::read_to_string(&repo_info_path)).await?;
    let repo_info: RepoInfo = serde_json_lenient::from_str_lenient(&repo_info_content)?;

    // Remove any quotes from the strings
    let remote_url = repo_info.remote_url.trim_matches('"').to_string();
    let head_sha = repo_info.head_sha.trim_matches('"').to_string();

    Ok(RepoInfo {
        remote_url,
        head_sha,
    })
}

pub async fn setup_temp_repo(exercise_path: &Path, _base_sha: &str) -> Result<TempDir> {
    let temp_dir = TempDir::new()?;

    // Check if this is an internal exercise by looking for repo_info.json
    let repo_info_path = exercise_path.join(".meta").join("repo_info.json");
    if repo_info_path.exists() {
        // This is an internal exercise, handle it differently
        let repo_info = read_repo_info(exercise_path).await?;

        // Clone the repository to the temp directory
        let url = repo_info.remote_url;
        let clone_path = temp_dir.path();
        println!(
            "Cloning repository from {} to {}",
            url,
            clone_path.display()
        );
        run_git(
            &std::env::current_dir()?,
            &["clone", &url, &clone_path.to_string_lossy()],
        )
        .await?;

        // Checkout the specified commit
        println!("Checking out commit: {}", repo_info.head_sha);
        run_git(temp_dir.path(), &["checkout", &repo_info.head_sha]).await?;

        println!("Successfully set up internal repository");
    } else {
        // Original code for regular exercises
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
        run_git(temp_dir.path(), &["init"]).await?;
        run_git(temp_dir.path(), &["add", "."]).await?;
        run_git(temp_dir.path(), &["commit", "-m", "Initial commit"]).await?;

        println!("Created temp repo without .docs and .meta directories");
    }

    Ok(temp_dir)
}
