use crate::{
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::json,
    DebugContext, Element, ElementBox, ElementRc, Event, EventContext, LayoutContext, PaintContext,
    RenderContext, SizeConstraint, View, ViewContext,
};
use std::{cell::RefCell, collections::VecDeque, ops::Range, rc::Rc};
use sum_tree::{Bias, SumTree};

pub struct List {
    state: ListState,
    invalidated_elements: Vec<ElementRc>,
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
    render_item: Box<dyn FnMut(usize, &mut LayoutContext) -> Option<ElementBox>>,
    rendered_range: Range<usize>,
    items: SumTree<ListItem>,
    logical_scroll_top: Option<ListOffset>,
    orientation: Orientation,
    overdraw: f32,
    scroll_handler: Option<Box<dyn FnMut(Range<usize>, &mut EventContext)>>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ListOffset {
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
        Self {
            state,
            invalidated_elements: Default::default(),
        }
    }
}

impl Element for List {
    type LayoutState = ListOffset;
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

        if cx.refreshing || state.last_layout_width != Some(size.x()) {
            state.rendered_range = 0..0;
            state.items = SumTree::from_iter(
                (0..state.items.summary().count).map(|_| ListItem::Unrendered),
                &(),
            )
        }

        let old_items = state.items.clone();
        let mut new_items = SumTree::new();
        let mut rendered_items = VecDeque::new();
        let mut rendered_height = 0.;
        let mut scroll_top = state
            .logical_scroll_top
            .unwrap_or_else(|| match state.orientation {
                Orientation::Top => ListOffset {
                    item_ix: 0,
                    offset_in_item: 0.,
                },
                Orientation::Bottom => ListOffset {
                    item_ix: state.items.summary().count,
                    offset_in_item: 0.,
                },
            });

        // Render items after the scroll top, including those in the trailing overdraw.
        let mut cursor = old_items.cursor::<Count>();
        cursor.seek(&Count(scroll_top.item_ix), Bias::Right, &());
        for (ix, item) in cursor.by_ref().enumerate() {
            if rendered_height - scroll_top.offset_in_item >= size.y() + state.overdraw {
                break;
            }

            if let Some(element) =
                state.render_item(scroll_top.item_ix + ix, item, item_constraint, cx)
            {
                rendered_height += element.size().y();
                rendered_items.push_back(ListItem::Rendered(element));
            }
        }

        // Prepare to start walking upward from the item at the scroll top.
        cursor.seek(&Count(scroll_top.item_ix), Bias::Right, &());

        // If the rendered items do not fill the visible region, then adjust
        // the scroll top upward.
        if rendered_height - scroll_top.offset_in_item < size.y() {
            while rendered_height < size.y() {
                cursor.prev(&());
                if let Some(item) = cursor.item() {
                    if let Some(element) =
                        state.render_item(cursor.start().0, item, item_constraint, cx)
                    {
                        rendered_height += element.size().y();
                        rendered_items.push_front(ListItem::Rendered(element));
                    }
                } else {
                    break;
                }
            }

            scroll_top = ListOffset {
                item_ix: cursor.start().0,
                offset_in_item: rendered_height - size.y(),
            };

            match state.orientation {
                Orientation::Top => {
                    scroll_top.offset_in_item = scroll_top.offset_in_item.max(0.);
                    state.logical_scroll_top = Some(scroll_top);
                }
                Orientation::Bottom => {
                    scroll_top = ListOffset {
                        item_ix: cursor.start().0,
                        offset_in_item: rendered_height - size.y(),
                    };
                    state.logical_scroll_top = None;
                }
            };
        }

        // Render items in the leading overdraw.
        let mut leading_overdraw = scroll_top.offset_in_item;
        while leading_overdraw < state.overdraw {
            cursor.prev(&());
            if let Some(item) = cursor.item() {
                if let Some(element) =
                    state.render_item(cursor.start().0, item, item_constraint, cx)
                {
                    leading_overdraw += element.size().y();
                    rendered_items.push_front(ListItem::Rendered(element));
                }
            } else {
                break;
            }
        }

        let new_rendered_range = cursor.start().0..(cursor.start().0 + rendered_items.len());

        let mut cursor = old_items.cursor::<Count>();

