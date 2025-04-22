use std::{
    cell::Ref,
    cmp, iter, mem,
    ops::{Deref, DerefMut, Range, Sub},
    sync::Arc,
};

use collections::HashMap;
use gpui::{App, Entity, Pixels};
use itertools::Itertools;
use language::{Bias, Point, Selection, SelectionGoal, TextDimension};
use util::post_inc;

use crate::{
    Anchor, DisplayPoint, DisplayRow, ExcerptId, MultiBuffer, MultiBufferSnapshot, SelectMode,
    ToOffset, ToPoint,
    display_map::{DisplayMap, DisplaySnapshot, ToDisplayPoint},
    movement::TextLayoutDetails,
};

#[derive(Debug, Clone)]
pub struct PendingSelection {
    pub selection: Selection<Anchor>,
    pub mode: SelectMode,
}

#[derive(Debug, Clone)]
pub struct SelectionsCollection {
    display_map: Entity<DisplayMap>,
    buffer: Entity<MultiBuffer>,
    pub next_selection_id: usize,
    pub line_mode: bool,
    pub vim_mode: bool,
    /// The non-pending, non-overlapping selections.
    /// The [SelectionsCollection::pending] selection could possibly overlap these
    pub disjoint: Arc<[Selection<Anchor>]>,
    /// A pending selection, such as when the mouse is being dragged
    pub pending: Option<PendingSelection>,
}

impl SelectionsCollection {
    pub fn new(display_map: Entity<DisplayMap>, buffer: Entity<MultiBuffer>) -> Self {
        Self {
            display_map,
            buffer,
            next_selection_id: 1,
            line_mode: false,
            vim_mode: false,
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
        }
    }

    pub fn display_map(&self, cx: &mut App) -> DisplaySnapshot {
        self.display_map.update(cx, |map, cx| map.snapshot(cx))
    }

