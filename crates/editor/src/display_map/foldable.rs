use gpui::AnyElement;
use multi_buffer::{Anchor, MultiBufferRow, MultiBufferSnapshot, ToPoint};
use std::{cmp::Ordering, ops::Range, sync::Arc};
use sum_tree::{Bias, SeekTarget, SumTree};
use text::Point;
use ui::{IntoElement, WindowContext};

use crate::{DisplayMap, FoldStatus, RenderFoldToggle};

#[derive(Default, Clone)]
pub struct FoldableRanges {
    foldables: SumTree<Foldable>,
}

impl FoldableRanges {
    pub fn insert(
        &mut self,
        foldables: impl IntoIterator<Item = Foldable>,
        snapshot: &MultiBufferSnapshot,
    ) {
        let mut cursor = self.foldables.cursor::<FoldableRangeSummary>();
        let mut new_foldables = SumTree::new();
        for foldable in foldables {
            let target = FoldableRangeSummary {
                range: foldable.range.clone(),
            };
            new_foldables.append(cursor.slice(&target, Bias::Left, snapshot), snapshot);
            new_foldables.push(foldable, snapshot);
        }
        new_foldables.append(cursor.suffix(snapshot), snapshot);
    }

    /// Returns the first FoldableRange starting on the specified buffer row.
    pub fn query<'a>(
        &'a self,
        row: MultiBufferRow,
        snapshot: &'a MultiBufferSnapshot,
    ) -> Option<&'a Foldable> {
        let start = snapshot.anchor_before(Point::new(row.0, 0));
        let mut cursor = self.foldables.cursor::<FoldableRangeSummary>();
        cursor.seek(&start, Bias::Left, snapshot);
        if let Some(item) = cursor.item() {
            if item.range.start.to_point(snapshot).row == row.0 {
                return Some(item);
            }
        }
        return None;
    }
}

pub struct FoldableId(usize);

#[derive(Clone)]
pub struct Foldable {
    pub range: Range<Anchor>,
    pub id: FoldableId,
}

impl Foldable {
    pub fn new<F, E>(range: Range<Anchor>, toggle: F) -> Foldable
    where
        F: 'static + Fn(FoldStatus, &mut WindowContext) -> E,
        E: IntoElement,
    {
        Foldable {
            range,
            toggle: Some(Arc::new(move |fs, cx| toggle(fs, cx).into_any_element())),
        }
    }

    pub fn render(
        &self,
        fold_status: FoldStatus,
        map: &DisplayMap,
        cx: &mut WindowContext,
    ) -> AnyElement {
        map.foldables.render_fold_toggle(self.id, fold_status, cx)
    }
}

#[derive(Debug, Clone)]
pub struct FoldableRangeSummary {
    range: Range<Anchor>,
}

impl Default for FoldableRangeSummary {
    fn default() -> Self {
        Self {
            range: Anchor::max()..Anchor::min(),
        }
    }
}

impl sum_tree::Summary for FoldableRangeSummary {
    type Context = MultiBufferSnapshot;

    fn add_summary(&mut self, other: &Self, snapshot: &Self::Context) {
        if other.range.start.cmp(&self.range.start, snapshot) == Ordering::Less {
            self.range.start = other.range.start;
        }
        if other.range.end.cmp(&self.range.end, snapshot) == Ordering::Greater {
            self.range.end = other.range.end;
        }
    }
}

impl sum_tree::Item for Foldable {
    type Summary = FoldableRangeSummary;

    fn summary(&self) -> Self::Summary {
        todo!()
    }
}

impl SeekTarget<'_, FoldableRangeSummary, FoldableRangeSummary> for Anchor {
    fn cmp(&self, other: &FoldableRangeSummary, snapshot: &MultiBufferSnapshot) -> Ordering {
        other.range.start.cmp(&self, snapshot)
    }
}

impl SeekTarget<'_, FoldableRangeSummary, FoldableRangeSummary> for FoldableRangeSummary {
    fn cmp(&self, other: &FoldableRangeSummary, snapshot: &MultiBufferSnapshot) -> Ordering {
        other
            .range
            .start
            .cmp(&self.range.start, snapshot)
            .then_with(|| other.range.end.cmp(&self.range.end, snapshot).reverse())
    }
}
