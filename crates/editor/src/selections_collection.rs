use std::{
    iter, mem,
    ops::{Deref, Range, Sub},
    sync::Arc,
};

use collections::HashMap;
use gpui::{AppContext, ModelHandle, MutableAppContext};
use itertools::Itertools;
use language::{rope::TextDimension, Bias, Point, Selection, SelectionGoal, ToPoint};
use util::post_inc;

use crate::{
    display_map::{DisplayMap, DisplaySnapshot, ToDisplayPoint},
    Anchor, DisplayPoint, ExcerptId, MultiBuffer, MultiBufferSnapshot, SelectMode, ToOffset,
};

#[derive(Clone)]
pub struct PendingSelection {
    pub selection: Selection<Anchor>,
    pub mode: SelectMode,
}

pub struct SelectionsCollection {
    display_map: ModelHandle<DisplayMap>,
    buffer: ModelHandle<MultiBuffer>,
    pub next_selection_id: usize,
    disjoint: Arc<[Selection<Anchor>]>,
    pending: Option<PendingSelection>,
}

impl SelectionsCollection {
    pub fn new(display_map: ModelHandle<DisplayMap>, buffer: ModelHandle<MultiBuffer>) -> Self {
        Self {
            display_map,
            buffer,
            next_selection_id: 1,
            disjoint: Arc::from([]),
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

    fn display_map(&self, cx: &mut MutableAppContext) -> DisplaySnapshot {
        self.display_map.update(cx, |map, cx| map.snapshot(cx))
    }

    fn buffer(&self, cx: &AppContext) -> MultiBufferSnapshot {
        self.buffer.read(cx).snapshot(cx)
    }

    pub fn count<'a>(&self) -> usize {
        let mut count = self.disjoint.len();
        if self.pending.is_some() {
            count += 1;
        }
        count
    }

    pub fn disjoint_anchors(&self) -> Arc<[Selection<Anchor>]> {
        self.disjoint.clone()
    }

    pub fn pending_anchor(&self) -> Option<Selection<Anchor>> {
        self.pending
            .as_ref()
            .map(|pending| pending.selection.clone())
    }

    pub fn pending<D: TextDimension + Ord + Sub<D, Output = D>>(
        &self,
        cx: &AppContext,
    ) -> Option<Selection<D>> {
        self.pending_anchor()
            .as_ref()
            .map(|pending| pending.map(|p| p.summary::<D>(&self.buffer(cx))))
    }

    pub fn pending_mode(&self) -> Option<SelectMode> {
        self.pending.as_ref().map(|pending| pending.mode.clone())
    }

    pub fn interleaved<'a, D>(&self, cx: &AppContext) -> Vec<Selection<D>>
    where
        D: 'a + TextDimension + Ord + Sub<D, Output = D> + std::fmt::Debug,
    {
        let disjoint_anchors = &self.disjoint;
        let mut disjoint =
            resolve_multiple::<D, _>(disjoint_anchors.iter(), &self.buffer(cx)).peekable();

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

    pub fn interleaved_in_range<'a>(
        &self,
        range: Range<Anchor>,
        cx: &AppContext,
    ) -> Vec<Selection<Point>> {
        let buffer = self.buffer(cx);
        let start_ix = match self
            .disjoint
            .binary_search_by(|probe| probe.end.cmp(&range.start, &buffer))
        {
            Ok(ix) | Err(ix) => ix,
        };
        let end_ix = match self
            .disjoint
            .binary_search_by(|probe| probe.start.cmp(&range.end, &buffer))
        {
            Ok(ix) => ix + 1,
            Err(ix) => ix,
        };

        fn point_selection(
            selection: &Selection<Anchor>,
            buffer: &MultiBufferSnapshot,
        ) -> Selection<Point> {
            let start = crate::ToPoint::to_point(&selection.start, &buffer);
            let end = crate::ToPoint::to_point(&selection.end, &buffer);
            Selection {
                id: selection.id,
                start,
                end,
                reversed: selection.reversed,
                goal: selection.goal,
            }
        }

        self.disjoint[start_ix..end_ix]
            .iter()
            .chain(self.pending.as_ref().map(|pending| &pending.selection))
            .map(|s| point_selection(s, &buffer))
            .collect()
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
        cx: &AppContext,
    ) -> Selection<D> {
        resolve(self.newest_anchor(), &self.buffer(cx))
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
        cx: &AppContext,
    ) -> Selection<D> {
        resolve(self.oldest_anchor(), &self.buffer(cx))
    }

    pub fn first<D: TextDimension + Ord + Sub<D, Output = D>>(
        &self,
        cx: &AppContext,
    ) -> Selection<D> {
        self.interleaved(cx).first().unwrap().clone()
    }

    pub fn last<D: TextDimension + Ord + Sub<D, Output = D>>(
        &self,
        cx: &AppContext,
    ) -> Selection<D> {
        self.interleaved(cx).last().unwrap().clone()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn selected_ranges<D: TextDimension + Ord + Sub<D, Output = D> + std::fmt::Debug>(
        &self,
        cx: &AppContext,
    ) -> Vec<Range<D>> {
        self.interleaved::<D>(cx)
            .iter()
            .map(|s| {
                if s.reversed {
                    s.end.clone()..s.start.clone()
                } else {
                    s.start.clone()..s.end.clone()
                }
            })
            .collect()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn selected_display_ranges(&self, cx: &mut MutableAppContext) -> Vec<Range<DisplayPoint>> {
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

    pub(crate) fn change_with<R>(
        &mut self,
        cx: &mut MutableAppContext,
        change: impl FnOnce(&mut MutableSelectionsCollection) -> R,
    ) -> R {
        let mut mutable_collection = MutableSelectionsCollection {
            collection: self,
            cx,
        };

        let result = change(&mut mutable_collection);
        assert!(
            !mutable_collection.disjoint.is_empty() || mutable_collection.pending.is_some(),
            "There must be at least one selection"
        );
        result
    }
}

pub struct MutableSelectionsCollection<'a> {
    collection: &'a mut SelectionsCollection,
    cx: &'a mut MutableAppContext,
}

impl<'a> MutableSelectionsCollection<'a> {
    fn display_map(&mut self) -> DisplaySnapshot {
        self.collection.display_map(self.cx)
    }

    fn buffer(&mut self) -> MultiBufferSnapshot {
        self.collection.buffer(self.cx)
    }

    pub fn clear_disjoint(&mut self) {
        self.collection.disjoint = Arc::from([]);
    }

    pub fn delete(&mut self, selection_id: usize) {
        self.collection.disjoint = self
            .disjoint
            .into_iter()
            .filter(|selection| selection.id != selection_id)
            .cloned()
            .collect();
    }

    pub fn clear_pending(&mut self) {
        self.collection.pending = None;
    }

    pub fn set_pending_range(&mut self, range: Range<Anchor>, mode: SelectMode) {
        self.collection.pending = Some(PendingSelection {
            selection: Selection {
                id: post_inc(&mut self.collection.next_selection_id),
                start: range.start,
                end: range.end,
                reversed: false,
                goal: SelectionGoal::None,
            },
            mode,
        })
    }
    pub fn pending_mut(&mut self) -> &mut Option<PendingSelection> {
        &mut self.collection.pending
    }

    pub fn try_cancel(&mut self) -> bool {
        let buffer = self.buffer.read(self.cx).snapshot(self.cx);
        if let Some(pending) = self.collection.pending.take() {
            if self.disjoint.is_empty() {
                self.collection.disjoint = Arc::from([pending.selection]);
            }
            return true;
        }

        let mut oldest = self.oldest_anchor().clone();
        if self.count() > 1 {
            self.collection.disjoint = Arc::from([oldest]);
            return true;
        }

        if !oldest.start.cmp(&oldest.end, &buffer).is_eq() {
            let head = oldest.head();
            oldest.start = head.clone();
            oldest.end = head;
            self.collection.disjoint = Arc::from([oldest]);
            return true;
        }

        return false;
    }

    pub fn reset_biases(&mut self) {
        let buffer = self.buffer.read(self.cx).snapshot(self.cx);
        self.collection.disjoint = self
            .collection
            .disjoint
            .into_iter()
            .cloned()
            .map(|selection| reset_biases(selection, &buffer))
            .collect();

        if let Some(pending) = self.collection.pending.as_mut() {
            pending.selection = reset_biases(pending.selection.clone(), &buffer);
        }
    }

    pub fn insert_range<T>(&mut self, range: Range<T>)
    where
        T: 'a + ToOffset + ToPoint + TextDimension + Ord + Sub<T, Output = T> + std::marker::Copy,
    {
        let mut selections = self.interleaved(self.cx);
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
    }

    pub fn select_anchors(&mut self, selections: Vec<Selection<Anchor>>) {
        let buffer = self.buffer.read(self.cx).snapshot(self.cx);
        let resolved_selections =
            resolve_multiple::<usize, _>(&selections, &buffer).collect::<Vec<_>>();
        self.select(resolved_selections);
    }

    pub fn select_ranges<I, T>(&mut self, ranges: I)
    where
        I: IntoIterator<Item = Range<T>>,
        T: ToOffset,
    {
        let buffer = self.buffer.read(self.cx).snapshot(self.cx);
        let selections = ranges
            .into_iter()
            .map(|range| {
                let mut start = range.start.to_offset(&buffer);
                let mut end = range.end.to_offset(&buffer);
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

    pub fn select_anchor_ranges<I: IntoIterator<Item = Range<Anchor>>>(&mut self, ranges: I) {
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

    #[cfg(any(test, feature = "test-support"))]
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

    pub fn move_with(
        &mut self,
        mut move_selection: impl FnMut(&DisplaySnapshot, &mut Selection<DisplayPoint>),
    ) {
        let display_map = self.display_map();
        let selections = self
            .interleaved::<Point>(self.cx)
            .into_iter()
            .map(|selection| {
                let mut selection = selection.map(|point| point.to_display_point(&display_map));
                move_selection(&display_map, &mut selection);
                selection.map(|display_point| display_point.to_point(&display_map))
            })
            .collect();

        self.select(selections)
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

    pub fn replace_cursors_with(
        &mut self,
        mut find_replacement_cursors: impl FnMut(&DisplaySnapshot) -> Vec<DisplayPoint>,
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
        // TODO: Pull disjoint constraint out of update_selections so we don't have to
        // store the pending_selection here.
        let buffer = self.buffer.read(self.cx).snapshot(self.cx);
        let mut pending = self.collection.pending.take();
        let mut selections_with_lost_position = HashMap::default();

        let anchors_with_status = buffer.refresh_anchors(
            self.disjoint
                .iter()
                .flat_map(|selection| [&selection.start, &selection.end]),
        );
        let adjusted_disjoint: Vec<_> = anchors_with_status
            .chunks(2)
            .map(|selection_anchors| {
                let (anchor_ix, start, kept_start) = selection_anchors[0].clone();
                let (_, end, kept_end) = selection_anchors[1].clone();
                let selection = &self.disjoint[anchor_ix / 2];
                let kept_head = if selection.reversed {
                    kept_start
                } else {
                    kept_end
                };
                if !kept_head {
                    selections_with_lost_position
                        .insert(selection.id, selection.head().excerpt_id.clone());
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
            self.select::<usize>(resolve_multiple(adjusted_disjoint.iter(), &buffer).collect());
        }

        if let Some(pending) = pending.as_mut() {
            let anchors =
                buffer.refresh_anchors([&pending.selection.start, &pending.selection.end]);
            let (_, start, kept_start) = anchors[0].clone();
            let (_, end, kept_end) = anchors[1].clone();
            let kept_head = if pending.selection.reversed {
                kept_start
            } else {
                kept_end
            };
            if !kept_head {
                selections_with_lost_position.insert(
                    pending.selection.id,
                    pending.selection.head().excerpt_id.clone(),
                );
            }

            pending.selection.start = start;
            pending.selection.end = end;
        }
        self.collection.pending = pending;

        selections_with_lost_position
    }
}

impl<'a> Deref for MutableSelectionsCollection<'a> {
    type Target = SelectionsCollection;
    fn deref(&self) -> &Self::Target {
        self.collection
    }
}

// Panics if passed selections are not in order
pub fn resolve_multiple<'a, D, I>(
    selections: I,
    snapshot: &MultiBufferSnapshot,
) -> impl 'a + Iterator<Item = Selection<D>>
where
    D: TextDimension + Ord + Sub<D, Output = D> + std::fmt::Debug,
    I: 'a + IntoIterator<Item = &'a Selection<Anchor>>,
{
    let (to_summarize, selections) = selections.into_iter().tee();
    let mut summaries = snapshot
        .summaries_for_anchors::<D, _>(
            to_summarize
                .flat_map(|s| [&s.start, &s.end])
                .collect::<Vec<_>>(),
        )
        .into_iter();
    selections.map(move |s| Selection {
        id: s.id,
        start: summaries.next().unwrap(),
        end: summaries.next().unwrap(),
        reversed: s.reversed,
        goal: s.goal,
    })
}

fn resolve<D: TextDimension + Ord + Sub<D, Output = D>>(
    selection: &Selection<Anchor>,
    buffer: &MultiBufferSnapshot,
) -> Selection<D> {
    selection.map(|p| p.summary::<D>(&buffer))
}

fn reset_biases(
    mut selection: Selection<Anchor>,
    buffer: &MultiBufferSnapshot,
) -> Selection<Anchor> {
    let end_bias = if selection.end.to_offset(buffer) > selection.start.to_offset(buffer) {
        Bias::Left
    } else {
        Bias::Right
    };
    selection.start = buffer.anchor_after(selection.start);
    selection.end = buffer.anchor_at(selection.end, end_bias);
    selection
}
