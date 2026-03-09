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

    fn make_auth_url(repo_url: &str, token: &str) -> Result<String> {
        if let Some(rest) = repo_url.strip_prefix("https://") {
            Ok(format!("https://oauth2:{}@{}", token, rest))
        } else {
            Err(anyhow::anyhow!("Only HTTPS repository URLs are supported for token authentication"))
        }
    }

    fn check_repo_is_private(repo_url: &str) -> Result<()> {
        let mut command = Command::new("git");
        command.env("GIT_TERMINAL_PROMPT", "0");
        command.env("GIT_ASKPASS", "echo");
        command.env("SSH_ASKPASS", "echo");
        #[cfg(target_os = "linux")]
        command.env("DISPLAY", "");
        command.args(["ls-remote", "--heads", repo_url]);

        let output = command.output().context("Failed to execute git command")?;
        if output.status.success() {
            return Err(anyhow::anyhow!(
                "This repository is public. For security, only private repositories are allowed. \
                Please use a private repository to store your settings."
            ));
        }
        // If git failed with an auth error, the repo is private (requires credentials)
        let stderr = String::from_utf8_lossy(&output.stderr).to_lowercase();
        let is_auth_error = stderr.contains("could not read username")
            || stderr.contains("authentication failed")
            || stderr.contains("terminal prompts disabled")
            || stderr.contains("403")
            || stderr.contains("401")
            || stderr.contains("access denied")
            || stderr.contains("not found");
        if is_auth_error {
            Ok(())
        } else {
            Err(anyhow::anyhow!("{}", String::from_utf8_lossy(&output.stderr).trim()))
        }
    }

    fn run_git_command(&self, args: &[&str], current_dir: Option<&PathBuf>) -> Result<()> {
        let mut command = Command::new("git");
        
        // Strictly disable any interactive prompts
        command.env("GIT_TERMINAL_PROMPT", "0");
        command.env("GIT_ASKPASS", "echo");
        command.env("SSH_ASKPASS", "echo");
        #[cfg(target_os = "linux")]
        command.env("DISPLAY", "");

        command.args(args);
        if let Some(dir) = current_dir {
            command.current_dir(dir);
        }
        
        let output = command.output().context("Failed to execute git command")?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Self::classify_git_error(&stderr, None));
        }
        Ok(())
    }

    fn classify_git_error(stderr: &str, _token: Option<&str>) -> anyhow::Error {
        let lower = stderr.to_lowercase();

        if lower.contains("write access to repository not granted") {
            return anyhow::anyhow!(
                "Write access denied. Your token does not have write permission to this repository. \
                Ensure the token has the 'repo' scope (not just 'read:repo')."
            );
        }
        if lower.contains("remote: repository not found") || lower.contains("not found") && lower.contains("404") {
            return anyhow::anyhow!(
                "Repository not found. Check that the URL is correct and your token has access to it."
            );
        }
        if lower.contains("could not read username")
            || lower.contains("terminal prompts disabled")
            || lower.contains("invalid credentials")
            || lower.contains("authentication failed")
            || lower.contains("401")
        {
            return anyhow::anyhow!(
                "Authentication failed. Ensure your token is valid and has the 'repo' scope."
            );
        }
        if lower.contains("403") || lower.contains("access denied") || lower.contains("forbidden") {
            return anyhow::anyhow!(
                "Access denied (403). Ensure your token has the 'repo' scope and write access to this repository."
            );
        }

        let message = stderr.trim();
        if message.is_empty() {
            anyhow::anyhow!("Git operation failed with no output.")
        } else {
            anyhow::anyhow!("{}", message)
        }
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
        let url = if let Some(token) = token {
            Self::make_auth_url(repo_url, token)?
        } else {
            repo_url.to_string()
        };
        self.run_git_command(
            &["clone", &url, self.mirror_dir.to_str().unwrap()],
            None,
        )?;
        Repository::open(&self.mirror_dir).map_err(|e| anyhow::anyhow!("Failed to open cloned repository: {}", e))
    }

    pub fn push(&self, repo_url: &str, branch: &str, token: Option<&str>) -> Result<()> {
        Self::check_repo_is_private(repo_url)?;
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

        // Use git binary for push to leverage auth url with token
        log::info!("Pushing to {} on branch {}", repo_url, branch);
        let push_url = if let Some(token) = token {
            Self::make_auth_url(repo_url, token)?
        } else {
            repo_url.to_string()
        };
        self.run_git_command(&["push", &push_url, branch], Some(&self.mirror_dir))?;

        Ok(())
    }

    pub fn pull(&self, repo_url: &str, branch: &str, token: Option<&str>) -> Result<()> {
        Self::check_repo_is_private(repo_url)?;
        let _repo = self.init_repo(repo_url, token)?;

        // Use git binary for pull to leverage auth url with token
        log::info!("Pulling from {} on branch {}", repo_url, branch);
        let pull_url = if let Some(token) = token {
            Self::make_auth_url(repo_url, token)?
        } else {
            repo_url.to_string()
        };
        self.run_git_command(&["pull", &pull_url, branch, "--rebase"], Some(&self.mirror_dir))?;

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
