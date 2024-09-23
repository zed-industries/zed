use language::{Bias, BufferSnapshot, ReplicaId};
use std::{
    fmt::{self, Debug, Formatter},
    ops::Range,
    path::PathBuf,
};
use sum_tree::{SumTree, TreeMap};

#[derive(Debug, Clone, PartialEq, Eq)]
struct BufferId {
    remote_id: text::BufferId,
    replica_id: ReplicaId,
}

pub struct MultiBuffer {
    snapshot: MultiBufferSnapshot,
}

pub struct MultiBufferSnapshot {
    excerpts: SumTree<Excerpt>,
    snapshots: TreeMap<BufferId, BufferSnapshot>,
}

#[derive(Clone)]
struct Excerpt {
    key: ExcerptKey,
    snapshot: BufferSnapshot,
}

impl Debug for Excerpt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Excerpt")
            .field("key", &self.key)
            .finish_non_exhaustive()
    }
}

impl sum_tree::Item for Excerpt {
    type Summary = ExcerptSummary;

    fn summary(&self) -> Self::Summary {
        ExcerptSummary {
            max_key: Some(self.key.clone()),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct ExcerptSummary {
    max_key: Option<ExcerptKey>,
}

#[derive(Clone, Debug)]
struct ExcerptKey {
    path: Option<PathBuf>,
    buffer_id: BufferId,
    range: Range<usize>,
}

impl sum_tree::Summary for ExcerptSummary {
    type Context = ();

    fn zero(_cx: &Self::Context) -> Self {
        Self::default()
    }

    fn add_summary(&mut self, summary: &Self, cx: &Self::Context) {
        self.max_key = summary.max_key.clone();
    }
}
