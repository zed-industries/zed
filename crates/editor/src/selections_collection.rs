use std::{
    cmp, fmt, iter, mem,
    ops::{AddAssign, Deref, DerefMut, Range, Sub},
    sync::Arc,
};

use collections::HashMap;
use gpui::Pixels;
use itertools::Itertools as _;
use language::{Bias, Point, Selection, SelectionGoal};
use multi_buffer::{MultiBufferDimension, MultiBufferOffset};
use util::post_inc;

use crate::{
    Anchor, DisplayPoint, DisplayRow, ExcerptId, MultiBufferSnapshot, SelectMode, ToOffset,
    display_map::{DisplaySnapshot, ToDisplayPoint},
    movement::TextLayoutDetails,
};

#[derive(Debug, Clone)]
pub struct PendingSelection {
    selection: Selection<Anchor>,
    mode: SelectMode,
}

#[derive(Debug, Clone)]
pub struct SelectionsCollection {
    next_selection_id: usize,
    line_mode: bool,
    /// The non-pending, non-overlapping selections.
    /// The [SelectionsCollection::pending] selection could possibly overlap these
    disjoint: Arc<[Selection<Anchor>]>,
    /// A pending selection, such as when the mouse is being dragged
    pending: Option<PendingSelection>,
    select_mode: SelectMode,
    is_extending: bool,
}

impl SelectionsCollection {
    pub fn new() -> Self {
        Self {
            next_selection_id: 1,
            line_mode: false,
            disjoint: Arc::default(),
            pending: Some(PendingSelection {
                selection: Selection {
                    id: 0,
                    start: Anchor::min(),
                    end: Anchor::min(),
                    reversed: false,
                    goal: SelectionGoal::None,
                },
                mode: SelectMode::Character,
            }),
            select_mode: SelectMode::Character,
            is_extending: false,
        }
    }

    pub fn clone_state(&mut self, other: &SelectionsCollection) {
        self.next_selection_id = other.next_selection_id;
        self.line_mode = other.line_mode;
        self.disjoint = other.disjoint.clone();
        self.pending.clone_from(&other.pending);
    }

    pub fn count(&self) -> usize {
        let mut count = self.disjoint.len();
        if self.pending.is_some() {
            count += 1;
        }
        count
    }

    /// The non-pending, non-overlapping selections. There could be a pending selection that
    /// overlaps these if the mouse is being dragged, etc. This could also be empty if there is a
    /// pending selection. Returned as selections over Anchors.
    pub fn disjoint_anchors_arc(&self) -> Arc<[Selection<Anchor>]> {
        self.disjoint.clone()
    }

    /// The non-pending, non-overlapping selections. There could be a pending selection that
    /// overlaps these if the mouse is being dragged, etc. This could also be empty if there is a
    /// pending selection. Returned as selections over Anchors.
    pub fn disjoint_anchors(&self) -> &[Selection<Anchor>] {
        &self.disjoint
    }

    pub fn disjoint_anchor_ranges(&self) -> impl Iterator<Item = Range<Anchor>> {
        // Mapping the Arc slice would borrow it, whereas indexing captures it.
        let disjoint = self.disjoint_anchors_arc();
        (0..disjoint.len()).map(move |ix| disjoint[ix].range())
    }

    /// Non-overlapping selections using anchors, including the pending selection.
    pub fn all_anchors(&self, snapshot: &DisplaySnapshot) -> Arc<[Selection<Anchor>]> {
        if self.pending.is_none() {
            self.disjoint_anchors_arc()
        } else {
            let all_offset_selections = self.all::<MultiBufferOffset>(snapshot);
            all_offset_selections
                .into_iter()
                .map(|selection| selection_to_anchor_selection(selection, snapshot))
                .collect()
        }
    }

    pub fn pending_anchor(&self) -> Option<&Selection<Anchor>> {
        self.pending.as_ref().map(|pending| &pending.selection)
    }

    pub fn pending_anchor_mut(&mut self) -> Option<&mut Selection<Anchor>> {
        self.pending.as_mut().map(|pending| &mut pending.selection)
    }

    pub fn pending<D>(&self, snapshot: &DisplaySnapshot) -> Option<Selection<D>>
    where
        D: MultiBufferDimension + Sub + AddAssign<<D as Sub>::Output> + Ord,
    {
        resolve_selections_wrapping_blocks(self.pending_anchor(), &snapshot).next()
    }

    pub(crate) fn pending_mode(&self) -> Option<SelectMode> {
        self.pending.as_ref().map(|pending| pending.mode.clone())
    }

