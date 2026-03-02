use std::{
    cell::{Cell, RefCell},
    ops::Range,
    rc::Rc,
    sync::Arc,
};

use collections::HashSet;
use gpui::{App, AppContext, Context, Entity};
use itertools::Itertools;
use language::{Buffer, BufferSnapshot};
use rope::Point;
use sum_tree::{Dimensions, SumTree};
use text::{Bias, BufferId, Edit, OffsetRangeExt, Patch};
use util::rel_path::RelPath;
use ztracing::instrument;

use crate::{
    Anchor, BufferState, DiffChangeKind, Event, Excerpt, ExcerptOffset, ExcerptRange,
    ExcerptSummary, ExpandExcerptDirection, MultiBuffer, MultiBufferOffset, PathKeyIndex,
    build_excerpt_ranges,
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
    pub fn paths(&self) -> impl Iterator<Item = &PathKey> + '_ {
        self.excerpts_by_path.keys()
    }

    pub fn excerpts_for_path(&self, path: &PathKey) -> impl '_ + Iterator<Item = ExcerptId> {
        self.excerpts_by_path
            .get(path)
            .map(|excerpts| excerpts.as_slice())
            .unwrap_or_default()
            .iter()
            .copied()
    }

    pub fn buffer_for_path(&self, path: &PathKey, cx: &App) -> Option<Entity<Buffer>> {
        let excerpt_id = self.excerpts_by_path.get(path)?.first()?;
        let snapshot = self.read(cx);
        let excerpt = snapshot.excerpt(*excerpt_id)?;
        self.buffer(excerpt.buffer_id)
    }

    pub fn location_for_path(&self, path: &PathKey, cx: &App) -> Option<Anchor> {
        let excerpt_id = self.excerpts_by_path.get(path)?.first()?;
        let snapshot = self.read(cx);
        let excerpt = snapshot.excerpt(*excerpt_id)?;
        Some(Anchor::text(excerpt.id, excerpt.range.context.start))
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

        let (new, _) = Self::merge_excerpt_ranges(&excerpt_ranges);
        let inserted =
            self.set_merged_excerpt_ranges_for_path(path, buffer, &buffer_snapshot, new, cx);
        // todo!() move this into the callers that care
        let anchors = ranges
            .into_iter()
            .map(|range| Anchor::range_in_buffer(buffer_snapshot.anchor_range_around(range)))
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
    ) -> (Vec<Range<Anchor>>, bool) {
        let (new, counts) = Self::merge_excerpt_ranges(&excerpt_ranges);
        let inserted =
            self.set_merged_excerpt_ranges_for_path(path, buffer, buffer_snapshot, new, cx);
        // todo!() move this into the callers that care
        let anchors = excerpt_ranges
            .into_iter()
            .map(|range| {
                Anchor::range_in_buffer(buffer_snapshot.anchor_range_around(range.primary))
            })
            .collect::<Vec<_>>();
        (anchors, inserted)
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
                    let (new, _) = Self::merge_excerpt_ranges(&excerpt_ranges);
                    (ranges, new)
                })
                .await;

            multi_buffer
                .update(&mut app, move |multi_buffer, cx| {
                    multi_buffer.set_merged_excerpt_ranges_for_path(
                        path_key,
                        buffer,
                        &buffer_snapshot,
                        merged_excerpt_ranges,
                        cx,
                    );
                    ranges
                        .into_iter()
                        .map(|range| Anchor::range_in_buffer(range))
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
        let mut sorted_anchors = anchors.into_iter().collect::<Vec<_>>();
        sorted_anchors.sort_by(|a, b| a.cmp(b, &snapshot));
        let mut cursor = snapshot.excerpts.cursor::<ExcerptSummary>(());
        let mut sorted_anchors = sorted_anchors.into_iter().peekable();
        while let Some(anchor) = sorted_anchors.next() {
            let path = snapshot.path_for_anchor(anchor);
            let Some(buffer) = self.buffer_for_path(&path, cx) else {
                continue;
            };
            let buffer_snapshot = buffer.read(cx).snapshot();

            let mut expanded_ranges = Vec::new();
            // Move to the first excerpt for this path
            cursor.seek_forward(&path, Bias::Left);
            while let Some(anchor) = sorted_anchors.peek().copied()
                && snapshot.path_for_anchor(anchor) == path
            {
                sorted_anchors.next();
                let Some(target) = snapshot.anchor_seek_target(anchor) else {
                    continue;
                };
                // Move to the next excerpt to be expanded, and push unchanged ranges for intervening excerpts
                expanded_ranges.extend(
                    cursor
                        .slice(&target, Bias::Left)
                        .iter()
                        .map(|excerpt| excerpt.range.clone()),
                );
                let Some(excerpt) = cursor.item() else {
                    continue;
                };
                if excerpt.path_key != path {
                    continue;
                }
                // Expand the range for this excerpt
                let mut context = excerpt.range.context.to_point(&buffer_snapshot);
                match direction {
                    ExpandExcerptDirection::Up => {
                        context.start.row = context.start.row.saturating_sub(line_count);
                        context.start.column = 0;
                    }
                    ExpandExcerptDirection::Down => {
                        context.end.row =
                            (context.end.row + line_count).min(excerpt.buffer.max_point().row);
                        context.end.column = excerpt.buffer.line_len(context.end.row);
                    }
                    ExpandExcerptDirection::UpAndDown => {
                        context.start.row = context.start.row.saturating_sub(line_count);
                        context.start.column = 0;
                        context.end.row =
                            (context.end.row + line_count).min(excerpt.buffer.max_point().row);
                        context.end.column = excerpt.buffer.line_len(context.end.row);
                    }
                }
                let context = excerpt.buffer.anchor_range_around(context);
                expanded_ranges.push(ExcerptRange {
                    context,
                    primary: excerpt.range.primary.clone(),
                });
                cursor.next();
            }

            // Add unchanged ranges for this path after the last expanded excerpt
            while let Some(excerpt) = cursor.item()
                && excerpt.path_key == path
            {
                expanded_ranges.push(excerpt.range.clone());
                cursor.next();
            }

            let mut merged_ranges: Vec<ExcerptRange<text::Anchor>> = Vec::new();
            for range in expanded_ranges {
                if let Some(last_range) = merged_ranges.last_mut()
                    && last_range
                        .context
                        .end
                        .cmp(&range.context.start, &buffer_snapshot)
                        .is_ge()
                {
                    last_range.context.end = range.context.end;
                    continue;
                }
                merged_ranges.push(range)
            }
            self.update_path_excerpts(path.clone(), buffer, &buffer_snapshot, &merged_ranges, cx);
        }
    }

    /// Sets excerpts, returns `true` if at least one new excerpt was added.
    fn set_merged_excerpt_ranges_for_path(
        &mut self,
        path: PathKey,
        buffer: Entity<Buffer>,
        buffer_snapshot: &BufferSnapshot,
        new: Vec<ExcerptRange<Point>>,
        cx: &mut Context<Self>,
    ) -> bool {
        let anchor_ranges = new
            .into_iter()
            .map(|r| ExcerptRange {
                context: buffer_snapshot.anchor_range_around(r.context),
                primary: buffer_snapshot.anchor_range_around(r.primary),
            })
            .collect::<Vec<_>>();
        self.update_path_excerpts(path, buffer, buffer_snapshot, &anchor_ranges, cx)
    }

    pub fn update_path_excerpts<'a>(
        &mut self,
        path_key: PathKey,
        buffer: Entity<Buffer>,
        buffer_snapshot: &BufferSnapshot,
        to_insert: &Vec<ExcerptRange<text::Anchor>>,
        cx: &mut Context<Self>,
    ) -> bool {
        if to_insert.len() == 0 {
            self.remove_excerpts_for_path(path_key.clone(), cx);
            if let Some(old_path_key) = self
                .snapshot(cx)
                .path_for_buffer(buffer_snapshot.remote_id())
                && old_path_key != &path_key
            {
                self.remove_excerpts_for_path(old_path_key.clone(), cx);
            }

            return false;
        }
        assert_eq!(self.history.transaction_depth(), 0);
        self.sync_mut(cx);

        let buffer_snapshot = buffer.read(cx).snapshot();
        let buffer_id = buffer_snapshot.remote_id();
        let buffer_state = self.buffers.entry(buffer_id).or_insert_with(|| {
            self.buffer_changed_since_sync.replace(true);
            buffer.update(cx, |buffer, _| {
                buffer.record_changes(Rc::downgrade(&self.buffer_changed_since_sync));
            });
            BufferState {
                last_version: RefCell::new(buffer_snapshot.version().clone()),
                last_non_text_state_update_count: Cell::new(
                    buffer_snapshot.non_text_state_update_count(),
                ),
                _subscriptions: [
                    cx.observe(&buffer, |_, _, cx| cx.notify()),
                    cx.subscribe(&buffer, Self::on_buffer_event),
                ],
                buffer: buffer.clone(),
            }
        });

        let mut snapshot = self.snapshot.get_mut();
        let mut cursor = snapshot
            .excerpts
            .cursor::<Dimensions<PathKey, ExcerptOffset>>(());
        let mut new_excerpts = SumTree::new(());

        let mut to_insert = to_insert.iter().peekable();
        let mut patch = Patch::empty();
        let mut added_new_excerpt = false;

        let path_key_index = snapshot
            .path_keys_by_index
            .iter()
            // todo!() perf? (but ExcerptIdMapping was doing this)
            .find(|(_, existing_path)| existing_path == &&path_key)
            .map(|(index, _)| *index);
        let path_key_index = path_key_index.unwrap_or_else(|| {
            let index = snapshot
                .path_keys_by_index
                .last()
                .map(|(index, _)| PathKeyIndex(index.0 + 1))
                .unwrap_or(PathKeyIndex(0));
            snapshot.path_keys_by_index.insert(index, path_key.clone());
            index
        });
        let old_path_key = snapshot
            .path_keys_by_buffer
            .insert_or_replace(buffer_id, path_key.clone());
        // handle the case where the buffer's path key has changed by
        // removing any old excerpts for the buffer
        if let Some(old_path_key) = &old_path_key
            && old_path_key < &path_key
        {
            new_excerpts.append(cursor.slice(old_path_key, Bias::Left), ());
            let before = cursor.position.1;
            cursor.seek_forward(old_path_key, Bias::Right);
            let after = cursor.position.1;
            patch.push(Edit {
                old: before..after,
                new: new_excerpts.summary().len()..new_excerpts.summary().len(),
            });
        }

        new_excerpts.append(cursor.slice(&path_key, Bias::Left), ());

        // handle the case where the path key used to be associated
        // with a different buffer by removing its excerpts.
        if let Some(excerpt) = cursor.item()
            && excerpt.path_key == path_key
            && excerpt.buffer.remote_id() != buffer_id
        {
            let before = cursor.position.1;
            cursor.seek_forward(&path_key, Bias::Right);
            let after = cursor.position.1;
            patch.push(Edit {
                old: before..after,
                new: new_excerpts.summary().len()..new_excerpts.summary().len(),
            });
        }

        let buffer_snapshot = Arc::new(buffer_snapshot);
        while let Some(excerpt) = cursor.item()
            && excerpt.path_key == path_key
        {
            assert_eq!(excerpt.buffer.remote_id(), buffer_id);
            let Some(next_excerpt) = to_insert.peek() else {
                break;
            };
            if &&excerpt.range == next_excerpt {
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
                new_excerpts.push(
                    Excerpt::new(
                        path_key.clone(),
                        buffer_snapshot.clone(),
                        next_excerpt.clone(),
                        to_insert.peek().is_some(),
                    ),
                    (),
                );
                let after = new_excerpts.summary().len();
                patch.push(Edit {
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

        // handle the case where the buffer's path key has changed by
        // removing any old excerpts for the buffer
        if let Some(old_path_key) = &old_path_key
            && old_path_key > &path_key
        {
            new_excerpts.append(cursor.slice(old_path_key, Bias::Left), ());
            let before = cursor.position.1;
            cursor.seek_forward(old_path_key, Bias::Right);
            let after = cursor.position.1;
            patch.push(Edit {
                old: before..after,
                new: new_excerpts.summary().len()..new_excerpts.summary().len(),
            });
        }

        let suffix = cursor.suffix();
        let changed_trailing_excerpt = suffix.is_empty();
        new_excerpts.append(suffix, ());
        let new_ranges = snapshot
            .excerpt_ranges_for_path(&path_key)
            .map(|range| range.context)
            .collect();
        drop(cursor);
        snapshot.excerpts = new_excerpts;
        if changed_trailing_excerpt {
            snapshot.trailing_excerpt_update_count += 1;
        }

        let edits = Self::sync_diff_transforms(
            &mut snapshot,
            patch.into_inner(),
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

        added_new_excerpt
    }

    pub fn remove_excerpts_for_path(&mut self, path: PathKey, cx: &mut Context<Self>) {
        let mut patch = Patch::empty();

        let mut snapshot = self.snapshot.get_mut();
        let mut cursor = snapshot
            .excerpts
            .cursor::<Dimensions<PathKey, ExcerptOffset>>(());
        let mut new_excerpts = SumTree::new(());

        if let Some(old_path_key) = old_path_key
            && old_path_key < path_key
        {
            new_excerpts.append(cursor.slice(&old_path_key, Bias::Left), ());
            let before = cursor.position.1;
            cursor.seek_forward(&old_path_key, Bias::Right);
            let after = cursor.position.1;
            patch.push(Edit {
                old: before..after,
                new: new_excerpts.summary().len()..new_excerpts.summary().len(),
            });
        }

        let suffix = cursor.suffix();
        let changed_trailing_excerpt = suffix.is_empty();
        new_excerpts.append(suffix, ());

        for buffer_id in removed_excerpts_for_buffers {
            match self.buffers.get(&buffer_id) {
                Some(buffer_state) => {
                    snapshot
                        .buffer_locators
                        .insert(buffer_id, buffer_state.excerpts.iter().cloned().collect());
                }
                None => {
                    snapshot.buffer_locators.remove(&buffer_id);
                }
            }
        }
        snapshot.excerpts = new_excerpts;
        if changed_trailing_excerpt {
            snapshot.trailing_excerpt_update_count += 1;
        }

        let edits = Self::sync_diff_transforms(
            &mut snapshot,
            patch.into_inner(),
            DiffChangeKind::BufferEdited,
        );
        if !edits.is_empty() {
            self.subscriptions.publish(edits);
        }

        cx.emit(Event::Edited {
            edited_buffer: None,
        });
        // todo!() is this right? different event?
        cx.emit(Event::BufferUpdated {
            buffer,
            path_key: path.clone(),
            ranges: Vec::new(),
        });
        cx.notify();
    }
}
