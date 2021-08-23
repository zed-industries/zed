use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    sum_tree::{self, Bias, SumTree},
    Element,
};
use parking_lot::Mutex;
use std::sync::Arc;

use crate::ElementBox;

pub struct List {
    state: ListState,
}

pub struct ListState(Arc<Mutex<StateInner>>);

struct StateInner {
    last_layout_width: f32,
    elements: Vec<ElementBox>,
    heights: SumTree<ElementHeight>,
}

#[derive(Clone, Debug)]
enum ElementHeight {
    Pending,
    Ready(f32),
}

#[derive(Clone, Debug, Default)]
struct ElementHeightSummary {
    count: usize,
    pending_count: usize,
    height: f32,
}

#[derive(Clone, Debug, Default)]
struct Count(usize);

#[derive(Clone, Debug, Default)]
struct PendingCount(usize);

#[derive(Clone, Debug, Default)]
struct Height(f32);

impl Element for List {
    type LayoutState = ();

    type PaintState = ();

    fn layout(
        &mut self,
        constraint: crate::SizeConstraint,
        cx: &mut crate::LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        // TODO: Fully invalidate if width has changed since the last layout.

        let state = &mut *self.state.0.lock();
        let mut old_heights = state.heights.cursor::<PendingCount, ElementHeightSummary>();
        let mut new_heights = old_heights.slice(&PendingCount(1), sum_tree::Bias::Left, &());

        let mut item_constraint = constraint;
        item_constraint.min.set_y(0.);
        item_constraint.max.set_y(f32::INFINITY);

        while let Some(height) = old_heights.item() {
            if height.is_pending() {
                let size =
                    state.elements[old_heights.sum_start().count].layout(item_constraint, cx);
                new_heights.push(ElementHeight::Ready(size.y()), &());
                old_heights.next(&());
            } else {
                new_heights.push_tree(
                    old_heights.slice(
                        &PendingCount(old_heights.sum_start().pending_count + 1),
                        Bias::Left,
                        &(),
                    ),
                    &(),
                );
            }
        }

        drop(old_heights);
        state.heights = new_heights;

        todo!()
    }

    fn paint(
        &mut self,
        bounds: RectF,
        layout: &mut Self::LayoutState,
        cx: &mut crate::PaintContext,
    ) -> Self::PaintState {
        todo!()
    }

    fn dispatch_event(
        &mut self,
        event: &crate::Event,
        bounds: RectF,
        layout: &mut Self::LayoutState,
        paint: &mut Self::PaintState,
        cx: &mut crate::EventContext,
    ) -> bool {
        todo!()
    }

    fn debug(
        &self,
        bounds: RectF,
        layout: &Self::LayoutState,
        paint: &Self::PaintState,
        cx: &crate::DebugContext,
    ) -> serde_json::Value {
        todo!()
    }
}

impl ListState {
    pub fn new(elements: Vec<ElementBox>) -> Self {
        let mut heights = SumTree::new();
        heights.extend(elements.iter().map(|_| ElementHeight::Pending), &());
        Self(Arc::new(Mutex::new(StateInner {
            last_layout_width: 0.,
            elements,
            heights,
        })))
    }
}

impl ElementHeight {
    fn is_pending(&self) -> bool {
        matches!(self, ElementHeight::Pending)
    }
}

impl sum_tree::Item for ElementHeight {
    type Summary = ElementHeightSummary;

    fn summary(&self) -> Self::Summary {
        match self {
            ElementHeight::Pending => ElementHeightSummary {
                count: 1,
                pending_count: 1,
                height: 0.,
            },
            ElementHeight::Ready(height) => ElementHeightSummary {
                count: 1,
                pending_count: 0,
                height: *height,
            },
        }
    }
}

impl sum_tree::Summary for ElementHeightSummary {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _: &()) {
        self.pending_count += summary.pending_count;
        self.height += summary.height;
    }
}

impl<'a> sum_tree::Dimension<'a, ElementHeightSummary> for ElementHeightSummary {
    fn add_summary(&mut self, summary: &'a ElementHeightSummary, _: &()) {
        sum_tree::Summary::add_summary(self, summary, &());
    }
}

impl<'a> sum_tree::Dimension<'a, ElementHeightSummary> for Count {
    fn add_summary(&mut self, summary: &'a ElementHeightSummary, _: &()) {
        self.0 += summary.count;
    }
}

impl<'a> sum_tree::Dimension<'a, ElementHeightSummary> for PendingCount {
    fn add_summary(&mut self, summary: &'a ElementHeightSummary, _: &()) {
        self.0 += summary.pending_count;
    }
}

impl<'a> sum_tree::SeekDimension<'a, ElementHeightSummary> for PendingCount {
    fn cmp(&self, other: &Self, _: &()) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<'a> sum_tree::Dimension<'a, ElementHeightSummary> for Height {
    fn add_summary(&mut self, summary: &'a ElementHeightSummary, _: &()) {
        self.0 += summary.height;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[crate::test(self)]
    fn test_layout(cx: &mut crate::MutableAppContext) {
        let mut presenter = cx.build_presenter(0, 20.0);
        let layout_cx = presenter.layout_cx(cx);
    }
}