        if state.rendered_range.start < new_rendered_range.start {
            new_items.push_tree(
                cursor.slice(&Count(state.rendered_range.start), Bias::Right, &()),
                &(),
            );
            let remove_to = state.rendered_range.end.min(new_rendered_range.start);
            while cursor.start().0 < remove_to {
                new_items.push(cursor.item().unwrap().remove(), &());
                cursor.next(&());
            }
        }
        new_items.push_tree(
            cursor.slice(&Count(new_rendered_range.start), Bias::Right, &()),
            &(),
        );

        new_items.extend(rendered_items, &());
        cursor.seek(&Count(new_rendered_range.end), Bias::Right, &());

        if new_rendered_range.end < state.rendered_range.start {
            new_items.push_tree(
                cursor.slice(&Count(state.rendered_range.start), Bias::Right, &()),
                &(),
            );
        }
        while cursor.start().0 < state.rendered_range.end {
            new_items.push(cursor.item().unwrap().remove(), &());
            cursor.next(&());
        }

        new_items.push_tree(cursor.suffix(&()), &());

        state.items = new_items;
        state.rendered_range = new_rendered_range;
        state.last_layout_width = Some(size.x());
        (size, scroll_top)
    }

    fn paint(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        scroll_top: &mut ListOffset,
        cx: &mut PaintContext,
    ) {
        cx.scene.push_layer(Some(bounds));

        let state = &mut *self.state.0.borrow_mut();
        for (mut element, origin) in state.visible_elements(bounds, scroll_top) {
            element.paint(origin, visible_bounds, cx);
        }

        cx.scene.pop_layer();
    }

    fn dispatch_event(
        &mut self,
        event: &Event,
        bounds: RectF,
        _: RectF,
        scroll_top: &mut ListOffset,
        _: &mut (),
        cx: &mut EventContext,
    ) -> bool {
        let mut handled = false;

        let mut state = self.state.0.borrow_mut();
        let mut item_origin = bounds.origin() - vec2f(0., scroll_top.offset_in_item);
        let mut cursor = state.items.cursor::<Count>();
        let mut new_items = cursor.slice(&Count(scroll_top.item_ix), Bias::Right, &());
        while let Some(item) = cursor.item() {
            if item_origin.y() > bounds.max_y() {
                break;
            }

            if let ListItem::Rendered(element) = item {
                let prev_notify_count = cx.notify_count();
                let mut element = element.clone();
                handled = element.dispatch_event(event, cx) || handled;
                item_origin.set_y(item_origin.y() + element.size().y());
                if cx.notify_count() > prev_notify_count {
                    new_items.push(ListItem::Unrendered, &());
                    self.invalidated_elements.push(element);
                } else {
                    new_items.push(item.clone(), &());
                }
                cursor.next(&());
            } else {
                unreachable!();
            }
        }

        new_items.push_tree(cursor.suffix(&()), &());
        drop(cursor);
        state.items = new_items;

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
            "scroll_top": state.logical_scroll_top.map(|top| (top.item_ix, top.offset_in_item)),
        })
    }
}

impl ListState {
    pub fn new<F, V>(
        element_count: usize,
        orientation: Orientation,
        overdraw: f32,
        cx: &mut ViewContext<V>,
        mut render_item: F,
    ) -> Self
    where
        V: View,
        F: 'static + FnMut(&mut V, usize, &mut RenderContext<V>) -> ElementBox,
    {
        let mut items = SumTree::new();
        items.extend((0..element_count).map(|_| ListItem::Unrendered), &());
        let handle = cx.handle();
        Self(Rc::new(RefCell::new(StateInner {
            last_layout_width: None,
            render_item: Box::new(move |ix, cx| {
                Some(cx.render(&handle, |view, cx| render_item(view, ix, cx)))
            }),
            rendered_range: 0..0,
            items,
            logical_scroll_top: None,
            orientation,
            overdraw,
            scroll_handler: None,
        })))
    }

    pub fn reset(&self, element_count: usize) {
        let state = &mut *self.0.borrow_mut();
        state.rendered_range = 0..0;
        state.logical_scroll_top = None;
        state.items = SumTree::new();
        state
            .items
            .extend((0..element_count).map(|_| ListItem::Unrendered), &());
    }

