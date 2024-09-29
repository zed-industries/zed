use collections::{BTreeMap, HashMap};
use gpui::{AppContext, Model, ModelContext};
use language::{
    AnchorRangeExt, Bias, Buffer, BufferSnapshot, OffsetRangeExt as _, ReplicaId, TextSummary,
    ToOffset as _,
};
use std::{
    cmp::{self, Ordering, Reverse},
    fmt::Debug,
    ops::Range,
    path::Path,
    sync::Arc,
};
use sum_tree::{SumTree, TreeMap};

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

    pub fn insert_excerpts(
        &mut self,
        new_excerpts: impl IntoIterator<Item = (Model<Buffer>, Range<language::Anchor>)>,
        cx: &mut ModelContext<Self>,
    ) {
        self.sync(cx);

        struct NewExcerpt {
            snapshot: BufferSnapshot,
            key: ExcerptKey,
        }

        impl Debug for NewExcerpt {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("NewExcerpt")
                    .field("key", &self.key)
                    .finish_non_exhaustive()
            }
        }

        let mut new_excerpts = new_excerpts
            .into_iter()
            .filter_map(|(buffer_handle, range)| {
                let buffer = buffer_handle.read(cx);
                if range.end.cmp(&range.start, buffer).is_gt() {
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
                        snapshot: buffer.snapshot(),
                        key: ExcerptKey {
                            path,
                            buffer_id,
                            range,
                        },
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        new_excerpts.sort_unstable_by(|a, b| {
            a.key
                .path
                .cmp(&b.key.path)
                .then_with(|| a.key.buffer_id.cmp(&b.key.buffer_id))
                .then_with(|| a.key.range.cmp(&b.key.range, &a.snapshot))
        });
        new_excerpts.dedup_by(|a, b| {
            if a.key.buffer_id == b.key.buffer_id
                && a.key.range.end.cmp(&b.key.range.start, &a.snapshot).is_ge()
                && a.key.range.start.cmp(&b.key.range.end, &a.snapshot).is_le()
            {
                if a.key
                    .range
                    .start
                    .cmp(&b.key.range.start, &a.snapshot)
                    .is_lt()
                {
                    b.key.range.start = a.key.range.start;
                }

                if a.key.range.end.cmp(&b.key.range.end, &a.snapshot).is_gt() {
                    b.key.range.end = a.key.range.end;
                }

                true
            } else {
                false
            }
        });

        dbg!(&new_excerpts);

        let mut cursor = self
            .snapshot
            .excerpts
            .cursor::<Option<ExcerptKey>>(&self.snapshot.buffer_snapshots);
        let mut new_tree = SumTree::<Excerpt>::new(&self.snapshot.buffer_snapshots);
        let mut new_excerpts = new_excerpts.into_iter().peekable();

        while let Some(new_excerpt) = new_excerpts.next() {
            new_tree.append(
                cursor.slice(
                    &new_excerpt.key,
                    Bias::Right,
                    &self.snapshot.buffer_snapshots,
                ),
                &self.snapshot.buffer_snapshots,
            );

            push_new_excerpt(
                &mut new_tree,
                new_excerpt.key.clone(),
                &self.snapshot.buffer_snapshots,
            );

            cursor.seek_forward(
                &ExcerptKey {
                    path: new_excerpt.key.path,
                    buffer_id: new_excerpt.key.buffer_id,
                    range: new_excerpt.key.range.end..new_excerpt.key.range.end,
                },
                Bias::Right,
                &self.snapshot.buffer_snapshots,
            );

            if let Some(prev_excerpt) = cursor.prev_item() {
                push_new_excerpt(
                    &mut new_tree,
                    prev_excerpt.key.clone(),
                    &self.snapshot.buffer_snapshots,
                );
            }
        }

        new_tree.append(
            cursor.suffix(&self.snapshot.buffer_snapshots),
            &self.snapshot.buffer_snapshots,
        );
        drop(cursor);
        self.snapshot.excerpts = new_tree;
        self.check_invariants();
    }

    fn sync(&mut self, cx: &mut ModelContext<Self>) {
        let mut renames = Vec::new();
        let mut edits = Vec::new();

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
                edits.push((new_path.clone(), *buffer_id, edit));
            }

            if changed {
                self.snapshot
                    .buffer_snapshots
                    .insert(*buffer_id, new_snapshot);
            }
        }
        edits.sort_by_key(|(path, buffer_id, _)| (path.clone(), *buffer_id));

        self.apply_renames(renames);
        self.apply_edits(edits);
        self.check_invariants();
    }

    fn apply_renames(&mut self, renames: Vec<(BufferId, Option<Arc<Path>>, Option<Arc<Path>>)>) {
        // Remove all the excerpts that have been renamed.
        // let mut renamed_excerpts = BTreeMap::default();
        // {
        //     let mut cursor = self.snapshot.excerpts.cursor::<Option<ExcerptKey>>(&());
        //     let mut new_tree = SumTree::default();
        //     for (buffer_id, old_path, new_path) in renames {
        //         let buffer_snapshot = self.snapshot.buffer_snapshots.get(&buffer_id).unwrap();
        //         new_tree.append(
        //             cursor.slice(
        //                 &Some(ExcerptKey {
        //                     path: old_path.clone(),
        //                     buffer_id,
        //                     range: buffer_snapshot.min_anchor()..buffer_snapshot.min_anchor(),
        //                 }),
        //                 Bias::Right,
        //                 &(),
        //             ),
        //             &(),
        //         );
        //         while let Some(excerpt) = cursor.item() {
        //             if excerpt.buffer_id == buffer_id {
        //                 renamed_excerpts
        //                     .entry((new_path.clone(), buffer_id))
        //                     .or_insert(Vec::new())
        //                     .push(ExcerptKey {
        //                         path: new_path.clone(),
        //                         buffer_id,
        //                         range: excerpt.range.clone(),
        //                     });
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

        // // Re-insert excerpts for the renamed buffers at the right location.
        // let mut cursor = self.snapshot.excerpts.cursor::<Option<ExcerptKey>>(&());
        // let mut new_tree = SumTree::default();
        // for ((path, buffer_id), excerpts) in renamed_excerpts {
        //     let buffer_snapshot = self.snapshot.buffer_snapshots.get(&buffer_id).unwrap();
        //     new_tree.append(
        //         cursor.slice(
        //             &Some(ExcerptKey {
        //                 path,
        //                 buffer_id,
        //                 range: buffer_snapshot.min_anchor()..buffer_snapshot.min_anchor(),
        //             }),
        //             Bias::Right,
        //             &(),
        //         ),
        //         &(),
        //     );
        //     new_tree.extend(excerpts, &());
        // }
        // new_tree.append(cursor.suffix(&()), &());
        // drop(cursor);
        // self.snapshot.excerpts = new_tree;
    }

    fn apply_edits(&mut self, edits: Vec<(Option<Arc<Path>>, BufferId, language::Edit<usize>)>) {
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

    fn check_invariants(&self) {
        #[cfg(debug_assertions)]
        {
            let mut cursor = self
                .snapshot
                .excerpts
                .cursor::<()>(&self.snapshot.buffer_snapshots);
            cursor.next(&self.snapshot.buffer_snapshots);
            while let Some(excerpt) = cursor.item() {
                if let Some(prev_excerpt) = cursor.prev_item() {
                    if excerpt.key.buffer_id == prev_excerpt.key.buffer_id {
                        let snapshot = self
                            .snapshot
                            .buffer_snapshots
                            .get(&excerpt.key.buffer_id)
                            .unwrap();
                        assert_eq!(
                            prev_excerpt
                                .key
                                .range
                                .end
                                .cmp(&excerpt.key.range.start, snapshot),
                            Ordering::Less,
                            "Overlapping excerpt ranges: {:?} and {:?}",
                            prev_excerpt,
                            excerpt
                        );
                    }
                }

                cursor.next(&self.snapshot.buffer_snapshots);
            }
        }
    }
}

