use crate::repository::{GitFileStatus, RepoPath};
use anyhow::{anyhow, Result};
use std::{path::Path, process::Stdio, sync::Arc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GitStatusPair {
    // Not both `None`.
    pub index_status: Option<GitFileStatus>,
    pub worktree_status: Option<GitFileStatus>,
}

impl GitStatusPair {
    pub fn is_staged(&self) -> Option<bool> {
        match (self.index_status, self.worktree_status) {
            (Some(_), None) => Some(true),
            (None, Some(_)) => Some(false),
            (Some(GitFileStatus::Untracked), Some(GitFileStatus::Untracked)) => Some(false),
            (Some(_), Some(_)) => None,
            (None, None) => unreachable!(),
        }
    }

    // TODO reconsider uses of this
    pub fn combined(&self) -> GitFileStatus {
        self.index_status.or(self.worktree_status).unwrap()
    }
}

#[derive(Clone)]
pub struct GitStatus {
    pub entries: Arc<[(RepoPath, GitStatusPair)]>,
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
                "--no-renames",
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
        let mut entries = stdout
            .split('\0')
            .filter_map(|entry| {
                let sep = entry.get(2..3)?;
                if sep != " " {
                    return None;
                };
                let path = &entry[3..];
                let status = entry[0..2].as_bytes();
                let index_status = GitFileStatus::from_byte(status[0]);
                let worktree_status = GitFileStatus::from_byte(status[1]);
                if (index_status, worktree_status) == (None, None) {
                    return None;
                }
                let path = RepoPath(Path::new(path).into());
                Some((
                    path,
                    GitStatusPair {
                        index_status,
                        worktree_status,
                    },
                ))
            })
            .collect::<Vec<_>>();
        entries.sort_unstable_by(|(a, _), (b, _)| a.cmp(&b));
        Ok(Self {
            entries: entries.into(),
        })
    }
}

impl Default for GitStatus {
    fn default() -> Self {
        Self {
            entries: Arc::new([]),
        }
    }
}
