use collections::{BTreeMap, HashMap};
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct BufferId {
    remote_id: text::BufferId,
    replica_id: ReplicaId,
}

pub struct MultiBuffer {
    snapshot: MultiBufferSnapshot,
    buffers: HashMap<BufferId, Model<Buffer>>,
}

impl MultiBuffer {
    pub fn new() -> Self {
        Self {
            snapshot: MultiBufferSnapshot::default(),
            buffers: HashMap::default(),
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
            .filter_map(|(buffer_handle, range)| {
                let buffer = buffer_handle.read(cx);
                let range = range.to_offset(buffer);
                if range.is_empty() {
                    None
                } else {
                    let key = ExcerptKey {
                        path: buffer.file().map(|file| file.full_path(cx).into()),
                        buffer_id: BufferId {
                            remote_id: buffer.remote_id(),
                            replica_id: buffer.replica_id(),
                        },
                        range,
                    };
                    Some((buffer_handle, key))
                }
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
        if let Some((_, key)) = new_excerpts.first() {
            let start_offset = ExcerptOffset {
                path: key.path.clone(),
                buffer_id: key.buffer_id,
                offset: key.range.start,
            };
            new_tree = cursor.slice(&start_offset, Bias::Left, &());
        }

        for (buffer, key) in new_excerpts {
            if self.buffers.insert(key.buffer_id, buffer.clone()).is_none() {
                self.snapshot
                    .buffer_snapshots
                    .insert(key.buffer_id, buffer.read(cx).snapshot());
            }

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

            if start_offset
                .cmp(&cursor.item().map(|item| item.key.clone()), &())
                .is_ge()
            {
                new_tree.append(cursor.slice(&start_offset, Bias::Left, &()), &());
                if let Some(excerpt) = cursor.item() {
                    if excerpt.key.intersects(&key) {
                        push_excerpt(
                            &mut new_tree,
                            &self.snapshot.buffer_snapshots,
                            excerpt.clone(),
                        );
                        cursor.next(&());
                    }
                }
            }

            push_excerpt(
                &mut new_tree,
                &self.snapshot.buffer_snapshots,
                Excerpt {
                    key: key.clone(),
                    text_summary: buffer.read(cx).text_summary_for_range(key.range.clone()),
                },
            );

            if end_offset
                .cmp(&cursor.item().map(|item| item.key.clone()), &())
                .is_ge()
            {
                cursor.seek(&end_offset, Bias::Left, &());
                if let Some(excerpt) = cursor.item() {
                    if excerpt.key.intersects(&key) {
                        push_excerpt(
                            &mut new_tree,
                            &self.snapshot.buffer_snapshots,
                            excerpt.clone(),
                        );
                        cursor.next(&());
                    }
                }
            }
        }

        new_tree.append(cursor.suffix(&()), &());
        drop(cursor);
        self.snapshot.excerpts = new_tree;
        self.check_invariants();
    }

    fn sync(&mut self, cx: &mut ModelContext<Self>) {
        let mut renames = Vec::new();
        let mut edits =
            BTreeMap::<(Option<Arc<Path>>, BufferId), Vec<language::Edit<usize>>>::new();

        for (buffer_id, old_snapshot) in self.snapshot.buffer_snapshots.clone().iter() {
            let new_snapshot = self.buffers[buffer_id].read(cx).snapshot();

            let mut changed = new_snapshot.non_text_state_update_count()
                != old_snapshot.non_text_state_update_count();

            let old_path = old_snapshot
                .file()
                .map(|file| Arc::from(file.full_path(cx)));
            let new_path = new_snapshot
                .file()
                .map(|file| Arc::from(file.full_path(cx)));
            if new_path != old_path {
                renames.push((*buffer_id, old_path, new_path.clone()));
                changed = true;
            }

            for edit in new_snapshot.edits_since::<usize>(&old_snapshot.version) {
                changed = true;
                edits
                    .entry((new_path.clone(), *buffer_id))
                    .or_default()
                    .push(edit);
            }

            if changed {
                self.snapshot
                    .buffer_snapshots
                    .insert(*buffer_id, new_snapshot);
            }
        }

        self.apply_renames(renames);
        self.apply_edits(edits);
        self.check_invariants();
    }

    fn apply_renames(&mut self, renames: Vec<(BufferId, Option<Arc<Path>>, Option<Arc<Path>>)>) {
        // Remove all the excerpts that have been renamed.
        let mut renamed_excerpts = Vec::new();
        {
            let mut cursor = self.snapshot.excerpts.cursor::<Option<ExcerptKey>>(&());
            let mut new_tree = SumTree::default();
            for (buffer_id, old_path, new_path) in renames {
                let buffer_start = ExcerptOffset {
                    path: old_path.clone(),
                    buffer_id,
                    offset: 0,
                };
                new_tree.append(cursor.slice(&buffer_start, Bias::Left, &()), &());
                while let Some(excerpt) = cursor.item() {
                    if excerpt.key.buffer_id == buffer_id {
                        renamed_excerpts.push(Excerpt {
                            key: ExcerptKey {
                                path: new_path.clone(),
                                buffer_id,
                                range: excerpt.key.range.clone(),
                            },
                            text_summary: excerpt.text_summary.clone(),
                        });
                        cursor.next(&());
                    } else {
                        break;
                    }
                }
            }
            new_tree.append(cursor.suffix(&()), &());
            drop(cursor);
            self.snapshot.excerpts = new_tree;
        }

        // Re-insert excerpts for the renamed buffers at the right location.
        let mut cursor = self.snapshot.excerpts.cursor::<Option<ExcerptKey>>(&());
        let mut new_tree = SumTree::default();
        for excerpt in renamed_excerpts {
            let buffer_start = ExcerptOffset {
                path: excerpt.key.path.clone(),
                buffer_id: excerpt.key.buffer_id,
                offset: excerpt.key.range.start,
            };
            new_tree.append(cursor.slice(&buffer_start, Bias::Right, &()), &());
            new_tree.push(excerpt, &());
        }
        new_tree.append(cursor.suffix(&()), &());
        drop(cursor);
        self.snapshot.excerpts = new_tree;
    }

    fn apply_edits(
        &mut self,
        edits: BTreeMap<(Option<Arc<Path>>, BufferId), Vec<language::Edit<usize>>>,
    ) {
        let mut cursor = self.snapshot.excerpts.cursor::<Option<ExcerptKey>>(&());
        let mut new_tree = SumTree::default();

        for ((path, buffer_id), buffer_edits) in edits {
            let mut buffer_edits = buffer_edits.into_iter().peekable();
            if let Some(buffer_edit) = buffer_edits.peek() {
                new_tree.append(
                    cursor.slice(
                        &ExcerptOffset::new(path.clone(), buffer_id, buffer_edit.old.start),
                        Bias::Left,
                        &(),
                    ),
                    &(),
                );
            }

            let mut buffer_old_start = cursor.item().unwrap().key.range.start;
            let mut buffer_new_start = buffer_old_start;
            while let Some(buffer_edit) = buffer_edits.next() {
                let buffer_old_end = cursor.item().unwrap().key.range.end;
                if buffer_edit.old.start > buffer_old_start {
                    push_excerpt(
                        &mut new_tree,
                        &self.snapshot.buffer_snapshots,
                        Excerpt {
                            key: ExcerptKey {
                                path: path.clone(),
                                buffer_id,
                                range: buffer_new_start..buffer_edit.new.start,
                            },
                            text_summary: TextSummary::default(), // todo!(change this)
                        },
                    );
                    buffer_old_start = buffer_edit.old.start;
                    buffer_new_start = buffer_edit.new.start;
                }


                push_excerpt(
                    &mut new_tree,
                    &self.snapshot.buffer_snapshots,
                    Excerpt {
                        key: ExcerptKey {
                            path: path.clone(),
                            buffer_id,
                            range: buffer_edit.new.clone(),
                        },
                        text_summary: TextSummary::default(), // todo!(change this)
                    },
                );
                [   (  ]   )
                let delta = buffer_old_end
                cursor.seek_forward(
                    &ExcerptOffset::new(path.clone(), buffer_id, buffer_edit.old.end),
                    Bias::Left,
                    &(),
                );

                // push_excerpt(
                //     &mut new_tree,
                //     &self.snapshot.buffer_snapshots,
                //     Excerpt {
                //         key: ExcerptKey {
                //             path: path.clone(),
                //             buffer_id,
                //             range: buffer_new_start
                //                 ..buffer_new_start + (buffer_old_end - buffer_old_start),
                //         },
                //         text_summary: TextSummary::default(), // todo!(change this)
                //     },
                // );
                // cursor.next(&());
                // new_tree.append(
                //     cursor.slice(
                //         &ExcerptOffset::new(path.clone(), buffer_id, buffer_edit.old.start),
                //         Bias::Left,
                //         &(),
                //     ),
                //     &(),
                // );
            }
        }
    }

    pub fn snapshot(&mut self, cx: &mut ModelContext<Self>) -> MultiBufferSnapshot {
        self.sync(cx);
        self.snapshot.clone()
    }

    fn check_invariants(&self) {
        #[cfg(debug_assertions)]
        {
            let mut cursor = self.snapshot.excerpts.cursor::<()>(&());
            cursor.next(&());
            while let Some(excerpt) = cursor.item() {
                if let Some(prev_excerpt) = cursor.prev_item() {
                    assert!(
                        !excerpt.key.intersects(&prev_excerpt.key),
                        "excerpts are not disjoint {:?}, {:?}",
                        prev_excerpt.key.range,
                        excerpt.key.range,
                    );
                }
                cursor.next(&());
            }
        }
    }
}

fn push_excerpt(
    excerpts: &mut SumTree<Excerpt>,
    buffer_snapshots: &TreeMap<BufferId, BufferSnapshot>,
    excerpt: Excerpt,
) {
    if excerpt.key.range.is_empty() {
        return;
    }

    let mut merged = false;
    excerpts.update_last(
        |last_excerpt| {
            if last_excerpt.key.intersects(&excerpt.key) {
                let snapshot = buffer_snapshots.get(&excerpt.key.buffer_id).unwrap();
                last_excerpt.key.range.start =
                    cmp::min(last_excerpt.key.range.start, excerpt.key.range.start);
                last_excerpt.key.range.end =
                    cmp::max(last_excerpt.key.range.end, excerpt.key.range.end);
                last_excerpt.text_summary =
                    snapshot.text_summary_for_range(last_excerpt.key.range.clone());
                merged = true;
            }
        },
        &(),
    );

    if !merged {
        excerpts.push(excerpt, &());
    }
}

#[derive(Clone, Default)]
pub struct MultiBufferSnapshot {
    excerpts: SumTree<Excerpt>,
    buffer_snapshots: TreeMap<BufferId, BufferSnapshot>,
}

impl MultiBufferSnapshot {
    #[cfg(any(test, feature = "test-support"))]
    fn text(&self) -> String {
        let mut text = String::new();
        for excerpt in self.excerpts.iter() {
            let snapshot = self.buffer_snapshots.get(&excerpt.key.buffer_id).unwrap();
            text.push('\n');
            text.extend(snapshot.text_for_range(excerpt.key.range.clone()));
        }
        text
    }

    pub fn len(&self) -> usize {
        self.excerpts.summary().text.len
    }
}

#[derive(Clone)]
struct Excerpt {
    key: ExcerptKey,
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

impl ExcerptOffset {
    fn new(path: Option<Arc<Path>>, buffer_id: BufferId, offset: usize) -> Self {
        Self {
            path,
            buffer_id,
            offset,
        }
    }
}

impl<'a> sum_tree::SeekTarget<'a, ExcerptSummary, Option<ExcerptKey>> for ExcerptOffset {
    fn cmp(&self, cursor_location: &Option<ExcerptKey>, _: &()) -> Ordering {
        if let Some(cursor_location) = cursor_location {
            self.path
                .cmp(&cursor_location.path)
                .then_with(|| self.buffer_id.cmp(&cursor_location.buffer_id))
                .then_with(|| {
                    if Ord::cmp(&self.offset, &cursor_location.range.start).is_lt() {
                        Ordering::Less
                    } else if Ord::cmp(&self.offset, &cursor_location.range.end).is_gt() {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                })
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
    use std::path::PathBuf;

    use super::*;
    use gpui::{AppContext, Context};
    use language::Buffer;
    use rand::prelude::*;

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

    #[gpui::test(iterations = 1000)]
    fn test_insert_random_excerpts(mut rng: StdRng, cx: &mut AppContext) {
        let buffer = cx.new_model(|cx| {
            let random_words: Vec<&str> = WORDS.choose_multiple(&mut rng, 10).cloned().collect();
            let content = random_words.join(" ");
            Buffer::local(&content, cx)
        });

        let buffer_len = buffer.read(cx).len();

        let generate_excerpts = |rng: &mut StdRng| {
            let mut ranges = Vec::new();
            for _ in 0..5 {
                let start = rng.gen_range(0..=buffer_len);
                let end = rng.gen_range(start..=buffer_len);
                ranges.push(start..end);
            }
            ranges
        };

        cx.new_model(|cx| {
            let mut multibuffer = MultiBuffer::new();
            let excerpts1 = generate_excerpts(&mut rng);
            let excerpts2 = generate_excerpts(&mut rng);

            multibuffer.insert_excerpts(
                excerpts1
                    .iter()
                    .map(|range| (buffer.clone(), range.clone())),
                cx,
            );
            multibuffer.insert_excerpts(
                excerpts2
                    .iter()
                    .map(|range| (buffer.clone(), range.clone())),
                cx,
            );

            let mut excerpt_ranges = excerpts1
                .iter()
                .chain(&excerpts2)
                .cloned()
                .collect::<Vec<_>>();
            excerpt_ranges.sort_by_key(|range| (range.start, range.end));
            excerpt_ranges.dedup_by(|a, b| {
                if a.start <= b.end && b.start <= a.end {
                    b.start = a.start.min(b.start);
                    b.end = a.end.max(b.end);
                    true
                } else {
                    false
                }
            });

            let expected_text = excerpt_ranges
                .into_iter()
                .filter_map(|range| {
                    if range.is_empty() {
                        None
                    } else {
                        Some(format!("\n{}", &buffer.read(cx).text()[range]))
                    }
                })
                .collect::<String>();
            assert_eq!(multibuffer.snapshot(cx).text(), expected_text);

            multibuffer
        });
    }

    #[gpui::test]
    fn test_rename_buffers(cx: &mut AppContext) {
        let buffer1 = cx.new_model(|cx| {
            let mut buffer = Buffer::local("The quick brown fox", cx);
            buffer.file_updated(
                Arc::new(TestFile {
                    path: Path::new("a.txt").into(),
                }),
                cx,
            );
            buffer
        });
        let buffer2 = cx.new_model(|cx| {
            let mut buffer = Buffer::local("jumps over the lazy dog", cx);
            buffer.file_updated(
                Arc::new(TestFile {
                    path: Path::new("b.txt").into(),
                }),
                cx,
            );
            buffer
        });

        cx.new_model(|cx| {
            let mut multibuffer = MultiBuffer::new();
            multibuffer.insert_excerpts(
                vec![
                    (buffer1.clone(), 0..9),
                    (buffer2.clone(), 0..5),
                    (buffer1.clone(), 10..19),
                    (buffer2.clone(), 6..23),
                ],
                cx,
            );
            assert_eq!(
                multibuffer.snapshot(cx).text(),
                "\nThe quick\nbrown fox\njumps\nover the lazy dog"
            );

            // Rename /b.txt to /0.txt
            buffer2.update(cx, |buffer, cx| {
                buffer.file_updated(
                    Arc::new(TestFile {
                        path: Path::new("/0.txt").into(),
                    }),
                    cx,
                );
            });
            assert_eq!(
                multibuffer.snapshot(cx).text(),
                "\njumps\nover the lazy dog\nThe quick\nbrown fox"
            );

            multibuffer
        });
    }

    struct TestFile {
        path: Arc<Path>,
    }

    impl language::File for TestFile {
        fn as_local(&self) -> Option<&dyn language::LocalFile> {
            None
        }

        fn mtime(&self) -> Option<std::time::SystemTime> {
            None
        }

        fn path(&self) -> &Arc<Path> {
            &self.path
        }

        fn full_path(&self, _: &AppContext) -> PathBuf {
            Path::new("root").join(&self.path)
        }

        fn file_name<'a>(&'a self, _: &'a AppContext) -> &'a std::ffi::OsStr {
            unimplemented!()
        }

        fn is_deleted(&self) -> bool {
            false
        }

        fn as_any(&self) -> &dyn std::any::Any {
            unimplemented!()
        }

        fn to_proto(&self, _: &AppContext) -> rpc::proto::File {
            unimplemented!()
        }

        fn worktree_id(&self, _: &AppContext) -> settings::WorktreeId {
            settings::WorktreeId::from_usize(0)
        }

        fn is_private(&self) -> bool {
            false
        }
    }

    const WORDS: &[&str] = &[
        "apple",
        "banana",
        "cherry",
        "date",
        "elderberry",
        "fig",
        "grape",
        "honeydew",
        "kiwi",
        "lemon",
        "mango",
        "nectarine",
        "orange",
        "papaya",
        "quince",
        "raspberry",
        "strawberry",
        "tangerine",
        "ugli",
        "vanilla",
        "watermelon",
        "xigua",
        "yuzu",
        "zucchini",
        "apricot",
        "blackberry",
        "coconut",
        "dragonfruit",
        "eggplant",
        "feijoa",
        "guava",
        "hazelnut",
        "jackfruit",
        "kumquat",
        "lime",
        "mulberry",
        "nance",
        "olive",
        "peach",
        "rambutan",
    ];
}
