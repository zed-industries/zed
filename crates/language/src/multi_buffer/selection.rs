use super::{Anchor, MultiBufferSnapshot, ToOffset};
use std::{
    ops::{Range, Sub},
    sync::Arc,
};
use sum_tree::Bias;
use text::{rope::TextDimension, Selection, SelectionSetId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectionSet {
    pub id: SelectionSetId,
    pub active: bool,
    pub selections: Arc<[Selection<Anchor>]>,
}

impl SelectionSet {
    pub fn len(&self) -> usize {
        self.selections.len()
    }

    pub fn selections<'a, D>(
        &'a self,
        snapshot: &'a MultiBufferSnapshot,
    ) -> impl 'a + Iterator<Item = Selection<D>>
    where
        D: TextDimension + Ord + Sub<D, Output = D>,
    {
        resolve_selections(&self.selections, snapshot)
    }

    pub fn intersecting_selections<'a, D, I>(
        &'a self,
        range: Range<(I, Bias)>,
        snapshot: &'a MultiBufferSnapshot,
    ) -> impl 'a + Iterator<Item = Selection<D>>
    where
        D: TextDimension + Ord + Sub<D, Output = D>,
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
        resolve_selections(&self.selections[start_ix..end_ix], snapshot)
    }

    pub fn oldest_selection<'a, D>(
        &'a self,
        snapshot: &'a MultiBufferSnapshot,
    ) -> Option<Selection<D>>
    where
        D: TextDimension + Ord + Sub<D, Output = D>,
    {
        self.selections
            .iter()
            .min_by_key(|selection| selection.id)
            .map(|selection| resolve_selection(selection, snapshot))
    }

    pub fn newest_selection<'a, D>(
        &'a self,
        snapshot: &'a MultiBufferSnapshot,
    ) -> Option<Selection<D>>
    where
        D: TextDimension + Ord + Sub<D, Output = D>,
    {
        self.selections
            .iter()
            .max_by_key(|selection| selection.id)
            .map(|selection| resolve_selection(selection, snapshot))
    }
}

fn resolve_selection<'a, D>(
    selection: &'a Selection<Anchor>,
    snapshot: &'a MultiBufferSnapshot,
) -> Selection<D>
where
    D: TextDimension + Ord + Sub<D, Output = D>,
{
    Selection {
        id: selection.id,
        start: selection.start.summary::<D>(snapshot),
        end: selection.end.summary::<D>(snapshot),
        reversed: selection.reversed,
        goal: selection.goal,
    }
}

fn resolve_selections<'a, D>(
    selections: &'a [Selection<Anchor>],
    snapshot: &'a MultiBufferSnapshot,
) -> impl 'a + Iterator<Item = Selection<D>>
where
    D: TextDimension + Ord + Sub<D, Output = D>,
{
    let mut summaries = snapshot
        .summaries_for_anchors::<D, _>(
            selections
                .iter()
                .flat_map(|selection| [&selection.start, &selection.end]),
        )
        .into_iter();
    selections.iter().map(move |selection| Selection {
        id: selection.id,
        start: summaries.next().unwrap(),
        end: summaries.next().unwrap(),
        reversed: selection.reversed,
        goal: selection.goal,
    })
}
