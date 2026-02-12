use anyhow::{Context as _, Result};
use collections::HashMap;
use futures::lock::{Mutex, OwnedMutexGuard};
use std::{
    cell::RefCell,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::paths::REPOS_DIR;

thread_local! {
    static REPO_LOCKS: RefCell<HashMap<PathBuf, Arc<Mutex<()>>>> = RefCell::new(HashMap::default());
}

#[must_use]
pub async fn lock_repo(path: impl AsRef<Path>) -> OwnedMutexGuard<()> {
    REPO_LOCKS
        .with(|cell| {
            cell.borrow_mut()
                .entry(path.as_ref().to_path_buf())
                .or_default()
                .clone()
        })
        .lock_owned()
        .await
}

pub async fn run_git(repo_path: &Path, args: &[&str]) -> Result<String> {
    let output = smol::process::Command::new("git")
        .current_dir(repo_path)
        .args(args)
        .output()
        .await?;

    anyhow::ensure!(
        output.status.success(),
        "`git {}` within `{}` failed with status: {}\nstderr:\n{}\nstdout:\n{}",
        args.join(" "),
        repo_path.display(),
        output.status,
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout),
    );
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

pub fn parse_repo_url(url: &str) -> Result<(String, String)> {
    if url.contains('@') {
        let (_, path) = url.split_once(':').context("expected : in git url")?;
        let (owner, repo) = path.split_once('/').context("expected / in git url")?;
        Ok((owner.to_string(), repo.trim_end_matches(".git").to_string()))
    } else {
        let parsed = http_client::Url::parse(url)?;
        let mut segments = parsed.path_segments().context("empty http url")?;
        let owner = segments.next().context("expected owner")?;
        let repo = segments.next().context("expected repo")?;
        Ok((owner.to_string(), repo.trim_end_matches(".git").to_string()))
    }
}

pub fn repo_path_for_url(url: &str) -> Result<PathBuf> {
    let (owner, name) = parse_repo_url(url)?;
    Ok(REPOS_DIR.join(&owner).join(&name))
}

pub async fn ensure_repo_cloned(repo_url: &str) -> Result<PathBuf> {
    let repo_path = repo_path_for_url(repo_url)?;
    let _lock = lock_repo(&repo_path).await;

    // Validate existing repo has correct origin, otherwise remove and re-init.
    let mut git_repo_exists = false;
    if repo_path.is_dir() {
        if run_git(&repo_path, &["remote", "get-url", "origin"])
            .await
            .map_or(false, |origin| origin.trim() == repo_url)
        {
            git_repo_exists = true;
        } else {
            std::fs::remove_dir_all(&repo_path).ok();
        }
    }

    if !git_repo_exists {
        log::info!("Cloning {} into {:?}", repo_url, repo_path);
        std::fs::create_dir_all(&repo_path)?;
        run_git(&repo_path, &["init"]).await?;
        run_git(&repo_path, &["remote", "add", "origin", repo_url])
            .await
            .ok();
    }

    // Always fetch to get latest commits
    run_git(&repo_path, &["fetch", "origin"]).await?;

    // Check if we have a valid HEAD, if not checkout FETCH_HEAD
    let has_head = run_git(&repo_path, &["rev-parse", "HEAD"]).await.is_ok();
    if !has_head {
        // Use reset to set HEAD without needing a branch
        run_git(&repo_path, &["reset", "--hard", "FETCH_HEAD"]).await?;
    }

    Ok(repo_path)
}

pub async fn fetch_if_needed(repo_path: &Path, revision: &str) -> Result<String> {
    let resolved = run_git(
        repo_path,
        &["rev-parse", &format!("{}^{{commit}}", revision)],
    )
    .await;

    if let Ok(sha) = resolved {
        return Ok(sha);
    }

    if run_git(repo_path, &["fetch", "--depth", "1", "origin", revision])
        .await
        .is_err()
    {
        run_git(repo_path, &["fetch", "origin"]).await?;
    }

    run_git(
        repo_path,
        &["rev-parse", &format!("{}^{{commit}}", revision)],
    )
    .await
}
