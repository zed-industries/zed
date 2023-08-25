use crate::{
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::json,
    AnyElement, Element, LayoutContext, MouseRegion, PaintContext, SceneBuilder, SizeConstraint,
    ViewContext,
};
use std::{cell::RefCell, collections::VecDeque, fmt::Debug, ops::Range, rc::Rc};
use sum_tree::{Bias, SumTree};

pub struct List<V> {
    state: ListState<V>,
}

pub struct ListState<V>(Rc<RefCell<StateInner<V>>>);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Orientation {
    Top,
    Bottom,
}

struct StateInner<V> {
    last_layout_width: Option<f32>,
    render_item: Box<dyn FnMut(&mut V, usize, &mut ViewContext<V>) -> AnyElement<V>>,
    rendered_range: Range<usize>,
    items: SumTree<ListItem<V>>,
    logical_scroll_top: Option<ListOffset>,
    orientation: Orientation,
    overdraw: f32,
    #[allow(clippy::type_complexity)]
    scroll_handler: Option<Box<dyn FnMut(Range<usize>, &mut V, &mut ViewContext<V>)>>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ListOffset {
    pub item_ix: usize,
    pub offset_in_item: f32,
}

enum ListItem<V> {
    Unrendered,
    Rendered(Rc<RefCell<AnyElement<V>>>),
    Removed(f32),
}

impl<V> Clone for ListItem<V> {
    fn clone(&self) -> Self {
        match self {
            Self::Unrendered => Self::Unrendered,
            Self::Rendered(element) => Self::Rendered(element.clone()),
            Self::Removed(height) => Self::Removed(*height),
        }
    }
}

impl<V> Debug for ListItem<V> {
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

impl<V> List<V> {
    pub fn new(state: ListState<V>) -> Self {
        Self { state }
    }
}

impl<V: 'static> Element<V> for List<V> {
    type LayoutState = ListOffset;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut LayoutContext<V>,
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
        let mut scroll_top = state.logical_scroll_top();

