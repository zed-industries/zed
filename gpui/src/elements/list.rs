use crate::{
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::json,
    sum_tree::{self, Bias, SumTree},
    DebugContext, Element, ElementBox, ElementRc, Event, EventContext, LayoutContext, PaintContext,
    RenderContext, SizeConstraint, View,
};
use std::{cell::RefCell, ops::Range, rc::Rc};

pub struct List {
    state: ListState,
}

#[derive(Clone)]
pub struct ListState(Rc<RefCell<StateInner>>);

#[derive(Eq, PartialEq)]
pub enum Orientation {
    Top,
    Bottom,
}

struct StateInner {
    last_layout_width: Option<f32>,
    elements: Vec<Option<ElementRc>>,
    heights: SumTree<ElementHeight>,
    scroll_position: f32,
    orientation: Orientation,
    scroll_handler: Option<Box<dyn FnMut(Range<usize>, &mut EventContext)>>,
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
    pub fn new<F, I, V>(state: ListState, cx: &RenderContext<V>, build_items: F) -> Self
    where
        F: Fn(Range<usize>) -> I,
        I: IntoIterator<Item = ElementBox>,
        V: View,
    {
        {
            let state = &mut *state.0.borrow_mut();
            if cx.refreshing {
                let elements = (build_items)(0..state.elements.len());
                state.last_layout_width = None;
                state.elements.clear();
                state
                    .elements
                    .extend(elements.into_iter().map(|e| Some(e.into())));
                state.heights = SumTree::new();
                state.heights.extend(
                    (0..state.elements.len()).map(|_| ElementHeight::Pending),
                    &(),
                );
            } else {
                let mut cursor = state.heights.cursor::<PendingCount, Count>();
                cursor.seek(&PendingCount(1), sum_tree::Bias::Left, &());

                while cursor.item().is_some() {
                    let start_ix = cursor.sum_start().0;
                    while cursor.item().map_or(false, |h| h.is_pending()) {
                        cursor.next(&());
                    }
                    let end_ix = cursor.sum_start().0;
                    if end_ix > start_ix {
                        state.elements.splice(
                            start_ix..end_ix,
                            (build_items)(start_ix..end_ix)
                                .into_iter()
                                .map(|e| Some(e.into())),
                        );
                    }

                    cursor.seek(&PendingCount(cursor.seek_start().0 + 1), Bias::Left, &());
                }
            }
        }

        Self { state }
    }
}

impl Element for List {
    type LayoutState = Vec<ElementRc>;

    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let state = &mut *self.state.0.borrow_mut();
        let mut item_constraint = constraint;
        item_constraint.min.set_y(0.);
        item_constraint.max.set_y(f32::INFINITY);

        let size = constraint.max;

        let visible_top = state.scroll_top(size.y());
        let visible_bottom = visible_top + size.y();

