use futures::channel::oneshot::Canceled;
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
    Started,
    Finished,
    Canceled,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PendingOps {
    pub repo_path: RepoPath,
    pub ops: Vec<PendingOp>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PendingOp {
    pub id: PendingOpId,
    pub git_status: GitStatus,
    pub job_status: JobStatus,
}

#[derive(Clone, Debug)]
pub struct PendingOpsSummary {
    pub max_id: PendingOpId,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PendingOpId(pub u16);

impl Item for PendingOps {
    type Summary = PathSummary<PendingOpsSummary>;

    fn summary(&self, _cx: ()) -> Self::Summary {
        PathSummary {
            max_path: self.repo_path.0.clone(),
            item_summary: PendingOpsSummary {
                max_id: self.ops.last().map(|op| op.id).unwrap_or_default(),
            },
        }
    }
}

impl ContextLessSummary for PendingOpsSummary {
    fn zero() -> Self {
        Self {
            max_id: PendingOpId::default(),
        }
    }

    fn add_summary(&mut self, summary: &Self) {
        self.max_id = summary.max_id;
    }
}

impl KeyedItem for PendingOps {
    type Key = PathKey;

    fn key(&self) -> Self::Key {
        PathKey(self.repo_path.0.clone())
    }
}

impl Add<u16> for PendingOpId {
    type Output = PendingOpId;

    fn add(self, rhs: u16) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl PendingOps {
    pub fn new(path: &RepoPath) -> Self {
        Self {
            repo_path: path.clone(),
            ops: Vec::new(),
        }
    }

    pub fn op_by_id(&self, id: PendingOpId) -> Option<&PendingOp> {
        self.ops.iter().find(|op| op.id == id)
    }

    pub fn op_by_id_mut(&mut self, id: PendingOpId) -> Option<&mut PendingOp> {
        self.ops.iter_mut().find(|op| op.id == id)
    }
}

impl From<Canceled> for JobStatus {
    fn from(_err: Canceled) -> Self {
        Self::Canceled
    }
}