        // Render items after the scroll top, including those in the trailing overdraw.
        let mut cursor = old_items.cursor::<Count>();
        cursor.seek(&Count(scroll_top.item_ix), Bias::Right, &());
        for (ix, item) in cursor.by_ref().enumerate() {
            let visible_height = rendered_height - scroll_top.offset_in_item;
            if visible_height >= size.y() + state.overdraw {
                break;
            }

            // Force re-render if the item is visible, but attempt to re-use an existing one
            // if we are inside the overdraw.
            let existing_element = if visible_height >= size.y() {
                Some(item)
            } else {
                None
            };
            if let Some(element) = state.render_item(
                scroll_top.item_ix + ix,
                existing_element,
                item_constraint,
                view,
                cx,
            ) {
                rendered_height += element.borrow().size().y();
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
                if cursor.item().is_some() {
                    if let Some(element) =
                        state.render_item(cursor.start().0, None, item_constraint, view, cx)
                    {
                        rendered_height += element.borrow().size().y();
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
                    state.render_item(cursor.start().0, Some(item), item_constraint, view, cx)
                {
                    leading_overdraw += element.borrow().size().y();
                    rendered_items.push_front(ListItem::Rendered(element));
                }
            } else {
                break;
            }
        }

        let new_rendered_range = cursor.start().0..(cursor.start().0 + rendered_items.len());

        let mut cursor = old_items.cursor::<Count>();

        if state.rendered_range.start < new_rendered_range.start {
            new_items.append(
                cursor.slice(&Count(state.rendered_range.start), Bias::Right, &()),
                &(),
            );
            let remove_to = state.rendered_range.end.min(new_rendered_range.start);
            while cursor.start().0 < remove_to {
                new_items.push(cursor.item().unwrap().remove(), &());
                cursor.next(&());
            }
        }
        new_items.append(
            cursor.slice(&Count(new_rendered_range.start), Bias::Right, &()),
            &(),
        );

        new_items.extend(rendered_items, &());
        cursor.seek(&Count(new_rendered_range.end), Bias::Right, &());

        if new_rendered_range.end < state.rendered_range.start {
            new_items.append(
                cursor.slice(&Count(state.rendered_range.start), Bias::Right, &()),
                &(),
            );
        }
        while cursor.start().0 < state.rendered_range.end {
            new_items.push(cursor.item().unwrap().remove(), &());
            cursor.next(&());
        }

        new_items.append(cursor.suffix(&()), &());

        state.items = new_items;
        state.rendered_range = new_rendered_range;
        state.last_layout_width = Some(size.x());
        (size, scroll_top)
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        scroll_top: &mut ListOffset,
        view: &mut V,
        cx: &mut PaintContext<V>,
    ) {
        let visible_bounds = visible_bounds.intersection(bounds).unwrap_or_default();
        scene.push_layer(Some(visible_bounds));
        scene.push_mouse_region(
            MouseRegion::new::<Self>(cx.view_id(), 0, bounds).on_scroll({
                let state = self.state.clone();
                let height = bounds.height();
                let scroll_top = scroll_top.clone();
                move |e, view, cx| {
                    state.0.borrow_mut().scroll(
                        &scroll_top,
                        height,
                        *e.platform_event.delta.raw(),
                        e.platform_event.delta.precise(),
                        view,
                        cx,
                    )
                }
            }),
        );

        let state = &mut *self.state.0.borrow_mut();
        for (element, origin) in state.visible_elements(bounds, scroll_top) {
            element
                .borrow_mut()
                .paint(scene, origin, visible_bounds, view, cx);
        }

        scene.pop_layer();
    }

    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        bounds: RectF,
        _: RectF,
        scroll_top: &Self::LayoutState,
        _: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> Option<RectF> {
        let state = self.state.0.borrow();
        let mut item_origin = bounds.origin() - vec2f(0., scroll_top.offset_in_item);
        let mut cursor = state.items.cursor::<Count>();
        cursor.seek(&Count(scroll_top.item_ix), Bias::Right, &());
        while let Some(item) = cursor.item() {
            if item_origin.y() > bounds.max_y() {
                break;
            }

            if let ListItem::Rendered(element) = item {
                if let Some(rect) =
                    element
                        .borrow()
                        .rect_for_text_range(range_utf16.clone(), view, cx)
                {
                    return Some(rect);
                }

                item_origin.set_y(item_origin.y() + element.borrow().size().y());
                cursor.next(&());
            } else {
                unreachable!();
            }
        }

        None
    }

    fn debug(
        &self,
        bounds: RectF,
        scroll_top: &Self::LayoutState,
        _: &(),
        view: &V,
        cx: &ViewContext<V>,
    ) -> serde_json::Value {
        let state = self.state.0.borrow_mut();
        let visible_elements = state
            .visible_elements(bounds, scroll_top)
            .map(|e| e.0.borrow().debug(view, cx))
            .collect::<Vec<_>>();
        let visible_range = scroll_top.item_ix..(scroll_top.item_ix + visible_elements.len());
        json!({
            "visible_range": visible_range,
            "visible_elements": visible_elements,
            "scroll_top": state.logical_scroll_top.map(|top| (top.item_ix, top.offset_in_item)),
        })
    }
}

impl<V: 'static> ListState<V> {
    pub fn new<D, F>(
        element_count: usize,
        orientation: Orientation,
        overdraw: f32,
        mut render_item: F,
    ) -> Self
    where
        D: Element<V>,
        F: 'static + FnMut(&mut V, usize, &mut ViewContext<V>) -> D,
    {
        let mut items = SumTree::new();
        items.extend((0..element_count).map(|_| ListItem::Unrendered), &());
        Self(Rc::new(RefCell::new(StateInner {
            last_layout_width: None,
            render_item: Box::new(move |view, ix, cx| render_item(view, ix, cx).into_any()),
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
        new_heights.append(old_heights.suffix(&()), &());
        drop(old_heights);
        state.items = new_heights;
    }

    pub fn set_scroll_handler(
        &mut self,
        handler: impl FnMut(Range<usize>, &mut V, &mut ViewContext<V>) + 'static,
    ) {
        self.0.borrow_mut().scroll_handler = Some(Box::new(handler))
    }

    pub fn logical_scroll_top(&self) -> ListOffset {
        self.0.borrow().logical_scroll_top()
    }

    pub fn scroll_to(&self, mut scroll_top: ListOffset) {
        let state = &mut *self.0.borrow_mut();
        let item_count = state.items.summary().count;
        if scroll_top.item_ix >= item_count {
            scroll_top.item_ix = item_count;
            scroll_top.offset_in_item = 0.;
        }
        state.logical_scroll_top = Some(scroll_top);
    }
}

impl<V> Clone for ListState<V> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<V: 'static> StateInner<V> {
    fn render_item(
        &mut self,
        ix: usize,
        existing_element: Option<&ListItem<V>>,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> Option<Rc<RefCell<AnyElement<V>>>> {
        if let Some(ListItem::Rendered(element)) = existing_element {
            Some(element.clone())
        } else {
            let mut element = (self.render_item)(view, ix, cx);
            element.layout(constraint, view, cx);
            Some(Rc::new(RefCell::new(element)))
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
    ) -> impl Iterator<Item = (Rc<RefCell<AnyElement<V>>>, Vector2F)> + 'a {
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
                    item_origin.set_y(item_origin.y() + element.borrow().size().y());
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
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) {
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
            self.scroll_handler.as_mut().unwrap()(visible_range, view, cx);
        }

        cx.notify();
    }

    fn logical_scroll_top(&self) -> ListOffset {
        self.logical_scroll_top
            .unwrap_or_else(|| match self.orientation {
                Orientation::Top => ListOffset {
                    item_ix: 0,
                    offset_in_item: 0.,
                },
                Orientation::Bottom => ListOffset {
                    item_ix: self.items.summary().count,
                    offset_in_item: 0.,
                },
            })
    }

    fn scroll_top(&self, logical_scroll_top: &ListOffset) -> f32 {
        let mut cursor = self.items.cursor::<ListItemSummary>();
        cursor.seek(&Count(logical_scroll_top.item_ix), Bias::Right, &());
        cursor.start().height + logical_scroll_top.offset_in_item
    }
}

impl<V> ListItem<V> {
    fn remove(&self) -> Self {
        match self {
            ListItem::Unrendered => ListItem::Unrendered,
            ListItem::Rendered(element) => ListItem::Removed(element.borrow().size().y()),
            ListItem::Removed(height) => ListItem::Removed(*height),
        }
    }
}

impl<V> sum_tree::Item for ListItem<V> {
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
                height: element.borrow().size().y(),
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
