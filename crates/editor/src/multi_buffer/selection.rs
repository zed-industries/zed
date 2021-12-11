use super::{Anchor, MultiBufferSnapshot};
use std::ops::Sub;
use text::{rope::TextDimension, Selection};

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
