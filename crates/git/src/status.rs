use crate::repository::{GitFileStatus, RepoPath};
use anyhow::{anyhow, Result};
use std::{path::Path, process::Stdio, sync::Arc};

#[derive(Clone, Debug)]
pub struct GitStatusItem {
    pub path: RepoPath,
    // Not both `None`.
    pub index_status: Option<GitFileStatus>,
    pub worktree_status: Option<GitFileStatus>,
}

impl GitStatusItem {}

#[derive(Clone)]
pub struct GitStatus {
    pub items: Arc<[GitStatusItem]>,
}

impl GitStatus {
    pub(crate) fn new(
        git_binary: &Path,
        working_directory: &Path,
        path_prefixes: &[RepoPath],
    ) -> Result<Self> {
        let child = util::command::new_std_command(git_binary)
            .current_dir(working_directory)
            .args([
                "--no-optional-locks",
                "status",
                "--porcelain=v1",
                "--untracked-files=all",
                "-z",
            ])
            .args(path_prefixes.iter().map(|path_prefix| {
                if path_prefix.0.as_ref() == Path::new("") {
                    Path::new(".")
                } else {
                    path_prefix
                }
            }))
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| anyhow!("Failed to start git status process: {}", e))?;

        let output = child
            .wait_with_output()
            .map_err(|e| anyhow!("Failed to read git blame output: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("git status process failed: {}", stderr));
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut items = stdout
            .split('\0')
            .filter_map(|entry| {
                if !entry.is_char_boundary(3) {
                    return None;
                }
                let (status, path) = entry.split_at(3);
                let status = status.trim_end().as_bytes();
                let index_status = GitFileStatus::from_byte(status.get(0).copied()?);
                let worktree_status = GitFileStatus::from_byte(status.get(1).copied()?);
                if (index_status, worktree_status) == (None, None) {
                    return None;
                }
                let path = RepoPath(Path::new(path).into());
                Some(GitStatusItem {
                    path,
                    index_status,
                    worktree_status,
                })
            })
            .collect::<Vec<_>>();
        items.sort_unstable_by(|a, b| a.path.cmp(&b.path));
        Ok(Self {
            items: items.into(),
        })
    }
}

impl Default for GitStatus {
    fn default() -> Self {
        Self {
            items: Arc::new([]),
        }
    }
}