    fn buffer<'a>(&self, cx: &'a App) -> Ref<'a, MultiBufferSnapshot> {
        self.buffer.read(cx).read(cx)
    }

    pub fn clone_state(&mut self, other: &SelectionsCollection) {
        self.next_selection_id = other.next_selection_id;
        self.line_mode = other.line_mode;
        self.vim_mode = other.vim_mode;
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

    /// The non-pending, non-overlapping selections. There could still be a pending
    /// selection that overlaps these if the mouse is being dragged, etc. Returned as
    /// selections over Anchors.
    pub fn disjoint_anchors(&self) -> Arc<[Selection<Anchor>]> {
        self.disjoint.clone()
    }

    pub fn disjoint_anchor_ranges(&self) -> impl Iterator<Item = Range<Anchor>> {
        // Mapping the Arc slice would borrow it, whereas indexing captures it.
        let disjoint = self.disjoint_anchors();
        (0..disjoint.len()).map(move |ix| disjoint[ix].range())
    }

    pub fn pending_anchor(&self) -> Option<Selection<Anchor>> {
        self.pending
            .as_ref()
            .map(|pending| pending.selection.clone())
    }

    pub fn pending<D: TextDimension + Ord + Sub<D, Output = D>>(
        &self,
        cx: &mut App,
    ) -> Option<Selection<D>> {
        let map = self.display_map(cx);
        let selection = resolve_selections(self.pending_anchor().as_ref(), &map).next();
        selection
    }

    pub(crate) fn pending_mode(&self) -> Option<SelectMode> {
        self.pending.as_ref().map(|pending| pending.mode.clone())
    }

    pub fn all<'a, D>(&self, cx: &mut App) -> Vec<Selection<D>>
    where
        D: 'a + TextDimension + Ord + Sub<D, Output = D>,
    {
        let map = self.display_map(cx);
        let disjoint_anchors = &self.disjoint;
        let mut disjoint = resolve_selections::<D, _>(disjoint_anchors.iter(), &map).peekable();
        let mut pending_opt = self.pending::<D>(cx);
        iter::from_fn(move || {
            if let Some(pending) = pending_opt.as_mut() {
                while let Some(next_selection) = disjoint.peek() {
                    if pending.start <= next_selection.end && pending.end >= next_selection.start {
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
    pub fn all_adjusted(&self, cx: &mut App) -> Vec<Selection<Point>> {
        let mut selections = self.all::<Point>(cx);
        if self.line_mode {
            let map = self.display_map(cx);
            for selection in &mut selections {
                let new_range = map.expand_to_line(selection.range());
                selection.start = new_range.start;
                selection.end = new_range.end;
            }
        }
        selections
    }

    /// Returns the newest selection, adjusted to take into account the selection line_mode
    pub fn newest_adjusted(&self, cx: &mut App) -> Selection<Point> {
        let mut selection = self.newest::<Point>(cx);
        if self.line_mode {
            let map = self.display_map(cx);
            let new_range = map.expand_to_line(selection.range());
            selection.start = new_range.start;
            selection.end = new_range.end;
        }
        selection
    }

    pub fn all_adjusted_display(
        &self,
        cx: &mut App,
    ) -> (DisplaySnapshot, Vec<Selection<DisplayPoint>>) {
        if self.line_mode {
            let selections = self.all::<Point>(cx);
            let map = self.display_map(cx);
            let result = selections
                .into_iter()
                .map(|mut selection| {
                    let new_range = map.expand_to_line(selection.range());
                    selection.start = new_range.start;
                    selection.end = new_range.end;
                    selection.map(|point| point.to_display_point(&map))
                })
                .collect();
            (map, result)
        } else {
            self.all_display(cx)
        }
    }

    pub fn disjoint_in_range<'a, D>(&self, range: Range<Anchor>, cx: &mut App) -> Vec<Selection<D>>
    where
        D: 'a + TextDimension + Ord + Sub<D, Output = D> + std::fmt::Debug,
    {
        let map = self.display_map(cx);
        let start_ix = match self
            .disjoint
            .binary_search_by(|probe| probe.end.cmp(&range.start, &map.buffer_snapshot))
        {
            Ok(ix) | Err(ix) => ix,
        };
        let end_ix = match self
            .disjoint
            .binary_search_by(|probe| probe.start.cmp(&range.end, &map.buffer_snapshot))
        {
            Ok(ix) => ix + 1,
            Err(ix) => ix,
        };
        resolve_selections(&self.disjoint[start_ix..end_ix], &map).collect()
    }

    pub fn all_display(&self, cx: &mut App) -> (DisplaySnapshot, Vec<Selection<DisplayPoint>>) {
        let map = self.display_map(cx);
        let disjoint_anchors = &self.disjoint;
        let mut disjoint = resolve_selections_display(disjoint_anchors.iter(), &map).peekable();
        let mut pending_opt =
            resolve_selections_display(self.pending_anchor().as_ref(), &map).next();
        let selections = iter::from_fn(move || {
            if let Some(pending) = pending_opt.as_mut() {
                while let Some(next_selection) = disjoint.peek() {
                    if pending.start <= next_selection.end && pending.end >= next_selection.start {
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
        .collect();
        (map, selections)
    }

    pub fn newest_anchor(&self) -> &Selection<Anchor> {
        self.pending
            .as_ref()
            .map(|s| &s.selection)
            .or_else(|| self.disjoint.iter().max_by_key(|s| s.id))
            .unwrap()
    }

    pub fn newest<D: TextDimension + Ord + Sub<D, Output = D>>(
        &self,
        cx: &mut App,
    ) -> Selection<D> {
        let map = self.display_map(cx);
        let selection = resolve_selections([self.newest_anchor()], &map)
            .next()
            .unwrap();
        selection
    }

    pub fn newest_display(&self, cx: &mut App) -> Selection<DisplayPoint> {
        let map = self.display_map(cx);
        let selection = resolve_selections_display([self.newest_anchor()], &map)
            .next()
            .unwrap();
        selection
    }

    pub fn oldest_anchor(&self) -> &Selection<Anchor> {
        self.disjoint
            .iter()
            .min_by_key(|s| s.id)
            .or_else(|| self.pending.as_ref().map(|p| &p.selection))
            .unwrap()
    }

    pub fn oldest<D: TextDimension + Ord + Sub<D, Output = D>>(
        &self,
        cx: &mut App,
    ) -> Selection<D> {
        let map = self.display_map(cx);
        let selection = resolve_selections([self.oldest_anchor()], &map)
            .next()
            .unwrap();
        selection
    }

    pub fn first_anchor(&self) -> Selection<Anchor> {
        self.pending
            .as_ref()
            .map(|pending| pending.selection.clone())
            .unwrap_or_else(|| self.disjoint.first().cloned().unwrap())
    }

    pub fn first<D: TextDimension + Ord + Sub<D, Output = D>>(&self, cx: &mut App) -> Selection<D> {
        self.all(cx).first().unwrap().clone()
    }

    pub fn last<D: TextDimension + Ord + Sub<D, Output = D>>(&self, cx: &mut App) -> Selection<D> {
        self.all(cx).last().unwrap().clone()
    }

    pub fn ranges<D: TextDimension + Ord + Sub<D, Output = D>>(
        &self,
        cx: &mut App,
    ) -> Vec<Range<D>> {
        self.all::<D>(cx)
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
    pub fn display_ranges(&self, cx: &mut App) -> Vec<Range<DisplayPoint>> {
        let display_map = self.display_map(cx);
        self.disjoint_anchors()
            .iter()
            .chain(self.pending_anchor().as_ref())
            .map(|s| {
                if s.reversed {
                    s.end.to_display_point(&display_map)..s.start.to_display_point(&display_map)
                } else {
                    s.start.to_display_point(&display_map)..s.end.to_display_point(&display_map)
                }
            })
            .collect()
    }

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
        if start_col < line_len || (is_empty && positions.start == line.width) {
            let start = DisplayPoint::new(row, start_col);
            let end_col = line.closest_index_for_x(positions.end) as u32;
            let end = DisplayPoint::new(row, end_col);

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
        } else {
            None
        }
    }

    pub fn change_with<R>(
        &mut self,
        cx: &mut App,
        change: impl FnOnce(&mut MutableSelectionsCollection) -> R,
    ) -> (bool, R) {
        let mut mutable_collection = MutableSelectionsCollection {
            collection: self,
            selections_changed: false,
            cx,
        };

        let result = change(&mut mutable_collection);
        assert!(
            !mutable_collection.disjoint.is_empty() || mutable_collection.pending.is_some(),
            "There must be at least one selection"
        );
        (mutable_collection.selections_changed, result)
    }
}

pub struct MutableSelectionsCollection<'a> {
    collection: &'a mut SelectionsCollection,
    selections_changed: bool,
    cx: &'a mut App,
}

impl<'a> MutableSelectionsCollection<'a> {
    pub fn display_map(&mut self) -> DisplaySnapshot {
        self.collection.display_map(self.cx)
    }

    pub fn buffer(&self) -> Ref<MultiBufferSnapshot> {
        self.collection.buffer(self.cx)
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

    pub fn clear_pending(&mut self) {
        if self.collection.pending.is_some() {
            self.collection.pending = None;
            self.selections_changed = true;
        }
    }

    pub(crate) fn set_pending_anchor_range(&mut self, range: Range<Anchor>, mode: SelectMode) {
        self.collection.pending = Some(PendingSelection {
            selection: Selection {
                id: post_inc(&mut self.collection.next_selection_id),
                start: range.start,
                end: range.end,
                reversed: false,
                goal: SelectionGoal::None,
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

        if !oldest.start.cmp(&oldest.end, &self.buffer()).is_eq() {
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
        T: 'a + ToOffset + ToPoint + TextDimension + Ord + Sub<T, Output = T> + std::marker::Copy,
    {
        let mut selections = self.collection.all(self.cx);
        let mut start = range.start.to_offset(&self.buffer());
        let mut end = range.end.to_offset(&self.buffer());
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

    pub fn select<T>(&mut self, mut selections: Vec<Selection<T>>)
    where
        T: ToOffset + ToPoint + Ord + std::marker::Copy + std::fmt::Debug,
    {
        let buffer = self.buffer.read(self.cx).snapshot(self.cx);
        selections.sort_unstable_by_key(|s| s.start);
        // Merge overlapping selections.
        let mut i = 1;
        while i < selections.len() {
            if selections[i - 1].end >= selections[i].start {
                let removed = selections.remove(i);
                if removed.start < selections[i - 1].start {
                    selections[i - 1].start = removed.start;
                }
                if removed.end > selections[i - 1].end {
                    selections[i - 1].end = removed.end;
                }
            } else {
                i += 1;
            }
        }

        self.collection.disjoint = Arc::from_iter(selections.into_iter().map(|selection| {
            let end_bias = if selection.end > selection.start {
                Bias::Left
            } else {
                Bias::Right
            };
            Selection {
                id: selection.id,
                start: buffer.anchor_after(selection.start),
                end: buffer.anchor_at(selection.end, end_bias),
                reversed: selection.reversed,
                goal: selection.goal,
            }
        }));

        self.collection.pending = None;
        self.selections_changed = true;
    }

    pub fn select_anchors(&mut self, selections: Vec<Selection<Anchor>>) {
        let map = self.display_map();
        let resolved_selections =
            resolve_selections::<usize, _>(&selections, &map).collect::<Vec<_>>();
        self.select(resolved_selections);
    }

    pub fn select_ranges<I, T>(&mut self, ranges: I)
    where
        I: IntoIterator<Item = Range<T>>,
        T: ToOffset,
    {
        let buffer = self.buffer.read(self.cx).snapshot(self.cx);
        let ranges = ranges
            .into_iter()
            .map(|range| range.start.to_offset(&buffer)..range.end.to_offset(&buffer));
        self.select_offset_ranges(ranges);
    }

    fn select_offset_ranges<I>(&mut self, ranges: I)
    where
        I: IntoIterator<Item = Range<usize>>,
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
        let buffer = self.buffer.read(self.cx).snapshot(self.cx);
        let selections = ranges
            .into_iter()
            .map(|range| {
                let mut start = range.start;
                let mut end = range.end;
                let reversed = if start.cmp(&end, &buffer).is_gt() {
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
        let display_map = self.display_map();
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
                    start: start.to_point(&display_map),
                    end: end.to_point(&display_map),
                    reversed,
                    goal: SelectionGoal::None,
                }
            })
            .collect();
        self.select(selections);
    }
    pub fn reverse_selections(&mut self) {
        let map = &self.display_map();
        let mut new_selections: Vec<Selection<Point>> = Vec::new();
        let disjoint = self.disjoint.clone();
        for selection in disjoint
            .iter()
            .sorted_by(|first, second| Ord::cmp(&second.id, &first.id))
            .collect::<Vec<&Selection<Anchor>>>()
        {
            new_selections.push(Selection {
                id: self.new_selection_id(),
                start: selection.start.to_display_point(map).to_point(map),
                end: selection.end.to_display_point(map).to_point(map),
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
        let display_map = self.display_map();
        let (_, selections) = self.collection.all_display(self.cx);
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
        mut move_selection: impl FnMut(&MultiBufferSnapshot, &mut Selection<usize>),
    ) {
        let mut changed = false;
        let snapshot = self.buffer().clone();
        let selections = self
            .collection
            .all::<usize>(self.cx)
            .into_iter()
            .map(|selection| {
                let mut moved_selection = selection.clone();
                move_selection(&snapshot, &mut moved_selection);
                if selection != moved_selection {
                    changed = true;
                }
                moved_selection
            })
            .collect();
        drop(snapshot);

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
        let display_map = self.display_map();
        let new_selections = find_replacement_cursors(&display_map)
            .into_iter()
            .map(|cursor| {
                let cursor_point = cursor.to_point(&display_map);
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
            let buffer = self.buffer();
            let disjoint_anchors = self
                .disjoint
                .iter()
                .flat_map(|selection| [&selection.start, &selection.end]);
            buffer.refresh_anchors(disjoint_anchors)
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
            let map = self.display_map();
            let resolved_selections = resolve_selections(adjusted_disjoint.iter(), &map).collect();
            self.select::<usize>(resolved_selections);
        }

        if let Some(pending) = pending.as_mut() {
            let buffer = self.buffer();
            let anchors =
                buffer.refresh_anchors([&pending.selection.start, &pending.selection.end]);
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

impl Deref for MutableSelectionsCollection<'_> {
    type Target = SelectionsCollection;
    fn deref(&self) -> &Self::Target {
        self.collection
    }
}

impl DerefMut for MutableSelectionsCollection<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.collection
    }
}

// Panics if passed selections are not in order
fn resolve_selections_display<'a>(
    selections: impl 'a + IntoIterator<Item = &'a Selection<Anchor>>,
    map: &'a DisplaySnapshot,
) -> impl 'a + Iterator<Item = Selection<DisplayPoint>> {
    let (to_summarize, selections) = selections.into_iter().tee();
    let mut summaries = map
        .buffer_snapshot
        .summaries_for_anchors::<Point, _>(to_summarize.flat_map(|s| [&s.start, &s.end]))
        .into_iter();
    let mut selections = selections
        .map(move |s| {
            let start = summaries.next().unwrap();
            let end = summaries.next().unwrap();

            let display_start = map.point_to_display_point(start, Bias::Left);
            let display_end = if start == end {
                map.point_to_display_point(end, Bias::Right)
            } else {
                map.point_to_display_point(end, Bias::Left)
            };

            Selection {
                id: s.id,
                start: display_start,
                end: display_end,
                reversed: s.reversed,
                goal: s.goal,
            }
        })
        .peekable();
    iter::from_fn(move || {
        let mut selection = selections.next()?;
        while let Some(next_selection) = selections.peek() {
            if selection.end >= next_selection.start {
                selection.end = cmp::max(selection.end, next_selection.end);
                selections.next();
            } else {
                break;
            }
        }
        Some(selection)
    })
}

// Panics if passed selections are not in order
pub(crate) fn resolve_selections<'a, D, I>(
    selections: I,
    map: &'a DisplaySnapshot,
) -> impl 'a + Iterator<Item = Selection<D>>
where
    D: TextDimension + Ord + Sub<D, Output = D>,
    I: 'a + IntoIterator<Item = &'a Selection<Anchor>>,
{
    let (to_convert, selections) = resolve_selections_display(selections, map).tee();
    let mut converted_endpoints =
        map.buffer_snapshot
            .dimensions_from_points::<D>(to_convert.flat_map(|s| {
                let start = map.display_point_to_point(s.start, Bias::Left);
                let end = map.display_point_to_point(s.end, Bias::Right);
                [start, end]
            }));
    selections.map(move |s| {
        let start = converted_endpoints.next().unwrap();
        let end = converted_endpoints.next().unwrap();
        Selection {
            id: s.id,
            start,
            end,
            reversed: s.reversed,
            goal: s.goal,
        }
    })
}
