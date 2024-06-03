use collections::HashMap;
use gpui::{AnyElement, IntoElement};
use multi_buffer::{Anchor, AnchorRangeExt, MultiBufferRow, MultiBufferSnapshot, ToPoint};
use std::{cmp::Ordering, ops::Range, sync::Arc};
use sum_tree::{Bias, SeekTarget, SumTree};
use text::Point;
use ui::WindowContext;

use crate::FoldPlaceholder;

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct FlapId(usize);

#[derive(Default)]
pub struct FlapMap {
    snapshot: FlapSnapshot,
    next_id: FlapId,
    id_to_range: HashMap<FlapId, Range<Anchor>>,
}

#[derive(Clone, Default)]
pub struct FlapSnapshot {
    flaps: SumTree<FlapItem>,
}

impl FlapSnapshot {
    /// Returns the first Flap starting on the specified buffer row.
    pub fn query_row<'a>(
        &'a self,
        row: MultiBufferRow,
        snapshot: &'a MultiBufferSnapshot,
    ) -> Option<&'a Flap> {
        let start = snapshot.anchor_before(Point::new(row.0, 0));
        let mut cursor = self.flaps.cursor::<ItemSummary>();
        cursor.seek(&start, Bias::Left, snapshot);
        while let Some(item) = cursor.item() {
            match Ord::cmp(&item.flap.range.start.to_point(snapshot).row, &row.0) {
                Ordering::Less => cursor.next(snapshot),
                Ordering::Equal => {
                    if item.flap.range.start.is_valid(snapshot) {
                        return Some(&item.flap);
                    } else {
                        cursor.next(snapshot);
                    }
                }
                Ordering::Greater => break,
            }
        }
        return None;
    }

    pub fn flap_items_with_offsets(
        &self,
        snapshot: &MultiBufferSnapshot,
    ) -> Vec<(FlapId, Range<Point>)> {
        let mut cursor = self.flaps.cursor::<ItemSummary>();
        let mut results = Vec::new();

        cursor.next(snapshot);
        while let Some(item) = cursor.item() {
            let start_point = item.flap.range.start.to_point(snapshot);
            let end_point = item.flap.range.end.to_point(snapshot);
            results.push((item.id, start_point..end_point));
            cursor.next(snapshot);
        }

        results
    }
}

type RenderToggleFn = Arc<
    dyn Send
        + Sync
        + Fn(
            MultiBufferRow,
            bool,
            Arc<dyn Send + Sync + Fn(bool, &mut WindowContext)>,
            &mut WindowContext,
        ) -> AnyElement,
>;
type RenderTrailerFn =
    Arc<dyn Send + Sync + Fn(MultiBufferRow, bool, &mut WindowContext) -> AnyElement>;

#[derive(Clone)]
pub struct Flap {
    pub range: Range<Anchor>,
    pub placeholder: FoldPlaceholder,
    pub render_toggle: RenderToggleFn,
    pub render_trailer: RenderTrailerFn,
}

impl Flap {
    pub fn new<RenderToggle, ToggleElement, RenderTrailer, TrailerElement>(
        range: Range<Anchor>,
        placeholder: FoldPlaceholder,
        render_toggle: RenderToggle,
        render_trailer: RenderTrailer,
    ) -> Self
    where
        RenderToggle: 'static
            + Send
            + Sync
            + Fn(
                MultiBufferRow,
                bool,
                Arc<dyn Send + Sync + Fn(bool, &mut WindowContext)>,
                &mut WindowContext,
            ) -> ToggleElement
            + 'static,
        ToggleElement: IntoElement,
        RenderTrailer: 'static
            + Send
            + Sync
            + Fn(MultiBufferRow, bool, &mut WindowContext) -> TrailerElement
            + 'static,
        TrailerElement: IntoElement,
    {
        Flap {
            range,
            placeholder,
            render_toggle: Arc::new(move |row, folded, toggle, cx| {
                render_toggle(row, folded, toggle, cx).into_any_element()
            }),
            render_trailer: Arc::new(move |row, folded, cx| {
                render_trailer(row, folded, cx).into_any_element()
            }),
        }
    }
}

impl std::fmt::Debug for Flap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Flap").field("range", &self.range).finish()
    }
}

#[derive(Clone, Debug)]
struct FlapItem {
    id: FlapId,
    flap: Flap,
}

impl FlapMap {
    pub fn snapshot(&self) -> FlapSnapshot {
        self.snapshot.clone()
    }

