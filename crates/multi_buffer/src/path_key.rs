use std::{ops::Range, rc::Rc, sync::Arc};

use gpui::{App, AppContext, Context, Entity};
use itertools::Itertools;
use language::{Buffer, BufferSnapshot};
use rope::Point;
use sum_tree::{Dimensions, SumTree};
use text::{Bias, BufferId, Edit, OffsetRangeExt, Patch};
use util::rel_path::RelPath;
use ztracing::instrument;

use crate::{
    Anchor, BufferState, BufferStateSnapshot, DiffChangeKind, Event, Excerpt, ExcerptOffset,
    ExcerptRange, ExcerptSummary, ExpandExcerptDirection, MultiBuffer, MultiBufferDimension,
    PathKeyIndex, ToOffset, build_excerpt_ranges,
};

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Hash, Debug)]
pub struct PathKey {
    // Used by the derived PartialOrd & Ord
    pub sort_prefix: Option<u64>,
    pub path: Arc<RelPath>,
}

impl PathKey {
    pub fn min() -> Self {
        Self {
            sort_prefix: None,
            path: RelPath::empty().into_arc(),
        }
    }

    pub fn sorted(sort_prefix: u64) -> Self {
        Self {
            sort_prefix: Some(sort_prefix),
            path: RelPath::empty().into_arc(),
        }
    }
    pub fn with_sort_prefix(sort_prefix: u64, path: Arc<RelPath>) -> Self {
        Self {
            sort_prefix: Some(sort_prefix),
            path,
        }
    }

    pub fn for_buffer(buffer: &Entity<Buffer>, cx: &App) -> Self {
        if let Some(file) = buffer.read(cx).file() {
            Self::with_sort_prefix(file.worktree_id(cx).to_proto(), file.path().clone())
        } else {
            Self {
                sort_prefix: None,
                path: RelPath::unix(&buffer.entity_id().to_string())
                    .unwrap()
                    .into_arc(),
            }
        }
    }
}

impl MultiBuffer {
    pub fn buffer_for_path(&self, path: &PathKey, cx: &App) -> Option<Entity<Buffer>> {
        let snapshot = self.snapshot(cx);
        let excerpt = snapshot.excerpts_for_path(path).next()?;
        self.buffer(excerpt.buffer_id)
    }

    pub fn location_for_path(&self, path: &PathKey, cx: &App) -> Option<Anchor> {
        let snapshot = self.snapshot(cx);
        let excerpt = snapshot.excerpts_for_path(path).next()?;
        Some(Anchor::in_buffer(
            excerpt.path_key_index,
            excerpt.range.context.start,
        ))
    }

    pub fn set_excerpts_for_buffer(
        &mut self,
        buffer: Entity<Buffer>,
        ranges: impl IntoIterator<Item = Range<Point>>,
        context_line_count: u32,
        cx: &mut Context<Self>,
    ) -> (Vec<Range<Anchor>>, bool) {
        let path = PathKey::for_buffer(&buffer, cx);
        self.set_excerpts_for_path(path, buffer, ranges, context_line_count, cx)
    }

    /// Sets excerpts, returns `true` if at least one new excerpt was added.
    #[instrument(skip_all)]
    pub fn set_excerpts_for_path(
        &mut self,
        path: PathKey,
        buffer: Entity<Buffer>,
        ranges: impl IntoIterator<Item = Range<Point>>,
        context_line_count: u32,
        cx: &mut Context<Self>,
    ) -> (Vec<Range<Anchor>>, bool) {
        let buffer_snapshot = buffer.read(cx).snapshot();
        let ranges: Vec<_> = ranges.into_iter().collect();
        let excerpt_ranges =
            build_excerpt_ranges(ranges.clone(), context_line_count, &buffer_snapshot);

        let merged = Self::merge_excerpt_ranges(&excerpt_ranges);
        let (inserted, path_key_index) =
            self.set_merged_excerpt_ranges_for_path(path, buffer, &buffer_snapshot, merged, cx);
        // todo!() move this into the callers that care
        let anchors = ranges
            .into_iter()
            .map(|range| {
                Anchor::range_in_buffer(path_key_index, buffer_snapshot.anchor_range_around(range))
            })
            .collect::<Vec<_>>();
        (anchors, inserted)
    }

