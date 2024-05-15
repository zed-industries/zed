use multi_buffer::{Anchor, MultiBufferSnapshot};
use std::{cmp::Ordering, iter, ops::Range};
use sum_tree::{Bias, SeekTarget, SumTree};
use text::Point;

#[derive(Default)]
pub struct FoldableRanges {
    ranges: SumTree<FoldableRange>,
}

impl FoldableRanges {
    pub fn insert(
        &mut self,
        ranges: impl IntoIterator<Item = Range<Anchor>>,
        snapshot: &MultiBufferSnapshot,
    ) {
        let mut cursor = self.ranges.cursor::<FoldableRangeSummary>();
        let mut new_ranges = SumTree::new();
        for range in ranges {
            let target = FoldableRangeSummary {
                range: range.clone(),
            };
            new_ranges.append(cursor.slice(&target, Bias::Left, snapshot), snapshot);
            new_ranges.push(FoldableRange { range }, snapshot);
        }
        new_ranges.append(cursor.suffix(snapshot), snapshot);
    }

    pub fn query<'a>(
        &'a self,
        visible_range: Range<Point>,
        snapshot: &'a MultiBufferSnapshot,
    ) -> impl 'a + Iterator<Item = &FoldableRange> {
        let start = snapshot.anchor_before(visible_range.start);
        let end = snapshot.anchor_after(visible_range.end);
        let mut cursor = self.ranges.cursor::<FoldableRangeSummary>();
        cursor.seek(
            &FoldableRangeSummary {
                range: start..Anchor::max(),
            },
            Bias::Left,
            snapshot,
        );

        iter::from_fn(move || {
            let item = cursor.item();
            cursor.next(snapshot);
            item
        })
        .take_while(move |item| item.range.start.cmp(&end, snapshot).is_le())
    }
}

#[derive(Clone)]
pub struct FoldableRange {
    range: Range<Anchor>,
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

impl SeekTarget<'_, FoldableRangeSummary, FoldableRangeSummary> for FoldableRangeSummary {
    fn cmp(&self, other: &FoldableRangeSummary, snapshot: &MultiBufferSnapshot) -> Ordering {
        other
            .range
            .start
            .cmp(&self.range.start, snapshot)
            .then_with(|| other.range.end.cmp(&self.range.end, snapshot).reverse())
    }
}
