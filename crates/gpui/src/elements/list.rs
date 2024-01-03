use crate::{
    point, px, AnyElement, AvailableSpace, BorrowAppContext, Bounds, DispatchPhase, Element,
    IntoElement, Pixels, Point, ScrollWheelEvent, Size, Style, StyleRefinement, Styled,
    WindowContext,
};
use collections::VecDeque;
use refineable::Refineable as _;
use std::{cell::RefCell, ops::Range, rc::Rc};
use sum_tree::{Bias, SumTree};

pub fn list(state: ListState) -> List {
    List {
        state,
        style: StyleRefinement::default(),
    }
}

pub struct List {
    state: ListState,
    style: StyleRefinement,
}

#[derive(Clone)]
pub struct ListState(Rc<RefCell<StateInner>>);

struct StateInner {
    last_layout_bounds: Option<Bounds<Pixels>>,
    render_item: Box<dyn FnMut(usize, &mut WindowContext) -> AnyElement>,
    items: SumTree<ListItem>,
    logical_scroll_top: Option<ListOffset>,
    alignment: ListAlignment,
    overdraw: Pixels,
    #[allow(clippy::type_complexity)]
    scroll_handler: Option<Box<dyn FnMut(&ListScrollEvent, &mut WindowContext)>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListAlignment {
    Top,
    Bottom,
}

pub struct ListScrollEvent {
    pub visible_range: Range<usize>,
    pub count: usize,
}

#[derive(Clone)]
enum ListItem {
    Unrendered,
    Rendered { height: Pixels },
}

#[derive(Clone, Debug, Default, PartialEq)]
struct ListItemSummary {
    count: usize,
    rendered_count: usize,
    unrendered_count: usize,
    height: Pixels,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Count(usize);

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct RenderedCount(usize);

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct UnrenderedCount(usize);

#[derive(Clone, Debug, Default)]
struct Height(Pixels);

impl ListState {
    pub fn new<F>(
        element_count: usize,
        orientation: ListAlignment,
        overdraw: Pixels,
        render_item: F,
    ) -> Self
    where
        F: 'static + FnMut(usize, &mut WindowContext) -> AnyElement,
    {
        let mut items = SumTree::new();
        items.extend((0..element_count).map(|_| ListItem::Unrendered), &());
        Self(Rc::new(RefCell::new(StateInner {
            last_layout_bounds: None,
            render_item: Box::new(render_item),
            items,
            logical_scroll_top: None,
            alignment: orientation,
            overdraw,
            scroll_handler: None,
        })))
    }

    pub fn reset(&self, element_count: usize) {
        let state = &mut *self.0.borrow_mut();
        state.logical_scroll_top = None;
        state.items = SumTree::new();
        state
            .items
            .extend((0..element_count).map(|_| ListItem::Unrendered), &());
    }

    pub fn item_count(&self) -> usize {
        self.0.borrow().items.summary().count
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
                *offset_in_item = px(0.);
            } else if old_range.end <= *item_ix {
                *item_ix = *item_ix - (old_range.end - old_range.start) + count;
            }
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
        &self,
        handler: impl FnMut(&ListScrollEvent, &mut WindowContext) + 'static,
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
            scroll_top.offset_in_item = px(0.);
        }
        state.logical_scroll_top = Some(scroll_top);
    }

    pub fn scroll_to_reveal_item(&self, ix: usize) {
        let state = &mut *self.0.borrow_mut();
        let mut scroll_top = state.logical_scroll_top();
        let height = state
            .last_layout_bounds
            .map_or(px(0.), |bounds| bounds.size.height);

        if ix <= scroll_top.item_ix {
            scroll_top.item_ix = ix;
            scroll_top.offset_in_item = px(0.);
        } else {
            let mut cursor = state.items.cursor::<ListItemSummary>();
            cursor.seek(&Count(ix + 1), Bias::Right, &());
            let bottom = cursor.start().height;
            let goal_top = px(0.).max(bottom - height);

            cursor.seek(&Height(goal_top), Bias::Left, &());
            let start_ix = cursor.start().count;
            let start_item_top = cursor.start().height;

            if start_ix >= scroll_top.item_ix {
                scroll_top.item_ix = start_ix;
                scroll_top.offset_in_item = goal_top - start_item_top;
            }
        }

        state.logical_scroll_top = Some(scroll_top);
    }