fn push_new_excerpt(
    excerpts: &mut SumTree<Excerpt>,
    key: ExcerptKey,
    snapshots: &TreeMap<BufferId, BufferSnapshot>,
) {
    let snapshot = snapshots.get(&key.buffer_id).unwrap();

    let mut merged_with_previous = false;
    let mut touches_previous = false;
    excerpts.update_last(
        |last_excerpt| {
            if last_excerpt.key.buffer_id == key.buffer_id {
                if last_excerpt
                    .key
                    .range
                    .end
                    .cmp(&key.range.start, snapshot)
                    .is_ge()
                {
                    merged_with_previous = true;
                    if key
                        .range
                        .end
                        .cmp(&last_excerpt.key.range.end, snapshot)
                        .is_gt()
                    {
                        last_excerpt.key.range.end = key.range.end;
                        last_excerpt.empty = last_excerpt.key.range.to_offset(snapshot).is_empty();
                    }
                } else {
                    touches_previous = last_excerpt.key.range.end.to_offset(snapshot)
                        == key.range.start.to_offset(snapshot);
                }
            }
        },
        snapshots,
    );

    if !merged_with_previous {
        let empty = key.range.to_offset(snapshot).is_empty();
        excerpts.push(
            Excerpt {
                key,
                touches_previous,
                empty,
            },
            snapshots,
        );
    }
}

