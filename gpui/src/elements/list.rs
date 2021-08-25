use crate::{
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::json,
    sum_tree::{self, Bias, SumTree},
    DebugContext, Element, Event, EventContext, LayoutContext, PaintContext, SizeConstraint,
};
use parking_lot::Mutex;
use std::{ops::Range, sync::Arc};

use crate::ElementBox;

pub struct List {
    state: ListState,
}

#[derive(Clone)]
pub struct ListState(Arc<Mutex<StateInner>>);

#[derive(Eq, PartialEq)]
pub enum Orientation {
    Top,
    Bottom,
}

struct StateInner {
    last_layout_width: f32,
    elements: Vec<ElementBox>,
    heights: SumTree<ElementHeight>,
    scroll_position: f32,
    orientation: Orientation,
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
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let state = &mut *self.state.0.lock();
        let mut item_constraint = constraint;
        item_constraint.min.set_y(0.);
        item_constraint.max.set_y(f32::INFINITY);

        let size = constraint.max;

        let visible_top = state.scroll_top(size.y());
        let visible_bottom = visible_top + size.y();

        if state.last_layout_width == constraint.max.x() {
            let mut old_heights = state.heights.cursor::<PendingCount, ElementHeightSummary>();
            let mut new_heights = old_heights.slice(&PendingCount(1), sum_tree::Bias::Left, &());

            while let Some(height) = old_heights.item() {
                if height.is_pending() {
                    let size =
                        state.elements[old_heights.sum_start().count].layout(item_constraint, cx);
                    new_heights.push(ElementHeight::Ready(size.y()), &());

                    // Adjust scroll position to keep visible elements stable
                    match state.orientation {
                        Orientation::Top => {
                            if new_heights.summary().height < visible_top {
                                state.scroll_position += size.y();
                            }
                        }
                        Orientation::Bottom => {
                            if new_heights.summary().height - size.y() > visible_bottom {
                                state.scroll_position += size.y();
                            }
                        }
                    }

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

        (size, ())
    }

    fn paint(&mut self, bounds: RectF, _: &mut (), cx: &mut PaintContext) {
        cx.scene.push_layer(Some(bounds));
        let state = &mut *self.state.0.lock();
        let visible_range = state.visible_range(bounds.height());

        let mut item_top = {
            let mut cursor = state.heights.cursor::<Count, Height>();
            cursor.seek(&Count(visible_range.start), Bias::Right, &());
            cursor.sum_start().0
        };
        if state.orientation == Orientation::Bottom
            && bounds.height() > state.heights.summary().height
        {
            item_top += bounds.height() - state.heights.summary().height;
        }
        let scroll_top = state.scroll_top(bounds.height());

        for element in &mut state.elements[visible_range] {
            let origin = bounds.origin() + vec2f(0., item_top - scroll_top);
            element.paint(origin, cx);
            item_top += element.size().y();
        }
        cx.scene.pop_layer();
    }

    fn dispatch_event(
        &mut self,
        event: &Event,
        bounds: RectF,
        _: &mut (),
        _: &mut (),
        cx: &mut EventContext,
    ) -> bool {
        let mut handled = false;

        let mut state = self.state.0.lock();
        let visible_range = state.visible_range(bounds.height());
        for item in &mut state.elements[visible_range] {
            handled = item.dispatch_event(event, cx) || handled;
        }

        match event {
            Event::ScrollWheel {
                position,
                delta,
                precise,
            } => {
                if bounds.contains_point(*position) {
                    if state.scroll(*position, *delta, *precise, bounds.height(), cx) {
                        handled = true;
                    }
                }
            }
            _ => {}
        }

        handled
    }

    fn debug(&self, bounds: RectF, _: &(), _: &(), cx: &DebugContext) -> serde_json::Value {
        let state = self.state.0.lock();
        let visible_range = state.visible_range(bounds.height());
        let visible_elements = state.elements[visible_range.clone()]
            .iter()
            .map(|e| e.debug(cx))
            .collect::<Vec<_>>();
        json!({
            "visible_range": visible_range,
            "visible_elements": visible_elements,
            "scroll_position": state.scroll_position,
        })
    }
}

impl ListState {
    pub fn new(elements: Vec<ElementBox>, orientation: Orientation) -> Self {
        let mut heights = SumTree::new();
        heights.extend(elements.iter().map(|_| ElementHeight::Pending), &());
        Self(Arc::new(Mutex::new(StateInner {
            last_layout_width: 0.,
            elements,
            heights,
            scroll_position: 0.,
            orientation,
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

impl StateInner {
    fn visible_range(&self, height: f32) -> Range<usize> {
        let mut cursor = self.heights.cursor::<Height, Count>();
        cursor.seek(&Height(self.scroll_top(height)), Bias::Right, &());
        let start_ix = cursor.sum_start().0;
        cursor.seek(&Height(self.scroll_top(height) + height), Bias::Left, &());
        let end_ix = cursor.sum_start().0;
        start_ix..self.elements.len().min(end_ix + 1)
    }

    fn scroll(
        &mut self,
        _: Vector2F,
        delta: Vector2F,
        precise: bool,
        height: f32,
        cx: &mut EventContext,
    ) -> bool {
        if !precise {
            todo!("still need to handle non-precise scroll events from a mouse wheel");
        }

        let scroll_max = (self.heights.summary().height - height).max(0.);
        let delta_y = match self.orientation {
            Orientation::Top => -delta.y(),
            Orientation::Bottom => delta.y(),
        };
        self.scroll_position = (self.scroll_position + delta_y).max(0.).min(scroll_max);
        cx.notify();

        true
    }

    fn scroll_top(&self, height: f32) -> f32 {
        match self.orientation {
            Orientation::Top => self.scroll_position,
            Orientation::Bottom => {
                (self.heights.summary().height - height - self.scroll_position).max(0.)
            }
        }
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

impl<'a> sum_tree::SeekDimension<'a, ElementHeightSummary> for Height {
    fn cmp(&self, other: &Self, _: &()) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
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
        let state = ListState::new(vec![item(20.), item(30.), item(10.)], Orientation::Top);
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
