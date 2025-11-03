use git::repository::RepoPath;
use std::ops::Add;
use sum_tree::{ContextLessSummary, Dimension, Item, KeyedItem};
use worktree::PathSummary;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PendingOpStatus {
    Staged,
    Unstaged,
    Reverted,
    Unchanged,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PendingOp {
    pub repo_path: RepoPath,
    pub id: PendingOpId,
    pub status: PendingOpStatus,
    pub finished: bool,
}

#[derive(Clone, Debug)]
pub struct PendingOpSummary {
    pub max_id: PendingOpId,
    pub staged_count: usize,
    pub unstaged_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PendingOpId(pub usize);

impl Item for PendingOp {
    type Summary = PathSummary<PendingOpSummary>;

    fn summary(&self, _cx: ()) -> Self::Summary {
        PathSummary {
            max_path: self.repo_path.0.clone(),
            item_summary: PendingOpSummary {
                max_id: self.id,
                staged_count: (self.status == PendingOpStatus::Staged) as usize,
                unstaged_count: (self.status == PendingOpStatus::Unstaged) as usize,
            },
        }
    }
}

impl ContextLessSummary for PendingOpSummary {
    fn zero() -> Self {
        Self {
            max_id: PendingOpId(0),
            staged_count: 0,
            unstaged_count: 0,
        }
    }

    fn add_summary(&mut self, summary: &Self) {
        self.max_id = summary.max_id;
        self.staged_count += summary.staged_count;
        self.unstaged_count += summary.unstaged_count;
    }
}

impl KeyedItem for PendingOp {
    type Key = PendingOpId;

    fn key(&self) -> Self::Key {
        self.id
    }
}

impl Dimension<'_, PathSummary<PendingOpSummary>> for PendingOpId {
    fn zero(_cx: ()) -> Self {
        Self(0)
    }

    fn add_summary(&mut self, summary: &PathSummary<PendingOpSummary>, _cx: ()) {
        *self = summary.item_summary.max_id;
    }
}

impl Add<usize> for PendingOpId {
    type Output = PendingOpId;

    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}
