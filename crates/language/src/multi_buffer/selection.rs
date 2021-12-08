use super::{anchor::AnchorRangeMap, MultiBufferSnapshot, ToOffset};
use std::{ops::Range, sync::Arc};
use sum_tree::Bias;
use text::{rope::TextDimension, Selection, SelectionSetId, SelectionState};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectionSet {
    pub id: SelectionSetId,
    pub active: bool,
    pub selections: Arc<AnchorRangeMap<SelectionState>>,
}

impl SelectionSet {
    pub fn len(&self) -> usize {
        self.selections.len()
    }

    pub fn selections<'a, D>(
        &'a self,
        content: &'a MultiBufferSnapshot,
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
        content: &'a MultiBufferSnapshot,
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

    pub fn oldest_selection<'a, D>(
        &'a self,
        content: &'a MultiBufferSnapshot,
    ) -> Option<Selection<D>>
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

    pub fn newest_selection<'a, D>(
        &'a self,
        content: &'a MultiBufferSnapshot,
    ) -> Option<Selection<D>>
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
