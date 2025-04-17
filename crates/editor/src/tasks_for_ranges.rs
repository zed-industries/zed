use std::ops::Range;

use gpui::{Entity, Task};
use language::{Buffer, BufferSnapshot};
use multi_buffer::{ExcerptId, MultiBuffer};
use text::{Bias, BufferId, ToOffset};
use ui::Context;

#[derive(Debug, Clone)]
pub struct QueryRanges {
    pub before_visible: Vec<Range<language::Anchor>>,
    pub visible: Vec<Range<language::Anchor>>,
    pub after_visible: Vec<Range<language::Anchor>>,
}

/// A logic to apply when querying for new semantic tokens or inlay hints and deciding what to do with the old entries in the cache in case of conflicts.
#[derive(Debug, Clone, Copy)]
pub(super) enum InvalidationStrategy {
    /// Hints reset is <a href="https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#workspace_inlayHint_refresh">requested</a> by the LSP server.
    /// Demands to re-query all inlay hints needed and invalidate all cached entries, but does not require instant update with invalidation.
    ///
    /// Despite nothing forbids language server from sending this request on every edit, it is expected to be sent only when certain internal server state update, invisible for the editor otherwise.
    RefreshRequested,
    /// Multibuffer excerpt(s) and/or singleton buffer(s) were edited at least on one place.
    /// Neither editor nor LSP is able to tell which open file hints' are not affected, so all of them have to be invalidated, re-queried and do that fast enough to avoid being slow, but also debounce to avoid loading hints on every fast keystroke sequence.
    BufferEdited,
    /// A new file got opened/new excerpt was added to a multibuffer/a [multi]buffer was scrolled to a new position.
    /// No invalidation should be done at all, all new hints are added to the cache.
    ///
    /// A special case is the settings change: in addition to LSP capabilities, Zed allows omitting certain hint kinds (defined by the corresponding LSP part: type/parameter/other).
    /// This does not lead to cache invalidation, but would require cache usage for determining which hints are not displayed and issuing an update to inlays on the screen.
    None,
}