    pub fn all<D>(&self, snapshot: &DisplaySnapshot) -> Vec<Selection<D>>
    where
        D: MultiBufferDimension + Sub + AddAssign<<D as Sub>::Output> + Ord,
    {
        let disjoint_anchors = &self.disjoint;
        let mut disjoint =
            resolve_selections_wrapping_blocks::<D, _>(disjoint_anchors.iter(), &snapshot)
                .peekable();
        let mut pending_opt = self.pending::<D>(&snapshot);
        iter::from_fn(move || {
            if let Some(pending) = pending_opt.as_mut() {
                while let Some(next_selection) = disjoint.peek() {
                    if should_merge(
                        pending.start,
                        pending.end,
                        next_selection.start,
                        next_selection.end,
                        false,
                    ) {
                        let next_selection = disjoint.next().unwrap();
                        if next_selection.start < pending.start {
                            pending.start = next_selection.start;
                        }
                        if next_selection.end > pending.end {
                            pending.end = next_selection.end;
                        }
                    } else if next_selection.end < pending.start {
                        return disjoint.next();
                    } else {
                        break;
                    }
                }

                pending_opt.take()
            } else {
                disjoint.next()
            }
        })
        .collect()
    }

    /// Returns all of the selections, adjusted to take into account the selection line_mode
    pub fn all_adjusted(&self, snapshot: &DisplaySnapshot) -> Vec<Selection<Point>> {
        let mut selections = self.all::<Point>(&snapshot);
        if self.line_mode {
            for selection in &mut selections {
                let new_range = snapshot.expand_to_line(selection.range());
                selection.start = new_range.start;
                selection.end = new_range.end;
            }
        }
        selections
    }

    /// Returns the newest selection, adjusted to take into account the selection line_mode
    pub fn newest_adjusted(&self, snapshot: &DisplaySnapshot) -> Selection<Point> {
        let mut selection = self.newest::<Point>(&snapshot);
        if self.line_mode {
            let new_range = snapshot.expand_to_line(selection.range());
            selection.start = new_range.start;
            selection.end = new_range.end;
        }
        selection
    }

    pub fn all_adjusted_display(
        &self,
        display_map: &DisplaySnapshot,
    ) -> Vec<Selection<DisplayPoint>> {
        if self.line_mode {
            let selections = self.all::<Point>(&display_map);
            let result = selections
                .into_iter()
                .map(|mut selection| {
                    let new_range = display_map.expand_to_line(selection.range());
                    selection.start = new_range.start;
                    selection.end = new_range.end;
                    selection.map(|point| point.to_display_point(&display_map))
                })
                .collect();
            result
        } else {
            self.all_display(display_map)
        }
    }

    pub fn disjoint_in_range<D>(
        &self,
        range: Range<Anchor>,
        snapshot: &DisplaySnapshot,
    ) -> Vec<Selection<D>>
    where
        D: MultiBufferDimension + Sub + AddAssign<<D as Sub>::Output> + Ord + std::fmt::Debug,
    {
        let start_ix = match self
            .disjoint
            .binary_search_by(|probe| probe.end.cmp(&range.start, snapshot.buffer_snapshot()))
        {
            Ok(ix) | Err(ix) => ix,
        };
        let end_ix = match self
            .disjoint
            .binary_search_by(|probe| probe.start.cmp(&range.end, snapshot.buffer_snapshot()))
        {
            Ok(ix) => ix + 1,
            Err(ix) => ix,
        };
        resolve_selections_wrapping_blocks(&self.disjoint[start_ix..end_ix], snapshot).collect()
    }

    pub fn all_display(&self, snapshot: &DisplaySnapshot) -> Vec<Selection<DisplayPoint>> {
        let disjoint_anchors = &self.disjoint;
        let mut disjoint =
            resolve_selections_display(disjoint_anchors.iter(), &snapshot).peekable();
        let mut pending_opt = resolve_selections_display(self.pending_anchor(), &snapshot).next();
        iter::from_fn(move || {
            if let Some(pending) = pending_opt.as_mut() {
                while let Some(next_selection) = disjoint.peek() {
                    if should_merge(
                        pending.start,
                        pending.end,
                        next_selection.start,
                        next_selection.end,
                        false,
                    ) {
                        let next_selection = disjoint.next().unwrap();
                        if next_selection.start < pending.start {
                            pending.start = next_selection.start;
                        }
                        if next_selection.end > pending.end {
                            pending.end = next_selection.end;
                        }
                    } else if next_selection.end < pending.start {
                        return disjoint.next();
                    } else {
                        break;
                    }
                }

                pending_opt.take()
            } else {
                disjoint.next()
            }
        })
        .collect()
    }

    pub fn newest_anchor(&self) -> &Selection<Anchor> {
        self.pending
            .as_ref()
            .map(|s| &s.selection)
            .or_else(|| self.disjoint.iter().max_by_key(|s| s.id))
            .unwrap()
    }

    pub fn newest<D>(&self, snapshot: &DisplaySnapshot) -> Selection<D>
    where
        D: MultiBufferDimension + Sub + AddAssign<<D as Sub>::Output> + Ord,
    {
        resolve_selections_wrapping_blocks([self.newest_anchor()], &snapshot)
            .next()
            .unwrap()
    }

    pub fn newest_display(&self, snapshot: &DisplaySnapshot) -> Selection<DisplayPoint> {
        resolve_selections_display([self.newest_anchor()], &snapshot)
            .next()
            .unwrap()
    }

