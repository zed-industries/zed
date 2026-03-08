use std::path::PathBuf;
use git2::{Repository, Signature};
use anyhow::{Result, Context};
use std::fs;
use chrono::Local;
use std::process::Command;

pub struct SyncEngine {
    mirror_dir: PathBuf,
}

impl SyncEngine {
    pub fn new() -> Self {
        Self {
            mirror_dir: paths::data_dir().join("settings_sync_mirror"),
        }
    }

    fn run_git_command(&self, args: &[&str], current_dir: Option<&PathBuf>, token: Option<&str>) -> Result<()> {
        let mut command = Command::new("git");
        
        if let Some(token) = token {
            // Use a credential helper that simply returns the token
            command.args(["-c", "credential.helper=!f() { echo \"username=PAT\"; echo \"password=$SYNC_TOKEN\"; }; f"]);
            command.env("SYNC_TOKEN", token);
        }
        
        command.args(args);
        if let Some(dir) = current_dir {
            command.current_dir(dir);
        }
        
        let output = command.output().context("Failed to execute git command")?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Git command failed: {} - {}", args.join(" "), stderr));
        }
        Ok(())
    }

    fn init_repo(&self, repo_url: &str, token: Option<&str>) -> Result<Repository> {
        if self.mirror_dir.exists() {
            if self.mirror_dir.join(".git").exists() {
                Repository::open(&self.mirror_dir).map_err(|e| anyhow::anyhow!("Failed to open mirror repository: {}", e))
            } else {
                // If directory exists but is not a git repo, remove and re-clone
                fs::remove_dir_all(&self.mirror_dir).context("Failed to remove invalid mirror directory")?;
                self.clone_repo(repo_url, token)
            }
        } else {
            self.clone_repo(repo_url, token)
        }
    }

    fn clone_repo(&self, repo_url: &str, token: Option<&str>) -> Result<Repository> {
        log::info!("Cloning settings repository {} to {:?}", repo_url, self.mirror_dir);
        self.run_git_command(&["clone", repo_url, self.mirror_dir.to_str().unwrap()], None, token)?;
        Repository::open(&self.mirror_dir).map_err(|e| anyhow::anyhow!("Failed to open cloned repository: {}", e))
    }

    pub fn push(&self, repo_url: &str, branch: &str, token: Option<&str>) -> Result<()> {
        let repo = self.init_repo(repo_url, token)?;

        let config_dir = paths::config_dir();
        for file_name in &["settings.json", "keymap.json", "tasks.json"] {
            let src = config_dir.join(file_name);
            let dest = self.mirror_dir.join(file_name);
            if src.exists() {
                fs::copy(&src, &dest).context(format!("Failed to copy {}", file_name))?;
            }
        }

        let mut index = repo.index()?;
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;

        let oid = index.write_tree()?;
        let tree = repo.find_tree(oid)?;

        let sig = Signature::now("Zed", "sync@zed.dev")?;
        let parent_commit = if repo.is_empty()? {
            None
        } else {
            let head = repo.head().ok();
            head.and_then(|h| h.peel_to_commit().ok())
        };

        let mut parents = Vec::new();
        if let Some(parent) = &parent_commit {
            parents.push(parent);
        }

        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let msg = format!("Sync from Zed [{}]", timestamp);

        repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            &msg,
            &tree,
            &parents,
        )?;

        // Use git binary for push to leverage system auth
        log::info!("Pushing to {} on branch {}", repo_url, branch);
        self.run_git_command(&["push", "origin", branch], Some(&self.mirror_dir), token)?;

        Ok(())
    }

    pub fn pull(&self, repo_url: &str, branch: &str, token: Option<&str>) -> Result<()> {
        let _repo = self.init_repo(repo_url, token)?;

        // Use git binary for pull to leverage system auth
        log::info!("Pulling from {} on branch {}", repo_url, branch);
        self.run_git_command(&["pull", "origin", branch, "--rebase"], Some(&self.mirror_dir), token)?;

        let config_dir = paths::config_dir();
        for file_name in &["settings.json", "keymap.json", "tasks.json"] {
            let src = self.mirror_dir.join(file_name);
            let dest = config_dir.join(file_name);
            if src.exists() {
                fs::copy(&src, &dest).context(format!("Failed to copy {}", file_name))?;
            }
        }

        Ok(())
    }
}
