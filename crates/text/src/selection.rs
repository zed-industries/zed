use crate::Anchor;
use crate::{rope::TextDimension, BufferSnapshot, ToOffset, ToPoint};
use std::{cmp::Ordering, ops::Range, sync::Arc};
use sum_tree::Bias;

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
    pub selections: Arc<[Selection<Anchor>]>,
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

impl<T: ToOffset + ToPoint + Copy + Ord> Selection<T> {
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn set_head(&mut self, head: T) {
        if head.cmp(&self.tail()) < Ordering::Equal {
            if !self.reversed {
                self.end = self.start;
                self.reversed = true;
            }
            self.start = head;
        } else {
            if self.reversed {
                self.start = self.end;
                self.reversed = false;
            }
            self.end = head;
        }
    }
}

impl Selection<Anchor> {
    pub fn resolve<'a, D: 'a + TextDimension>(
        &'a self,
        snapshot: &'a BufferSnapshot,
    ) -> Selection<D> {
        Selection {
            id: self.id,
            start: snapshot.summary_for_anchor(&self.start),
            end: snapshot.summary_for_anchor(&self.end),
            reversed: self.reversed,
            goal: self.goal,
        }
    }
}

impl SelectionSet {
    pub fn len(&self) -> usize {
        self.selections.len()
    }

    pub fn selections<'a, D>(
        &'a self,
        snapshot: &'a BufferSnapshot,
    ) -> impl 'a + Iterator<Item = Selection<D>>
    where
        D: TextDimension,
    {
        let anchors = self
            .selections
            .iter()
            .flat_map(|selection| [&selection.start, &selection.end].into_iter());
        let mut positions = snapshot.summaries_for_anchors::<D, _>(anchors);
        self.selections.iter().map(move |selection| Selection {
            start: positions.next().unwrap(),
            end: positions.next().unwrap(),
            goal: selection.goal,
            reversed: selection.reversed,
            id: selection.id,
        })
    }

    pub fn intersecting_selections<'a, D, I>(
        &'a self,
        range: Range<(I, Bias)>,
        snapshot: &'a BufferSnapshot,
    ) -> impl 'a + Iterator<Item = Selection<D>>
    where
        D: TextDimension,
        I: 'a + ToOffset,
    {
        let start = snapshot.anchor_at(range.start.0, range.start.1);
        let end = snapshot.anchor_at(range.end.0, range.end.1);
        let start_ix = match self
            .selections
            .binary_search_by(|probe| probe.end.cmp(&start, snapshot).unwrap())
        {
            Ok(ix) | Err(ix) => ix,
        };
        let end_ix = match self
            .selections
            .binary_search_by(|probe| probe.start.cmp(&end, snapshot).unwrap())
        {
            Ok(ix) | Err(ix) => ix,
        };
        self.selections[start_ix..end_ix]
            .iter()
            .map(|s| s.resolve(snapshot))
    }

    pub fn oldest_selection<'a, D>(&'a self, snapshot: &'a BufferSnapshot) -> Option<Selection<D>>
    where
        D: TextDimension,
    {
        self.selections
            .iter()
            .min_by_key(|s| s.id)
            .map(|s| s.resolve(snapshot))
    }

    pub fn newest_selection<'a, D>(&'a self, snapshot: &'a BufferSnapshot) -> Option<Selection<D>>
    where
        D: TextDimension,
    {
        self.selections
            .iter()
            .max_by_key(|s| s.id)
            .map(|s| s.resolve(snapshot))
    }
}
