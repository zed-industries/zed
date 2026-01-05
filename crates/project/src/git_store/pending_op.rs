use git::repository::RepoPath;
use std::ops::Add;
use sum_tree::{ContextLessSummary, Item, KeyedItem};
use worktree::{PathKey, PathSummary};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GitStatus {
    Staged,
    Unstaged,
    Reverted,
    Unchanged,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobStatus {
    Running,
    Finished,
    Skipped,
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PendingOps {
    pub repo_path: RepoPath,
    pub ops: Vec<PendingOp>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PendingOp {
    pub id: PendingOpId,
    pub git_status: GitStatus,
    pub job_status: JobStatus,
}

#[derive(Clone, Debug)]
pub struct PendingOpsSummary {
    pub staged_count: usize,
    pub staging_count: usize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PendingOpId(pub u16);

impl Item for PendingOps {
    type Summary = PathSummary<PendingOpsSummary>;

    fn summary(&self, _cx: ()) -> Self::Summary {
        PathSummary {
            max_path: self.repo_path.as_ref().clone(),
            item_summary: PendingOpsSummary {
                staged_count: self.staged() as usize,
                staging_count: self.staging() as usize,
            },
        }
    }
}

impl ContextLessSummary for PendingOpsSummary {
    fn zero() -> Self {
        Self {
            staged_count: 0,
            staging_count: 0,
        }
    }

    fn add_summary(&mut self, summary: &Self) {
        self.staged_count += summary.staged_count;
        self.staging_count += summary.staging_count;
    }
}

impl KeyedItem for PendingOps {
    type Key = PathKey;

    fn key(&self) -> Self::Key {
        PathKey(self.repo_path.as_ref().clone())
    }
}

impl Add<u16> for PendingOpId {
    type Output = PendingOpId;

    fn add(self, rhs: u16) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl From<u16> for PendingOpId {
    fn from(id: u16) -> Self {
        Self(id)
    }
}

impl PendingOps {
    pub fn new(path: &RepoPath) -> Self {
        Self {
            repo_path: path.clone(),
            ops: Vec::new(),
        }
    }

    pub fn max_id(&self) -> PendingOpId {
        self.ops.last().map(|op| op.id).unwrap_or_default()
    }

    pub fn op_by_id(&self, id: PendingOpId) -> Option<&PendingOp> {
        self.ops.iter().find(|op| op.id == id)
    }

    pub fn op_by_id_mut(&mut self, id: PendingOpId) -> Option<&mut PendingOp> {
        self.ops.iter_mut().find(|op| op.id == id)
    }

    /// File is staged if the last job is finished and has status Staged.
    pub fn staged(&self) -> bool {
        if let Some(last) = self.ops.last() {
            if last.git_status == GitStatus::Staged && last.job_status == JobStatus::Finished {
                return true;
            }
        }
        false
    }

    /// File is staged if the last job is not finished and has status Staged.
    pub fn staging(&self) -> bool {
        if let Some(last) = self.ops.last() {
            if last.git_status == GitStatus::Staged && last.job_status != JobStatus::Finished {
                return true;
            }
        }
        false
    }
}

impl PendingOp {
    pub fn running(&self) -> bool {
        self.job_status == JobStatus::Running
    }

    pub fn finished(&self) -> bool {
        matches!(self.job_status, JobStatus::Finished | JobStatus::Skipped)
    }

    pub fn error(&self) -> bool {
        self.job_status == JobStatus::Error
    }
}
