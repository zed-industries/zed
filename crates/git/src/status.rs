use crate::repository::RepoPath;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{path::Path, process::Stdio, sync::Arc};
use util::ResultExt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileStatus {
    Untracked,
    Ignored,
    Unmerged(UnmergedStatus),
    Tracked(TrackedStatus),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UnmergedStatus {
    pub first_head: UnmergedStatusCode,
    pub second_head: UnmergedStatusCode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnmergedStatusCode {
    Added,
    Deleted,
    Updated,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TrackedStatus {
    pub index_status: StatusCode,
    pub worktree_status: StatusCode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusCode {
    Modified,
    TypeChanged,
    Added,
    Deleted,
    Renamed,
    Copied,
    Unmodified,
}

impl From<UnmergedStatus> for FileStatus {
    fn from(value: UnmergedStatus) -> Self {
        FileStatus::Unmerged(value)
    }
}

impl From<TrackedStatus> for FileStatus {
    fn from(value: TrackedStatus) -> Self {
        FileStatus::Tracked(value)
    }
}

impl FileStatus {
    pub const fn worktree(worktree_status: StatusCode) -> Self {
        FileStatus::Tracked(TrackedStatus {
            index_status: StatusCode::Unmodified,
            worktree_status,
        })
    }

    /// Generate a FileStatus Code from a byte pair, as described in
    /// https://git-scm.com/docs/git-status#_output
    ///
    /// NOTE: That instead of '', we use ' ' to denote no change
    fn from_bytes(bytes: [u8; 2]) -> anyhow::Result<Self> {
        let status = match bytes {
            [b'?', b'?'] => FileStatus::Untracked,
            [b'!', b'!'] => FileStatus::Ignored,
            [b'A', b'A'] => UnmergedStatus {
                first_head: UnmergedStatusCode::Added,
                second_head: UnmergedStatusCode::Added,
            }
            .into(),
            [b'D', b'D'] => UnmergedStatus {
                first_head: UnmergedStatusCode::Added,
                second_head: UnmergedStatusCode::Added,
            }
            .into(),
            [x, b'U'] => UnmergedStatus {
                first_head: UnmergedStatusCode::from_byte(x)?,
                second_head: UnmergedStatusCode::Updated,
            }
            .into(),
            [b'U', y] => UnmergedStatus {
                first_head: UnmergedStatusCode::Updated,
                second_head: UnmergedStatusCode::from_byte(y)?,
            }
            .into(),
            [x, y] => TrackedStatus {
                index_status: StatusCode::from_byte(x)?,
                worktree_status: StatusCode::from_byte(y)?,
            }
            .into(),
        };
        Ok(status)
    }

    pub fn is_staged(self) -> Option<bool> {
        match self {
            FileStatus::Untracked | FileStatus::Ignored | FileStatus::Unmerged { .. } => {
                Some(false)
            }
            FileStatus::Tracked(tracked) => match (tracked.index_status, tracked.worktree_status) {
                (StatusCode::Unmodified, _) => Some(false),
                (_, StatusCode::Unmodified) => Some(true),
                _ => None,
            },
        }
    }

    pub fn is_conflicted(self) -> bool {
        match self {
            FileStatus::Unmerged { .. } => true,
            _ => false,
        }
    }

    pub fn is_ignored(self) -> bool {
        match self {
            FileStatus::Ignored => true,
            _ => false,
        }
    }

    pub fn is_modified(self) -> bool {
        match self {
            FileStatus::Tracked(tracked) => match (tracked.index_status, tracked.worktree_status) {
                (StatusCode::Modified, _) | (_, StatusCode::Modified) => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn is_created(self) -> bool {
        match self {
            FileStatus::Tracked(tracked) => match (tracked.index_status, tracked.worktree_status) {
                (StatusCode::Added, _) | (_, StatusCode::Added) => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn is_deleted(self) -> bool {
        match self {
            FileStatus::Tracked(tracked) => match (tracked.index_status, tracked.worktree_status) {
                (StatusCode::Deleted, _) | (_, StatusCode::Deleted) => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn is_untracked(self) -> bool {
        match self {
            FileStatus::Untracked => true,
            _ => false,
        }
    }

    pub fn summary(self) -> GitSummary {
        match self {
            FileStatus::Ignored => GitSummary::UNCHANGED,
            FileStatus::Untracked => GitSummary::UNTRACKED,
            FileStatus::Unmerged(_) => GitSummary::CONFLICT,
            FileStatus::Tracked(TrackedStatus {
                index_status,
                worktree_status,
            }) => GitSummary {
                index: index_status.to_summary(),
                worktree: worktree_status.to_summary(),
                conflict: 0,
                untracked: 0,
                count: 1,
            },
        }
    }
}

impl StatusCode {
    fn from_byte(byte: u8) -> anyhow::Result<Self> {
        match byte {
            b'M' => Ok(StatusCode::Modified),
            b'T' => Ok(StatusCode::TypeChanged),
            b'A' => Ok(StatusCode::Added),
            b'D' => Ok(StatusCode::Deleted),
            b'R' => Ok(StatusCode::Renamed),
            b'C' => Ok(StatusCode::Copied),
            b' ' => Ok(StatusCode::Unmodified),
            _ => Err(anyhow!("Invalid status code: {byte}")),
        }
    }

    fn to_summary(self) -> TrackedSummary {
        match self {
            StatusCode::Modified | StatusCode::TypeChanged => TrackedSummary {
                modified: 1,
                ..TrackedSummary::UNCHANGED
            },
            StatusCode::Added => TrackedSummary {
                added: 1,
                ..TrackedSummary::UNCHANGED
            },
            StatusCode::Deleted => TrackedSummary {
                deleted: 1,
                ..TrackedSummary::UNCHANGED
            },
            StatusCode::Renamed | StatusCode::Copied | StatusCode::Unmodified => {
                TrackedSummary::UNCHANGED
            }
        }
    }

    pub fn index(self) -> FileStatus {
        FileStatus::Tracked(TrackedStatus {
            index_status: self,
            worktree_status: StatusCode::Unmodified,
        })
    }

    pub fn worktree(self) -> FileStatus {
        FileStatus::Tracked(TrackedStatus {
            index_status: StatusCode::Unmodified,
            worktree_status: self,
        })
    }
}

impl UnmergedStatusCode {
    fn from_byte(byte: u8) -> anyhow::Result<Self> {
        match byte {
            b'A' => Ok(UnmergedStatusCode::Added),
            b'D' => Ok(UnmergedStatusCode::Deleted),
            b'U' => Ok(UnmergedStatusCode::Updated),
            _ => Err(anyhow!("Invalid unmerged status code: {byte}")),
        }
    }
}

#[derive(Clone, Debug, Default, Copy, PartialEq, Eq)]
pub struct TrackedSummary {
    pub added: usize,
    pub modified: usize,
    pub deleted: usize,
}

impl TrackedSummary {
    pub const UNCHANGED: Self = Self {
        added: 0,
        modified: 0,
        deleted: 0,
    };

    pub const ADDED: Self = Self {
        added: 1,
        modified: 0,
        deleted: 0,
    };

    pub const MODIFIED: Self = Self {
        added: 0,
        modified: 1,
        deleted: 0,
    };

    pub const DELETED: Self = Self {
        added: 0,
        modified: 0,
        deleted: 1,
    };
}

impl std::ops::AddAssign for TrackedSummary {
    fn add_assign(&mut self, rhs: Self) {
        self.added += rhs.added;
        self.modified += rhs.modified;
        self.deleted += rhs.deleted;
    }
}

impl std::ops::Add for TrackedSummary {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        TrackedSummary {
            added: self.added + rhs.added,
            modified: self.modified + rhs.modified,
            deleted: self.deleted + rhs.deleted,
        }
    }
}

impl std::ops::Sub for TrackedSummary {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        TrackedSummary {
            added: self.added - rhs.added,
            modified: self.modified - rhs.modified,
            deleted: self.deleted - rhs.deleted,
        }
    }
}

#[derive(Clone, Debug, Default, Copy, PartialEq, Eq)]
pub struct GitSummary {
    pub index: TrackedSummary,
    pub worktree: TrackedSummary,
    pub conflict: usize,
    pub untracked: usize,
    pub count: usize,
}

impl GitSummary {
    pub const CONFLICT: Self = Self {
        conflict: 1,
        count: 1,
        ..Self::UNCHANGED
    };

    pub const UNTRACKED: Self = Self {
        untracked: 1,
        count: 1,
        ..Self::UNCHANGED
    };

    pub const UNCHANGED: Self = Self {
        index: TrackedSummary::UNCHANGED,
        worktree: TrackedSummary::UNCHANGED,
        conflict: 0,
        untracked: 0,
        count: 0,
    };
}

impl From<FileStatus> for GitSummary {
    fn from(status: FileStatus) -> Self {
        status.summary()
    }
}

impl sum_tree::Summary for GitSummary {
    type Context = ();

    fn zero(_: &Self::Context) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, rhs: &Self, _: &Self::Context) {
        *self += *rhs;
    }
}

impl std::ops::Add<Self> for GitSummary {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl std::ops::AddAssign for GitSummary {
    fn add_assign(&mut self, rhs: Self) {
        self.index += rhs.index;
        self.worktree += rhs.worktree;
        self.conflict += rhs.conflict;
        self.untracked += rhs.untracked;
        self.count += rhs.count;
    }
}

impl std::ops::Sub for GitSummary {
    type Output = GitSummary;

    fn sub(self, rhs: Self) -> Self::Output {
        GitSummary {
            index: self.index - rhs.index,
            worktree: self.worktree - rhs.worktree,
            conflict: self.conflict - rhs.conflict,
            untracked: self.untracked - rhs.untracked,
            count: self.count - rhs.count,
        }
    }
}

#[derive(Clone)]
pub struct GitStatus {
    pub entries: Arc<[(RepoPath, FileStatus)]>,
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
            .map_err(|e| anyhow!("Failed to start git status process: {e}"))?;

        let output = child
            .wait_with_output()
            .map_err(|e| anyhow!("Failed to read git status output: {e}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("git status process failed: {stderr}"));
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
                // The git status output includes untracked directories as well as untracked files.
                // We do our own processing to compute the "summary" status of each directory,
                // so just skip any directories in the output, since they'll otherwise interfere
                // with our handling of nested repositories.
                if path.ends_with('/') {
                    return None;
                }
                let status = entry[0..2].as_bytes().try_into().unwrap();
                let status = FileStatus::from_bytes(status).log_err()?;
                let path = RepoPath(Path::new(path).into());
                Some((path, status))
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