    pub fn oldest_anchor(&self) -> &Selection<Anchor> {
        self.disjoint
            .iter()
            .min_by_key(|s| s.id)
            .or_else(|| self.pending.as_ref().map(|p| &p.selection))
            .unwrap()
    }

    pub fn oldest<D>(&self, snapshot: &DisplaySnapshot) -> Selection<D>
    where
        D: MultiBufferDimension + Sub + AddAssign<<D as Sub>::Output> + Ord,
    {
        resolve_selections_wrapping_blocks([self.oldest_anchor()], &snapshot)
            .next()
            .unwrap()
    }

    pub fn first_anchor(&self) -> Selection<Anchor> {
        self.pending
            .as_ref()
            .map(|pending| pending.selection.clone())
            .unwrap_or_else(|| self.disjoint.first().cloned().unwrap())
    }

    pub fn first<D>(&self, snapshot: &DisplaySnapshot) -> Selection<D>
    where
        D: MultiBufferDimension + Sub + AddAssign<<D as Sub>::Output> + Ord,
    {
        self.all(snapshot).first().unwrap().clone()
    }

    pub fn last<D>(&self, snapshot: &DisplaySnapshot) -> Selection<D>
    where
        D: MultiBufferDimension + Sub + AddAssign<<D as Sub>::Output> + Ord,
    {
        self.all(snapshot).last().unwrap().clone()
    }

