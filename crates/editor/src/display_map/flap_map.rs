use collections::HashMap;
use gpui::{AnyElement, IntoElement};
use multi_buffer::{Anchor, AnchorRangeExt, MultiBufferRow, MultiBufferSnapshot, ToPoint};
use std::{cmp::Ordering, ops::Range, sync::Arc};
use sum_tree::{Bias, SeekTarget, SumTree};
use text::Point;
use ui::WindowContext;

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
    /// Returns the first FoldableRange starting on the specified buffer row.
    pub fn query_row<'a>(
        &'a self,
        row: MultiBufferRow,
        snapshot: &'a MultiBufferSnapshot,
    ) -> Option<&'a Flap> {
        let start = snapshot.anchor_before(Point::new(row.0, 0));
        let mut cursor = self.flaps.cursor::<ItemSummary>();
        cursor.seek(&start, Bias::Left, snapshot);
        if let Some(item) = cursor.item() {
            if item.flap.range.start.to_point(snapshot).row == row.0 {
                return Some(&item.flap);
            }
        }
        return None;
    }
}

#[derive(Clone)]
pub struct Flap {
    pub range: Range<Anchor>,
    pub render_toggle: Arc<
        dyn Send
            + Sync
            + Fn(
                MultiBufferRow,
                bool,
                Arc<dyn Send + Sync + Fn(bool, &mut WindowContext)>,
                &mut WindowContext,
            ) -> AnyElement,
    >,
}

impl Flap {
    pub fn new<F, E>(range: Range<Anchor>, render_toggle: F) -> Self
    where
        F: 'static
            + Send
            + Sync
            + Fn(
                MultiBufferRow,
                bool,
                Arc<dyn Send + Sync + Fn(bool, &mut WindowContext)>,
                &mut WindowContext,
            ) -> E
            + 'static,
        E: IntoElement,
    {
        Flap {
            range,
            render_toggle: Arc::new(move |row, folded, toggle, cx| {
                render_toggle(row, folded, toggle, cx).into_any_element()
            }),
        }
    }
}

#[derive(Clone)]
struct FlapItem {
    id: FlapId,
    flap: Flap,
}

impl SeekTarget<'_, ItemSummary, ItemSummary> for Range<Anchor> {
    fn cmp(&self, cursor_location: &ItemSummary, snapshot: &MultiBufferSnapshot) -> Ordering {
        AnchorRangeExt::cmp(self, &cursor_location.range, snapshot)
    }
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
            let mut new_foldables = SumTree::new();
            let mut cursor = self.snapshot.flaps.cursor::<ItemSummary>();

            for (id, range) in removals {
                new_foldables.append(cursor.slice(&range, Bias::Left, snapshot), snapshot);
                while let Some(item) = cursor.item() {
                    cursor.next(snapshot);
                    if item.id == id {
                        break;
                    } else {
                        new_foldables.push(item.clone(), snapshot);
                    }
                }
            }

            new_foldables.append(cursor.suffix(snapshot), snapshot);
            new_foldables
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
            range: Anchor::max()..Anchor::min(),
        }
    }
}

impl sum_tree::Summary for ItemSummary {
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

impl sum_tree::Item for FlapItem {
    type Summary = ItemSummary;

    fn summary(&self) -> Self::Summary {
        ItemSummary {
            range: self.flap.range.clone(),
        }
    }
}

impl SeekTarget<'_, ItemSummary, ItemSummary> for Anchor {
    fn cmp(&self, other: &ItemSummary, snapshot: &MultiBufferSnapshot) -> Ordering {
        other.range.start.cmp(&self, snapshot)
    }
}

impl SeekTarget<'_, ItemSummary, ItemSummary> for ItemSummary {
    fn cmp(&self, other: &ItemSummary, snapshot: &MultiBufferSnapshot) -> Ordering {
        other
            .range
            .start
            .cmp(&self.range.start, snapshot)
            .then_with(|| other.range.end.cmp(&self.range.end, snapshot).reverse())
    }
}