    /// Get the bounds for the given item in window coordinates.
    pub fn bounds_for_item(&self, ix: usize) -> Option<Bounds<Pixels>> {
        let state = &*self.0.borrow();
        let bounds = state.last_layout_bounds.unwrap_or_default();
        let scroll_top = state.logical_scroll_top();

        if ix < scroll_top.item_ix {
            return None;
        }

        let mut cursor = state.items.cursor::<(Count, Height)>();
        cursor.seek(&Count(scroll_top.item_ix), Bias::Right, &());

        let scroll_top = cursor.start().1 .0 + scroll_top.offset_in_item;

        cursor.seek_forward(&Count(ix), Bias::Right, &());
        if let Some(&ListItem::Rendered { height }) = cursor.item() {
            let &(Count(count), Height(top)) = cursor.start();
            if count == ix {
                let top = bounds.top() + top - scroll_top;
                return Some(Bounds::from_corners(
                    point(bounds.left(), top),
                    point(bounds.right(), top + height),
                ));
            }
        }
        None
    }
}

impl StateInner {
    fn visible_range(&self, height: Pixels, scroll_top: &ListOffset) -> Range<usize> {
        let mut cursor = self.items.cursor::<ListItemSummary>();
        cursor.seek(&Count(scroll_top.item_ix), Bias::Right, &());
        let start_y = cursor.start().height + scroll_top.offset_in_item;
        cursor.seek_forward(&Height(start_y + height), Bias::Left, &());
        scroll_top.item_ix..cursor.start().count + 1
    }

    fn scroll(
        &mut self,
        scroll_top: &ListOffset,
        height: Pixels,
        delta: Point<Pixels>,
        cx: &mut WindowContext,
    ) {
        let scroll_max = (self.items.summary().height - height).max(px(0.));
        let new_scroll_top = (self.scroll_top(scroll_top) - delta.y)
            .max(px(0.))
            .min(scroll_max);

        if self.alignment == ListAlignment::Bottom && new_scroll_top == scroll_max {
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
            self.scroll_handler.as_mut().unwrap()(
                &ListScrollEvent {
                    visible_range,
                    count: self.items.summary().count,
                },
                cx,
            );
        }

        cx.notify();
    }

    fn logical_scroll_top(&self) -> ListOffset {
        self.logical_scroll_top
            .unwrap_or_else(|| match self.alignment {
                ListAlignment::Top => ListOffset {
                    item_ix: 0,
                    offset_in_item: px(0.),
                },
                ListAlignment::Bottom => ListOffset {
                    item_ix: self.items.summary().count,
                    offset_in_item: px(0.),
                },
            })
    }

    fn scroll_top(&self, logical_scroll_top: &ListOffset) -> Pixels {
        let mut cursor = self.items.cursor::<ListItemSummary>();
        cursor.seek(&Count(logical_scroll_top.item_ix), Bias::Right, &());
        cursor.start().height + logical_scroll_top.offset_in_item
    }
}

