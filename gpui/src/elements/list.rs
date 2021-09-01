use crate::{
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::json,
    sum_tree::{self, Bias, SumTree},
    DebugContext, Element, ElementBox, ElementRc, Event, EventContext, LayoutContext, PaintContext,
    SizeConstraint,
};
use std::{cell::RefCell, ops::Range, rc::Rc};

pub struct List {
    state: ListState,
}

#[derive(Clone)]
pub struct ListState(Rc<RefCell<StateInner>>);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Orientation {
    Top,
    Bottom,
}

struct StateInner {
    last_layout_width: Option<f32>,
    render_item: Box<dyn FnMut(usize, &mut LayoutContext) -> ElementBox>,
    rendered_range: Range<usize>,
    items: SumTree<ListItem>,
    scroll_top: Option<ScrollTop>,
    orientation: Orientation,
    overdraw: usize,
    scroll_handler: Option<Box<dyn FnMut(Range<usize>, &mut EventContext)>>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ScrollTop {
    item_ix: usize,
    offset_in_item: f32,
}

#[derive(Clone)]
enum ListItem {
    Unrendered,
    Rendered(ElementRc),
    Removed(f32),
}

impl std::fmt::Debug for ListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unrendered => write!(f, "Unrendered"),
            Self::Rendered(_) => f.debug_tuple("Rendered").finish(),
            Self::Removed(height) => f.debug_tuple("Removed").field(height).finish(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct ListItemSummary {
    count: usize,
    rendered_count: usize,
    unrendered_count: usize,
    height: f32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Count(usize);

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct RenderedCount(usize);

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct UnrenderedCount(usize);

#[derive(Clone, Debug, Default)]
struct Height(f32);

impl List {
    pub fn new(state: ListState) -> Self {
        Self { state }
    }
}

impl Element for List {
    type LayoutState = ScrollTop;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let state = &mut *self.state.0.borrow_mut();
        let size = constraint.max;
        let mut item_constraint = constraint;
        item_constraint.min.set_y(0.);
        item_constraint.max.set_y(f32::INFINITY);

        if state.last_layout_width != Some(constraint.max.x()) {
            state.rendered_range = 0..0;
            state.items = SumTree::from_iter(
                (0..state.items.summary().count).map(|_| ListItem::Unrendered),
                &(),
            )
        }

        let overdraw = state.overdraw;
        let old_rendered_range = state.rendered_range.clone();
        let old_items = state.items.clone();
        let orientation = state.orientation;
        let stored_scroll_top = state.scroll_top;
        let mut new_items = SumTree::new();

        let mut render_item = |ix, old_item: &ListItem| {
            let element = if let ListItem::Rendered(element) = old_item {
                element.clone()
            } else {
                let mut element = (state.render_item)(ix, cx);
                element.layout(item_constraint, cx);
                element.into()
            };
            element
        };

        // Determine the scroll top. When parked at the end of a bottom-oriented
        // list, this requires rendering items starting from the end of the list
        // until the visible region is full. In other cases, the stored scroll
        // can be used.
        let scroll_top;
        let trailing_items;
        if let (Orientation::Bottom, None) = (orientation, stored_scroll_top) {
            let mut rendered_height = 0.;
            let mut cursor = old_items.cursor::<Count, ()>();

            let mut visible_items = Vec::new();
            cursor.seek(&Count(old_items.summary().count), Bias::Left, &());
            while let Some(item) = cursor.item() {
                if rendered_height >= size.y() {
                    break;
                }

                let element = render_item(cursor.seek_start().0, item);
                rendered_height += element.size().y();
                visible_items.push(ListItem::Rendered(element));
                cursor.prev(&());
            }

            scroll_top = ScrollTop {
                item_ix: cursor.seek_start().0,
                offset_in_item: rendered_height - size.y(),
            };
            visible_items.reverse();
            trailing_items = Some(visible_items);
        } else {
            scroll_top = stored_scroll_top.unwrap_or_default();
            trailing_items = None;
        }

        let new_rendered_range_start = scroll_top.item_ix.saturating_sub(overdraw);
        let mut cursor = old_items.cursor::<Count, ()>();

        // Discard any rendered elements before the overdraw window.
        if old_rendered_range.start < new_rendered_range_start {
            new_items.push_tree(
                cursor.slice(&Count(old_rendered_range.start), Bias::Right, &()),
                &(),
            );
            let remove_to = old_rendered_range.end.min(new_rendered_range_start);
            while cursor.seek_start().0 < remove_to {
                new_items.push(cursor.item().unwrap().remove(), &());
                cursor.next(&());
            }
        }

        new_items.push_tree(
            cursor.slice(&Count(new_rendered_range_start), Bias::Right, &()),
            &(),
        );

        // Ensure that all items in the overdraw window before the visible range are rendered.
        while cursor.seek_start().0 < scroll_top.item_ix {
            new_items.push(
                ListItem::Rendered(render_item(cursor.seek_start().0, cursor.item().unwrap())),
                &(),
            );
            cursor.next(&());
        }

        // The remaining items may have already been rendered, when parked at the
        // end of a bottom-oriented list. If so, append them.
        let new_rendered_range_end;
        if let Some(trailing_items) = trailing_items {
            new_rendered_range_end = new_rendered_range_start + trailing_items.len();
            new_items.extend(trailing_items, &());
        } else {
            // Ensure that enough items are rendered to fill the visible range.
            let mut rendered_top = -scroll_top.offset_in_item;
            while let Some(item) = cursor.item() {
                if rendered_top >= size.y() {
                    break;
                }

                let element = render_item(cursor.seek_start().0, item);
                rendered_top += element.size().y();
                new_items.push(ListItem::Rendered(element), &());
                cursor.next(&());
            }

            // Ensure that all items in the overdraw window after the visible range
            // are rendered.
            new_rendered_range_end =
                (cursor.seek_start().0 + overdraw).min(old_items.summary().count);
            while cursor.seek_start().0 < new_rendered_range_end {
                new_items.push(
                    ListItem::Rendered(render_item(cursor.seek_start().0, cursor.item().unwrap())),
                    &(),
                );
                cursor.next(&());
            }

            // Preserve the remainder of the items, but discard any rendered items after
            // the overdraw window.
            if cursor.seek_start().0 < old_rendered_range.start {
                new_items.push_tree(
                    cursor.slice(&Count(old_rendered_range.start), Bias::Right, &()),
                    &(),
                );
            }
            while cursor.seek_start().0 < old_rendered_range.end {
                new_items.push(cursor.item().unwrap().remove(), &());
                cursor.next(&());
            }
            new_items.push_tree(cursor.suffix(&()), &());
        }

        drop(cursor);
        state.items = new_items;
        state.rendered_range = new_rendered_range_start..new_rendered_range_end;
        state.last_layout_width = Some(size.x());
        (size, scroll_top)
    }

    fn paint(&mut self, bounds: RectF, scroll_top: &mut ScrollTop, cx: &mut PaintContext) {
        cx.scene.push_layer(Some(bounds));

        let state = &mut *self.state.0.borrow_mut();
        for (mut element, origin) in state.visible_elements(bounds, scroll_top) {
            element.paint(origin, cx);
        }

        cx.scene.pop_layer();
    }

    fn dispatch_event(
        &mut self,
        event: &Event,
        bounds: RectF,
        scroll_top: &mut ScrollTop,
        _: &mut (),
        cx: &mut EventContext,
    ) -> bool {
        let mut handled = false;

        let mut state = self.state.0.borrow_mut();
        for (mut element, _) in state.visible_elements(bounds, scroll_top) {
            handled = element.dispatch_event(event, cx) || handled;
        }

        match event {
            Event::ScrollWheel {
                position,
                delta,
                precise,
            } => {
                if bounds.contains_point(*position) {
                    if state.scroll(scroll_top, bounds.height(), *delta, *precise, cx) {
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
        scroll_top: &Self::LayoutState,
        _: &(),
        cx: &DebugContext,
    ) -> serde_json::Value {
        let state = self.state.0.borrow_mut();
        let visible_elements = state
            .visible_elements(bounds, scroll_top)
            .map(|e| e.0.debug(cx))
            .collect::<Vec<_>>();
        let visible_range = scroll_top.item_ix..(scroll_top.item_ix + visible_elements.len());
        json!({
            "visible_range": visible_range,
            "visible_elements": visible_elements,
            "scroll_top": state.scroll_top.map(|top| (top.item_ix, top.offset_in_item)),
        })
    }
}

impl ListState {
    pub fn new<F>(
        element_count: usize,
        orientation: Orientation,
        min_overdraw: usize,
        render_item: F,
    ) -> Self
    where
        F: 'static + FnMut(usize, &mut LayoutContext) -> ElementBox,
    {
        let mut items = SumTree::new();
        items.extend((0..element_count).map(|_| ListItem::Unrendered), &());
        Self(Rc::new(RefCell::new(StateInner {
            last_layout_width: None,
            render_item: Box::new(render_item),
            rendered_range: 0..0,
            items,
            scroll_top: None,
            orientation,
            overdraw: min_overdraw,
            scroll_handler: None,
        })))
    }

    pub fn reset(&self, element_count: usize) {
        let state = &mut *self.0.borrow_mut();
        state.scroll_top = None;
        state.items = SumTree::new();
        state
            .items
            .extend((0..element_count).map(|_| ListItem::Unrendered), &());
    }

    pub fn splice(&self, old_range: Range<usize>, count: usize) {
        let state = &mut *self.0.borrow_mut();

        if let Some(ScrollTop {
            item_ix,
            offset_in_item,
        }) = state.scroll_top.as_mut()
        {
            if old_range.contains(item_ix) {
                *item_ix = old_range.start;
                *offset_in_item = 0.;
            } else if old_range.end <= *item_ix {
                *item_ix = *item_ix - (old_range.end - old_range.start) + count;
            }
        }

        let new_end = old_range.start + count;
        if old_range.start < state.rendered_range.start {
            state.rendered_range.start =
                new_end + state.rendered_range.start.saturating_sub(old_range.end);
        }
        if old_range.start < state.rendered_range.end {
            state.rendered_range.end =
                new_end + state.rendered_range.end.saturating_sub(old_range.end);
        }

        let mut old_heights = state.items.cursor::<Count, ()>();
        let mut new_heights = old_heights.slice(&Count(old_range.start), Bias::Right, &());
        old_heights.seek_forward(&Count(old_range.end), Bias::Right, &());

        new_heights.extend((0..count).map(|_| ListItem::Unrendered), &());
        new_heights.push_tree(old_heights.suffix(&()), &());
        drop(old_heights);
        state.items = new_heights;
    }

    pub fn set_scroll_handler(
        &mut self,
        handler: impl FnMut(Range<usize>, &mut EventContext) + 'static,
    ) {
        self.0.borrow_mut().scroll_handler = Some(Box::new(handler))
    }
}

impl StateInner {
    fn visible_range(&self, height: f32, scroll_top: &ScrollTop) -> Range<usize> {
        let mut cursor = self.items.cursor::<Count, Height>();
        cursor.seek(&Count(scroll_top.item_ix), Bias::Right, &());
        let start_y = cursor.sum_start().0 + scroll_top.offset_in_item;
        let mut cursor = cursor.swap_dimensions();
        cursor.seek_forward(&Height(start_y + height), Bias::Left, &());
        scroll_top.item_ix..cursor.sum_start().0 + 1
    }

    fn visible_elements<'a>(
        &'a self,
        bounds: RectF,
        scroll_top: &ScrollTop,
    ) -> impl Iterator<Item = (ElementRc, Vector2F)> + 'a {
        let mut item_origin = bounds.origin() - vec2f(0., scroll_top.offset_in_item);
        let mut cursor = self.items.cursor::<Count, ()>();
        cursor.seek(&Count(scroll_top.item_ix), Bias::Right, &());
        std::iter::from_fn(move || {
            while let Some(item) = cursor.item() {
                if item_origin.y() > bounds.max_y() {
                    break;
                }

                if let ListItem::Rendered(element) = item {
                    let result = (element.clone(), item_origin);
                    item_origin.set_y(item_origin.y() + element.size().y());
                    cursor.next(&());
                    return Some(result);
                }

                cursor.next(&());
            }

            None
        })
    }

    fn scroll(
        &mut self,
        scroll_top: &ScrollTop,
        height: f32,
        mut delta: Vector2F,
        precise: bool,
        cx: &mut EventContext,
    ) -> bool {
        if !precise {
            delta *= 20.;
        }

        let scroll_max = (self.items.summary().height - height).max(0.);
        let new_scroll_top = (self.scroll_top(height) + delta.y())
            .max(0.)
            .min(scroll_max);

        if self.orientation == Orientation::Bottom && new_scroll_top == scroll_max {
            self.scroll_top = None;
        } else {
            let mut cursor = self.items.cursor::<Height, Count>();
            cursor.seek(&Height(new_scroll_top), Bias::Right, &());
            let item_ix = cursor.sum_start().0;
            let offset_in_item = new_scroll_top - cursor.seek_start().0;
            self.scroll_top = Some(ScrollTop {
                item_ix,
                offset_in_item,
            });
        }

        if self.scroll_handler.is_some() {
            let visible_range = self.visible_range(height, scroll_top);
            self.scroll_handler.as_mut().unwrap()(visible_range, cx);
        }
        cx.notify();

        true
    }

    fn scroll_top(&self, height: f32) -> f32 {
        let scroll_max = (self.items.summary().height - height).max(0.);
        if let Some(ScrollTop {
            item_ix,
            offset_in_item,
        }) = self.scroll_top
        {
            let mut cursor = self.items.cursor::<Count, Height>();
            cursor.seek(&Count(item_ix), Bias::Right, &());
            (cursor.sum_start().0 + offset_in_item).min(scroll_max)
        } else {
            match self.orientation {
                Orientation::Top => 0.,
                Orientation::Bottom => scroll_max,
            }
        }
    }
}

impl ListItem {
    fn remove(&self) -> Self {
        match self {
            ListItem::Unrendered => ListItem::Unrendered,
            ListItem::Rendered(element) => ListItem::Removed(element.size().y()),
            ListItem::Removed(height) => ListItem::Removed(*height),
        }
    }
}

impl sum_tree::Item for ListItem {
    type Summary = ListItemSummary;

    fn summary(&self) -> Self::Summary {
        match self {
            ListItem::Unrendered => ListItemSummary {
                count: 1,
                rendered_count: 0,
                unrendered_count: 1,
                height: 0.,
            },
            ListItem::Rendered(element) => ListItemSummary {
                count: 1,
                rendered_count: 1,
                unrendered_count: 0,
                height: element.size().y(),
            },
            ListItem::Removed(height) => ListItemSummary {
                count: 1,
                rendered_count: 0,
                unrendered_count: 1,
                height: *height,
            },
        }
    }
}

impl sum_tree::Summary for ListItemSummary {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _: &()) {
        self.count += summary.count;
        self.rendered_count += summary.rendered_count;
        self.unrendered_count += summary.unrendered_count;
        self.height += summary.height;
    }
}

impl<'a> sum_tree::Dimension<'a, ListItemSummary> for ListItemSummary {
    fn add_summary(&mut self, summary: &'a ListItemSummary, _: &()) {
        sum_tree::Summary::add_summary(self, summary, &());
    }
}

impl<'a> sum_tree::Dimension<'a, ListItemSummary> for Count {
    fn add_summary(&mut self, summary: &'a ListItemSummary, _: &()) {
        self.0 += summary.count;
    }
}

impl<'a> sum_tree::Dimension<'a, ListItemSummary> for RenderedCount {
    fn add_summary(&mut self, summary: &'a ListItemSummary, _: &()) {
        self.0 += summary.rendered_count;
    }
}

impl<'a> sum_tree::Dimension<'a, ListItemSummary> for UnrenderedCount {
    fn add_summary(&mut self, summary: &'a ListItemSummary, _: &()) {
        self.0 += summary.unrendered_count;
    }
}

impl<'a> sum_tree::Dimension<'a, ListItemSummary> for Height {
    fn add_summary(&mut self, summary: &'a ListItemSummary, _: &()) {
        self.0 += summary.height;
    }
}

impl<'a> sum_tree::SeekDimension<'a, ListItemSummary> for Height {
    fn cmp(&self, other: &Self, _: &()) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        elements::{ConstrainedBox, Empty},
        geometry::vector::vec2f,
        Entity, RenderContext, View,
    };
    use rand::prelude::*;
    use std::env;

    #[crate::test(self)]
    fn test_layout(cx: &mut crate::MutableAppContext) {
        let mut presenter = cx.build_presenter(0, 0.);

        let elements = Rc::new(RefCell::new(vec![20., 30., 100.]));
        let state = ListState::new(elements.borrow().len(), Orientation::Top, 1000, {
            let elements = elements.clone();
            move |ix, _| item(elements.borrow()[ix])
        });

        let mut list = List::new(state.clone()).boxed();
        let size = list.layout(
            SizeConstraint::new(vec2f(0., 0.), vec2f(100., 40.)),
            &mut presenter.build_layout_context(cx),
        );
        assert_eq!(size, vec2f(100., 40.));
        assert_eq!(
            state.0.borrow().items.summary(),
            ListItemSummary {
                count: 3,
                rendered_count: 3,
                unrendered_count: 0,
                height: 150.
            }
        );

        state.0.borrow_mut().scroll(
            &ScrollTop {
                item_ix: 0,
                offset_in_item: 0.,
            },
            40.,
            vec2f(0., 54.),
            true,
            &mut presenter.build_event_context(cx),
        );
        assert_eq!(
            state.0.borrow().scroll_top,
            Some(ScrollTop {
                item_ix: 2,
                offset_in_item: 4.
            })
        );
        assert_eq!(state.0.borrow().scroll_top(size.y()), 54.);

        elements.borrow_mut().splice(1..2, vec![40., 50.]);
        elements.borrow_mut().push(60.);
        state.splice(1..2, 2);
        state.splice(4..4, 1);
        assert_eq!(
            state.0.borrow().items.summary(),
            ListItemSummary {
                count: 5,
                rendered_count: 2,
                unrendered_count: 3,
                height: 120.
            }
        );

        let mut list = List::new(state.clone()).boxed();
        let size = list.layout(
            SizeConstraint::new(vec2f(0., 0.), vec2f(100., 40.)),
            &mut presenter.build_layout_context(cx),
        );
        assert_eq!(size, vec2f(100., 40.));
        assert_eq!(
            state.0.borrow().items.summary(),
            ListItemSummary {
                count: 5,
                rendered_count: 5,
                unrendered_count: 0,
                height: 270.
            }
        );
        assert_eq!(
            state.0.borrow().scroll_top,
            Some(ScrollTop {
                item_ix: 3,
                offset_in_item: 4.
            })
        );
        assert_eq!(state.0.borrow().scroll_top(size.y()), 114.);
    }

    #[crate::test(self, iterations = 10000, seed = 2515)]
    fn test_random(cx: &mut crate::MutableAppContext, mut rng: StdRng) {
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let mut presenter = cx.build_presenter(0, 0.);
        let elements = Rc::new(RefCell::new(
            (0..rng.gen_range(0..=20))
                .map(|_| rng.gen_range(0_f32..=100_f32))
                .collect::<Vec<_>>(),
        ));
        let orientation = *[Orientation::Top, Orientation::Bottom]
            .choose(&mut rng)
            .unwrap();
        let min_overdraw = rng.gen_range(0..=20);
        let state = ListState::new(elements.borrow().len(), orientation, min_overdraw, {
            let elements = elements.clone();
            move |ix, _| item(elements.borrow()[ix])
        });

        let mut width = rng.gen_range(0_f32..=1000_f32);
        let mut height = rng.gen_range(0_f32..=1000_f32);
        log::info!("orientation: {:?}", orientation);
        log::info!("min_overdraw: {}", min_overdraw);
        log::info!("elements: {:?}", elements.borrow());
        log::info!("size: ({:?}, {:?})", width, height);
        log::info!("==================");

        let mut scroll_top = None;
        for _ in 0..operations {
            match rng.gen_range(0..=100) {
                0..=29 if scroll_top.is_some() => {
                    let delta = vec2f(0., rng.gen_range(-100_f32..=100_f32));
                    log::info!(
                        "Scrolling by {:?}, previous scroll top: {:?}",
                        delta,
                        scroll_top.unwrap()
                    );
                    state.0.borrow_mut().scroll(
                        scroll_top.as_ref().unwrap(),
                        height,
                        delta,
                        true,
                        &mut presenter.build_event_context(cx),
                    );
                }
                30..=34 => {
                    width = rng.gen_range(0_f32..=1000_f32);
                    log::info!("changing width: {:?}", width);
                }
                35..=54 => {
                    height = rng.gen_range(0_f32..=1000_f32);
                    log::info!("changing height: {:?}", height);
                }
                _ => {
                    let mut elements = elements.borrow_mut();
                    let end_ix = rng.gen_range(0..=elements.len());
                    let start_ix = rng.gen_range(0..=end_ix);
                    let new_elements = (0..rng.gen_range(0..10))
                        .map(|_| rng.gen_range(0_f32..=100_f32))
                        .collect::<Vec<_>>();
                    log::info!("splice({:?}, {:?})", start_ix..end_ix, new_elements);
                    state.splice(start_ix..end_ix, new_elements.len());
                    elements.splice(start_ix..end_ix, new_elements);
                }
            }

            let mut list = List::new(state.clone());
            let (size, new_scroll_top) = list.layout(
                SizeConstraint::new(vec2f(0., 0.), vec2f(width, height)),
                &mut presenter.build_layout_context(cx),
            );
            assert_eq!(size, vec2f(width, height));
            scroll_top = Some(new_scroll_top);

            let state = state.0.borrow();
            let visible_range = state.visible_range(height, &new_scroll_top);
            let rendered_range =
                visible_range.start.saturating_sub(min_overdraw)..visible_range.end + min_overdraw;
            log::info!("visible range {:?}", visible_range);
            log::info!("items {:?}", state.items.items(&()));
            for (ix, item) in state.items.cursor::<Count, ()>().enumerate() {
                if rendered_range.contains(&ix) {
                    assert!(
                        matches!(item, ListItem::Rendered(_)),
                        "item {:?} was not rendered",
                        ix
                    );
                } else {
                    assert!(
                        !matches!(item, ListItem::Rendered(_)),
                        "item {:?} was incorrectly rendered",
                        ix
                    );
                }
            }
        }
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

        fn render(&mut self, _: &mut RenderContext<'_, Self>) -> ElementBox {
            unimplemented!()
        }
    }
}
