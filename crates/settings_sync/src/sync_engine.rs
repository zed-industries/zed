use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use git2::{Repository, Signature};
use anyhow::{Result, Context};
use std::fs;
use chrono::Local;
use std::process::Command;

pub struct SyncEngine {
    mirror_dir: PathBuf,
    lock: Arc<Mutex<()>>,
}

impl SyncEngine {
    pub fn new() -> Self {
        Self {
            mirror_dir: paths::data_dir().join("settings_sync_mirror"),
            lock: Arc::new(Mutex::new(())),
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
        command.args(["-c", "credential.helper=", "ls-remote", "--heads", repo_url]);

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
                Repository::open(&self.mirror_dir).map_err(|error| anyhow::anyhow!("Failed to open mirror repository: {}", error))
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
        let mirror_dir_str = self.mirror_dir
            .to_str()
            .context("Mirror directory path contains invalid UTF-8")?;
        self.run_git_command(&["clone", &url, mirror_dir_str], None)?;
        Repository::open(&self.mirror_dir).map_err(|error| anyhow::anyhow!("Failed to open cloned repository: {}", error))
    }

    pub fn push(&self, repo_url: &str, branch: &str, token: Option<&str>) -> Result<()> {
        let _guard = self.lock.try_lock().map_err(|_| {
            anyhow::anyhow!("A sync operation is already in progress. Please wait.")
        })?;

        Self::check_repo_is_private(repo_url)?;
        let repo = self.init_repo(repo_url, token)?;

        let config_dir = paths::config_dir();
        for file_name in &["settings.json", "keymap.json", "tasks.json"] {
            let source_path = config_dir.join(file_name);
            let destination_path = self.mirror_dir.join(file_name);
            if source_path.exists() {
                fs::copy(&source_path, &destination_path).context(format!("Failed to copy {}", file_name))?;
            }
        }

        let mut index = repo.index()?;
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;

        let object_id = index.write_tree()?;
        let tree = repo.find_tree(object_id)?;

        let signature = Signature::now("Zed", "sync@zed.dev")?;
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
        let commit_message = format!("Sync from Zed [{}]", timestamp);

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &commit_message,
            &tree,
            &parents,
        )?;

        log::info!("Pushing to {} on branch {}", repo_url, branch);
        let push_url = if let Some(token) = token {
            Self::make_auth_url(repo_url, token)?
        } else {
            repo_url.to_string()
        };

        let push_result = self.run_git_command(&["push", &push_url, branch], Some(&self.mirror_dir));
        if let Err(ref push_error) = push_result {
            let error_message = push_error.to_string().to_lowercase();
            if error_message.contains("non-fast-forward") || error_message.contains("fetch first") || error_message.contains("rejected") {
                log::info!("Push rejected (non-fast-forward), pulling and retrying");
                self.run_git_command(&["pull", &push_url, branch, "--rebase"], Some(&self.mirror_dir))?;
                self.run_git_command(&["push", &push_url, branch], Some(&self.mirror_dir))?;
            } else {
                push_result?;
            }
        }

        Ok(())
    }

    pub fn pull(&self, repo_url: &str, branch: &str, token: Option<&str>) -> Result<()> {
        let _guard = self.lock.try_lock().map_err(|_| {
            anyhow::anyhow!("A sync operation is already in progress. Please wait.")
        })?;

        Self::check_repo_is_private(repo_url)?;
        let _repo = self.init_repo(repo_url, token)?;

        log::info!("Pulling from {} on branch {}", repo_url, branch);
        let pull_url = if let Some(token) = token {
            Self::make_auth_url(repo_url, token)?
        } else {
            repo_url.to_string()
        };

        // Force overwrite local changes: fetch then hard reset to remote state
        self.run_git_command(&["fetch", &pull_url, branch], Some(&self.mirror_dir))?;
        self.run_git_command(&["reset", "--hard", "FETCH_HEAD"], Some(&self.mirror_dir))?;

        let config_dir = paths::config_dir();
        for file_name in &["settings.json", "keymap.json", "tasks.json"] {
            let source_path = self.mirror_dir.join(file_name);
            let destination_path = config_dir.join(file_name);
            if source_path.exists() {
                fs::copy(&source_path, &destination_path).context(format!("Failed to copy {}", file_name))?;
            }
        }

        Ok(())
    }
}
