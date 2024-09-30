use collections::{BTreeMap, HashMap};
use gpui::{Model, ModelContext};
use language::{
    AnchorRangeExt, Bias, Buffer, BufferSnapshot, Edit, OffsetRangeExt as _, ReplicaId,
    TextSummary, ToOffset as _,
};
use std::{cmp::Ordering, fmt::Debug, ops::Range, path::Path, sync::Arc};
use sum_tree::{Item, SeekTarget, SumTree, TreeMap};
use text::Patch;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct BufferId {
    remote_id: text::BufferId,
    replica_id: ReplicaId,
}

pub struct MultiBuffer {
    snapshot: MultiBufferSnapshot,
    buffers: HashMap<BufferId, Model<Buffer>>,
    edits: text::Topic,
}

impl MultiBuffer {
    pub fn new() -> Self {
        Self {
            snapshot: MultiBufferSnapshot::default(),
            buffers: HashMap::default(),
            edits: text::Topic::default(),
        }
    }

    pub fn subscribe(&mut self) -> text::Subscription {
        self.edits.subscribe()
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
        new_excerpts.sort_unstable_by(|a, b| a.key.cmp(&b.key, &a.snapshot));
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

        log::info!(
            "inserting new excerpts... {:?}",
            new_excerpts
                .iter()
                .map(|excerpt| { excerpt.key.range.to_offset(&excerpt.snapshot) })
                .collect::<Vec<_>>()
        );

        let mut cursor = self
            .snapshot
            .excerpts
            .cursor::<(Option<ExcerptKey>, usize)>(&self.snapshot.buffer_snapshots);
        let mut new_tree = SumTree::<Excerpt>::new(&self.snapshot.buffer_snapshots);
        let mut new_excerpts = new_excerpts.into_iter().peekable();
        let mut edits = Patch::default();
        while let Some(mut new_excerpt) = new_excerpts.next() {
            log::info!(
                "inserting new excerpt {:?}",
                new_excerpt.key.range.to_offset(&new_excerpt.snapshot)
            );

            // Reinsert all excerpts preceding the start of the new excerpt
            new_tree.append(
                cursor.slice(
                    &ExcerptAnchor {
                        path: new_excerpt.key.path.clone(),
                        buffer_id: new_excerpt.key.buffer_id,
                        anchor: new_excerpt.key.range.start,
                    },
                    Bias::Left,
                    &self.snapshot.buffer_snapshots,
                ),
                &self.snapshot.buffer_snapshots,
            );

            let mut edit_old_start = cursor.start().1;

            // If the new excerpt start is inside of an existing excerpt, adjust the start
            // offset of the edit by the overshoot.
            if let Some(old_excerpt) = cursor.item() {
                if old_excerpt.key.buffer_id == new_excerpt.key.buffer_id
                    && new_excerpt
                        .key
                        .range
                        .start
                        .cmp(&old_excerpt.key.range.start, &new_excerpt.snapshot)
                        .is_ge()
                {
                    edit_old_start += new_excerpt.snapshot.text_summary_for_range::<usize, _>(
                        old_excerpt.key.range.start..new_excerpt.key.range.start,
                    );
                    new_excerpt.key.range.start = old_excerpt.key.range.start;
                }
            }

            let edit_old_end = loop {
                cursor.seek_forward(
                    &ExcerptAnchor {
                        path: new_excerpt.key.path.clone(),
                        buffer_id: new_excerpt.key.buffer_id,
                        anchor: new_excerpt.key.range.end,
                    },
                    Bias::Left,
                    &self.snapshot.buffer_snapshots,
                );

                let mut edit_old_end_offset = cursor.start().1;

                // If the new excerpt end is inside of an existing excerpt, adjust the end
                // offset of the edit by the overshoot.
                if let Some(old_excerpt) = cursor.item() {
                    if old_excerpt.key.buffer_id == new_excerpt.key.buffer_id
                        && new_excerpt
                            .key
                            .range
                            .end
                            .cmp(&old_excerpt.key.range.start, &new_excerpt.snapshot)
                            .is_ge()
                    {
                        edit_old_end_offset +=
                            new_excerpt.snapshot.text_summary_for_range::<usize, _>(
                                old_excerpt.key.range.start..new_excerpt.key.range.end,
                            );
                        new_excerpt.key.range.end = old_excerpt.key.range.end;
                        cursor.next(&self.snapshot.buffer_snapshots);
                    }
                }

                if let Some(next_new_excerpt) = new_excerpts.peek() {
                    if next_new_excerpt.key.buffer_id == new_excerpt.key.buffer_id
                        && next_new_excerpt
                            .key
                            .range
                            .start
                            .cmp(&new_excerpt.key.range.end, &new_excerpt.snapshot)
                            .is_le()
                    {
                        if next_new_excerpt
                            .key
                            .range
                            .end
                            .cmp(&new_excerpt.key.range.end, &new_excerpt.snapshot)
                            .is_gt()
                        {
                            new_excerpt.key.range.end = next_new_excerpt.key.range.end;
                        }
                        new_excerpts.next();
                    } else {
                        break edit_old_end_offset;
                    }
                } else {
                    break edit_old_end_offset;
                }
            };

            let edit_new_start = new_tree.summary().text.len;
            let new_excerpt_offset_range = new_excerpt.key.range.to_offset(&new_excerpt.snapshot);
            // Don't show a header if this excerpt is empty or is adjacent to a previous excerpt
            let show_header = !(new_excerpt_offset_range.is_empty()
                || new_tree
                    .summary()
                    .last_header
                    .as_ref()
                    .map_or(false, |last_header| {
                        last_header.buffer_id == new_excerpt.key.buffer_id
                            && last_header.range.end.to_offset(&new_excerpt.snapshot)
                                == new_excerpt_offset_range.start
                    }));
            new_tree.push(
                Excerpt {
                    key: new_excerpt.key.clone(),
                    show_header,
                },
                &self.snapshot.buffer_snapshots,
            );
            let edit_new_end = new_tree.summary().text.len;
            edits.push(Edit {
                old: edit_old_start..edit_old_end,
                new: edit_new_start..edit_new_end,
            });

            // If any old excerpts are adjacent to the new excerpts in offset
            // space, ensure they don't have a header.
            while let Some(next_old_excerpt) = cursor.item() {
                if next_old_excerpt.key.buffer_id != new_excerpt.key.buffer_id {
                    break;
                }

                // Only fix up subsequent old excerpts if no new excerpts intersect
                // them. If new excerpts intersect them, we'll reconstruct them when
                // processing those new excerpts.
                if let Some(next_new_excerpt) = new_excerpts.peek() {
                    if next_new_excerpt.key.buffer_id == new_excerpt.key.buffer_id
                        && next_new_excerpt
                            .key
                            .range
                            .start
                            .cmp(&next_old_excerpt.key.range.end, &next_new_excerpt.snapshot)
                            .is_le()
                    {
                        break;
                    }
                }

                if new_excerpt.key.range.end.to_offset(&new_excerpt.snapshot)
                    == next_old_excerpt
                        .key
                        .range
                        .start
                        .to_offset(&new_excerpt.snapshot)
                {
                    let mut next_old_excerpt = next_old_excerpt.clone();
                    cursor.next(&self.snapshot.buffer_snapshots);

                    if next_old_excerpt.show_header {
                        edits.push(Edit {
                            old: edit_old_end..edit_old_end + 1,
                            new: edit_new_end..edit_new_end,
                        });

                        next_old_excerpt.show_header = false;
                        new_tree.push(next_old_excerpt, &self.snapshot.buffer_snapshots);
                        break;
                    } else {
                        new_tree.push(next_old_excerpt, &self.snapshot.buffer_snapshots);
                    }
                } else {
                    break;
                }
            }
        }

        new_tree.append(
            cursor.suffix(&self.snapshot.buffer_snapshots),
            &self.snapshot.buffer_snapshots,
        );
        drop(cursor);
        self.snapshot.excerpts = new_tree;

        log::info!("========================");
        for excerpt in self.snapshot.excerpts.iter() {
            log::info!(
                "range: {:?}",
                excerpt.key.range.to_offset(
                    self.snapshot
                        .buffer_snapshots
                        .get(&excerpt.key.buffer_id)
                        .unwrap(),
                )
            )
        }

        self.check_invariants();

        self.edits.publish(edits);
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
            if new_path == old_path {
                for edit in new_snapshot.edits_since::<usize>(&old_snapshot.version) {
                    changed = true;
                    edits.push((new_path.clone(), *buffer_id, edit));
                }
            } else {
                renames.push((*buffer_id, old_path, new_path.clone()));
                changed = true;
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
        let mut renamed_excerpts = BTreeMap::default();
        {
            let mut cursor = self
                .snapshot
                .excerpts
                .cursor::<(Option<ExcerptKey>, usize)>(&self.snapshot.buffer_snapshots);
            let mut new_tree = SumTree::new(&self.snapshot.buffer_snapshots);
            let mut deletions = Patch::default();
            for (buffer_id, old_path, new_path) in renames {
                let buffer_snapshot = self.snapshot.buffer_snapshots.get(&buffer_id).unwrap();
                new_tree.append(
                    cursor.slice(
                        &ExcerptKey {
                            path: old_path.clone(),
                            buffer_id,
                            range: buffer_snapshot.min_anchor()..buffer_snapshot.max_anchor(),
                        },
                        Bias::Left,
                        &self.snapshot.buffer_snapshots,
                    ),
                    &self.snapshot.buffer_snapshots,
                );

                let edit_old_start = cursor.start().1;
                let edit_new_start = new_tree.summary().text.len;
                while let Some(excerpt) = cursor.item() {
                    if excerpt.key.buffer_id == buffer_id {
                        renamed_excerpts
                            .entry((new_path.clone(), buffer_id))
                            .or_insert(Vec::new())
                            .push(Excerpt {
                                key: ExcerptKey {
                                    path: new_path.clone(),
                                    buffer_id,
                                    range: excerpt.key.range.clone(),
                                },
                                show_header: excerpt.show_header,
                            });
                        cursor.next(&self.snapshot.buffer_snapshots);
                    } else {
                        break;
                    }
                }
                let edit_old_end = cursor.start().1;
                deletions.push(Edit {
                    old: edit_old_start..edit_old_end,
                    new: edit_new_start..edit_new_start,
                });
            }
            new_tree.append(
                cursor.suffix(&self.snapshot.buffer_snapshots),
                &self.snapshot.buffer_snapshots,
            );
            drop(cursor);
            self.snapshot.excerpts = new_tree;
            self.edits.publish(deletions);
        }

        // Re-insert excerpts for the renamed buffers at the right location.
        let mut cursor = self
            .snapshot
            .excerpts
            .cursor::<(Option<ExcerptKey>, usize)>(&self.snapshot.buffer_snapshots);
        let mut new_tree = SumTree::<Excerpt>::new(&self.snapshot.buffer_snapshots);
        let mut insertions = Patch::default();
        for ((new_path, buffer_id), excerpts) in renamed_excerpts {
            let buffer_snapshot = self.snapshot.buffer_snapshots.get(&buffer_id).unwrap();
            let edit_old_start = cursor.start().1;
            let edit_new_start = new_tree.summary().text.len;
            new_tree.append(
                cursor.slice(
                    &ExcerptKey {
                        path: new_path,
                        buffer_id,
                        range: buffer_snapshot.min_anchor()..buffer_snapshot.max_anchor(),
                    },
                    Bias::Left,
                    &self.snapshot.buffer_snapshots,
                ),
                &self.snapshot.buffer_snapshots,
            );
            new_tree.extend(excerpts, &self.snapshot.buffer_snapshots);
            let edit_new_end = new_tree.summary().text.len;
            insertions.push(Edit {
                old: edit_old_start..edit_old_start,
                new: edit_new_start..edit_new_end,
            });
        }
        new_tree.append(
            cursor.suffix(&self.snapshot.buffer_snapshots),
            &self.snapshot.buffer_snapshots,
        );
        drop(cursor);
        self.snapshot.excerpts = new_tree;
        self.edits.publish(insertions);
    }

    fn apply_edits(&mut self, edits: Vec<(Option<Arc<Path>>, BufferId, language::Edit<usize>)>) {
        let mut cursor = self
            .snapshot
            .excerpts
            .cursor::<Option<ExcerptKey>>(&self.snapshot.buffer_snapshots);
        let mut new_tree = SumTree::new(&self.snapshot.buffer_snapshots);
        for (path, buffer_id, edit) in edits {
            let snapshot = self.snapshot.buffer_snapshots.get(&buffer_id).unwrap();
            let edit_start = snapshot.anchor_before(edit.new.start);
            let edit_end = snapshot.anchor_after(edit.new.end);
            new_tree.append(
                cursor.slice(
                    &ExcerptAnchor {
                        path: path.clone(),
                        buffer_id,
                        anchor: edit_start,
                    },
                    Bias::Left,
                    &self.snapshot.buffer_snapshots,
                ),
                &self.snapshot.buffer_snapshots,
            );

            while let Some(excerpt) = cursor.item() {
                if excerpt.key.buffer_id == buffer_id
                    && excerpt.key.range.start.cmp(&edit_end, &snapshot).is_le()
                {
                    push_new_excerpt(
                        &mut new_tree,
                        excerpt.key.clone(),
                        &self.snapshot.buffer_snapshots,
                    );
                    cursor.next(&self.snapshot.buffer_snapshots);
                } else {
                    break;
                }
            }
        }

        new_tree.append(
            cursor.suffix(&self.snapshot.buffer_snapshots),
            &self.snapshot.buffer_snapshots,
        );
        drop(cursor);
        self.snapshot.excerpts = new_tree;
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
                .cursor::<TextSummary>(&self.snapshot.buffer_snapshots);
            cursor.next(&self.snapshot.buffer_snapshots);
            let mut summary = TextSummary::default();
            while let Some(excerpt) = cursor.item() {
                let snapshot = self
                    .snapshot
                    .buffer_snapshots
                    .get(&excerpt.key.buffer_id)
                    .unwrap();

                if let Some(prev_excerpt) = cursor.prev_item() {
                    if excerpt.key.buffer_id == prev_excerpt.key.buffer_id {
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

                summary += &excerpt.summary(&self.snapshot.buffer_snapshots).text;
                cursor.next(&self.snapshot.buffer_snapshots);
                assert_eq!(cursor.start().clone(), summary);
            }
        }
    }
}

fn push_new_excerpt(
    excerpts: &mut SumTree<Excerpt>,
    new_key: ExcerptKey,
    snapshots: &TreeMap<BufferId, BufferSnapshot>,
) {
    let snapshot = snapshots.get(&new_key.buffer_id).unwrap();
    // dbg!(
    //     snapshot.text(),
    //     snapshot
    //         .text_for_range(new_key.range.clone())
    //         .collect::<String>()
    // );

    let last_header = excerpts.summary().last_header.clone();
    let mut merged_with_previous = false;
    excerpts.update_last(
        |last_excerpt| {
            if last_excerpt.key.buffer_id == new_key.buffer_id {
                if last_excerpt
                    .key
                    .range
                    .end
                    .cmp(&new_key.range.start, snapshot)
                    .is_ge()
                {
                    merged_with_previous = true;
                    if new_key
                        .range
                        .end
                        .cmp(&last_excerpt.key.range.end, snapshot)
                        .is_gt()
                    {
                        last_excerpt.key.range.end = new_key.range.end;
                        if !last_excerpt.show_header {
                            last_excerpt.show_header = should_show_header(
                                &last_excerpt.key,
                                last_header.as_ref(),
                                snapshot,
                            );
                        }
                    }
                }
            }
        },
        snapshots,
    );

    // dbg!(merged_with_previous);
    if !merged_with_previous {
        excerpts.push(
            Excerpt {
                show_header: should_show_header(&new_key, last_header.as_ref(), snapshot),
                key: new_key,
            },
            snapshots,
        );
    }

    /// Show header if new excerpt is non-empty and not touching a previous excerpt showing header.
    fn should_show_header(
        key: &ExcerptKey,
        last_header: Option<&ExcerptKey>,
        snapshot: &BufferSnapshot,
    ) -> bool {
        let offset_range = key.range.to_offset(snapshot);
        !offset_range.is_empty()
            && last_header.map_or(true, |last_header| {
                last_header.buffer_id != key.buffer_id
                    || last_header.range.end.to_offset(&snapshot) < offset_range.start
            })
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
        let mut text = String::new();
        let mut cursor = self.excerpts.cursor::<()>(&self.buffer_snapshots);
        cursor.next(&self.buffer_snapshots);
        while let Some(excerpt) = cursor.item() {
            let snapshot = self.buffer_snapshots.get(&excerpt.key.buffer_id).unwrap();
            if excerpt.show_header {
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
    show_header: bool,
}

#[derive(Clone, Debug)]
struct ExcerptKey {
    path: Option<Arc<Path>>,
    buffer_id: BufferId,
    range: Range<language::Anchor>,
}

impl ExcerptKey {
    fn cmp(&self, other: &Self, snapshot: &BufferSnapshot) -> Ordering {
        self.path
            .cmp(&other.path)
            .then_with(|| self.buffer_id.cmp(&other.buffer_id))
            .then_with(|| self.range.cmp(&other.range, snapshot))
    }

    fn intersects(&self, other: &Self, snapshot: &BufferSnapshot) -> bool {
        if self.buffer_id == other.buffer_id {
            self.range.end.cmp(&other.range.start, snapshot).is_ge()
                && other.range.end.cmp(&self.range.start, snapshot).is_ge()
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct ExcerptAnchor {
    path: Option<Arc<Path>>,
    buffer_id: BufferId,
    anchor: language::Anchor,
}

impl ExcerptAnchor {
    fn cmp(&self, key: &Option<ExcerptKey>, snapshot: &BufferSnapshot) -> Ordering {
        if let Some(cursor_location) = key {
            self.path
                .cmp(&cursor_location.path)
                .then_with(|| self.buffer_id.cmp(&cursor_location.buffer_id))
                .then_with(|| {
                    if self
                        .anchor
                        .cmp(&cursor_location.range.start, snapshot)
                        .is_lt()
                    {
                        Ordering::Less
                    } else if self
                        .anchor
                        .cmp(&cursor_location.range.end, snapshot)
                        .is_gt()
                    {
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

impl<'a> sum_tree::SeekTarget<'a, ExcerptSummary, Option<ExcerptKey>> for ExcerptAnchor {
    fn cmp(
        &self,
        cursor_location: &Option<ExcerptKey>,
        buffer_snapshots: &TreeMap<BufferId, BufferSnapshot>,
    ) -> Ordering {
        let snapshot = buffer_snapshots.get(&self.buffer_id).unwrap();
        self.cmp(cursor_location, snapshot)
    }
}

impl<'a> sum_tree::SeekTarget<'a, ExcerptSummary, (Option<ExcerptKey>, usize)> for ExcerptAnchor {
    fn cmp(
        &self,
        cursor_location: &(Option<ExcerptKey>, usize),
        buffer_snapshots: &TreeMap<BufferId, BufferSnapshot>,
    ) -> Ordering {
        let snapshot = buffer_snapshots.get(&self.buffer_id).unwrap();
        self.cmp(&cursor_location.0, snapshot)
    }
}

impl sum_tree::Item for Excerpt {
    type Summary = ExcerptSummary;

    fn summary(&self, buffer_snapshots: &TreeMap<BufferId, BufferSnapshot>) -> Self::Summary {
        let snapshot = buffer_snapshots
            .get(&self.key.buffer_id)
            .expect("buffer snapshot not found");
        let range_summary: TextSummary = snapshot.text_summary_for_range(self.key.range.clone());
        let mut text = if self.show_header {
            TextSummary::from("\n")
        } else {
            TextSummary::default()
        };
        text += range_summary;
        ExcerptSummary {
            max_key: Some(self.key.clone()),
            text,
            last_header: self.show_header.then_some(self.key.clone()),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct ExcerptSummary {
    max_key: Option<ExcerptKey>,
    text: TextSummary,
    last_header: Option<ExcerptKey>,
}

impl sum_tree::Summary for ExcerptSummary {
    type Context = TreeMap<BufferId, BufferSnapshot>;

    fn zero(_cx: &Self::Context) -> Self {
        Self::default()
    }

    fn add_summary(&mut self, summary: &Self, snapshots: &TreeMap<BufferId, BufferSnapshot>) {
        self.max_key = summary.max_key.clone();
        self.text += &summary.text;
        if summary.last_header.is_some() {
            self.last_header = summary.last_header.clone();
        } else if let Some(last_header) = self.last_header.as_mut() {
            if let Some(other_max_key) = summary.max_key.as_ref() {
                if last_header.buffer_id == other_max_key.buffer_id {
                    let snapshot = snapshots.get(&last_header.buffer_id).unwrap();
                    if last_header.range.end.to_offset(snapshot)
                        == other_max_key.range.start.to_offset(snapshot)
                    {
                        last_header.range.end = other_max_key.range.end;
                    }
                }
            }
        }
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

impl<'a> sum_tree::Dimension<'a, ExcerptSummary> for TextSummary {
    fn zero(_cx: &TreeMap<BufferId, BufferSnapshot>) -> Self {
        TextSummary::default()
    }

    fn add_summary(
        &mut self,
        summary: &'a ExcerptSummary,
        _cx: &TreeMap<BufferId, BufferSnapshot>,
    ) {
        self.add_summary(&summary.text, &());
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
            self.cmp(
                cursor_location,
                buffer_snapshots.get(&self.buffer_id).unwrap(),
            )
        } else {
            Ordering::Greater
        }
    }
}

impl<'a> sum_tree::SeekTarget<'a, ExcerptSummary, (Option<ExcerptKey>, usize)> for ExcerptKey {
    fn cmp(
        &self,
        cursor_location: &(Option<ExcerptKey>, usize),
        buffer_snapshots: &TreeMap<BufferId, BufferSnapshot>,
    ) -> Ordering {
        if let Some(cursor_location) = cursor_location.0.as_ref() {
            self.cmp(
                cursor_location,
                buffer_snapshots.get(&self.buffer_id).unwrap(),
            )
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

    #[gpui::test]
    fn test_edit_subscription(cx: &mut AppContext) {
        let buffer_handle = cx.new_model(|cx| Buffer::local("abcdefghijklmnopqrstuvwxyz", cx));
        cx.new_model(|cx| {
            let mut multibuffer = MultiBuffer::new();
            let subscription = multibuffer.subscribe();

            let buffer = buffer_handle.read(cx);
            multibuffer.insert_excerpts(
                vec![
                    (
                        buffer_handle.clone(),
                        buffer.anchor_before(0)..buffer.anchor_after(5),
                    ),
                    (
                        buffer_handle.clone(),
                        buffer.anchor_before(10)..buffer.anchor_after(15),
                    ),
                ],
                cx,
            );
            assert_eq!(multibuffer.snapshot(cx).text(), "\nabcde\nklmno");
            assert_eq!(
                subscription.consume().into_inner(),
                &[Edit {
                    old: 0..0,
                    new: 0..12
                }]
            );

            buffer_handle.update(cx, |buffer, cx| {
                buffer.edit([(1..3, "ABCD")], None, cx);
            });
            assert_eq!(multibuffer.snapshot(cx).text(), "\naABCDe\nklmno");
            assert_eq!(
                subscription.consume().into_inner(),
                &[Edit {
                    old: 2..4,
                    new: 2..6
                }]
            );

            multibuffer
        });
    }

    #[gpui::test(iterations = 1000)]
    fn test_random_multibuffer(mut rng: StdRng, cx: &mut AppContext) {
        let operations = std::env::var("OPERATIONS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(20);

        let fruits = cx.new_model(|cx| {
            let random_words: Vec<&str> = FRUITS.choose_multiple(&mut rng, 10).cloned().collect();
            let content = random_words.join(" ");
            Buffer::local(&content, cx)
        });
        let cars = cx.new_model(|cx| {
            let random_words: Vec<&str> = CARS.choose_multiple(&mut rng, 10).cloned().collect();
            let content = random_words.join(" ");
            Buffer::local(&content, cx)
        });
        let animals = cx.new_model(|cx| {
            let random_words: Vec<&str> = ANIMALS.choose_multiple(&mut rng, 10).cloned().collect();
            let content = random_words.join(" ");
            Buffer::local(&content, cx)
        });

        cx.new_model(|cx| {
            let mut multibuffer = MultiBuffer::new();
            let mut excerpts = Vec::new();

            for _ in 0..operations {
                log::info!("=====================================");
                let buffer_handle = match rng.gen_range(0..3) {
                    0 => fruits.clone(),
                    1 => cars.clone(),
                    _ => animals.clone(),
                };

                log::info!(
                    "{} ({}):",
                    buffer_handle
                        .read(cx)
                        .file()
                        .map_or(Path::new("<untitled>"), |file| file.path())
                        .display(),
                    buffer_handle.read(cx).remote_id()
                );
                match rng.gen_range(0..100) {
                    0..35 => {
                        let mut new_excerpts = Vec::new();
                        for _ in 0..5 {
                            let buffer = buffer_handle.read(cx);
                            let range = buffer.random_byte_range(0, &mut rng);
                            let start_bias = if rng.gen() { Bias::Left } else { Bias::Right };
                            let end_bias = if rng.gen() { Bias::Left } else { Bias::Right };
                            new_excerpts.push((
                                buffer_handle.clone(),
                                buffer.anchor_at(range.start, start_bias)
                                    ..buffer.anchor_at(range.end, end_bias),
                            ));
                        }

                        log::info!("inserting excerpts {:?}", new_excerpts);
                        multibuffer.insert_excerpts(new_excerpts.iter().cloned(), cx);
                        excerpts.append(&mut new_excerpts);
                    }
                    35..50 => {
                        let file = Arc::new(TestFile {
                            path: Path::new(DESSERTS.choose(&mut rng).unwrap()).into(),
                        });
                        log::info!("renaming to {:?}", file.path);
                        buffer_handle.update(cx, |buffer, cx| buffer.file_updated(file, cx));
                    }
                    _ => {
                        let edit_count = rng.gen_range(1..=5);
                        buffer_handle.update(cx, |buffer, cx| {
                            buffer.randomly_edit(&mut rng, edit_count, cx)
                        });
                    }
                }

                let mut expected_excerpts = excerpts
                    .iter()
                    .filter_map(|(buffer, range)| {
                        let range = range.to_offset(buffer.read(cx));
                        if range.is_empty() {
                            None
                        } else {
                            Some((buffer, range))
                        }
                    })
                    .collect::<Vec<_>>();
                expected_excerpts.sort_by(|(buffer_a, range_a), (buffer_b, range_b)| {
                    buffer_a
                        .read(cx)
                        .file()
                        .map(|file| file.full_path(cx))
                        .cmp(&buffer_b.read(cx).file().map(|file| file.full_path(cx)))
                        .then_with(|| {
                            buffer_a
                                .read(cx)
                                .remote_id()
                                .cmp(&buffer_b.read(cx).remote_id())
                        })
                        .then_with(|| Ord::cmp(&range_a.start, &range_b.start))
                        .then_with(|| Ord::cmp(&range_b.end, &range_a.end))
                });
                expected_excerpts.dedup_by(|(buffer_a, range_a), (buffer_b, range_b)| {
                    let buffer_a = buffer_a.read(cx);
                    let buffer_b = buffer_b.read(cx);

                    if buffer_a.remote_id() == buffer_b.remote_id()
                        && range_a.start <= range_b.end
                        && range_b.start <= range_a.end
                    {
                        range_b.start = range_a.start.min(range_b.start);
                        range_b.end = range_a.end.max(range_b.end);
                        true
                    } else {
                        false
                    }
                });

                let mut expected_text = String::new();
                let mut last_header: Option<(language::BufferId, Range<usize>)> = None;

                for (buffer, range) in expected_excerpts {
                    let buffer = buffer.read(cx);
                    let offset_range = range.to_offset(&buffer);
                    if !offset_range.is_empty() {
                        if last_header.as_ref().map_or(
                            true,
                            |(last_header_buffer_id, last_header_range)| {
                                *last_header_buffer_id != buffer.remote_id()
                                    || last_header_range.end < offset_range.start
                            },
                        ) {
                            expected_text.push('\n');
                            last_header = Some((buffer.remote_id(), offset_range.clone()));
                        }

                        expected_text.extend(buffer.text_for_range(offset_range.clone()));
                    }
                }
                assert_eq!(multibuffer.snapshot(cx).text(), expected_text);
                log::info!("text: {:?}", expected_text);
            }

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

    #[ctor::ctor]
    fn init_logger() {
        if std::env::var("RUST_LOG").is_ok() {
            env_logger::init();
        }
    }

    const FRUITS: &[&str] = &[
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

    const CARS: &[&str] = &[
        "Acura",
        "Audi",
        "BMW",
        "Buick",
        "Cadillac",
        "Chevrolet",
        "Chrysler",
        "Dodge",
        "Ferrari",
        "Ford",
        "GMC",
        "Honda",
        "Hyundai",
        "Infiniti",
        "Jaguar",
        "Jeep",
        "Kia",
        "Lamborghini",
        "Lexus",
        "Lincoln",
        "Maserati",
        "Mazda",
        "Mercedes-Benz",
        "Mini",
        "Mitsubishi",
        "Nissan",
        "Porsche",
        "Ram",
        "Subaru",
        "Tesla",
        "Toyota",
        "Volkswagen",
        "Volvo",
    ];

    const ANIMALS: &[&str] = &[
        "ant",
        "bear",
        "cat",
        "dog",
        "elephant",
        "fox",
        "giraffe",
        "hippo",
        "iguana",
        "jaguar",
        "kangaroo",
        "lion",
        "monkey",
        "newt",
        "owl",
        "penguin",
        "quokka",
        "rabbit",
        "snake",
        "tiger",
        "unicorn",
        "vulture",
        "walrus",
        "xerus",
        "yak",
        "zebra",
        "alligator",
        "bison",
        "camel",
        "dolphin",
        "emu",
        "flamingo",
        "gorilla",
        "hedgehog",
        "ibex",
        "jellyfish",
        "koala",
        "lemur",
        "meerkat",
        "narwhal",
    ];

    const DESSERTS: &[&str] = &[
        "tiramisu",
        "cheesecake",
        "brownie",
        "gelato",
        "pie",
        "mousse",
        "baklava",
        "cannoli",
        "pavlova",
        "macarons",
        "flan",
        "churros",
        "trifle",
        "eclair",
        "profiteroles",
        "pudding",
        "cake",
        "tart",
        "affogato",
        "beignets",
        "souffle",
    ];
}