    /// Returns a list of (potentially backwards!) ranges representing the selections.
    /// Useful for test assertions, but prefer `.all()` instead.
    #[cfg(any(test, feature = "test-support"))]
    pub fn ranges<D>(&self, snapshot: &DisplaySnapshot) -> Vec<Range<D>>
    where
        D: MultiBufferDimension + Sub + AddAssign<<D as Sub>::Output> + Ord,
    {
        self.all::<D>(snapshot)
            .iter()
            .map(|s| {
                if s.reversed {
                    s.end..s.start
                } else {
                    s.start..s.end
                }
            })
            .collect()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn display_ranges(&self, display_snapshot: &DisplaySnapshot) -> Vec<Range<DisplayPoint>> {
        self.disjoint_anchors_arc()
            .iter()
            .chain(self.pending_anchor())
            .map(|s| {
                if s.reversed {
                    s.end.to_display_point(display_snapshot)
                        ..s.start.to_display_point(display_snapshot)
                } else {
                    s.start.to_display_point(display_snapshot)
                        ..s.end.to_display_point(display_snapshot)
                }
            })
            .collect()
    }

    /// Attempts to build a selection in the provided `DisplayRow` within the
    /// same range as the provided range of `Pixels`.
    /// Returns `None` if the range is not empty but it starts past the line's
    /// length, meaning that the line isn't long enough to be contained within
    /// part of the provided range.
    pub fn build_columnar_selection(
        &mut self,
        display_map: &DisplaySnapshot,
        row: DisplayRow,
        positions: &Range<Pixels>,
        reversed: bool,
        text_layout_details: &TextLayoutDetails,
    ) -> Option<Selection<Point>> {
        let is_empty = positions.start == positions.end;
        let line_len = display_map.line_len(row);
        let line = display_map.layout_row(row, text_layout_details);
        let start_col = line.closest_index_for_x(positions.start) as u32;

        let (start, end) = if is_empty {
            let point = DisplayPoint::new(row, std::cmp::min(start_col, line_len));
            (point, point)
        } else {
            if start_col >= line_len {
                return None;
            }
            let start = DisplayPoint::new(row, start_col);
            let end_col = line.closest_index_for_x(positions.end) as u32;
            let end = DisplayPoint::new(row, end_col);
            (start, end)
        };

        Some(Selection {
            id: post_inc(&mut self.next_selection_id),
            start: start.to_point(display_map),
            end: end.to_point(display_map),
            reversed,
            goal: SelectionGoal::HorizontalRange {
                start: positions.start.into(),
                end: positions.end.into(),
            },
        })
    }

    /// Attempts to build a selection in the provided buffer row using the
    /// same buffer column range as specified.
    /// Returns `None` if the range is not empty but it starts past the line's
    /// length, meaning that the line isn't long enough to be contained within
    /// part of the provided range.
    pub fn build_columnar_selection_from_buffer_columns(
        &mut self,
        display_map: &DisplaySnapshot,
        buffer_row: u32,
        positions: &Range<u32>,
        reversed: bool,
        text_layout_details: &TextLayoutDetails,
    ) -> Option<Selection<Point>> {
        let is_empty = positions.start == positions.end;
        let line_len = display_map
            .buffer_snapshot()
            .line_len(multi_buffer::MultiBufferRow(buffer_row));

        let (start, end) = if is_empty {
            let column = std::cmp::min(positions.start, line_len);
            let point = Point::new(buffer_row, column);
            (point, point)
        } else {
            if positions.start >= line_len {
                return None;
            }

            let start = Point::new(buffer_row, positions.start);
            let end_column = std::cmp::min(positions.end, line_len);
            let end = Point::new(buffer_row, end_column);
            (start, end)
        };

        let start_display_point = start.to_display_point(display_map);
        let end_display_point = end.to_display_point(display_map);
        let start_x = display_map.x_for_display_point(start_display_point, text_layout_details);
        let end_x = display_map.x_for_display_point(end_display_point, text_layout_details);

        Some(Selection {
            id: post_inc(&mut self.next_selection_id),
            start,
            end,
            reversed,
            goal: SelectionGoal::HorizontalRange {
                start: start_x.min(end_x).into(),
                end: start_x.max(end_x).into(),
            },
        })
    }

    pub fn change_with<R>(
        &mut self,
        snapshot: &DisplaySnapshot,
        change: impl FnOnce(&mut MutableSelectionsCollection<'_, '_>) -> R,
    ) -> (bool, R) {
        let mut mutable_collection = MutableSelectionsCollection {
            snapshot,
            collection: self,
            selections_changed: false,
        };

        let result = change(&mut mutable_collection);
        assert!(
            !mutable_collection.disjoint.is_empty() || mutable_collection.pending.is_some(),
            "There must be at least one selection"
        );
        if cfg!(debug_assertions) {
            mutable_collection.disjoint.iter().for_each(|selection| {
                assert!(
                    snapshot.can_resolve(&selection.start),
                    "disjoint selection start is not resolvable for the given snapshot:\n{selection:?}, {excerpt:?}",
                    excerpt = snapshot.buffer_for_excerpt(selection.start.excerpt_id).map(|snapshot| snapshot.remote_id()),
                );
                assert!(
                    snapshot.can_resolve(&selection.end),
                    "disjoint selection end is not resolvable for the given snapshot: {selection:?}, {excerpt:?}",
                    excerpt = snapshot.buffer_for_excerpt(selection.end.excerpt_id).map(|snapshot| snapshot.remote_id()),
                );
            });
            if let Some(pending) = &mutable_collection.pending {
                let selection = &pending.selection;
                assert!(
                    snapshot.can_resolve(&selection.start),
                    "pending selection start is not resolvable for the given snapshot: {pending:?}, {excerpt:?}",
                    excerpt = snapshot
                        .buffer_for_excerpt(selection.start.excerpt_id)
                        .map(|snapshot| snapshot.remote_id()),
                );
                assert!(
                    snapshot.can_resolve(&selection.end),
                    "pending selection end is not resolvable for the given snapshot: {pending:?}, {excerpt:?}",
                    excerpt = snapshot
                        .buffer_for_excerpt(selection.end.excerpt_id)
                        .map(|snapshot| snapshot.remote_id()),
                );
            }
        }
        (mutable_collection.selections_changed, result)
    }

    pub fn next_selection_id(&self) -> usize {
        self.next_selection_id
    }

    pub fn line_mode(&self) -> bool {
        self.line_mode
    }

    pub fn set_line_mode(&mut self, line_mode: bool) {
        self.line_mode = line_mode;
    }

    pub fn select_mode(&self) -> &SelectMode {
        &self.select_mode
    }

    pub fn set_select_mode(&mut self, select_mode: SelectMode) {
        self.select_mode = select_mode;
    }

    pub fn is_extending(&self) -> bool {
        self.is_extending
    }

    pub fn set_is_extending(&mut self, is_extending: bool) {
        self.is_extending = is_extending;
    }
}

pub struct MutableSelectionsCollection<'snap, 'a> {
    collection: &'a mut SelectionsCollection,
    snapshot: &'snap DisplaySnapshot,
    selections_changed: bool,
}

impl<'snap, 'a> fmt::Debug for MutableSelectionsCollection<'snap, 'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MutableSelectionsCollection")
            .field("collection", &self.collection)
            .field("selections_changed", &self.selections_changed)
            .finish()
    }
}

impl<'snap, 'a> MutableSelectionsCollection<'snap, 'a> {
    pub fn display_snapshot(&self) -> DisplaySnapshot {
        self.snapshot.clone()
    }

    pub fn clear_disjoint(&mut self) {
        self.collection.disjoint = Arc::default();
    }

    pub fn delete(&mut self, selection_id: usize) {
        let mut changed = false;
        self.collection.disjoint = self
            .disjoint
            .iter()
            .filter(|selection| {
                let found = selection.id == selection_id;
                changed |= found;
                !found
            })
            .cloned()
            .collect();

        self.selections_changed |= changed;
    }

