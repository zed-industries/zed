use collections::{BTreeMap, HashMap};
use gpui::{AppContext, Model, ModelContext};
use language::{Bias, Buffer, BufferSnapshot, OffsetRangeExt as _, ReplicaId};
use std::{
    cmp::{self, Reverse},
    fmt::Debug,
    ops::Range,
    path::Path,
    sync::Arc,
};
use sum_tree::{SumTree, TreeMap};
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

        struct NewExcerpt {
            path: Option<Arc<Path>>,
            buffer_id: BufferId,
            snapshot: BufferSnapshot,
            range: Range<usize>,
        }

        let mut new_excerpts = new_excerpts
            .into_iter()
            .filter_map(|(buffer_handle, range)| {
                let buffer = buffer_handle.read(cx);
                let range = range.to_offset(buffer);
                if range.is_empty() {
                    None
                } else {
                    let path: Option<Arc<Path>> =
                        buffer.file().map(|file| file.full_path(cx).into());
                    let buffer_id = BufferId {
                        remote_id: buffer.remote_id(),
                        replica_id: buffer.replica_id(),
                    };

                    if self.buffers.insert(buffer_id, buffer_handle).is_none() {
                        self.snapshot
                            .buffer_snapshots
                            .insert(buffer_id, buffer.snapshot());
                    }

                    Some(NewExcerpt {
                        path,
                        buffer_id,
                        snapshot: buffer.snapshot(),
                        range,
                    })
                }
            })
            .collect::<Vec<_>>();
        new_excerpts.sort_unstable_by_key(|excerpt| {
            (
                excerpt.path.clone(),
                excerpt.buffer_id,
                excerpt.range.start,
                Reverse(excerpt.range.end),
            )
        });
        new_excerpts.dedup_by(|excerpt_a, excerpt_b| {
            if excerpt_a.buffer_id == excerpt_b.buffer_id {
                if excerpt_a.range.end < excerpt_b.range.start
                    || excerpt_a.range.start > excerpt_b.range.end
                {
                    false
                } else {
                    excerpt_b.range.start =
                        cmp::min(excerpt_a.range.start.clone(), excerpt_b.range.start.clone());
                    excerpt_b.range.end =
                        cmp::max(excerpt_a.range.end.clone(), excerpt_b.range.end.clone());
                    true
                }
            } else {
                false
            }
        });

        let mut cursor = self.snapshot.excerpts.cursor::<Option<ExcerptOffset>>(&());
        let mut new_tree = SumTree::<Excerpt>::default();
        let mut new_excerpts = new_excerpts.into_iter().peekable();
        while let Some(new_excerpt) = new_excerpts.next() {
            new_tree.append(
                cursor.slice(
                    &Some(ExcerptOffset {
                        path: new_excerpt.path.clone(),
                        buffer_id: new_excerpt.buffer_id,
                        buffer_offset: new_excerpt.range.start,
                    }),
                    Bias::Right,
                    &(),
                ),
                &(),
            );

            let prev_excerpt_end = if let Some(max_offset) = new_tree.summary().max_offset.as_ref()
            {
                if max_offset.buffer_id == new_excerpt.buffer_id {
                    max_offset.buffer_offset
                } else {
                    0
                }
            } else {
                0
            };
            if new_excerpt.range.start > prev_excerpt_end {
                let prefix_visible = cursor.item().map_or(false, |excerpt| excerpt.visible);
                push_excerpt(
                    &mut new_tree,
                    Excerpt {
                        path: new_excerpt.path.clone(),
                        buffer_id: new_excerpt.buffer_id,
                        text_summary: new_excerpt
                            .snapshot
                            .text_summary_for_range(prev_excerpt_end..new_excerpt.range.start),
                        visible: prefix_visible,
                    },
                );
            }
            push_excerpt(
                &mut new_tree,
                Excerpt {
                    path: new_excerpt.path.clone(),
                    buffer_id: new_excerpt.buffer_id,
                    text_summary: new_excerpt
                        .snapshot
                        .text_summary_for_range(new_excerpt.range.clone()),
                    visible: true,
                },
            );
            cursor.seek_forward(
                &Some(ExcerptOffset {
                    path: new_excerpt.path.clone(),
                    buffer_id: new_excerpt.buffer_id,
                    buffer_offset: new_excerpt.range.end,
                }),
                Bias::Right,
                &(),
            );

            let old_excerpt_buffer_id;
            let old_excerpt_end;
            let old_excerpt_visible;
            if let Some(old_excerpt) = cursor.item() {
                old_excerpt_buffer_id = Some(old_excerpt.buffer_id);
                if old_excerpt.buffer_id == new_excerpt.buffer_id {
                    old_excerpt_end = cursor.end(&()).unwrap().buffer_offset;
                    old_excerpt_visible = old_excerpt.visible;
                } else {
                    old_excerpt_end = new_excerpt.snapshot.len();
                    old_excerpt_visible = false;
                }
            } else {
                old_excerpt_buffer_id = None;
                old_excerpt_end = new_excerpt.snapshot.len();
                old_excerpt_visible = false;
            };

            if new_excerpts.peek().map_or(true, |next_excerpt| {
                next_excerpt.buffer_id != new_excerpt.buffer_id
                    || next_excerpt.range.start > old_excerpt_end
            }) {
                push_excerpt(
                    &mut new_tree,
                    Excerpt {
                        path: new_excerpt.path.clone(),
                        buffer_id: new_excerpt.buffer_id,
                        text_summary: new_excerpt
                            .snapshot
                            .text_summary_for_range(new_excerpt.range.end..old_excerpt_end),
                        visible: old_excerpt_visible,
                    },
                );

                if old_excerpt_buffer_id == Some(new_excerpt.buffer_id) {
                    cursor.next(&());
                }
            }
        }

        new_tree.append(cursor.suffix(&()), &());
        drop(cursor);
        self.snapshot.excerpts = new_tree;
        self.check_invariants(cx);
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
        self.check_invariants(cx);
    }

    fn apply_renames(&mut self, renames: Vec<(BufferId, Option<Arc<Path>>, Option<Arc<Path>>)>) {

        // Remove all the excerpts that have been renamed.
        // let mut renamed_excerpts = Vec::new();
        // {
        //     let mut cursor = self.snapshot.excerpts.cursor::<Option<ExcerptKey>>(&());
        //     let mut new_tree = SumTree::default();
        //     for (buffer_id, old_path, new_path) in renames {
        //         let buffer_start = ExcerptOffset {
        //             path: old_path.clone(),
        //             buffer_id,
        //             offset: 0,
        //         };
        //         new_tree.append(cursor.slice(&buffer_start, Bias::Left, &()), &());
        //         while let Some(excerpt) = cursor.item() {
        //             if excerpt.key.buffer_id == buffer_id {
        //                 renamed_excerpts.push(Excerpt {
        //                     key: ExcerptKey {
        //                         path: new_path.clone(),
        //                         buffer_id,
        //                         range: excerpt.key.range.clone(),
        //                     },
        //                     text_summary: excerpt.text_summary.clone(),
        //                 });
        //                 cursor.next(&());
        //             } else {
        //                 break;
        //             }
        //         }
        //     }
        //     new_tree.append(cursor.suffix(&()), &());
        //     drop(cursor);
        //     self.snapshot.excerpts = new_tree;
        // }

        // Re-insert excerpts for the renamed buffers at the right location.
        // let mut cursor = self.snapshot.excerpts.cursor::<Option<ExcerptKey>>(&());
        // let mut new_tree = SumTree::default();
        // for excerpt in renamed_excerpts {
        //     let buffer_start = ExcerptOffset {
        //         path: excerpt.key.path.clone(),
        //         buffer_id: excerpt.key.buffer_id,
        //         offset: excerpt.key.range.start,
        //     };
        //     new_tree.append(cursor.slice(&buffer_start, Bias::Right, &()), &());
        //     new_tree.push(excerpt, &());
        // }
        // new_tree.append(cursor.suffix(&()), &());
        // drop(cursor);
        // self.snapshot.excerpts = new_tree;
    }

    fn apply_edits(
        &mut self,
        edits: BTreeMap<(Option<Arc<Path>>, BufferId), Vec<language::Edit<usize>>>,
    ) {
        // let mut cursor = self.snapshot.excerpts.cursor::<Option<ExcerptKey>>(&());
        // let mut new_tree = SumTree::default();

        // for ((path, buffer_id), buffer_edits) in edits {
        //     let mut buffer_edits = buffer_edits.into_iter().peekable();
        //     if let Some(buffer_edit) = buffer_edits.peek() {
        //         new_tree.append(
        //             cursor.slice(
        //                 &ExcerptOffset::new(path.clone(), buffer_id, buffer_edit.old.start),
        //                 Bias::Left,
        //                 &(),
        //             ),
        //             &(),
        //         );
        //     }

        //     let mut buffer_old_start = cursor.item().unwrap().key.range.start;
        //     let mut buffer_new_start = buffer_old_start;
        //     while let Some(buffer_edit) = buffer_edits.next() {
        //         let buffer_old_end = cursor.item().unwrap().key.range.end;

        //         if buffer_edit.old.start > buffer_old_start {
        //             push_excerpt(
        //                 &mut new_tree,
        //                 &self.snapshot.buffer_snapshots,
        //                 Excerpt {
        //                     key: ExcerptKey {
        //                         path: path.clone(),
        //                         buffer_id,
        //                         range: buffer_new_start..buffer_edit.new.start,
        //                     },
        //                     text_summary: TextSummary::default(), // todo!(change this)
        //                 },
        //             );
        //             buffer_old_start = buffer_edit.old.start;
        //             buffer_new_start = buffer_edit.new.start;
        //         }

        //         cursor.seek_forward(
        //             &ExcerptOffset::new(path.clone(), buffer_id, buffer_edit.old.end),
        //             Bias::Left,
        //             &(),
        //         );
        //         let buffer_old_end = cursor.item().unwrap().key.range.end;

        //         //  (         [  )   ]
        //         // if buffer_edit.old.end >

        //         // push_excerpt(
        //         //     &mut new_tree,
        //         //     &self.snapshot.buffer_snapshots,
        //         //     Excerpt {
        //         //         key: ExcerptKey {
        //         //             path: path.clone(),
        //         //             buffer_id,
        //         //             range: buffer_edit.new.clone(),
        //         //         },
        //         //         text_summary: TextSummary::default(), // todo!(change this)
        //         //     },
        //         // );

        //         cursor.seek_forward(
        //             &ExcerptOffset::new(path.clone(), buffer_id, buffer_edit.old.end),
        //             Bias::Left,
        //             &(),
        //         );

        //         // todo!("if the edit extends into another fragment, merge the two fragments.")
        //         let deleted = cmp::min(buffer_edit.old.end, buffer_old_end) - buffer_edit.old.start;
        //         let inserted = buffer_edit.new.len();

        //         // push_excerpt(
        //         //     &mut new_tree,
        //         //     &self.snapshot.buffer_snapshots,
        //         //     Excerpt {
        //         //         key: ExcerptKey {
        //         //             path: path.clone(),
        //         //             buffer_id,
        //         //             range: buffer_new_start
        //         //                 ..buffer_new_start + (buffer_old_end - buffer_old_start),
        //         //         },
        //         //         text_summary: TextSummary::default(), // todo!(change this)
        //         //     },
        //         // );
        //         // cursor.next(&());
        //         // new_tree.append(
        //         //     cursor.slice(
        //         //         &ExcerptOffset::new(path.clone(), buffer_id, buffer_edit.old.start),
        //         //         Bias::Left,
        //         //         &(),
        //         //     ),
        //         //     &(),
        //         // );
        //     }
        // }
    }

    pub fn snapshot(&mut self, cx: &mut ModelContext<Self>) -> MultiBufferSnapshot {
        self.sync(cx);
        self.snapshot.clone()
    }

    fn check_invariants(&self, cx: &AppContext) {
        #[cfg(debug_assertions)]
        {
            let mut cursor = self.snapshot.excerpts.cursor::<Option<ExcerptOffset>>(&());
            cursor.next(&());
            while let Some(excerpt) = cursor.item() {
                let buffer = self
                    .snapshot
                    .buffer_snapshots
                    .get(&excerpt.buffer_id)
                    .unwrap();
                let start = cursor
                    .start()
                    .as_ref()
                    .map_or(0, |start| start.buffer_offset);
                let end = cursor.end(&()).as_ref().unwrap().buffer_offset;
                assert_eq!(
                    excerpt.text_summary,
                    buffer.text_summary_for_range(start..end)
                );
                cursor.next(&());
            }

            for (buffer_id, buffer_snapshot) in self.snapshot.buffer_snapshots.iter() {
                let excerpt_offset = ExcerptOffset {
                    path: buffer_snapshot.file().map(|file| file.full_path(cx).into()),
                    buffer_id: *buffer_id,
                    buffer_offset: buffer_snapshot.len(),
                };
                cursor.seek(&Some(excerpt_offset.clone()), Bias::Left, &());
                assert_eq!(cursor.end(&()), Some(excerpt_offset));
            }
        }
    }
}