#[derive(Clone)]
pub struct MultiBufferSnapshot {
    excerpts: SumTree<Excerpt>,
    buffer_snapshots: TreeMap<BufferId, BufferSnapshot>,
}

impl Default for MultiBufferSnapshot {
    fn default() -> Self {
        let buffer_snapshots = TreeMap::default();
        Self {
            excerpts: SumTree::new(&buffer_snapshots),
            buffer_snapshots,
        }
    }
}

impl MultiBufferSnapshot {
    #[cfg(any(test, feature = "test-support"))]
    fn text(&self) -> String {
        dbg!(self.excerpts.items(&self.buffer_snapshots));
        let mut text = String::new();
        let mut cursor = self.excerpts.cursor::<()>(&self.buffer_snapshots);
        cursor.next(&self.buffer_snapshots);
        while let Some(excerpt) = cursor.item() {
            let snapshot = self.buffer_snapshots.get(&excerpt.key.buffer_id).unwrap();
            if excerpt.show_header() {
                text.push('\n');
            }
            text.extend(snapshot.text_for_range(excerpt.key.range.clone()));
            cursor.next(&self.buffer_snapshots);
        }
        text
    }

    pub fn len(&self) -> usize {
        self.excerpts.summary().text.len
    }
}

#[derive(Clone, Debug)]
struct Excerpt {
    key: ExcerptKey,
    touches_previous: bool,
    empty: bool,
}

impl Excerpt {
    fn show_header(&self) -> bool {
        !self.touches_previous && !self.empty
    }
}

#[derive(Clone, Debug)]
struct ExcerptKey {
    path: Option<Arc<Path>>,
    buffer_id: BufferId,
    range: Range<language::Anchor>,
}

impl sum_tree::Item for Excerpt {
    type Summary = ExcerptSummary;

    fn summary(&self, buffer_snapshots: &TreeMap<BufferId, BufferSnapshot>) -> Self::Summary {
        let snapshot = buffer_snapshots
            .get(&self.key.buffer_id)
            .expect("buffer snapshot not found");
        let range_summary: TextSummary = snapshot.text_summary_for_range(self.key.range.clone());
        let mut text = if self.show_header() {
            TextSummary::from("\n")
        } else {
            TextSummary::default()
        };
        text += range_summary;
        ExcerptSummary {
            max_key: Some(self.key.clone()),
            text,
        }
    }
}

#[derive(Clone, Debug, Default)]
struct ExcerptSummary {
    max_key: Option<ExcerptKey>,
    text: TextSummary,
}

impl sum_tree::Summary for ExcerptSummary {
    type Context = TreeMap<BufferId, BufferSnapshot>;

    fn zero(_cx: &Self::Context) -> Self {
        Self::default()
    }

    fn add_summary(&mut self, summary: &Self, _cx: &Self::Context) {
        self.max_key = summary.max_key.clone();
        self.text += &summary.text;
    }
}