    pub fn remove_selections_from_buffer(&mut self, buffer_id: language::BufferId) {
        let mut changed = false;

        let filtered_selections: Arc<[Selection<Anchor>]> = {
            self.disjoint
                .iter()
                .filter(|selection| {
                    if let Some(selection_buffer_id) =
                        self.snapshot.buffer_id_for_anchor(selection.start)
                    {
                        let should_remove = selection_buffer_id == buffer_id;
                        changed |= should_remove;
                        !should_remove
                    } else {
                        true
                    }
                })
                .cloned()
                .collect()
        };

        if filtered_selections.is_empty() {
            let buffer_snapshot = self.snapshot.buffer_snapshot();
            let anchor = buffer_snapshot
                .excerpts()
                .find(|(_, buffer, _)| buffer.remote_id() == buffer_id)
                .and_then(|(excerpt_id, _, range)| {
                    buffer_snapshot.anchor_in_excerpt(excerpt_id, range.context.start)
                })
                .unwrap_or_else(|| self.snapshot.anchor_before(MultiBufferOffset(0)));
            self.collection.disjoint = Arc::from([Selection {
                id: post_inc(&mut self.collection.next_selection_id),
                start: anchor,
                end: anchor,
                reversed: false,
                goal: SelectionGoal::None,
            }]);
        } else {
            self.collection.disjoint = filtered_selections;
        }

        self.selections_changed |= changed;
    }

    pub fn clear_pending(&mut self) {
        if self.collection.pending.is_some() {
            self.collection.pending = None;
            self.selections_changed = true;
        }
    }

    pub(crate) fn set_pending_anchor_range(&mut self, range: Range<Anchor>, mode: SelectMode) {
        self.collection.pending = Some(PendingSelection {
            selection: {
                let mut start = range.start;
                let mut end = range.end;
                let reversed = if start.cmp(&end, self.snapshot).is_gt() {
                    mem::swap(&mut start, &mut end);
                    true
                } else {
                    false
                };
                Selection {
                    id: post_inc(&mut self.collection.next_selection_id),
                    start,
                    end,
                    reversed,
                    goal: SelectionGoal::None,
                }
            },
            mode,
        });
        self.selections_changed = true;
    }

    pub(crate) fn set_pending(&mut self, selection: Selection<Anchor>, mode: SelectMode) {
        self.collection.pending = Some(PendingSelection { selection, mode });
        self.selections_changed = true;
    }

    pub fn try_cancel(&mut self) -> bool {
        if let Some(pending) = self.collection.pending.take() {
            if self.disjoint.is_empty() {
                self.collection.disjoint = Arc::from([pending.selection]);
            }
            self.selections_changed = true;
            return true;
        }

        let mut oldest = self.oldest_anchor().clone();
        if self.count() > 1 {
            self.collection.disjoint = Arc::from([oldest]);
            self.selections_changed = true;
            return true;
        }

        if !oldest.start.cmp(&oldest.end, self.snapshot).is_eq() {
            let head = oldest.head();
            oldest.start = head;
            oldest.end = head;
            self.collection.disjoint = Arc::from([oldest]);
            self.selections_changed = true;
            return true;
        }

        false
    }

    pub fn insert_range<T>(&mut self, range: Range<T>)
    where
        T: ToOffset,
    {
        let display_map = self.display_snapshot();
        let mut selections = self.collection.all(&display_map);
        let mut start = range.start.to_offset(self.snapshot);
        let mut end = range.end.to_offset(self.snapshot);
        let reversed = if start > end {
            mem::swap(&mut start, &mut end);
            true
        } else {
            false
        };
        selections.push(Selection {
            id: post_inc(&mut self.collection.next_selection_id),
            start,
            end,
            reversed,
            goal: SelectionGoal::None,
        });
        self.select(selections);
    }

    pub fn select<T>(&mut self, selections: Vec<Selection<T>>)
    where
        T: ToOffset + std::marker::Copy + std::fmt::Debug,
    {
        let mut selections = selections
            .into_iter()
            .map(|selection| selection.map(|it| it.to_offset(self.snapshot)))
            .map(|mut selection| {
                if selection.start > selection.end {
                    mem::swap(&mut selection.start, &mut selection.end);
                    selection.reversed = true
                }
                selection
            })
            .collect::<Vec<_>>();
        selections.sort_unstable_by_key(|s| s.start);

        let mut i = 1;
        while i < selections.len() {
            let prev = &selections[i - 1];
            let current = &selections[i];

            if should_merge(prev.start, prev.end, current.start, current.end, true) {
                let removed = selections.remove(i);
                if removed.start < selections[i - 1].start {
                    selections[i - 1].start = removed.start;
                }
                if selections[i - 1].end < removed.end {
                    selections[i - 1].end = removed.end;
                }
            } else {
                i += 1;
            }
        }

        self.collection.disjoint = Arc::from_iter(
            selections
                .into_iter()
                .map(|selection| selection_to_anchor_selection(selection, self.snapshot)),
        );
        self.collection.pending = None;
        self.selections_changed = true;
    }

    pub fn select_anchors(&mut self, selections: Vec<Selection<Anchor>>) {
        let map = self.display_snapshot();
        let resolved_selections =
            resolve_selections_wrapping_blocks::<MultiBufferOffset, _>(&selections, &map)
                .collect::<Vec<_>>();
        self.select(resolved_selections);
    }

