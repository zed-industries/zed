use gpui::{Context, Model};
use language::{Buffer, BufferSnapshot, ReplicaId};
use std::{
    fmt::{self, Debug, Formatter},
    ops::Range,
    path::PathBuf,
};
use sum_tree::{SumTree, TreeMap};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct BufferId {
    remote_id: text::BufferId,
    replica_id: ReplicaId,
}

pub struct MultiBuffer {
    snapshot: MultiBufferSnapshot,
}

impl MultiBuffer {
    pub fn new() -> Self {
        Self {
            snapshot: MultiBufferSnapshot::default(),
        }
    }

    pub fn insert_excerpts<T>(
        &mut self,
        new_excerpts: impl IntoIterator<Item = (Model<Buffer>, Range<T>)>,
    ) {
        let mut new_excerpts = new_excerpts.into_iter().collect::<Vec<_>>();
    }
}

#[derive(Default)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::AppContext;
    use language::Buffer;

    #[gpui::test]
    fn test_insert_excerpts(cx: &mut AppContext) {
        let buffer1 = cx.new_model(|cx| Buffer::local("abc\ndef\nghi", cx));
        let buffer2 = cx.new_model(|cx| Buffer::local("jkl\nmno\npqr", cx));

        let multi = cx.new_model(|cx| {
            let mut multi = MultiBuffer::new();
            multi.insert_excerpts(vec![(buffer1.clone(), 0..4), (buffer2.clone(), 8..11)]);
            multi
        });
    }
}