fn push_excerpt(excerpts: &mut SumTree<Excerpt>, excerpt: Excerpt) {
    let mut merged = false;
    excerpts.update_last(
        |last_excerpt| {
            if last_excerpt.buffer_id == excerpt.buffer_id
                && last_excerpt.visible == excerpt.visible
            {
                last_excerpt.text_summary += &excerpt.text_summary;
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
        let mut cursor = self.excerpts.cursor::<Option<ExcerptOffset>>(&());
        cursor.next(&());
        while let Some(excerpt) = cursor.item() {
            if excerpt.visible {
                let start = cursor
                    .start()
                    .as_ref()
                    .map_or(0, |start| start.buffer_offset);
                let end = start + excerpt.text_summary.len;
                let snapshot = self.buffer_snapshots.get(&excerpt.buffer_id).unwrap();
                text.push('\n');
                text.extend(snapshot.text_for_range(start..end));
            }
            cursor.next(&());
        }
        text
    }

    pub fn len(&self) -> usize {
        self.excerpts.summary().text.len
    }
}

#[derive(Clone)]
struct Excerpt {
    path: Option<Arc<Path>>,
    buffer_id: BufferId,
    text_summary: TextSummary,
    visible: bool,
}

impl sum_tree::Item for Excerpt {
    type Summary = ExcerptSummary;

    fn summary(&self) -> Self::Summary {
        ExcerptSummary {
            max_offset: Some(ExcerptOffset {
                path: self.path.clone(),
                buffer_id: self.buffer_id,
                buffer_offset: self.text_summary.len,
            }),
            text: self.text_summary.clone(),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct ExcerptSummary {
    max_offset: Option<ExcerptOffset>,
    text: TextSummary,
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct ExcerptOffset {
    path: Option<Arc<Path>>,
    buffer_id: BufferId,
    buffer_offset: usize,
}

impl sum_tree::Summary for ExcerptSummary {
    type Context = ();

    fn zero(_cx: &Self::Context) -> Self {
        Self::default()
    }

    fn add_summary(&mut self, summary: &Self, _cx: &Self::Context) {
        if let Some(excerpt_offset) = self.max_offset.as_mut() {
            let other_excerpt_offset = summary.max_offset.as_ref().unwrap();
            if excerpt_offset.path == other_excerpt_offset.path
                && excerpt_offset.buffer_id == other_excerpt_offset.buffer_id
            {
                excerpt_offset.buffer_offset += other_excerpt_offset.buffer_offset;
            } else {
                self.max_offset = Some(other_excerpt_offset.clone());
            }
        } else {
            self.max_offset = summary.max_offset.clone();
        }

        self.text += &summary.text;
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

impl<'a> sum_tree::Dimension<'a, ExcerptSummary> for Option<ExcerptOffset> {
    fn zero(_cx: &()) -> Self {
        None
    }

    fn add_summary(&mut self, summary: &'a ExcerptSummary, _cx: &()) {
        if let Some(excerpt_offset) = self.as_mut() {
            let other_excerpt_offset = summary.max_offset.as_ref().unwrap();
            if excerpt_offset.path == other_excerpt_offset.path
                && excerpt_offset.buffer_id == other_excerpt_offset.buffer_id
            {
                excerpt_offset.buffer_offset += other_excerpt_offset.buffer_offset;
            } else {
            }
        } else {
            *self = summary.max_offset.clone();
        }
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