    pub fn select_ranges<I, T>(&mut self, ranges: I)
    where
        I: IntoIterator<Item = Range<T>>,
        T: ToOffset,
    {
        let ranges = ranges
            .into_iter()
            .map(|range| range.start.to_offset(self.snapshot)..range.end.to_offset(self.snapshot));
        self.select_offset_ranges(ranges);
    }

    fn select_offset_ranges<I>(&mut self, ranges: I)
    where
        I: IntoIterator<Item = Range<MultiBufferOffset>>,
    {
        let selections = ranges
            .into_iter()
            .map(|range| {
                let mut start = range.start;
                let mut end = range.end;
                let reversed = if start > end {
                    mem::swap(&mut start, &mut end);
                    true
                } else {
                    false
                };
                Selection {
                    id: post_inc(&mut self.collection.next_selection_id),
                    start,
                    end,
                    reversed,
                    goal: SelectionGoal::None,
                }
            })
            .collect::<Vec<_>>();

        self.select(selections)
    }

    pub fn select_anchor_ranges<I>(&mut self, ranges: I)
    where
        I: IntoIterator<Item = Range<Anchor>>,
    {
        let selections = ranges
            .into_iter()
            .map(|range| {
                let mut start = range.start;
                let mut end = range.end;
                let reversed = if start.cmp(&end, self.snapshot).is_gt() {
                    mem::swap(&mut start, &mut end);
                    true
                } else {
                    false
                };
                Selection {
                    id: post_inc(&mut self.collection.next_selection_id),
                    start,
                    end,
                    reversed,
                    goal: SelectionGoal::None,
                }
            })
            .collect::<Vec<_>>();
        self.select_anchors(selections)
    }

    pub fn new_selection_id(&mut self) -> usize {
        post_inc(&mut self.next_selection_id)
    }

    pub fn select_display_ranges<T>(&mut self, ranges: T)
    where
        T: IntoIterator<Item = Range<DisplayPoint>>,
    {
        let selections = ranges
            .into_iter()
            .map(|range| {
                let mut start = range.start;
                let mut end = range.end;
                let reversed = if start > end {
                    mem::swap(&mut start, &mut end);
                    true
                } else {
                    false
                };
                Selection {
                    id: post_inc(&mut self.collection.next_selection_id),
                    start: start.to_point(self.snapshot),
                    end: end.to_point(self.snapshot),
                    reversed,
                    goal: SelectionGoal::None,
                }
            })
            .collect();
        self.select(selections);
    }

    pub fn reverse_selections(&mut self) {
        let mut new_selections: Vec<Selection<Point>> = Vec::new();
        let disjoint = self.disjoint.clone();
        for selection in disjoint
            .iter()
            .sorted_by(|first, second| Ord::cmp(&second.id, &first.id))
            .collect::<Vec<&Selection<Anchor>>>()
        {
            new_selections.push(Selection {
                id: self.new_selection_id(),
                start: selection
                    .start
                    .to_display_point(self.snapshot)
                    .to_point(self.snapshot),
                end: selection
                    .end
                    .to_display_point(self.snapshot)
                    .to_point(self.snapshot),
                reversed: selection.reversed,
                goal: selection.goal,
            });
        }
        self.select(new_selections);
    }

    pub fn move_with(
        &mut self,
        mut move_selection: impl FnMut(&DisplaySnapshot, &mut Selection<DisplayPoint>),
    ) {
        let mut changed = false;
        let display_map = self.display_snapshot();
        let selections = self.collection.all_display(&display_map);
        let selections = selections
            .into_iter()
            .map(|selection| {
                let mut moved_selection = selection.clone();
                move_selection(&display_map, &mut moved_selection);
                if selection != moved_selection {
                    changed = true;
                }
                moved_selection.map(|display_point| display_point.to_point(&display_map))
            })
            .collect();

        if changed {
            self.select(selections)
        }
    }

    pub fn move_offsets_with(
        &mut self,
        mut move_selection: impl FnMut(&MultiBufferSnapshot, &mut Selection<MultiBufferOffset>),
    ) {
        let mut changed = false;
        let display_map = self.display_snapshot();
        let selections = self
            .collection
            .all::<MultiBufferOffset>(&display_map)
            .into_iter()
            .map(|selection| {
                let mut moved_selection = selection.clone();
                move_selection(self.snapshot, &mut moved_selection);
                if selection != moved_selection {
                    changed = true;
                }
                moved_selection
            })
            .collect();

        if changed {
            self.select(selections)
        }
    }

    pub fn move_heads_with(
        &mut self,
        mut update_head: impl FnMut(
            &DisplaySnapshot,
            DisplayPoint,
            SelectionGoal,
        ) -> (DisplayPoint, SelectionGoal),
    ) {
        self.move_with(|map, selection| {
            let (new_head, new_goal) = update_head(map, selection.head(), selection.goal);
            selection.set_head(new_head, new_goal);
        });
    }

    pub fn move_cursors_with(
        &mut self,
        mut update_cursor_position: impl FnMut(
            &DisplaySnapshot,
            DisplayPoint,
            SelectionGoal,
        ) -> (DisplayPoint, SelectionGoal),
    ) {
        self.move_with(|map, selection| {
            let (cursor, new_goal) = update_cursor_position(map, selection.head(), selection.goal);
            selection.collapse_to(cursor, new_goal)
        });
    }