    pub fn insert(
        &mut self,
        flaps: impl IntoIterator<Item = Flap>,
        snapshot: &MultiBufferSnapshot,
    ) -> Vec<FlapId> {
        let mut new_ids = Vec::new();
        self.snapshot.flaps = {
            let mut new_flaps = SumTree::new();
            let mut cursor = self.snapshot.flaps.cursor::<ItemSummary>();
            for flap in flaps {
                new_flaps.append(cursor.slice(&flap.range, Bias::Left, snapshot), snapshot);

                let id = self.next_id;
                self.next_id.0 += 1;
                self.id_to_range.insert(id, flap.range.clone());
                new_flaps.push(FlapItem { flap, id }, snapshot);
                new_ids.push(id);
            }
            new_flaps.append(cursor.suffix(snapshot), snapshot);
            new_flaps
        };
        new_ids
    }

    pub fn remove(
        &mut self,
        ids: impl IntoIterator<Item = FlapId>,
        snapshot: &MultiBufferSnapshot,
    ) {
        let mut removals = Vec::new();
        for id in ids {
            if let Some(range) = self.id_to_range.remove(&id) {
                removals.push((id, range.clone()));
            }
        }
        removals.sort_unstable_by(|(a_id, a_range), (b_id, b_range)| {
            AnchorRangeExt::cmp(a_range, b_range, snapshot).then(b_id.cmp(&a_id))
        });

        self.snapshot.flaps = {
            let mut new_flaps = SumTree::new();
            let mut cursor = self.snapshot.flaps.cursor::<ItemSummary>();

            for (id, range) in removals {
                new_flaps.append(cursor.slice(&range, Bias::Left, snapshot), snapshot);
                while let Some(item) = cursor.item() {
                    cursor.next(snapshot);
                    if item.id == id {
                        break;
                    } else {
                        new_flaps.push(item.clone(), snapshot);
                    }
                }
            }

            new_flaps.append(cursor.suffix(snapshot), snapshot);
            new_flaps
        };
    }
}

#[derive(Debug, Clone)]
pub struct ItemSummary {
    range: Range<Anchor>,
}

impl Default for ItemSummary {
    fn default() -> Self {
        Self {
            range: Anchor::min()..Anchor::min(),
        }
    }
}

impl sum_tree::Summary for ItemSummary {
    type Context = MultiBufferSnapshot;

    fn add_summary(&mut self, other: &Self, _snapshot: &MultiBufferSnapshot) {
        self.range = other.range.clone();
    }
}

impl sum_tree::Item for FlapItem {
    type Summary = ItemSummary;

    fn summary(&self) -> Self::Summary {
        ItemSummary {
            range: self.flap.range.clone(),
        }
    }
}

/// Implements `SeekTarget` for `Range<Anchor>` to enable seeking within a `SumTree` of `FlapItem`s.
impl SeekTarget<'_, ItemSummary, ItemSummary> for Range<Anchor> {
    fn cmp(&self, cursor_location: &ItemSummary, snapshot: &MultiBufferSnapshot) -> Ordering {
        AnchorRangeExt::cmp(self, &cursor_location.range, snapshot)
    }
}

impl SeekTarget<'_, ItemSummary, ItemSummary> for Anchor {
    fn cmp(&self, other: &ItemSummary, snapshot: &MultiBufferSnapshot) -> Ordering {
        self.cmp(&other.range.start, snapshot)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use gpui::{div, AppContext};
    use multi_buffer::MultiBuffer;

    #[gpui::test]
    fn test_insert_and_remove_flaps(cx: &mut AppContext) {
        let text = "line1\nline2\nline3\nline4\nline5";
        let buffer = MultiBuffer::build_simple(text, cx);
        let snapshot = buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));
        let mut flap_map = FlapMap::default();

        // Insert flaps
        let flaps = [
            Flap::new(
                snapshot.anchor_before(Point::new(1, 0))..snapshot.anchor_after(Point::new(1, 5)),
                FoldPlaceholder::test(),
                |_row, _folded, _toggle, _cx| div(),
                |_row, _folded, _cx| div(),
            ),
            Flap::new(
                snapshot.anchor_before(Point::new(3, 0))..snapshot.anchor_after(Point::new(3, 5)),
                FoldPlaceholder::test(),
                |_row, _folded, _toggle, _cx| div(),
                |_row, _folded, _cx| div(),
            ),
        ];
        let flap_ids = flap_map.insert(flaps, &snapshot);
        assert_eq!(flap_ids.len(), 2);

        // Verify flaps are inserted
        let flap_snapshot = flap_map.snapshot();
        assert!(flap_snapshot
            .query_row(MultiBufferRow(1), &snapshot)
            .is_some());
        assert!(flap_snapshot
            .query_row(MultiBufferRow(3), &snapshot)
            .is_some());

        // Remove flaps
        flap_map.remove(flap_ids, &snapshot);

        // Verify flaps are removed
        let flap_snapshot = flap_map.snapshot();
        assert!(flap_snapshot
            .query_row(MultiBufferRow(1), &snapshot)
            .is_none());
        assert!(flap_snapshot
            .query_row(MultiBufferRow(3), &snapshot)
            .is_none());
    }
}
