use sum_tree::Bias;

use crate::{rope::TextDimension, Snapshot};

use super::{AnchorRangeMap, ToOffset};
use std::{cmp::Ordering, ops::Range, sync::Arc};

pub type SelectionSetId = clock::Lamport;
pub type SelectionsVersion = usize;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SelectionGoal {
    None,
    Column(u32),
    ColumnRange { start: u32, end: u32 },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Selection<T> {
    pub id: usize,
    pub start: T,
    pub end: T,
    pub reversed: bool,
    pub goal: SelectionGoal,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectionSet {
    pub id: SelectionSetId,
    pub active: bool,
    pub selections: Arc<AnchorRangeMap<SelectionState>>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct SelectionState {
    pub id: usize,
    pub reversed: bool,
    pub goal: SelectionGoal,
}

impl<T: Clone> Selection<T> {
    pub fn head(&self) -> T {
        if self.reversed {
            self.start.clone()
        } else {
            self.end.clone()
        }
    }

    pub fn tail(&self) -> T {
        if self.reversed {
            self.end.clone()
        } else {
            self.start.clone()
        }
    }
}

impl<T: Clone + Ord> Selection<T> {
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn set_head(&mut self, head: T) {
        if head.cmp(&self.tail()) < Ordering::Equal {
            if !self.reversed {
                self.end = self.start.clone();
                self.reversed = true;
            }
            self.start = head;
        } else {
            if self.reversed {
                self.start = self.end.clone();
                self.reversed = false;
            }
            self.end = head;
        }
    }
}

impl SelectionSet {
    pub fn len(&self) -> usize {
        self.selections.len()
    }

    pub fn selections<'a, D>(
        &'a self,
        content: &'a Snapshot,
    ) -> impl 'a + Iterator<Item = Selection<D>>
    where
        D: TextDimension,
    {
        self.selections
            .ranges(content)
            .map(|(range, state)| Selection {
                id: state.id,
                start: range.start,
                end: range.end,
                reversed: state.reversed,
                goal: state.goal,
            })
    }

    pub fn intersecting_selections<'a, D, I>(
        &'a self,
        range: Range<(I, Bias)>,
        content: &'a Snapshot,
    ) -> impl 'a + Iterator<Item = Selection<D>>
    where
        D: TextDimension,
        I: 'a + ToOffset,
    {
        self.selections
            .intersecting_ranges(range, content)
            .map(|(range, state)| Selection {
                id: state.id,
                start: range.start,
                end: range.end,
                reversed: state.reversed,
                goal: state.goal,
            })
    }

    pub fn oldest_selection<'a, D>(&'a self, content: &'a Snapshot) -> Option<Selection<D>>
    where
        D: TextDimension,
    {
        self.selections
            .min_by_key(content, |selection| selection.id)
            .map(|(range, state)| Selection {
                id: state.id,
                start: range.start,
                end: range.end,
                reversed: state.reversed,
                goal: state.goal,
            })
    }

    pub fn newest_selection<'a, D>(&'a self, content: &'a Snapshot) -> Option<Selection<D>>
    where
        D: TextDimension,
    {
        self.selections
            .max_by_key(content, |selection| selection.id)
            .map(|(range, state)| Selection {
                id: state.id,
                start: range.start,
                end: range.end,
                reversed: state.reversed,
                goal: state.goal,
            })
    }
}