    pub fn maybe_move_cursors_with(
        &mut self,
        mut update_cursor_position: impl FnMut(
            &DisplaySnapshot,
            DisplayPoint,
            SelectionGoal,
        ) -> Option<(DisplayPoint, SelectionGoal)>,
    ) {
        self.move_cursors_with(|map, point, goal| {
            update_cursor_position(map, point, goal).unwrap_or((point, goal))
        })
    }

    pub fn replace_cursors_with(
        &mut self,
        find_replacement_cursors: impl FnOnce(&DisplaySnapshot) -> Vec<DisplayPoint>,
    ) {
        let new_selections = find_replacement_cursors(self.snapshot)
            .into_iter()
            .map(|cursor| {
                let cursor_point = cursor.to_point(self.snapshot);
                Selection {
                    id: post_inc(&mut self.collection.next_selection_id),
                    start: cursor_point,
                    end: cursor_point,
                    reversed: false,
                    goal: SelectionGoal::None,
                }
            })
            .collect();
        self.select(new_selections);
    }

    /// Compute new ranges for any selections that were located in excerpts that have
    /// since been removed.
    ///
    /// Returns a `HashMap` indicating which selections whose former head position
    /// was no longer present. The keys of the map are selection ids. The values are
    /// the id of the new excerpt where the head of the selection has been moved.
    pub fn refresh(&mut self) -> HashMap<usize, ExcerptId> {
        let mut pending = self.collection.pending.take();
        let mut selections_with_lost_position = HashMap::default();

        let anchors_with_status = {
            let disjoint_anchors = self
                .disjoint
                .iter()
                .flat_map(|selection| [&selection.start, &selection.end]);
            self.snapshot.refresh_anchors(disjoint_anchors)
        };
        let adjusted_disjoint: Vec<_> = anchors_with_status
            .chunks(2)
            .map(|selection_anchors| {
                let (anchor_ix, start, kept_start) = selection_anchors[0];
                let (_, end, kept_end) = selection_anchors[1];
                let selection = &self.disjoint[anchor_ix / 2];
                let kept_head = if selection.reversed {
                    kept_start
                } else {
                    kept_end
                };
                if !kept_head {
                    selections_with_lost_position.insert(selection.id, selection.head().excerpt_id);
                }

                Selection {
                    id: selection.id,
                    start,
                    end,
                    reversed: selection.reversed,
                    goal: selection.goal,
                }
            })
            .collect();

        if !adjusted_disjoint.is_empty() {
            let map = self.display_snapshot();
            let resolved_selections =
                resolve_selections_wrapping_blocks(adjusted_disjoint.iter(), &map).collect();
            self.select::<MultiBufferOffset>(resolved_selections);
        }

        if let Some(pending) = pending.as_mut() {
            let anchors = self
                .snapshot
                .refresh_anchors([&pending.selection.start, &pending.selection.end]);
            let (_, start, kept_start) = anchors[0];
            let (_, end, kept_end) = anchors[1];
            let kept_head = if pending.selection.reversed {
                kept_start
            } else {
                kept_end
            };
            if !kept_head {
                selections_with_lost_position
                    .insert(pending.selection.id, pending.selection.head().excerpt_id);
            }

            pending.selection.start = start;
            pending.selection.end = end;
        }
        self.collection.pending = pending;
        self.selections_changed = true;

        selections_with_lost_position
    }
}

impl Deref for MutableSelectionsCollection<'_, '_> {
    type Target = SelectionsCollection;
    fn deref(&self) -> &Self::Target {
        self.collection
    }
}

impl DerefMut for MutableSelectionsCollection<'_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.collection
    }
}

fn selection_to_anchor_selection(
    selection: Selection<MultiBufferOffset>,
    buffer: &MultiBufferSnapshot,
) -> Selection<Anchor> {
    let end_bias = if selection.start == selection.end {
        Bias::Right
    } else {
        Bias::Left
    };
    Selection {
        id: selection.id,
        start: buffer.anchor_after(selection.start),
        end: buffer.anchor_at(selection.end, end_bias),
        reversed: selection.reversed,
        goal: selection.goal,
    }
}

fn resolve_selections_point<'a>(
    selections: impl 'a + IntoIterator<Item = &'a Selection<Anchor>>,
    map: &'a DisplaySnapshot,
) -> impl 'a + Iterator<Item = Selection<Point>> {
    let (to_summarize, selections) = selections.into_iter().tee();
    let mut summaries = map
        .buffer_snapshot()
        .summaries_for_anchors::<Point, _>(to_summarize.flat_map(|s| [&s.start, &s.end]))
        .into_iter();
    selections.map(move |s| {
        let start = summaries.next().unwrap();
        let end = summaries.next().unwrap();
        assert!(start <= end, "start: {:?}, end: {:?}", start, end);
        Selection {
            id: s.id,
            start,
            end,
            reversed: s.reversed,
            goal: s.goal,
        }
    })
}