    pub fn splice(&self, old_range: Range<usize>, count: usize) {
        let state = &mut *self.0.borrow_mut();

        if let Some(ListOffset {
            item_ix,
            offset_in_item,
        }) = state.logical_scroll_top.as_mut()
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

        let mut old_heights = state.items.cursor::<Count>();
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
    fn render_item(
        &mut self,
        ix: usize,
        existing_item: &ListItem,
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> Option<ElementRc> {
        if let ListItem::Rendered(element) = existing_item {
            Some(element.clone())
        } else {
            let mut element = (self.render_item)(ix, cx)?;
            element.layout(constraint, cx);
            Some(element.into())
        }
    }

    fn visible_range(&self, height: f32, scroll_top: &ListOffset) -> Range<usize> {
        let mut cursor = self.items.cursor::<ListItemSummary>();
        cursor.seek(&Count(scroll_top.item_ix), Bias::Right, &());
        let start_y = cursor.start().height + scroll_top.offset_in_item;
        cursor.seek_forward(&Height(start_y + height), Bias::Left, &());
        scroll_top.item_ix..cursor.start().count + 1
    }

    fn visible_elements<'a>(
        &'a self,
        bounds: RectF,
        scroll_top: &ListOffset,
    ) -> impl Iterator<Item = (ElementRc, Vector2F)> + 'a {
        let mut item_origin = bounds.origin() - vec2f(0., scroll_top.offset_in_item);
        let mut cursor = self.items.cursor::<Count>();
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
        scroll_top: &ListOffset,
        height: f32,
        mut delta: Vector2F,
        precise: bool,
        cx: &mut EventContext,
    ) -> bool {
        if !precise {
            delta *= 20.;
        }

        let scroll_max = (self.items.summary().height - height).max(0.);
        let new_scroll_top = (self.scroll_top(scroll_top) - delta.y())
            .max(0.)
            .min(scroll_max);

        if self.orientation == Orientation::Bottom && new_scroll_top == scroll_max {
            self.logical_scroll_top = None;
        } else {
            let mut cursor = self.items.cursor::<ListItemSummary>();
            cursor.seek(&Height(new_scroll_top), Bias::Right, &());
            let item_ix = cursor.start().count;
            let offset_in_item = new_scroll_top - cursor.start().height;
            self.logical_scroll_top = Some(ListOffset {
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

    fn scroll_top(&self, logical_scroll_top: &ListOffset) -> f32 {
        let mut cursor = self.items.cursor::<ListItemSummary>();
        cursor.seek(&Count(logical_scroll_top.item_ix), Bias::Right, &());
        cursor.start().height + logical_scroll_top.offset_in_item
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

impl<'a> sum_tree::SeekTarget<'a, ListItemSummary, ListItemSummary> for Count {
    fn cmp(&self, other: &ListItemSummary, _: &()) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.count).unwrap()
    }
}

impl<'a> sum_tree::SeekTarget<'a, ListItemSummary, ListItemSummary> for Height {
    fn cmp(&self, other: &ListItemSummary, _: &()) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.height).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{elements::Empty, geometry::vector::vec2f, Entity};
    use rand::prelude::*;
    use std::env;

    #[crate::test(self)]
    fn test_layout(cx: &mut crate::MutableAppContext) {
        let mut presenter = cx.build_presenter(0, 0.);
        let (_, view) = cx.add_window(Default::default(), |_| TestView);
        let constraint = SizeConstraint::new(vec2f(0., 0.), vec2f(100., 40.));

        let elements = Rc::new(RefCell::new(vec![(0, 20.), (1, 30.), (2, 100.)]));

        let state = view.update(cx, |_, cx| {
            ListState::new(elements.borrow().len(), Orientation::Top, 1000.0, cx, {
                let elements = elements.clone();
                move |_, ix, _| {
                    let (id, height) = elements.borrow()[ix];
                    TestElement::new(id, height).boxed()
                }
            })
        });

        let mut list = List::new(state.clone());
        let (size, _) = list.layout(constraint, &mut presenter.build_layout_context(false, cx));
        assert_eq!(size, vec2f(100., 40.));
        assert_eq!(
            state.0.borrow().items.summary().clone(),
            ListItemSummary {
                count: 3,
                rendered_count: 3,
                unrendered_count: 0,
                height: 150.
            }
        );

        state.0.borrow_mut().scroll(
            &ListOffset {
                item_ix: 0,
                offset_in_item: 0.,
            },
            40.,
            vec2f(0., -54.),
            true,
            &mut presenter.build_event_context(cx),
        );
        let (_, logical_scroll_top) =
            list.layout(constraint, &mut presenter.build_layout_context(false, cx));
        assert_eq!(
            logical_scroll_top,
            ListOffset {
                item_ix: 2,
                offset_in_item: 4.
            }
        );
        assert_eq!(state.0.borrow().scroll_top(&logical_scroll_top), 54.);

        elements.borrow_mut().splice(1..2, vec![(3, 40.), (4, 50.)]);
        elements.borrow_mut().push((5, 60.));
        state.splice(1..2, 2);
        state.splice(4..4, 1);
        assert_eq!(
            state.0.borrow().items.summary().clone(),
            ListItemSummary {
                count: 5,
                rendered_count: 2,
                unrendered_count: 3,
                height: 120.
            }
        );

        let (size, logical_scroll_top) =
            list.layout(constraint, &mut presenter.build_layout_context(false, cx));
        assert_eq!(size, vec2f(100., 40.));
        assert_eq!(
            state.0.borrow().items.summary().clone(),
            ListItemSummary {
                count: 5,
                rendered_count: 5,
                unrendered_count: 0,
                height: 270.
            }
        );
        assert_eq!(
            logical_scroll_top,
            ListOffset {
                item_ix: 3,
                offset_in_item: 4.
            }
        );
        assert_eq!(state.0.borrow().scroll_top(&logical_scroll_top), 114.);
    }

    #[crate::test(self, iterations = 10, seed = 0)]
    fn test_random(cx: &mut crate::MutableAppContext, mut rng: StdRng) {
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let (_, view) = cx.add_window(Default::default(), |_| TestView);
        let mut presenter = cx.build_presenter(0, 0.);
        let mut next_id = 0;
        let elements = Rc::new(RefCell::new(
            (0..rng.gen_range(0..=20))
                .map(|_| {
                    let id = next_id;
                    next_id += 1;
                    (id, rng.gen_range(0..=200) as f32 / 2.0)
                })
                .collect::<Vec<_>>(),
        ));
        let orientation = *[Orientation::Top, Orientation::Bottom]
            .choose(&mut rng)
            .unwrap();
        let overdraw = rng.gen_range(1..=100) as f32;

        let state = view.update(cx, |_, cx| {
            ListState::new(elements.borrow().len(), orientation, overdraw, cx, {
                let elements = elements.clone();
                move |_, ix, _| {
                    let (id, height) = elements.borrow()[ix];
                    TestElement::new(id, height).boxed()
                }
            })
        });

        let mut width = rng.gen_range(0..=2000) as f32 / 2.;
        let mut height = rng.gen_range(0..=2000) as f32 / 2.;
        log::info!("orientation: {:?}", orientation);
        log::info!("overdraw: {}", overdraw);
        log::info!("elements: {:?}", elements.borrow());
        log::info!("size: ({:?}, {:?})", width, height);
        log::info!("==================");

        let mut last_logical_scroll_top = None;
        for _ in 0..operations {
            match rng.gen_range(0..=100) {
                0..=29 if last_logical_scroll_top.is_some() => {
                    let delta = vec2f(0., rng.gen_range(-overdraw..=overdraw));
                    log::info!(
                        "Scrolling by {:?}, previous scroll top: {:?}",
                        delta,
                        last_logical_scroll_top.unwrap()
                    );
                    state.0.borrow_mut().scroll(
                        last_logical_scroll_top.as_ref().unwrap(),
                        height,
                        delta,
                        true,
                        &mut presenter.build_event_context(cx),
                    );
                }
                30..=34 => {
                    width = rng.gen_range(0..=2000) as f32 / 2.;
                    log::info!("changing width: {:?}", width);
                }
                35..=54 => {
                    height = rng.gen_range(0..=1000) as f32 / 2.;
                    log::info!("changing height: {:?}", height);
                }
                _ => {
                    let mut elements = elements.borrow_mut();
                    let end_ix = rng.gen_range(0..=elements.len());
                    let start_ix = rng.gen_range(0..=end_ix);
                    let new_elements = (0..rng.gen_range(0..10))
                        .map(|_| {
                            let id = next_id;
                            next_id += 1;
                            (id, rng.gen_range(0..=200) as f32 / 2.)
                        })
                        .collect::<Vec<_>>();
                    log::info!("splice({:?}, {:?})", start_ix..end_ix, new_elements);
                    state.splice(start_ix..end_ix, new_elements.len());
                    elements.splice(start_ix..end_ix, new_elements);
                    for (ix, item) in state.0.borrow().items.cursor::<()>().enumerate() {
                        if let ListItem::Rendered(element) = item {
                            let (expected_id, _) = elements[ix];
                            element.with_metadata(|metadata: Option<&usize>| {
                                assert_eq!(*metadata.unwrap(), expected_id);
                            });
                        }
                    }
                }
            }

            let mut list = List::new(state.clone());
            let (size, logical_scroll_top) = list.layout(
                SizeConstraint::new(vec2f(0., 0.), vec2f(width, height)),
                &mut presenter.build_layout_context(false, cx),
            );
            assert_eq!(size, vec2f(width, height));
            last_logical_scroll_top = Some(logical_scroll_top);

            let state = state.0.borrow();
            log::info!("items {:?}", state.items.items(&()));

            let scroll_top = state.scroll_top(&logical_scroll_top);
            let rendered_top = (scroll_top - overdraw).max(0.);
            let rendered_bottom = scroll_top + height + overdraw;
            let mut item_top = 0.;

            log::info!(
                "rendered top {:?}, rendered bottom {:?}, scroll top {:?}",
                rendered_top,
                rendered_bottom,
                scroll_top,
            );

            let mut first_rendered_element_top = None;
            let mut last_rendered_element_bottom = None;
            assert_eq!(state.items.summary().count, elements.borrow().len());
            for (ix, item) in state.items.cursor::<()>().enumerate() {
                match item {
                    ListItem::Unrendered => {
                        let item_bottom = item_top;
                        assert!(item_bottom <= rendered_top || item_top >= rendered_bottom);
                        item_top = item_bottom;
                    }
                    ListItem::Removed(height) => {
                        let (id, expected_height) = elements.borrow()[ix];
                        assert_eq!(
                            *height, expected_height,
                            "element {} height didn't match",
                            id
                        );
                        let item_bottom = item_top + height;
                        assert!(item_bottom <= rendered_top || item_top >= rendered_bottom);
                        item_top = item_bottom;
                    }
                    ListItem::Rendered(element) => {
                        let (expected_id, expected_height) = elements.borrow()[ix];
                        element.with_metadata(|metadata: Option<&usize>| {
                            assert_eq!(*metadata.unwrap(), expected_id);
                        });
                        assert_eq!(element.size().y(), expected_height);
                        let item_bottom = item_top + element.size().y();
                        first_rendered_element_top.get_or_insert(item_top);
                        last_rendered_element_bottom = Some(item_bottom);
                        assert!(item_bottom > rendered_top || item_top < rendered_bottom);
                        item_top = item_bottom;
                    }
                }
            }

            match orientation {
                Orientation::Top => {
                    if let Some(first_rendered_element_top) = first_rendered_element_top {
                        assert!(first_rendered_element_top <= scroll_top);
                    }
                }
                Orientation::Bottom => {
                    if let Some(last_rendered_element_bottom) = last_rendered_element_bottom {
                        assert!(last_rendered_element_bottom >= scroll_top + height);
                    }
                }
            }
        }
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
            Empty::new().boxed()
        }
    }

    struct TestElement {
        id: usize,
        size: Vector2F,
    }

    impl TestElement {
        fn new(id: usize, height: f32) -> Self {
            Self {
                id,
                size: vec2f(100., height),
            }
        }
    }

    impl Element for TestElement {
        type LayoutState = ();
        type PaintState = ();

        fn layout(&mut self, _: SizeConstraint, _: &mut LayoutContext) -> (Vector2F, ()) {
            (self.size, ())
        }

        fn paint(&mut self, _: RectF, _: RectF, _: &mut (), _: &mut PaintContext) {
            todo!()
        }

        fn dispatch_event(
            &mut self,
            _: &Event,
            _: RectF,
            _: RectF,
            _: &mut (),
            _: &mut (),
            _: &mut EventContext,
        ) -> bool {
            todo!()
        }

        fn debug(&self, _: RectF, _: &(), _: &(), _: &DebugContext) -> serde_json::Value {
            self.id.into()
        }

        fn metadata(&self) -> Option<&dyn std::any::Any> {
            Some(&self.id)
        }
    }
}
