use gpui::{AppContext, Context, Model, ModelContext};
use language::{Buffer, BufferSnapshot, OffsetRangeExt, ReplicaId, ToOffset};
use std::{
    cmp,
    fmt::{self, Debug, Formatter},
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
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

    pub fn insert_excerpts<T: ToOffset>(
        &mut self,
        new_excerpts: impl IntoIterator<Item = (Model<Buffer>, Range<T>)>,
        cx: &mut ModelContext<Self>,
    ) {
        let mut new_excerpts = new_excerpts
            .into_iter()
            .map(|(buffer_handle, range)| {
                let buffer = buffer_handle.read(cx);
                let range = range.to_offset(buffer);
                let key = ExcerptKey {
                    path: buffer.file().map(|file| file.full_path(cx).into()),
                    buffer_id: BufferId {
                        remote_id: buffer.remote_id(),
                        replica_id: buffer.replica_id(),
                    },
                    range,
                };
                (buffer, key)
            })
            .collect::<Vec<_>>();
        new_excerpts.sort_unstable_by_key(|(_, key)| key.clone());
        new_excerpts.dedup_by(|(_, key_a), (_, key_b)| {
            if key_a.intersects(&key_b) {
                key_b.range.start = cmp::min(key_a.range.start, key_b.range.start);
                key_b.range.end = cmp::max(key_a.range.end, key_b.range.end);
                true
            } else {
                false
            }
        });
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct ExcerptKey {
    path: Option<Arc<Path>>,
    buffer_id: BufferId,
    range: Range<usize>,
}

impl ExcerptKey {
    fn intersects(&self, other: &Self) -> bool {
        self.buffer_id == other.buffer_id
            && self.range.start <= other.range.end
            && other.range.start <= self.range.end
    }
}

impl Ord for ExcerptKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.path
            .cmp(&other.path)
            .then_with(|| self.buffer_id.cmp(&other.buffer_id))
            .then_with(|| self.range.start.cmp(&other.range.start))
            .then_with(|| other.range.end.cmp(&self.range.end))
    }
}

impl PartialOrd for ExcerptKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
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
            multi.insert_excerpts(vec![(buffer1.clone(), 0..4), (buffer2.clone(), 8..11)], cx);
            multi
        });
    }
}