/// Panics if passed selections are not in order
/// Resolves the anchors to display positions
fn resolve_selections_display<'a>(
    selections: impl 'a + IntoIterator<Item = &'a Selection<Anchor>>,
    map: &'a DisplaySnapshot,
) -> impl 'a + Iterator<Item = Selection<DisplayPoint>> {
    let selections = resolve_selections_point(selections, map).map(move |s| {
        let display_start = map.point_to_display_point(s.start, Bias::Left);
        let display_end = map.point_to_display_point(
            s.end,
            if s.start == s.end {
                Bias::Right
            } else {
                Bias::Left
            },
        );
        assert!(
            display_start <= display_end,
            "display_start: {:?}, display_end: {:?}",
            display_start,
            display_end
        );
        Selection {
            id: s.id,
            start: display_start,
            end: display_end,
            reversed: s.reversed,
            goal: s.goal,
        }
    });
    coalesce_selections(selections)
}

/// Resolves the passed in anchors to [`MultiBufferDimension`]s `D`
/// wrapping around blocks inbetween.
///
/// # Panics
///
/// Panics if passed selections are not in order
pub(crate) fn resolve_selections_wrapping_blocks<'a, D, I>(
    selections: I,
    map: &'a DisplaySnapshot,
) -> impl 'a + Iterator<Item = Selection<D>>
where
    D: MultiBufferDimension + Sub + AddAssign<<D as Sub>::Output> + Ord,
    I: 'a + IntoIterator<Item = &'a Selection<Anchor>>,
{
    // Transforms `Anchor -> DisplayPoint -> Point -> DisplayPoint -> D`
    // todo(lw): We should be able to short circuit the `Anchor -> DisplayPoint -> Point` to `Anchor -> Point`
    let (to_convert, selections) = resolve_selections_display(selections, map).tee();
    let mut converted_endpoints =
        map.buffer_snapshot()
            .dimensions_from_points::<D>(to_convert.flat_map(|s| {
                let start = map.display_point_to_point(s.start, Bias::Left);
                let end = map.display_point_to_point(s.end, Bias::Right);
                assert!(start <= end, "start: {:?}, end: {:?}", start, end);
                [start, end]
            }));
    selections.map(move |s| {
        let start = converted_endpoints.next().unwrap();
        let end = converted_endpoints.next().unwrap();
        assert!(start <= end, "start: {:?}, end: {:?}", start, end);
        Selection {
            id: s.id,
            start,
            end,
            reversed: s.reversed,
            goal: s.goal,
        }
    })
}

fn coalesce_selections<D: Ord + fmt::Debug + Copy>(
    selections: impl Iterator<Item = Selection<D>>,
) -> impl Iterator<Item = Selection<D>> {
    let mut selections = selections.peekable();
    iter::from_fn(move || {
        let mut selection = selections.next()?;
        while let Some(next_selection) = selections.peek() {
            if should_merge(
                selection.start,
                selection.end,
                next_selection.start,
                next_selection.end,
                true,
            ) {
                if selection.reversed == next_selection.reversed {
                    selection.end = cmp::max(selection.end, next_selection.end);
                    selections.next();
                } else {
                    selection.end = cmp::max(selection.start, next_selection.start);
                    break;
                }
            } else {
                break;
            }
        }
        assert!(
            selection.start <= selection.end,
            "selection.start: {:?}, selection.end: {:?}, selection.reversed: {:?}",
            selection.start,
            selection.end,
            selection.reversed
        );
        Some(selection)
    })
}

/// Determines whether two selections should be merged into one.
///
/// Two selections should be merged when:
/// 1. They overlap: the selections share at least one position
/// 2. They have the same start position: one contains or equals the other
/// 3. A cursor touches a selection boundary: a zero-width selection (cursor) at the
///    start or end of another selection should be absorbed into it
///
/// Note: two selections that merely touch (one ends exactly where the other begins)
/// but don't share any positions remain separate, see: https://github.com/zed-industries/zed/issues/24748
fn should_merge<T: Ord + Copy>(a_start: T, a_end: T, b_start: T, b_end: T, sorted: bool) -> bool {
    let is_overlapping = if sorted {
        // When sorted, `a` starts before or at `b`, so overlap means `b` starts before `a` ends
        b_start < a_end
    } else {
        a_start < b_end && b_start < a_end
    };

    // Selections starting at the same position should always merge (one contains the other)
    let same_start = a_start == b_start;

    // A cursor (zero-width selection) touching another selection's boundary should merge.
    // This handles cases like a cursor at position X merging with a selection that
    // starts or ends at X.
    let is_cursor_a = a_start == a_end;
    let is_cursor_b = b_start == b_end;
    let cursor_at_boundary = (is_cursor_a && (a_start == b_start || a_end == b_end))
        || (is_cursor_b && (b_start == a_start || b_end == a_end));

    is_overlapping || same_start || cursor_at_boundary
}
