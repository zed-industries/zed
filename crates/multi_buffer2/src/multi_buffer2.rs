use gpui::{Model, ModelContext};
use language::{Bias, Buffer, BufferSnapshot, OffsetRangeExt as _, ReplicaId};
use std::{
    cmp::{self, Ordering},
    fmt::{self, Debug, Formatter},
    ops::Range,
    path::Path,
    sync::Arc,
};
use sum_tree::{SeekTarget, SumTree, TreeMap};
use text::TextSummary;

const NEWLINES: &[u8] = &[b'\n'; u8::MAX as usize];

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    pub fn insert_excerpts<T: language::ToOffset>(
        &mut self,
        new_excerpts: impl IntoIterator<Item = (Model<Buffer>, Range<T>)>,
        cx: &mut ModelContext<Self>,
    ) {
        self.sync(cx);

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
                (buffer_handle, key)
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

        let mut cursor = self.snapshot.excerpts.cursor::<Option<ExcerptKey>>(&());
        let mut new_tree = SumTree::<Excerpt>::default();

        for (buffer, key) in new_excerpts {
            let start_offset = ExcerptOffset {
                path: key.path.clone(),
                buffer_id: key.buffer_id,
                offset: key.range.start,
            };
            let end_offset = ExcerptOffset {
                path: key.path.clone(),
                buffer_id: key.buffer_id,
                offset: key.range.end,
            };

            if start_offset.cmp(cursor.start(), &()).is_ge() {
                new_tree.append(cursor.slice(&start_offset, Bias::Left, &()), &());
                if let Some(excerpt) = cursor.item() {
                    if excerpt.key.intersects(&key) {
                        push_excerpt(&mut new_tree, excerpt.clone());
                        cursor.next(&());
                    }
                }
            }

            push_excerpt(
                &mut new_tree,
                Excerpt {
                    key: key.clone(),
                    snapshot: buffer.read(cx).snapshot(),
                    text_summary: buffer.read(cx).text_summary_for_range(key.range.clone()),
                },
            );

            if end_offset.cmp(cursor.start(), &()).is_ge() {
                cursor.seek(&end_offset, Bias::Left, &());
                if let Some(excerpt) = cursor.item() {
                    if excerpt.key.intersects(&key) {
                        push_excerpt(&mut new_tree, excerpt.clone());
                        cursor.next(&());
                    }
                }
            }
        }

        new_tree.append(cursor.suffix(&()), &());
        drop(cursor);
        self.snapshot.excerpts = new_tree;
    }

    fn sync(&mut self, cx: &mut ModelContext<Self>) {}

    fn snapshot(&mut self, cx: &mut ModelContext<Self>) -> MultiBufferSnapshot {
        self.sync(cx);
        self.snapshot.clone()
    }
}

fn push_excerpt(excerpts: &mut SumTree<Excerpt>, excerpt: Excerpt) {
    let mut excerpt = Some(excerpt);
    excerpts.update_last(
        |last_excerpt| {
            if last_excerpt.key.intersects(&excerpt.as_ref().unwrap().key) {
                let excerpt = excerpt.take().unwrap();
                last_excerpt.key.range.start =
                    cmp::min(last_excerpt.key.range.start, excerpt.key.range.start);
                last_excerpt.key.range.end =
                    cmp::max(last_excerpt.key.range.end, excerpt.key.range.end);
                last_excerpt.text_summary = last_excerpt
                    .snapshot
                    .text_summary_for_range(last_excerpt.key.range.clone());
            }
        },
        &(),
    );

    if let Some(excerpt) = excerpt {
        excerpts.push(excerpt, &());
    }
}

#[derive(Clone, Default)]
pub struct MultiBufferSnapshot {
    excerpts: SumTree<Excerpt>,
    snapshots: TreeMap<BufferId, BufferSnapshot>,
}

impl MultiBufferSnapshot {
    #[cfg(any(test, feature = "test-support"))]
    fn text(&self) -> String {
        let mut text = String::new();
        for excerpt in self.excerpts.iter() {
            text.push('\n');
            text.extend(excerpt.snapshot.text_for_range(excerpt.key.range.clone()));
        }
        text
    }

