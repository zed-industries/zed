use crate::{
    px, AnyElement, AvailableSpace, BorrowAppContext, DispatchPhase, Element, IntoElement, Pixels,
    Point, ScrollWheelEvent, Size, Style, StyleRefinement, ViewContext, WindowContext,
};
use collections::VecDeque;
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
    last_layout_width: Option<Pixels>,
    render_item: Box<dyn FnMut(usize, &mut WindowContext) -> AnyElement>,
    items: SumTree<ListItem>,
    logical_scroll_top: Option<ListOffset>,
    orientation: Orientation,
    overdraw: Pixels,
    #[allow(clippy::type_complexity)]
    scroll_handler: Option<Box<dyn FnMut(&ListScrollEvent, &mut WindowContext)>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Orientation {
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
    pub fn new<F, V>(
        element_count: usize,
        orientation: Orientation,
        overdraw: Pixels,
        cx: &mut ViewContext<V>,
        mut render_item: F,
    ) -> Self
    where
        F: 'static + FnMut(&mut V, usize, &mut ViewContext<V>) -> AnyElement,
        V: 'static,
    {
        let mut items = SumTree::new();
        items.extend((0..element_count).map(|_| ListItem::Unrendered), &());
        let view = cx.view().clone();
        Self(Rc::new(RefCell::new(StateInner {
            last_layout_width: None,
            render_item: Box::new(move |ix, cx| {
                view.update(cx, |view, cx| render_item(view, ix, cx))
            }),
            items,
            logical_scroll_top: None,
            orientation,
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
        &mut self,
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
            .unwrap_or_else(|| match self.orientation {
                Orientation::Top => ListOffset {
                    item_ix: 0,
                    offset_in_item: px(0.),
                },
                Orientation::Bottom => ListOffset {
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

#[derive(Debug, Clone, Copy)]
pub struct ListOffset {
    pub item_ix: usize,
    pub offset_in_item: Pixels,
}

impl Element for List {
    type State = ();

    fn layout(
        &mut self,
        _state: Option<Self::State>,
        cx: &mut crate::WindowContext,
    ) -> (crate::LayoutId, Self::State) {
        let style = Style::from(self.style.clone());
        let layout_id = cx.with_text_style(style.text_style().cloned(), |cx| {
            cx.request_layout(&style, None)
        });
        (layout_id, ())
    }

    fn paint(
        self,
        bounds: crate::Bounds<crate::Pixels>,
        _state: &mut Self::State,
        cx: &mut crate::WindowContext,
    ) {
        let state = &mut *self.state.0.borrow_mut();

        // If the width of the list has changed, invalidate all cached item heights
        if state.last_layout_width != Some(bounds.size.width) {
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

            match state.orientation {
                Orientation::Top => {
                    scroll_top.offset_in_item = scroll_top.offset_in_item.max(px(0.));
                    state.logical_scroll_top = Some(scroll_top);
                }
                Orientation::Bottom => {
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
        for mut item_element in item_elements {
            let item_height = item_element.measure(available_item_space, cx).height;
            item_element.draw(item_origin, available_item_space, cx);
            item_origin.y += item_height;
        }

        state.items = new_items;
        state.last_layout_width = Some(bounds.size.width);

        let list_state = self.state.clone();
        let height = bounds.size.height;
        cx.on_mouse_event(move |event: &ScrollWheelEvent, phase, cx| {
            if phase == DispatchPhase::Bubble {
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