    pub fn set_excerpt_ranges_for_path(
        &mut self,
        path: PathKey,
        buffer: Entity<Buffer>,
        buffer_snapshot: &BufferSnapshot,
        excerpt_ranges: Vec<ExcerptRange<Point>>,
        cx: &mut Context<Self>,
    ) -> (Vec<Range<Anchor>>, PathKeyIndex, bool) {
        let merged = Self::merge_excerpt_ranges(&excerpt_ranges);
        let (inserted, path_key_index) =
            self.set_merged_excerpt_ranges_for_path(path, buffer, buffer_snapshot, merged, cx);
        // todo!() move this into the callers that care
        let anchors = excerpt_ranges
            .into_iter()
            .map(|range| {
                Anchor::range_in_buffer(
                    path_key_index,
                    buffer_snapshot.anchor_range_around(range.primary),
                )
            })
            .collect::<Vec<_>>();
        (anchors, path_key_index, inserted)
    }

    pub fn set_anchored_excerpts_for_path(
        &self,
        path_key: PathKey,
        buffer: Entity<Buffer>,
        ranges: Vec<Range<text::Anchor>>,
        context_line_count: u32,
        cx: &Context<Self>,
    ) -> impl Future<Output = Vec<Range<Anchor>>> + use<> {
        let buffer_snapshot = buffer.read(cx).snapshot();
        let multi_buffer = cx.weak_entity();
        let mut app = cx.to_async();
        async move {
            let snapshot = buffer_snapshot.clone();
            let (ranges, merged_excerpt_ranges) = app
                .background_spawn(async move {
                    let point_ranges = ranges.iter().map(|range| range.to_point(&snapshot));
                    let excerpt_ranges =
                        build_excerpt_ranges(point_ranges, context_line_count, &snapshot);
                    let merged = Self::merge_excerpt_ranges(&excerpt_ranges);
                    (ranges, merged)
                })
                .await;

            multi_buffer
                .update(&mut app, move |multi_buffer, cx| {
                    let (_, path_key_index) = multi_buffer.set_merged_excerpt_ranges_for_path(
                        path_key,
                        buffer,
                        &buffer_snapshot,
                        merged_excerpt_ranges,
                        cx,
                    );
                    ranges
                        .into_iter()
                        .map(|range| Anchor::range_in_buffer(path_key_index, range))
                        .collect()
                })
                .ok()
                .unwrap_or_default()
        }
    }

