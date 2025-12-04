use std::{mem, ops::Range, sync::Arc};

use collections::HashSet;
use gpui::{App, AppContext, Context, Entity};
use itertools::Itertools;
use language::{Buffer, BufferSnapshot};
use rope::Point;
use text::{Bias, BufferId, OffsetRangeExt, locator::Locator};
use util::{post_inc, rel_path::RelPath};

use crate::{
    Anchor, ExcerptId, ExcerptRange, ExpandExcerptDirection, MultiBuffer, build_excerpt_ranges,
};

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Hash, Debug)]
pub struct PathKey {
    // Used by the derived PartialOrd & Ord
    pub sort_prefix: Option<u64>,
    pub path: Arc<RelPath>,
}

impl PathKey {
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
    pub fn paths(&self) -> impl Iterator<Item = PathKey> + '_ {
        self.excerpts_by_path.keys().cloned()
    }

    pub fn remove_excerpts_for_path(&mut self, path: PathKey, cx: &mut Context<Self>) {
        if let Some(to_remove) = self.excerpts_by_path.remove(&path) {
            self.remove_excerpts(to_remove, cx)
        }
        if let Some(follower) = &self.follower {
            follower.update(cx, |follower, cx| {
                follower.remove_excerpts_for_path(path, cx);
            });
        }
    }

    pub fn location_for_path(&self, path: &PathKey, cx: &App) -> Option<Anchor> {
        let excerpt_id = self.excerpts_by_path.get(path)?.first()?;
        let snapshot = self.read(cx);
        let excerpt = snapshot.excerpt(*excerpt_id)?;
        Some(Anchor::in_buffer(excerpt.id, excerpt.range.context.start))
    }

    pub fn excerpt_paths(&self) -> impl Iterator<Item = &PathKey> {
        self.excerpts_by_path.keys()
    }

    /// Sets excerpts, returns `true` if at least one new excerpt was added.
    pub fn set_excerpts_for_path(
        &mut self,
        path: PathKey,
        buffer: Entity<Buffer>,
        ranges: impl IntoIterator<Item = Range<Point>>,
        context_line_count: u32,
        cx: &mut Context<Self>,
    ) -> (Vec<Range<Anchor>>, bool) {
        let buffer_snapshot = buffer.read(cx).snapshot();
        let excerpt_ranges = build_excerpt_ranges(ranges, context_line_count, &buffer_snapshot);

        let (new, counts) = Self::merge_excerpt_ranges(&excerpt_ranges);
        self.set_merged_excerpt_ranges_for_path(
            path,
            buffer,
            excerpt_ranges,
            &buffer_snapshot,
            new,
            counts,
            cx,
        )
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
        self.set_merged_excerpt_ranges_for_path(
            path,
            buffer,
            excerpt_ranges,
            buffer_snapshot,
            new,
            counts,
            cx,
        )
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
            let (excerpt_ranges, new, counts) = app
                .background_spawn(async move {
                    let ranges = ranges.into_iter().map(|range| range.to_point(&snapshot));
                    let excerpt_ranges =
                        build_excerpt_ranges(ranges, context_line_count, &snapshot);
                    let (new, counts) = Self::merge_excerpt_ranges(&excerpt_ranges);
                    (excerpt_ranges, new, counts)
                })
                .await;

            multi_buffer
                .update(&mut app, move |multi_buffer, cx| {
                    let (ranges, _) = multi_buffer.set_merged_excerpt_ranges_for_path(
                        path_key,
                        buffer,
                        excerpt_ranges,
                        &buffer_snapshot,
                        new,
                        counts,
                        cx,
                    );
                    ranges
                })
                .ok()
                .unwrap_or_default()
        }
    }

    pub fn remove_excerpts_for_buffer(&mut self, buffer: BufferId, cx: &mut Context<Self>) {
        self.remove_excerpts(
            self.excerpts_for_buffer(buffer, cx)
                .into_iter()
                .map(|(excerpt, _)| excerpt),
            cx,
        );
    }

    pub(super) fn expand_excerpts_with_paths(
        &mut self,
        ids: impl IntoIterator<Item = ExcerptId>,
        line_count: u32,
        direction: ExpandExcerptDirection,
        cx: &mut Context<Self>,
    ) {
        let grouped = ids
            .into_iter()
            .chunk_by(|id| self.paths_by_excerpt.get(id).cloned())
            .into_iter()
            .filter_map(|(k, v)| Some((k?, v.into_iter().collect::<Vec<_>>())))
            .collect::<Vec<_>>();
        let snapshot = self.snapshot(cx);

        for (path, ids) in grouped.into_iter() {
            let Some(excerpt_ids) = self.excerpts_by_path.get(&path) else {
                continue;
            };

            let ids_to_expand = HashSet::from_iter(ids);
            let mut excerpt_id_ = None;
            let expanded_ranges = excerpt_ids.iter().filter_map(|excerpt_id| {
                let excerpt = snapshot.excerpt(*excerpt_id)?;
                let excerpt_id = excerpt.id;
                if excerpt_id_.is_none() {
                    excerpt_id_ = Some(excerpt_id);
                }

                let mut context = excerpt.range.context.to_point(&excerpt.buffer);
                if ids_to_expand.contains(&excerpt_id) {
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
                }

                Some(ExcerptRange {
                    context,
                    primary: excerpt.range.primary.to_point(&excerpt.buffer),
                })
            });
            let mut merged_ranges: Vec<ExcerptRange<Point>> = Vec::new();
            for range in expanded_ranges {
                if let Some(last_range) = merged_ranges.last_mut()
                    && last_range.context.end >= range.context.start
                {
                    last_range.context.end = range.context.end;
                    continue;
                }
                merged_ranges.push(range)
            }
            let Some(excerpt_id) = excerpt_id_ else {
                continue;
            };
            let Some(buffer_id) = &snapshot.buffer_id_for_excerpt(excerpt_id) else {
                continue;
            };

            let Some(buffer) = self.buffers.get(buffer_id).map(|b| b.buffer.clone()) else {
                continue;
            };

            let buffer_snapshot = buffer.read(cx).snapshot();
            self.update_path_excerpts(path.clone(), buffer, &buffer_snapshot, merged_ranges, cx);
        }
    }

    /// Sets excerpts, returns `true` if at least one new excerpt was added.
    fn set_merged_excerpt_ranges_for_path(
        &mut self,
        path: PathKey,
        buffer: Entity<Buffer>,
        ranges: Vec<ExcerptRange<Point>>,
        buffer_snapshot: &BufferSnapshot,
        new: Vec<ExcerptRange<Point>>,
        counts: Vec<usize>,
        cx: &mut Context<Self>,
    ) -> (Vec<Range<Anchor>>, bool) {
        let (excerpt_ids, added_a_new_excerpt) =
            self.update_path_excerpts(path, buffer, buffer_snapshot, new, cx);

        let mut result = Vec::new();
        let mut ranges = ranges.into_iter();
        for (excerpt_id, range_count) in excerpt_ids.into_iter().zip(counts.into_iter()) {
            for range in ranges.by_ref().take(range_count) {
                let range = Anchor::range_in_buffer(
                    excerpt_id,
                    buffer_snapshot.anchor_before(&range.primary.start)
                        ..buffer_snapshot.anchor_after(&range.primary.end),
                );
                result.push(range)
            }
        }
        (result, added_a_new_excerpt)
    }

    fn update_path_excerpts(
        &mut self,
        path: PathKey,
        buffer: Entity<Buffer>,
        buffer_snapshot: &BufferSnapshot,
        new: Vec<ExcerptRange<Point>>,
        cx: &mut Context<Self>,
    ) -> (Vec<ExcerptId>, bool) {
        let mut insert_after = self
            .excerpts_by_path
            .range(..path.clone())
            .next_back()
            .and_then(|(_, value)| value.last().copied())
            .unwrap_or(ExcerptId::min());

        let existing = self
            .excerpts_by_path
            .get(&path)
            .cloned()
            .unwrap_or_default();
        let mut new_iter = new.into_iter().peekable();
        let mut existing_iter = existing.into_iter().peekable();

        let mut excerpt_ids = Vec::new();
        let mut to_remove = Vec::new();
        let mut to_insert: Vec<(ExcerptId, ExcerptRange<Point>)> = Vec::new();
        let mut added_a_new_excerpt = false;
        let snapshot = self.snapshot(cx);

        let mut next_excerpt_id =
            // is this right? What if we remove the last excerpt, then we might reallocate with a wrong mapping?
            if let Some(last_entry) = self.snapshot.borrow().excerpt_ids.last() {
                last_entry.id.0 + 1
            } else {
                1
            };

        let mut next_excerpt_id = move || ExcerptId(post_inc(&mut next_excerpt_id));

        let mut excerpts_cursor = snapshot.excerpts.cursor::<Option<&Locator>>(());
        excerpts_cursor.next();

        loop {
            let existing = if let Some(&existing_id) = existing_iter.peek() {
                let locator = snapshot.excerpt_locator_for_id(existing_id);
                excerpts_cursor.seek_forward(&Some(locator), Bias::Left);
                if let Some(excerpt) = excerpts_cursor.item() {
                    if excerpt.buffer_id != buffer_snapshot.remote_id() {
                        to_remove.push(existing_id);
                        existing_iter.next();
                        continue;
                    }
                    Some((existing_id, excerpt.range.context.to_point(buffer_snapshot)))
                } else {
                    None
                }
            } else {
                None
            };

            let new = new_iter.peek();
            if let Some((last_id, last)) = to_insert.last_mut() {
                if let Some(new) = new
                    && last.context.end >= new.context.start
                {
                    last.context.end = last.context.end.max(new.context.end);
                    excerpt_ids.push(*last_id);
                    new_iter.next();
                    continue;
                }
                if let Some((existing_id, existing_range)) = &existing
                    && last.context.end >= existing_range.start
                {
                    last.context.end = last.context.end.max(existing_range.end);
                    to_remove.push(*existing_id);
                    self.snapshot
                        .get_mut()
                        .replaced_excerpts
                        .insert(*existing_id, *last_id);
                    existing_iter.next();
                    continue;
                }
            }

            match (new, existing) {
                (None, None) => break,
                (None, Some((existing_id, _))) => {
                    existing_iter.next();
                    to_remove.push(existing_id);
                    continue;
                }
                (Some(_), None) => {
                    added_a_new_excerpt = true;
                    let new_id = next_excerpt_id();
                    excerpt_ids.push(new_id);
                    to_insert.push((new_id, new_iter.next().unwrap()));
                    continue;
                }
                (Some(new), Some((_, existing_range))) => {
                    if existing_range.end < new.context.start {
                        let existing_id = existing_iter.next().unwrap();
                        to_remove.push(existing_id);
                        continue;
                    } else if existing_range.start > new.context.end {
                        let new_id = next_excerpt_id();
                        excerpt_ids.push(new_id);
                        to_insert.push((new_id, new_iter.next().unwrap()));
                        continue;
                    }

                    if existing_range.start == new.context.start
                        && existing_range.end == new.context.end
                    {
                        self.insert_excerpts_with_ids_after(
                            insert_after,
                            buffer.clone(),
                            mem::take(&mut to_insert),
                            cx,
                        );
                        insert_after = existing_iter.next().unwrap();
                        excerpt_ids.push(insert_after);
                        new_iter.next();
                    } else {
                        let existing_id = existing_iter.next().unwrap();
                        let new_id = next_excerpt_id();
                        self.snapshot
                            .get_mut()
                            .replaced_excerpts
                            .insert(existing_id, new_id);
                        to_remove.push(existing_id);
                        let mut range = new_iter.next().unwrap();
                        range.context.start = range.context.start.min(existing_range.start);
                        range.context.end = range.context.end.max(existing_range.end);
                        excerpt_ids.push(new_id);
                        to_insert.push((new_id, range));
                    }
                }
            };
        }

        self.insert_excerpts_with_ids_after(insert_after, buffer, to_insert, cx);
        // todo(lw): There is a logic bug somewhere that causes the to_remove vector to be not ordered correctly
        to_remove.sort_by_cached_key(|&id| snapshot.excerpt_locator_for_id(id));
        self.remove_excerpts(to_remove, cx);

        if excerpt_ids.is_empty() {
            self.excerpts_by_path.remove(&path);
        } else {
            for excerpt_id in &excerpt_ids {
                self.paths_by_excerpt.insert(*excerpt_id, path.clone());
            }
            let snapshot = &*self.snapshot.get_mut();
            let mut excerpt_ids: Vec<_> = excerpt_ids.iter().dedup().cloned().collect();
            excerpt_ids.sort_by_cached_key(|&id| snapshot.excerpt_locator_for_id(id));
            self.excerpts_by_path.insert(path, excerpt_ids);
        }

        (excerpt_ids, added_a_new_excerpt)
    }
}