impl<'a> sum_tree::Dimension<'a, ExcerptSummary> for usize {
    fn zero(_cx: &TreeMap<BufferId, BufferSnapshot>) -> Self {
        0
    }

    fn add_summary(
        &mut self,
        summary: &'a ExcerptSummary,
        _cx: &TreeMap<BufferId, BufferSnapshot>,
    ) {
        *self += summary.text.len;
    }
}

impl<'a> sum_tree::Dimension<'a, ExcerptSummary> for Option<ExcerptKey> {
    fn zero(_cx: &TreeMap<BufferId, BufferSnapshot>) -> Self {
        None
    }

    fn add_summary(
        &mut self,
        summary: &'a ExcerptSummary,
        _cx: &TreeMap<BufferId, BufferSnapshot>,
    ) {
        *self = summary.max_key.clone();
    }
}

impl<'a> sum_tree::SeekTarget<'a, ExcerptSummary, Option<ExcerptKey>> for ExcerptKey {
    fn cmp(
        &self,
        cursor_location: &Option<ExcerptKey>,
        buffer_snapshots: &TreeMap<BufferId, BufferSnapshot>,
    ) -> Ordering {
        if let Some(cursor_location) = cursor_location {
            self.path
                .cmp(&cursor_location.path)
                .then_with(|| self.buffer_id.cmp(&cursor_location.buffer_id))
                .then_with(|| {
                    let snapshot = buffer_snapshots.get(&self.buffer_id).unwrap();
                    self.range.cmp(&cursor_location.range, snapshot)
                })
        } else {
            Ordering::Greater
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
        let buffer_handle = cx.new_model(|cx| Buffer::local("abcdefghijklmnopqrstuvwxyz", cx));
        cx.new_model(|cx| {
            let mut multibuffer = MultiBuffer::new();
            let buffer = buffer_handle.read(cx);
            multibuffer.insert_excerpts(
                vec![
                    (
                        buffer_handle.clone(),
                        buffer.anchor_before(0)..buffer.anchor_after(2),
                    ),
                    (
                        buffer_handle.clone(),
                        buffer.anchor_before(4)..buffer.anchor_after(12),
                    ),
                ],
                cx,
            );
            assert_eq!(multibuffer.snapshot(cx).text(), "\nab\nefghijkl");

            let buffer = buffer_handle.read(cx);
            multibuffer.insert_excerpts(
                vec![
                    (
                        buffer_handle.clone(),
                        buffer.anchor_before(4)..buffer.anchor_after(6),
                    ),
                    (
                        buffer_handle.clone(),
                        buffer.anchor_before(8)..buffer.anchor_after(10),
                    ),
                ],
                cx,
            );
            assert_eq!(multibuffer.snapshot(cx).text(), "\nab\nefghijkl");

            let buffer = buffer_handle.read(cx);
            multibuffer.insert_excerpts(
                vec![
                    (
                        buffer_handle.clone(),
                        buffer.anchor_before(10)..buffer.anchor_after(14),
                    ),
                    (
                        buffer_handle.clone(),
                        buffer.anchor_before(16)..buffer.anchor_after(18),
                    ),
                ],
                cx,
            );
            assert_eq!(multibuffer.snapshot(cx).text(), "\nab\nefghijklmn\nqr");

            let buffer = buffer_handle.read(cx);
            multibuffer.insert_excerpts(
                vec![(
                    buffer_handle.clone(),
                    buffer.anchor_before(12)..buffer.anchor_after(17),
                )],
                cx,
            );
            assert_eq!(multibuffer.snapshot(cx).text(), "\nab\nefghijklmnopqr");

            multibuffer
        });
    }

    // #[gpui::test(iterations = 1000)]
    // fn test_insert_random_excerpts(mut rng: StdRng, cx: &mut AppContext) {
    //     let buffer = cx.new_model(|cx| {
    //         let random_words: Vec<&str> = WORDS.choose_multiple(&mut rng, 10).cloned().collect();
    //         let content = random_words.join(" ");
    //         Buffer::local(&content, cx)
    //     });

    //     let buffer_len = buffer.read(cx).len();

    //     let generate_excerpts = |rng: &mut StdRng| {
    //         let mut ranges = Vec::new();
    //         for _ in 0..5 {
    //             let start = rng.gen_range(0..=buffer_len);
    //             let end = rng.gen_range(start..=buffer_len);
    //             ranges.push(start..end);
    //         }
    //         ranges
    //     };

    //     cx.new_model(|cx| {
    //         let mut multibuffer = MultiBuffer::new();
    //         let excerpts1 = generate_excerpts(&mut rng);
    //         let excerpts2 = generate_excerpts(&mut rng);

    //         multibuffer.insert_excerpts(
    //             excerpts1
    //                 .iter()
    //                 .map(|range| (buffer.clone(), range.clone())),
    //             cx,
    //         );
    //         dbg!(multibuffer.snapshot.excerpts.items(&()));
    //         multibuffer.insert_excerpts(
    //             excerpts2
    //                 .iter()
    //                 .map(|range| (buffer.clone(), range.clone())),
    //             cx,
    //         );
    //         dbg!(multibuffer.snapshot.excerpts.items(&()));

    //         let mut excerpt_ranges = excerpts1
    //             .iter()
    //             .chain(&excerpts2)
    //             .cloned()
    //             .collect::<Vec<_>>();
    //         excerpt_ranges.sort_by_key(|range| (range.start, range.end));
    //         excerpt_ranges.dedup_by(|a, b| {
    //             if a.start <= b.end && b.start <= a.end {
    //                 b.start = a.start.min(b.start);
    //                 b.end = a.end.max(b.end);
    //                 true
    //             } else {
    //                 false
    //             }
    //         });

    //         let expected_text = excerpt_ranges
    //             .into_iter()
    //             .filter_map(|range| {
    //                 if range.is_empty() {
    //                     None
    //                 } else {
    //                     Some(format!("\n{}", &buffer.read(cx).text()[range]))
    //                 }
    //             })
    //             .collect::<String>();
    //         assert_eq!(multibuffer.snapshot(cx).text(), expected_text);

    //         multibuffer
    //     });
    // }

    // #[gpui::test]
    // fn test_rename_buffers(cx: &mut AppContext) {
    //     let buffer1 = cx.new_model(|cx| {
    //         let mut buffer = Buffer::local("The quick brown fox", cx);
    //         buffer.file_updated(
    //             Arc::new(TestFile {
    //                 path: Path::new("a.txt").into(),
    //             }),
    //             cx,
    //         );
    //         buffer
    //     });
    //     let buffer2 = cx.new_model(|cx| {
    //         let mut buffer = Buffer::local("jumps over the lazy dog", cx);
    //         buffer.file_updated(
    //             Arc::new(TestFile {
    //                 path: Path::new("b.txt").into(),
    //             }),
    //             cx,
    //         );
    //         buffer
    //     });

    //     cx.new_model(|cx| {
    //         let mut multibuffer = MultiBuffer::new();
    //         multibuffer.insert_excerpts(
    //             vec![
    //                 (buffer1.clone(), 0..9),
    //                 (buffer2.clone(), 0..5),
    //                 (buffer1.clone(), 10..19),
    //                 (buffer2.clone(), 6..23),
    //             ],
    //             cx,
    //         );
    //         assert_eq!(
    //             multibuffer.snapshot(cx).text(),
    //             "\nThe quick\nbrown fox\njumps\nover the lazy dog"
    //         );

    //         // Rename /b.txt to /0.txt
    //         buffer2.update(cx, |buffer, cx| {
    //             buffer.file_updated(
    //                 Arc::new(TestFile {
    //                     path: Path::new("/0.txt").into(),
    //                 }),
    //                 cx,
    //             );
    //         });
    //         assert_eq!(
    //             multibuffer.snapshot(cx).text(),
    //             "\njumps\nover the lazy dog\nThe quick\nbrown fox"
    //         );
    //         multibuffer.check_invariants(cx);

    //         multibuffer
    //     });
    // }

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