    pub(super) fn expand_excerpts_with_paths(
        &mut self,
        anchors: impl IntoIterator<Item = Anchor>,
        line_count: u32,
        direction: ExpandExcerptDirection,
        cx: &mut Context<Self>,
    ) {
        let snapshot = self.snapshot(cx);
        let mut sorted_anchors = anchors
            .into_iter()
            .filter_map(|anchor| anchor.excerpt_anchor())
            .collect::<Vec<_>>();
        sorted_anchors.sort_by(|a, b| a.cmp(b, &snapshot));
        let buffers = sorted_anchors.into_iter().chunk_by(|anchor| anchor.path);
        let mut cursor = snapshot.excerpts.cursor::<ExcerptSummary>(());

        for (path_index, excerpt_anchors) in &buffers {
            let path = snapshot
                .path_keys_by_index
                .get(&path_index)
                .expect("anchor from wrong multibuffer");
            let Some((buffer, buffer_snapshot)) = cursor
                .item()
                .map(|excerpt| (excerpt.buffer(&self), excerpt.buffer_snapshot(&snapshot)))
            else {
                continue;
            };

            let mut excerpt_anchors = excerpt_anchors.peekable();
            let mut ranges = Vec::new();

            cursor.seek_forward(path, Bias::Left);
            while let Some(excerpt) = cursor.item()
                && &excerpt.path_key == path
            {
                let mut range = ExcerptRange {
                    context: excerpt.range.context.to_point(buffer_snapshot),
                    primary: excerpt.range.primary.to_point(buffer_snapshot),
                };

                let mut needs_expand = false;
                while excerpt_anchors.peek().is_some_and(|anchor| {
                    excerpt
                        .range
                        .contains(&anchor.text_anchor(), buffer_snapshot)
                }) {
                    needs_expand = true;
                    excerpt_anchors.next();
                }

                if needs_expand {
                    match direction {
                        ExpandExcerptDirection::Up => {
                            range.context.start.row =
                                range.context.start.row.saturating_sub(line_count);
                            range.context.start.column = 0;
                        }
                        ExpandExcerptDirection::Down => {
                            range.context.end.row = (range.context.end.row + line_count)
                                .min(excerpt.buffer_snapshot(&snapshot).max_point().row);
                            range.context.end.column = excerpt
                                .buffer_snapshot(&snapshot)
                                .line_len(range.context.end.row);
                        }
                        ExpandExcerptDirection::UpAndDown => {
                            range.context.start.row =
                                range.context.start.row.saturating_sub(line_count);
                            range.context.start.column = 0;
                            range.context.end.row = (range.context.end.row + line_count)
                                .min(excerpt.buffer_snapshot(&snapshot).max_point().row);
                            range.context.end.column = excerpt
                                .buffer_snapshot(&snapshot)
                                .line_len(range.context.end.row);
                        }
                    }
                }

                ranges.push(range);
                cursor.next();
            }

            self.set_excerpt_ranges_for_path(path.clone(), buffer, buffer_snapshot, ranges, cx);
        }
    }

    /// Sets excerpts, returns `true` if at least one new excerpt was added.
    pub(crate) fn set_merged_excerpt_ranges_for_path<T>(
        &mut self,
        path: PathKey,
        buffer: Entity<Buffer>,
        buffer_snapshot: &BufferSnapshot,
        new: Vec<ExcerptRange<T>>,
        cx: &mut Context<Self>,
    ) -> (bool, PathKeyIndex)
    where
        T: language::ToOffset,
    {
        let anchor_ranges = new
            .into_iter()
            .map(|r| ExcerptRange {
                context: buffer_snapshot.anchor_before(r.context.start)
                    ..buffer_snapshot.anchor_after(r.context.end),
                primary: buffer_snapshot.anchor_before(r.primary.start)
                    ..buffer_snapshot.anchor_after(r.primary.end),
            })
            .collect::<Vec<_>>();
        self.update_path_excerpts(path, buffer, buffer_snapshot, &anchor_ranges, cx)
    }

    fn get_or_create_path_key_index(&mut self, path_key: &PathKey) -> PathKeyIndex {
        let mut snapshot = self.snapshot.borrow_mut();
        let existing = snapshot
            .path_keys_by_index
            .iter()
            // todo!() perf? (but ExcerptIdMapping was doing this)
            .find(|(_, existing_path)| existing_path == &path_key)
            .map(|(index, _)| *index);

        if let Some(existing) = existing {
            return existing;
        }

        let index = snapshot
            .path_keys_by_index
            .last()
            .map(|(index, _)| PathKeyIndex(index.0 + 1))
            .unwrap_or(PathKeyIndex(0));
        snapshot.path_keys_by_index.insert(index, path_key.clone());
        index
    }