    pub fn len(&self) -> usize {
        self.excerpts.summary().text.len
    }

    pub fn chunks<T: ToOffset>(&self, range: Range<T>, language_aware: bool) {
        todo!()
    }
}

#[derive(Clone)]
struct Excerpt {
    key: ExcerptKey,
    snapshot: BufferSnapshot,
    text_summary: TextSummary,
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
            text: self.text_summary.clone(),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct ExcerptSummary {
    max_key: Option<ExcerptKey>,
    text: TextSummary,
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
            .then_with(|| Ord::cmp(&self.range.start, &other.range.start))
            .then_with(|| Ord::cmp(&other.range.end, &self.range.end))
    }
}

impl PartialOrd for ExcerptKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ExcerptOffset {
    path: Option<Arc<Path>>,
    buffer_id: BufferId,
    offset: usize,
}

impl<'a> sum_tree::SeekTarget<'a, ExcerptSummary, Option<ExcerptKey>> for ExcerptOffset {
    fn cmp(&self, cursor_location: &Option<ExcerptKey>, _: &()) -> Ordering {
        if let Some(cursor_location) = cursor_location {
            self.path
                .cmp(&cursor_location.path)
                .then_with(|| self.buffer_id.cmp(&cursor_location.buffer_id))
                .then_with(|| Ord::cmp(&self.offset, &cursor_location.range.end))
        } else {
            Ordering::Greater
        }
    }
}

impl sum_tree::Summary for ExcerptSummary {
    type Context = ();

    fn zero(_cx: &Self::Context) -> Self {
        Self::default()
    }

    fn add_summary(&mut self, summary: &Self, _cx: &Self::Context) {
        self.max_key = summary.max_key.clone();
        self.text.add_summary(&summary.text, &());
    }
}

impl<'a> sum_tree::Dimension<'a, ExcerptSummary> for usize {
    fn zero(_cx: &()) -> Self {
        0
    }

    fn add_summary(&mut self, summary: &'a ExcerptSummary, _cx: &()) {
        *self += summary.text.len;
    }
}

impl<'a> sum_tree::Dimension<'a, ExcerptSummary> for Option<ExcerptKey> {
    fn zero(_cx: &()) -> Self {
        None
    }

    fn add_summary(&mut self, summary: &'a ExcerptSummary, _cx: &()) {
        debug_assert!(summary.max_key >= *self);
        *self = summary.max_key.clone();
    }
}

pub trait ToOffset {
    fn to_offset(&self, snapshot: &MultiBufferSnapshot) -> usize;
}

impl ToOffset for usize {
    fn to_offset<'a>(&self, snapshot: &MultiBufferSnapshot) -> usize {
        assert!(*self <= snapshot.len(), "offset is out of range");
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{AppContext, Context};
    use language::Buffer;

    #[gpui::test]
    fn test_insert_excerpts(cx: &mut AppContext) {
        let buffer1 = cx.new_model(|cx| Buffer::local("abcdefghijklmnopqrstuvwxyz", cx));
        cx.new_model(|cx| {
            let mut multibuffer = MultiBuffer::new();
            multibuffer
                .insert_excerpts(vec![(buffer1.clone(), 0..2), (buffer1.clone(), 4..12)], cx);
            assert_eq!(multibuffer.snapshot(cx).text(), "\nab\nefghijkl");

            multibuffer
                .insert_excerpts(vec![(buffer1.clone(), 4..6), (buffer1.clone(), 8..10)], cx);
            assert_eq!(multibuffer.snapshot(cx).text(), "\nab\nefghijkl");

            multibuffer.insert_excerpts(
                vec![(buffer1.clone(), 10..14), (buffer1.clone(), 16..18)],
                cx,
            );
            assert_eq!(multibuffer.snapshot(cx).text(), "\nab\nefghijklmn\nqr");

            multibuffer.insert_excerpts(vec![(buffer1.clone(), 12..17)], cx);
            assert_eq!(multibuffer.snapshot(cx).text(), "\nab\nefghijklmnopqr");

            multibuffer
        });
    }
}
