use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    sum_tree::{self, Bias, SumTree},
    Element,
};
use parking_lot::Mutex;
use std::{ops::Range, sync::Arc};

use crate::ElementBox;

pub struct List {
    state: ListState,
}

#[derive(Clone)]
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

#[derive(Clone, Debug, Default, PartialEq)]
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

impl List {
    pub fn new(state: ListState) -> Self {
        Self { state }
    }
}

impl Element for List {
    type LayoutState = ();

    type PaintState = ();

    fn layout(
        &mut self,
        constraint: crate::SizeConstraint,
        cx: &mut crate::LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let state = &mut *self.state.0.lock();
        let mut item_constraint = constraint;
        item_constraint.min.set_y(0.);
        item_constraint.max.set_y(f32::INFINITY);

        if state.last_layout_width == constraint.max.x() {
            let mut old_heights = state.heights.cursor::<PendingCount, ElementHeightSummary>();
            let mut new_heights = old_heights.slice(&PendingCount(1), sum_tree::Bias::Left, &());

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
        } else {
            state.heights = SumTree::new();
            for element in &mut state.elements {
                let size = element.layout(item_constraint, cx);
                state.heights.push(ElementHeight::Ready(size.y()), &());
            }
            state.last_layout_width = constraint.max.x();
        }

        (constraint.max, ())
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

    pub fn splice(
        &self,
        old_range: Range<usize>,
        new_elements: impl IntoIterator<Item = ElementBox>,
    ) {
        let state = &mut *self.0.lock();

        let mut old_heights = state.heights.cursor::<Count, ()>();
        let mut new_heights = old_heights.slice(&Count(old_range.start), Bias::Right, &());
        old_heights.seek_forward(&Count(old_range.end), Bias::Right, &());

        let mut len = 0;
        let old_elements = state.elements.splice(
            old_range,
            new_elements.into_iter().map(|e| {
                len += 1;
                e
            }),
        );
        drop(old_elements);

        new_heights.extend((0..len).map(|_| ElementHeight::Pending), &());
        new_heights.push_tree(old_heights.suffix(&()), &());
        drop(old_heights);
        state.heights = new_heights;
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
        self.count += summary.count;
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

impl<'a> sum_tree::SeekDimension<'a, ElementHeightSummary> for Count {
    fn cmp(&self, other: &Self, _: &()) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
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
    use crate::{elements::*, geometry::vector::vec2f};

    #[crate::test(self)]
    fn test_layout(cx: &mut crate::MutableAppContext) {
        let mut presenter = cx.build_presenter(0, 20.0);
        let mut layout_cx = presenter.layout_cx(cx);
        let state = ListState::new(vec![item(20.), item(30.), item(10.)]);
        let mut list = List::new(state.clone()).boxed();

        let size = list.layout(
            SizeConstraint::new(vec2f(0., 0.), vec2f(100., 40.)),
            &mut layout_cx,
        );
        assert_eq!(size, vec2f(100., 40.));
        assert_eq!(
            state.0.lock().heights.summary(),
            ElementHeightSummary {
                count: 3,
                pending_count: 0,
                height: 60.
            }
        );

        state.splice(1..2, vec![item(40.), item(50.)]);
        state.splice(3..3, vec![item(60.)]);
        assert_eq!(
            state.0.lock().heights.summary(),
            ElementHeightSummary {
                count: 5,
                pending_count: 3,
                height: 30.
            }
        );
        let size = list.layout(
            SizeConstraint::new(vec2f(0., 0.), vec2f(100., 40.)),
            &mut layout_cx,
        );
        assert_eq!(size, vec2f(100., 40.));
        assert_eq!(
            state.0.lock().heights.summary(),
            ElementHeightSummary {
                count: 5,
                pending_count: 0,
                height: 180.
            }
        );
    }

    fn item(height: f32) -> ElementBox {
        ConstrainedBox::new(Empty::new().boxed())
            .with_height(height)
            .with_width(100.)
            .boxed()
    }
}