    // todo!() re-instate nonshrinking version for project diff / diagnostics
    pub fn update_path_excerpts<'a>(
        &mut self,
        path_key: PathKey,
        buffer: Entity<Buffer>,
        buffer_snapshot: &BufferSnapshot,
        to_insert: &Vec<ExcerptRange<text::Anchor>>,
        cx: &mut Context<Self>,
    ) -> (bool, PathKeyIndex) {
        let path_key_index = self.get_or_create_path_key_index(&path_key);
        if let Some(old_path_key) = self
            .snapshot(cx)
            .path_for_buffer(buffer_snapshot.remote_id())
            && old_path_key != &path_key
        {
            self.remove_excerpts_for_path(old_path_key.clone(), cx);
        }

        if to_insert.len() == 0 {
            self.remove_excerpts_for_path(path_key.clone(), cx);

            return (false, path_key_index);
        }
        assert_eq!(self.history.transaction_depth(), 0);
        self.sync_mut(cx);

        let buffer_id = buffer_snapshot.remote_id();

        let mut snapshot = self.snapshot.get_mut();
        let mut cursor = snapshot
            .excerpts
            .cursor::<Dimensions<PathKey, ExcerptOffset>>(());
        let mut new_excerpts = SumTree::new(());

        let new_ranges = to_insert.clone();
        let mut to_insert = to_insert.iter().peekable();
        let mut patch = Patch::empty();
        let mut added_new_excerpt = false;

        new_excerpts.append(cursor.slice(&path_key, Bias::Left), ());

        // handle the case where the path key used to be associated
        // with a different buffer by removing its excerpts.
        if let Some(excerpt) = cursor.item()
            && excerpt.path_key == path_key
            && excerpt.buffer_id != buffer_id
        {
            let before = cursor.position.1;
            cursor.seek_forward(&path_key, Bias::Right);
            let after = cursor.position.1;
            patch.push(Edit {
                old: before..after,
                new: new_excerpts.summary().len()..new_excerpts.summary().len(),
            });
        }

        while let Some(excerpt) = cursor.item()
            && excerpt.path_key == path_key
        {
            assert_eq!(excerpt.buffer_id, buffer_id);
            let Some(next_excerpt) = to_insert.peek() else {
                break;
            };
            if &excerpt.range == *next_excerpt {
                new_excerpts.push(excerpt.clone(), ());
                to_insert.next();
                cursor.next();
                continue;
            }

            if excerpt
                .range
                .context
                .start
                .cmp(&next_excerpt.context.start, &buffer_snapshot)
                .is_le()
            {
                // remove old excerpt
                let before = cursor.position.1;
                cursor.next();
                let after = cursor.position.1;
                patch.push(Edit {
                    old: before..after,
                    new: new_excerpts.summary().len()..new_excerpts.summary().len(),
                });
            } else {
                // insert new excerpt
                let next_excerpt = to_insert.next().unwrap();
                added_new_excerpt = true;
                let before = new_excerpts.summary().len();
                new_excerpts.update_last(|excerpt| excerpt.has_trailing_newline = true, ());
                new_excerpts.push(
                    Excerpt::new(
                        path_key.clone(),
                        path_key_index,
                        &buffer_snapshot,
                        next_excerpt.clone(),
                        to_insert.peek().is_some() || cursor.item().is_some(),
                    ),
                    (),
                );
                let after = new_excerpts.summary().len();
                patch.push_maybe_empty(Edit {
                    old: cursor.position.1..cursor.position.1,
                    new: before..after,
                });
            }
        }

        // remove any further trailing excerpts
        let before = cursor.position.1;
        cursor.seek_forward(&path_key, Bias::Right);
        let after = cursor.position.1;
        patch.push(Edit {
            old: before..after,
            new: new_excerpts.summary().len()..new_excerpts.summary().len(),
        });

        while let Some(next_excerpt) = to_insert.next() {
            added_new_excerpt = true;
            let before = new_excerpts.summary().len();
            new_excerpts.update_last(|excerpt| excerpt.has_trailing_newline = true, ());
            new_excerpts.push(
                Excerpt::new(
                    path_key.clone(),
                    path_key_index,
                    &buffer_snapshot,
                    next_excerpt.clone(),
                    to_insert.peek().is_some() || cursor.item().is_some(),
                ),
                (),
            );
            let after = new_excerpts.summary().len();
            patch.push_maybe_empty(Edit {
                old: cursor.position.1..cursor.position.1,
                new: before..after,
            });
        }

        let suffix = cursor.suffix();
        let changed_trailing_excerpt = suffix.is_empty();
        new_excerpts.append(suffix, ());
        drop(cursor);
        snapshot.excerpts = new_excerpts;
        snapshot.buffers.insert(
            buffer_id,
            BufferStateSnapshot {
                path_key: path_key.clone(),
                buffer_snapshot: buffer_snapshot.clone(),
            },
        );

        self.buffers.entry(buffer_id).or_insert_with(|| {
            self.buffer_changed_since_sync.replace(true);
            buffer.update(cx, |buffer, _| {
                buffer.record_changes(Rc::downgrade(&self.buffer_changed_since_sync));
            });
            BufferState {
                _subscriptions: [
                    cx.observe(&buffer, |_, _, cx| cx.notify()),
                    cx.subscribe(&buffer, Self::on_buffer_event),
                ],
                buffer: buffer.clone(),
            }
        });

        if changed_trailing_excerpt {
            snapshot.trailing_excerpt_update_count += 1;
        }

        let edits = Self::sync_diff_transforms(
            &mut snapshot,
            dbg!(patch.into_inner()),
            DiffChangeKind::BufferEdited,
        );
        if !edits.is_empty() {
            self.subscriptions.publish(edits);
        }

        cx.emit(Event::Edited {
            edited_buffer: None,
        });
        cx.emit(Event::BufferUpdated {
            buffer,
            path_key: path_key.clone(),
            ranges: new_ranges,
        });
        cx.notify();

        (added_new_excerpt, path_key_index)
    }

    pub fn remove_excerpts_for_buffer(&mut self, buffer: BufferId, cx: &mut Context<Self>) {
        let snapshot = self.sync_mut(cx);
        let Some(path) = snapshot.path_for_buffer(buffer).cloned() else {
            return;
        };
        self.remove_excerpts_for_path(path, cx);
    }

    pub fn remove_excerpts_for_path(&mut self, path: PathKey, cx: &mut Context<Self>) {
        assert_eq!(self.history.transaction_depth(), 0);
        self.sync_mut(cx);

        let mut snapshot = self.snapshot.get_mut();
        let mut cursor = snapshot
            .excerpts
            .cursor::<Dimensions<PathKey, ExcerptOffset>>(());
        let mut new_excerpts = SumTree::new(());
        new_excerpts.append(cursor.slice(&path, Bias::Left), ());
        let edit_start = cursor.position.1;
        let mut buffer_id = None;
        if let Some(excerpt) = cursor.item()
            && excerpt.path_key == path
        {
            buffer_id = Some(excerpt.buffer_id);
        }
        cursor.seek(&path, Bias::Right);
        let edit_end = cursor.position.1;
        let suffix = cursor.suffix();
        let changed_trailing_excerpt = suffix.is_empty();
        new_excerpts.append(suffix, ());

        let edit = Edit {
            old: edit_start..edit_end,
            new: edit_start..edit_start,
        };

        if let Some(buffer_id) = buffer_id {
            snapshot.buffers.remove(&buffer_id);
            self.buffers.remove(&buffer_id);
            cx.emit(Event::BuffersRemoved {
                removed_buffer_ids: vec![buffer_id],
            })
        }
        drop(cursor);
        snapshot.excerpts = new_excerpts;
        if changed_trailing_excerpt {
            snapshot.trailing_excerpt_update_count += 1;
        }

        let edits =
            Self::sync_diff_transforms(&mut snapshot, vec![edit], DiffChangeKind::BufferEdited);
        if !edits.is_empty() {
            self.subscriptions.publish(edits);
        }

        cx.emit(Event::Edited {
            edited_buffer: None,
        });
        cx.notify();
    }
}
