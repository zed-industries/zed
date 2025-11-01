use git::repository::RepoPath;
use sum_tree::{ContextLessSummary, Dimension, Item, KeyedItem, NoSummary, SumTree};
use worktree::{PathKey, PathSummary};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Staged,
    Unstaged,
    Reverted,
    Unchanged,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PendingOp {
    pub id: u16,
    pub status: Status,
    pub finished: bool,
}

#[derive(Clone, Debug)]
pub struct PendingOpSummary {
    pub max_id: u16,
    pub staged_count: u32,
    pub unstaged_count: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PendingOps {
    pub repo_path: RepoPath,
    pub ops: SumTree<PendingOp>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PendingOpId(u16);

impl Item for PendingOps {
    type Summary = PathSummary<NoSummary>;

    fn summary(&self, _cx: ()) -> Self::Summary {
        PathSummary {
            max_path: self.repo_path.0.clone(),
            item_summary: NoSummary,
        }
    }
}

impl KeyedItem for PendingOps {
    type Key = PathKey;

    fn key(&self) -> Self::Key {
        PathKey(self.repo_path.0.clone())
    }
}

impl Item for PendingOp {
    type Summary = PendingOpSummary;

    fn summary(&self, _cx: ()) -> Self::Summary {
        PendingOpSummary {
            max_id: self.id,
            staged_count: (self.status == Status::Staged) as u32,
            unstaged_count: (self.status == Status::Unstaged) as u32,
        }
    }
}

impl ContextLessSummary for PendingOpSummary {
    fn zero() -> Self {
        Self {
            max_id: 0,
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
        PendingOpId(self.id)
    }
}

impl Dimension<'_, PendingOpSummary> for PendingOpId {
    fn zero(_cx: ()) -> Self {
        Self(0)
    }

    fn add_summary(&mut self, summary: &PendingOpSummary, _cx: ()) {
        self.0 = summary.max_id;
    }
}