        if state.last_layout_width == Some(constraint.max.x()) {
            let mut old_heights = state.heights.cursor::<PendingCount, ElementHeightSummary>();
            let mut new_heights = old_heights.slice(&PendingCount(1), sum_tree::Bias::Left, &());

            while let Some(height) = old_heights.item() {
                if height.is_pending() {
                    let element = &mut state.elements[old_heights.sum_start().count];
                    let element_size = element.as_mut().unwrap().layout(item_constraint, cx);
                    new_heights.push(ElementHeight::Ready(element_size.y()), &());

                    // Adjust scroll position to keep visible elements stable
                    match state.orientation {
                        Orientation::Top => {
                            if new_heights.summary().height < visible_top {
                                state.scroll_position += element_size.y();
                            }
                        }
                        Orientation::Bottom => {
                            if new_heights.summary().height - element_size.y() > visible_bottom {
                                state.scroll_position += element_size.y();
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
                let element = element.as_mut().unwrap();
                let size = element.layout(item_constraint, cx);
                state.heights.push(ElementHeight::Ready(size.y()), &());
            }
            state.last_layout_width = Some(constraint.max.x());
        }

        let visible_elements = state.elements[state.visible_range(size.y())]
            .iter()
            .map(|e| e.clone().unwrap())
            .collect();
        (size, visible_elements)
    }

    fn paint(
        &mut self,
        bounds: RectF,
        visible_elements: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) {
        cx.scene.push_layer(Some(bounds));
        let state = &mut *self.state.0.borrow_mut();
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

        for element in visible_elements {
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
        visible_elements: &mut Self::LayoutState,
        _: &mut (),
        cx: &mut EventContext,
    ) -> bool {
        let mut handled = false;

        let mut state = self.state.0.borrow_mut();
        for element in visible_elements {
            handled = element.dispatch_event(event, cx) || handled;
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

    fn debug(
        &self,
        bounds: RectF,
        visible_elements: &Self::LayoutState,
        _: &(),
        cx: &DebugContext,
    ) -> serde_json::Value {
        let state = self.state.0.borrow_mut();
        let visible_range = state.visible_range(bounds.height());
        let visible_elements = visible_elements
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
    pub fn new(element_count: usize, orientation: Orientation) -> Self {
        let mut heights = SumTree::new();
        heights.extend((0..element_count).map(|_| ElementHeight::Pending), &());
        Self(Rc::new(RefCell::new(StateInner {
            last_layout_width: None,
            elements: (0..element_count).map(|_| None).collect(),
            heights,
            scroll_position: 0.,
            orientation,
            scroll_handler: None,
        })))
    }

    pub fn splice(&self, old_range: Range<usize>, count: usize) {
        let state = &mut *self.0.borrow_mut();

        let mut old_heights = state.heights.cursor::<Count, ()>();
        let mut new_heights = old_heights.slice(&Count(old_range.start), Bias::Right, &());
        old_heights.seek_forward(&Count(old_range.end), Bias::Right, &());

        let old_elements = state.elements.splice(old_range, (0..count).map(|_| None));
        drop(old_elements);

        new_heights.extend((0..count).map(|_| ElementHeight::Pending), &());
        new_heights.push_tree(old_heights.suffix(&()), &());
        drop(old_heights);
        state.heights = new_heights;
    }

    pub fn set_scroll_handler(
        &mut self,
        handler: impl FnMut(Range<usize>, &mut EventContext) + 'static,
    ) {
        self.0.borrow_mut().scroll_handler = Some(Box::new(handler))
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

        if self.scroll_handler.is_some() {
            let range = self.visible_range(height);
            self.scroll_handler.as_mut().unwrap()(range, cx);
        }
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
    use crate::{elements::*, geometry::vector::vec2f, Entity};

    #[crate::test(self)]
    fn test_layout(cx: &mut crate::MutableAppContext) {
        let mut presenter = cx.build_presenter(0, 0.);

        let mut elements = vec![20., 30., 10.];
        let state = ListState::new(elements.len(), Orientation::Top);

        let mut list = List::new(
            state.clone(),
            &cx.render_cx::<TestView>(0, 0, 0., false),
            |range| elements[range].iter().copied().map(item),
        )
        .boxed();
        let size = list.layout(
            SizeConstraint::new(vec2f(0., 0.), vec2f(100., 40.)),
            &mut presenter.layout_cx(cx),
        );
        assert_eq!(size, vec2f(100., 40.));
        assert_eq!(
            state.0.borrow().heights.summary(),
            ElementHeightSummary {
                count: 3,
                pending_count: 0,
                height: 60.
            }
        );

        elements.splice(1..2, vec![40., 50.]);
        elements.push(60.);
        state.splice(1..2, 2);
        state.splice(4..4, 1);
        assert_eq!(
            state.0.borrow().heights.summary(),
            ElementHeightSummary {
                count: 5,
                pending_count: 3,
                height: 30.
            }
        );

        let mut list = List::new(
            state.clone(),
            &cx.render_cx::<TestView>(0, 0, 0., false),
            |range| elements[range].iter().copied().map(item),
        )
        .boxed();
        let size = list.layout(
            SizeConstraint::new(vec2f(0., 0.), vec2f(100., 40.)),
            &mut presenter.layout_cx(cx),
        );
        assert_eq!(size, vec2f(100., 40.));
        assert_eq!(
            state.0.borrow().heights.summary(),
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

    struct TestView;

    impl Entity for TestView {
        type Event = ();
    }

    impl View for TestView {
        fn ui_name() -> &'static str {
            "TestView"
        }

        fn render(&self, _: &mut RenderContext<'_, Self>) -> ElementBox {
            unimplemented!()
        }
    }
}
