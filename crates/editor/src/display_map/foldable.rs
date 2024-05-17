use gpui::AnyElement;
use multi_buffer::{Anchor, MultiBufferRow, MultiBufferSnapshot, ToPoint};
use std::{cmp::Ordering, ops::Range, rc::Rc};
use sum_tree::{Bias, SeekTarget, SumTree};
use text::Point;
use ui::WindowContext;

use crate::{DisplayMap, FoldStatus};

pub struct FoldableRangeId(usize);

#[derive(Default, Clone)]
pub struct FoldableRanges {
    foldables: SumTree<(FoldableRangeId, FoldableRange)>,
    next_id: FoldableRangeId,
}

#[derive(Clone)]
pub struct FoldableRange {
    pub range: Range<Anchor>,
    pub render_toggle: Rc<dyn Fn(bool, &mut WindowContext) -> AnyElement>,
}

impl FoldableRanges {
    pub fn insert(
        &mut self,
        ranges: impl IntoIterator<Item = FoldableRange>,
        snapshot: &MultiBufferSnapshot,
    ) -> Vec<FoldableRangeId> {
        let mut new_ids = Vec::new();
        self.foldables = {
            let mut new_foldables = SumTree::new();
            let mut cursor = self.foldables.cursor::<FoldableRangeSummary>();
            for range in ranges {
                let target = FoldableRangeSummary {
                    range: range.clone(),
                };
                new_foldables.append(cursor.slice(&target, Bias::Left, snapshot), snapshot);

                new_foldables.push(FoldableRange { range, id }, snapshot);
                new_ids.push(id);
            }
            new_foldables.append(cursor.suffix(snapshot), snapshot);
            new_foldables
        };
        new_ids
    }

    pub fn remove(
        &mut self,
        ids: &std::collections::HashSet<FoldableRangeId>,
        snapshot: &MultiBufferSnapshot,
    ) {
        self.foldables = {
            let mut new_foldables = SumTree::new();
            let mut cursor = self.foldables.cursor::<FoldableRangeSummary>();
            while let Some(item) = cursor.item() {
                if !ids.contains(&item.id) {
                    new_foldables.push(item.clone(), snapshot);
                }
                cursor.next(snapshot);
            }
            new_foldables
        };
    }

    /// Returns the first FoldableRange starting on the specified buffer row.
    pub fn query_row<'a>(
        &'a self,
        row: MultiBufferRow,
        snapshot: &'a MultiBufferSnapshot,
    ) -> Option<&'a FoldableRange> {
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

impl sum_tree::Item for FoldableRange {
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