impl std::fmt::Debug for ListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unrendered => write!(f, "Unrendered"),
            Self::Rendered { height, .. } => {
                f.debug_struct("Rendered").field("height", height).finish()
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ListOffset {
    pub item_ix: usize,
    pub offset_in_item: Pixels,
}

impl Element for List {
    type State = ();

    fn request_layout(
        &mut self,
        _state: Option<Self::State>,
        cx: &mut crate::WindowContext,
    ) -> (crate::LayoutId, Self::State) {
        let mut style = Style::default();
        style.refine(&self.style);
        let layout_id = cx.with_text_style(style.text_style().cloned(), |cx| {
            cx.request_layout(&style, None)
        });
        (layout_id, ())
    }

    fn paint(
        &mut self,
        bounds: crate::Bounds<crate::Pixels>,
        _state: &mut Self::State,
        cx: &mut crate::WindowContext,
    ) {
        let state = &mut *self.state.0.borrow_mut();

        // If the width of the list has changed, invalidate all cached item heights
        if state.last_layout_bounds.map_or(true, |last_bounds| {
            last_bounds.size.width != bounds.size.width
        }) {
            state.items = SumTree::from_iter(
                (0..state.items.summary().count).map(|_| ListItem::Unrendered),
                &(),
            )
        }

        let old_items = state.items.clone();
        let mut measured_items = VecDeque::new();
        let mut item_elements = VecDeque::new();
        let mut rendered_height = px(0.);
        let mut scroll_top = state.logical_scroll_top();

        let available_item_space = Size {
            width: AvailableSpace::Definite(bounds.size.width),
            height: AvailableSpace::MinContent,
        };

        // Render items after the scroll top, including those in the trailing overdraw
        let mut cursor = old_items.cursor::<Count>();
        cursor.seek(&Count(scroll_top.item_ix), Bias::Right, &());
        for (ix, item) in cursor.by_ref().enumerate() {
            let visible_height = rendered_height - scroll_top.offset_in_item;
            if visible_height >= bounds.size.height + state.overdraw {
                break;
            }

            // Use the previously cached height if available
            let mut height = if let ListItem::Rendered { height } = item {
                Some(*height)
            } else {
                None
            };

            // If we're within the visible area or the height wasn't cached, render and measure the item's element
            if visible_height < bounds.size.height || height.is_none() {
                let mut element = (state.render_item)(scroll_top.item_ix + ix, cx);
                let element_size = element.measure(available_item_space, cx);
                height = Some(element_size.height);
                if visible_height < bounds.size.height {
                    item_elements.push_back(element);
                }
            }

            let height = height.unwrap();
            rendered_height += height;
            measured_items.push_back(ListItem::Rendered { height });
        }

        // Prepare to start walking upward from the item at the scroll top.
        cursor.seek(&Count(scroll_top.item_ix), Bias::Right, &());

        // If the rendered items do not fill the visible region, then adjust
        // the scroll top upward.
        if rendered_height - scroll_top.offset_in_item < bounds.size.height {
            while rendered_height < bounds.size.height {
                cursor.prev(&());
                if cursor.item().is_some() {
                    let mut element = (state.render_item)(cursor.start().0, cx);
                    let element_size = element.measure(available_item_space, cx);

                    rendered_height += element_size.height;
                    measured_items.push_front(ListItem::Rendered {
                        height: element_size.height,
                    });
                    item_elements.push_front(element)
                } else {
                    break;
                }
            }

            scroll_top = ListOffset {
                item_ix: cursor.start().0,
                offset_in_item: rendered_height - bounds.size.height,
            };

            match state.alignment {
                ListAlignment::Top => {
                    scroll_top.offset_in_item = scroll_top.offset_in_item.max(px(0.));
                    state.logical_scroll_top = Some(scroll_top);
                }
                ListAlignment::Bottom => {
                    scroll_top = ListOffset {
                        item_ix: cursor.start().0,
                        offset_in_item: rendered_height - bounds.size.height,
                    };
                    state.logical_scroll_top = None;
                }
            };
        }

        // Measure items in the leading overdraw
        let mut leading_overdraw = scroll_top.offset_in_item;
        while leading_overdraw < state.overdraw {
            cursor.prev(&());
            if let Some(item) = cursor.item() {
                let height = if let ListItem::Rendered { height } = item {
                    *height
                } else {
                    let mut element = (state.render_item)(cursor.start().0, cx);
                    element.measure(available_item_space, cx).height
                };

                leading_overdraw += height;
                measured_items.push_front(ListItem::Rendered { height });
            } else {
                break;
            }
        }

        let measured_range = cursor.start().0..(cursor.start().0 + measured_items.len());
        let mut cursor = old_items.cursor::<Count>();
        let mut new_items = cursor.slice(&Count(measured_range.start), Bias::Right, &());
        new_items.extend(measured_items, &());
        cursor.seek(&Count(measured_range.end), Bias::Right, &());
        new_items.append(cursor.suffix(&()), &());

        // Paint the visible items
        let mut item_origin = bounds.origin;
        item_origin.y -= scroll_top.offset_in_item;
        for item_element in &mut item_elements {
            let item_height = item_element.measure(available_item_space, cx).height;
            item_element.draw(item_origin, available_item_space, cx);
            item_origin.y += item_height;
        }

        state.items = new_items;
        state.last_layout_bounds = Some(bounds);

        let list_state = self.state.clone();
        let height = bounds.size.height;
        cx.on_mouse_event(move |event: &ScrollWheelEvent, phase, cx| {
            if phase == DispatchPhase::Bubble
                && bounds.contains(&event.position)
                && cx.was_top_layer(&event.position, cx.stacking_order())
            {
                list_state.0.borrow_mut().scroll(
                    &scroll_top,
                    height,
                    event.delta.pixel_delta(px(20.)),
                    cx,
                )
            }
        });
    }
}

impl IntoElement for List {
    type Element = Self;

    fn element_id(&self) -> Option<crate::ElementId> {
        None
    }

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Styled for List {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
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
                height: px(0.),
            },
            ListItem::Rendered { height } => ListItemSummary {
                count: 1,
                rendered_count: 1,
                unrendered_count: 0,
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