impl InvalidationStrategy {
    pub fn should_invalidate(&self) -> bool {
        matches!(
            self,
            InvalidationStrategy::RefreshRequested | InvalidationStrategy::BufferEdited
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ExcerptQuery {
    pub buffer_id: BufferId,
    pub excerpt_id: ExcerptId,
    pub cache_version: usize,
    pub invalidate: InvalidationStrategy,
    pub reason: &'static str,
}

#[derive(Debug)]
pub struct TasksForRanges {
    tasks: Vec<Task<()>>,
    sorted_ranges: Vec<Range<language::Anchor>>,
}

impl TasksForRanges {
    pub fn new(query_ranges: QueryRanges, task: Task<()>) -> Self {
        Self {
            tasks: vec![task],
            sorted_ranges: query_ranges.into_sorted_query_ranges(),
        }
    }

    pub fn update_cached_tasks(
        &mut self,
        buffer_snapshot: &BufferSnapshot,
        query_ranges: QueryRanges,
        invalidate: InvalidationStrategy,
        spawn_task: impl FnOnce(QueryRanges) -> Task<()>,
    ) {
        let query_ranges = if invalidate.should_invalidate() {
            self.tasks.clear();
            self.sorted_ranges = query_ranges.clone().into_sorted_query_ranges();
            query_ranges
        } else {
            let mut non_cached_query_ranges = query_ranges;
            non_cached_query_ranges.before_visible = non_cached_query_ranges
                .before_visible
                .into_iter()
                .flat_map(|query_range| {
                    self.remove_cached_ranges_from_query(buffer_snapshot, query_range)
                })
                .collect();
            non_cached_query_ranges.visible = non_cached_query_ranges
                .visible
                .into_iter()
                .flat_map(|query_range| {
                    self.remove_cached_ranges_from_query(buffer_snapshot, query_range)
                })
                .collect();
            non_cached_query_ranges.after_visible = non_cached_query_ranges
                .after_visible
                .into_iter()
                .flat_map(|query_range| {
                    self.remove_cached_ranges_from_query(buffer_snapshot, query_range)
                })
                .collect();
            non_cached_query_ranges
        };

        if !query_ranges.is_empty() {
            self.tasks.push(spawn_task(query_ranges));
        }
    }

    pub fn remove_cached_ranges_from_query(
        &mut self,
        buffer_snapshot: &BufferSnapshot,
        query_range: Range<language::Anchor>,
    ) -> Vec<Range<language::Anchor>> {
        let mut ranges_to_query = Vec::new();
        let mut latest_cached_range = None::<&mut Range<language::Anchor>>;
        for cached_range in self
            .sorted_ranges
            .iter_mut()
            .skip_while(|cached_range| {
                cached_range
                    .end
                    .cmp(&query_range.start, buffer_snapshot)
                    .is_lt()
            })
            .take_while(|cached_range| {
                cached_range
                    .start
                    .cmp(&query_range.end, buffer_snapshot)
                    .is_le()
            })
        {
            match latest_cached_range {
                Some(latest_cached_range) => {
                    if latest_cached_range.end.offset.saturating_add(1) < cached_range.start.offset
                    {
                        ranges_to_query.push(latest_cached_range.end..cached_range.start);
                        cached_range.start = latest_cached_range.end;
                    }
                }
                None => {
                    if query_range
                        .start
                        .cmp(&cached_range.start, buffer_snapshot)
                        .is_lt()
                    {
                        ranges_to_query.push(query_range.start..cached_range.start);
                        cached_range.start = query_range.start;
                    }
                }
            }
            latest_cached_range = Some(cached_range);
        }

        match latest_cached_range {
            Some(latest_cached_range) => {
                if latest_cached_range.end.offset.saturating_add(1) < query_range.end.offset {
                    ranges_to_query.push(latest_cached_range.end..query_range.end);
                    latest_cached_range.end = query_range.end;
                }
            }
            None => {
                ranges_to_query.push(query_range.clone());
                self.sorted_ranges.push(query_range);
                self.sorted_ranges
                    .sort_by(|range_a, range_b| range_a.start.cmp(&range_b.start, buffer_snapshot));
            }
        }

        ranges_to_query
    }

    pub fn invalidate_range(&mut self, buffer: &BufferSnapshot, range: &Range<language::Anchor>) {
        self.sorted_ranges = self
            .sorted_ranges
            .drain(..)
            .filter_map(|mut cached_range| {
                if cached_range.start.cmp(&range.end, buffer).is_gt()
                    || cached_range.end.cmp(&range.start, buffer).is_lt()
                {
                    Some(vec![cached_range])
                } else if cached_range.start.cmp(&range.start, buffer).is_ge()
                    && cached_range.end.cmp(&range.end, buffer).is_le()
                {
                    None
                } else if range.start.cmp(&cached_range.start, buffer).is_ge()
                    && range.end.cmp(&cached_range.end, buffer).is_le()
                {
                    Some(vec![
                        cached_range.start..range.start,
                        range.end..cached_range.end,
                    ])
                } else if cached_range.start.cmp(&range.start, buffer).is_ge() {
                    cached_range.start = range.end;
                    Some(vec![cached_range])
                } else {
                    cached_range.end = range.start;
                    Some(vec![cached_range])
                }
            })
            .flatten()
            .collect();
    }
}

impl QueryRanges {
    pub fn is_empty(&self) -> bool {
        self.before_visible.is_empty() && self.visible.is_empty() && self.after_visible.is_empty()
    }

    pub fn into_sorted_query_ranges(self) -> Vec<Range<text::Anchor>> {
        let mut sorted_ranges = Vec::with_capacity(
            self.before_visible.len() + self.visible.len() + self.after_visible.len(),
        );
        sorted_ranges.extend(self.before_visible);
        sorted_ranges.extend(self.visible);
        sorted_ranges.extend(self.after_visible);
        sorted_ranges
    }
}

pub fn determine_query_ranges(
    multi_buffer: &mut MultiBuffer,
    excerpt_id: ExcerptId,
    excerpt_buffer: &Entity<Buffer>,
    excerpt_visible_range: Range<usize>,
    cx: &mut Context<MultiBuffer>,
) -> Option<QueryRanges> {
    let buffer = excerpt_buffer.read(cx);
    let full_excerpt_range = multi_buffer
        .excerpts_for_buffer(buffer.remote_id(), cx)
        .into_iter()
        .find(|(id, _)| id == &excerpt_id)
        .map(|(_, range)| range.context)?;
    let snapshot = buffer.snapshot();
    let excerpt_visible_len = excerpt_visible_range.end - excerpt_visible_range.start;

    let visible_range = if excerpt_visible_range.start == excerpt_visible_range.end {
        return None;
    } else {
        vec![
            buffer.anchor_before(snapshot.clip_offset(excerpt_visible_range.start, Bias::Left))
                ..buffer.anchor_after(snapshot.clip_offset(excerpt_visible_range.end, Bias::Right)),
        ]
    };

    let full_excerpt_range_end_offset = full_excerpt_range.end.to_offset(&snapshot);
    let after_visible_range_start = excerpt_visible_range
        .end
        .saturating_add(1)
        .min(full_excerpt_range_end_offset)
        .min(buffer.len());
    let after_visible_range = if after_visible_range_start == full_excerpt_range_end_offset {
        Vec::new()
    } else {
        let after_range_end_offset = after_visible_range_start
            .saturating_add(excerpt_visible_len)
            .min(full_excerpt_range_end_offset)
            .min(buffer.len());
        vec![
            buffer.anchor_before(snapshot.clip_offset(after_visible_range_start, Bias::Left))
                ..buffer.anchor_after(snapshot.clip_offset(after_range_end_offset, Bias::Right)),
        ]
    };

    let full_excerpt_range_start_offset = full_excerpt_range.start.to_offset(&snapshot);
    let before_visible_range_end = excerpt_visible_range
        .start
        .saturating_sub(1)
        .max(full_excerpt_range_start_offset);
    let before_visible_range = if before_visible_range_end == full_excerpt_range_start_offset {
        Vec::new()
    } else {
        let before_range_start_offset = before_visible_range_end
            .saturating_sub(excerpt_visible_len)
            .max(full_excerpt_range_start_offset);
        vec![
            buffer.anchor_before(snapshot.clip_offset(before_range_start_offset, Bias::Left))
                ..buffer.anchor_after(snapshot.clip_offset(before_visible_range_end, Bias::Right)),
        ]
    };

    Some(QueryRanges {
        before_visible: before_visible_range,
        visible: visible_range,
        after_visible: after_visible_range,
    })
}

pub fn contains_position(
    range: &Range<language::Anchor>,
    position: language::Anchor,
    buffer_snapshot: &BufferSnapshot,
) -> bool {
    range.start.cmp(&position, buffer_snapshot).is_le()
        && range.end.cmp(&position, buffer_snapshot).is_ge()
}
