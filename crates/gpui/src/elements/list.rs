//! A list element that can be used to render a large number of differently sized elements
//! efficiently. Clients of this API need to ensure that elements outside of the scrolled
//! area do not change their height for this element to function correctly. In order to minimize
//! re-renders, this element's state is stored intrusively on your own views, so that your code
//! can coordinate directly with the list element's cached state.
//!
//! If all of your elements are the same height, see [`UniformList`] for a simpler API

use crate::{
    point, px, size, AnyElement, AvailableSpace, Bounds, ContentMask, DispatchPhase, Edges,
    Element, FocusHandle, GlobalElementId, Hitbox, IntoElement, Pixels, Point, ScrollWheelEvent,
    Size, Style, StyleRefinement, Styled, WindowContext,
};
use collections::VecDeque;
use refineable::Refineable as _;
use std::{cell::RefCell, ops::Range, rc::Rc};
use sum_tree::{Bias, SumTree};
use taffy::style::Overflow;

/// Construct a new list element
pub fn list(state: ListState) -> List {
    List {
        state,
        style: StyleRefinement::default(),
        sizing_behavior: ListSizingBehavior::default(),
    }
}

/// A list element
pub struct List {
    state: ListState,
    style: StyleRefinement,
    sizing_behavior: ListSizingBehavior,
}

impl List {
    /// Set the sizing behavior for the list.
    pub fn with_sizing_behavior(mut self, behavior: ListSizingBehavior) -> Self {
        self.sizing_behavior = behavior;
        self
    }
}

/// The list state that views must hold on behalf of the list element.
#[derive(Clone)]
pub struct ListState(Rc<RefCell<StateInner>>);

struct StateInner {
    last_layout_bounds: Option<Bounds<Pixels>>,
    last_padding: Option<Edges<Pixels>>,
    render_item: Box<dyn FnMut(usize, &mut WindowContext) -> AnyElement>,
    items: SumTree<ListItem>,
    logical_scroll_top: Option<ListOffset>,
    alignment: ListAlignment,
    overdraw: Pixels,
    reset: bool,
    #[allow(clippy::type_complexity)]
    scroll_handler: Option<Box<dyn FnMut(&ListScrollEvent, &mut WindowContext)>>,
}

/// Whether the list is scrolling from top to bottom or bottom to top.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListAlignment {
    /// The list is scrolling from top to bottom, like most lists.
    Top,
    /// The list is scrolling from bottom to top, like a chat log.
    Bottom,
}

/// A scroll event that has been converted to be in terms of the list's items.
pub struct ListScrollEvent {
    /// The range of items currently visible in the list, after applying the scroll event.
    pub visible_range: Range<usize>,

    /// The number of items that are currently visible in the list, after applying the scroll event.
    pub count: usize,

    /// Whether the list has been scrolled.
    pub is_scrolled: bool,
}

/// The sizing behavior to apply during layout.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ListSizingBehavior {
    /// The list should calculate its size based on the size of its items.
    Infer,
    /// The list should not calculate a fixed size.
    #[default]
    Auto,
}

struct LayoutItemsResponse {
    max_item_width: Pixels,
    scroll_top: ListOffset,
    item_layouts: VecDeque<ItemLayout>,
}

struct ItemLayout {
    index: usize,
    element: AnyElement,
    size: Size<Pixels>,
}

/// Frame state used by the [List] element after layout.
pub struct ListPrepaintState {
    hitbox: Hitbox,
    layout: LayoutItemsResponse,
}

#[derive(Clone)]
enum ListItem {
    Unmeasured {
        focus_handle: Option<FocusHandle>,
    },
    Measured {
        size: Size<Pixels>,
        focus_handle: Option<FocusHandle>,
    },
}

impl ListItem {
    fn size(&self) -> Option<Size<Pixels>> {
        if let ListItem::Measured { size, .. } = self {
            Some(*size)
        } else {
            None
        }
    }

    fn focus_handle(&self) -> Option<FocusHandle> {
        match self {
            ListItem::Unmeasured { focus_handle } | ListItem::Measured { focus_handle, .. } => {
                focus_handle.clone()
            }
        }
    }

    fn contains_focused(&self, cx: &WindowContext) -> bool {
        match self {
            ListItem::Unmeasured { focus_handle } | ListItem::Measured { focus_handle, .. } => {
                focus_handle
                    .as_ref()
                    .is_some_and(|handle| handle.contains_focused(cx))
            }
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct ListItemSummary {
    count: usize,
    rendered_count: usize,
    unrendered_count: usize,
    height: Pixels,
    has_focus_handles: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Count(usize);

#[derive(Clone, Debug, Default)]
struct Height(Pixels);

impl ListState {
    /// Construct a new list state, for storage on a view.
    ///
    /// The overdraw parameter controls how much extra space is rendered
    /// above and below the visible area. Elements within this area will
    /// be measured even though they are not visible. This can help ensure
    /// that the list doesn't flicker or pop in when scrolling.
    pub fn new<R>(
        item_count: usize,
        alignment: ListAlignment,
        overdraw: Pixels,
        render_item: R,
    ) -> Self
    where
        R: 'static + FnMut(usize, &mut WindowContext) -> AnyElement,
    {
        let this = Self(Rc::new(RefCell::new(StateInner {
            last_layout_bounds: None,
            last_padding: None,
            render_item: Box::new(render_item),
            items: SumTree::new(),
            logical_scroll_top: None,
            alignment,
            overdraw,
            scroll_handler: None,
            reset: false,
        })));
        this.splice(0..0, item_count);
        this
    }

    /// Reset this instantiation of the list state.
    ///
    /// Note that this will cause scroll events to be dropped until the next paint.
    pub fn reset(&self, element_count: usize) {
        let old_count = {
            let state = &mut *self.0.borrow_mut();
            state.reset = true;
            state.logical_scroll_top = None;
            state.items.summary().count
        };

        self.splice(0..old_count, element_count);
    }

    /// The number of items in this list.
    pub fn item_count(&self) -> usize {
        self.0.borrow().items.summary().count
    }

    /// Inform the list state that the items in `old_range` have been replaced
    /// by `count` new items that must be recalculated.
    pub fn splice(&self, old_range: Range<usize>, count: usize) {
        self.splice_focusable(old_range, (0..count).map(|_| None))
    }

    /// Register with the list state that the items in `old_range` have been replaced
    /// by new items. As opposed to [`splice`], this method allows an iterator of optional focus handles
    /// to be supplied to properly integrate with items in the list that can be focused. If a focused item
    /// is scrolled out of view, the list will continue to render it to allow keyboard interaction.
    pub fn splice_focusable(
        &self,
        old_range: Range<usize>,
        focus_handles: impl IntoIterator<Item = Option<FocusHandle>>,
    ) {
        let state = &mut *self.0.borrow_mut();

        let mut old_items = state.items.cursor::<Count>();
        let mut new_items = old_items.slice(&Count(old_range.start), Bias::Right, &());
        old_items.seek_forward(&Count(old_range.end), Bias::Right, &());

        let mut spliced_count = 0;
        new_items.extend(
            focus_handles.into_iter().map(|focus_handle| {
                spliced_count += 1;
                ListItem::Unmeasured { focus_handle }
            }),
            &(),
        );
        new_items.append(old_items.suffix(&()), &());
        drop(old_items);
        state.items = new_items;

        if let Some(ListOffset {
            item_ix,
            offset_in_item,
        }) = state.logical_scroll_top.as_mut()
        {
            if old_range.contains(item_ix) {
                *item_ix = old_range.start;
                *offset_in_item = px(0.);
            } else if old_range.end <= *item_ix {
                *item_ix = *item_ix - (old_range.end - old_range.start) + spliced_count;
            }
        }
    }

    /// Set a handler that will be called when the list is scrolled.
    pub fn set_scroll_handler(
        &self,
        handler: impl FnMut(&ListScrollEvent, &mut WindowContext) + 'static,
    ) {
        self.0.borrow_mut().scroll_handler = Some(Box::new(handler))
    }

    /// Get the current scroll offset, in terms of the list's items.
    pub fn logical_scroll_top(&self) -> ListOffset {
        self.0.borrow().logical_scroll_top()
    }

    /// Scroll the list to the given offset
    pub fn scroll_to(&self, mut scroll_top: ListOffset) {
        let state = &mut *self.0.borrow_mut();
        let item_count = state.items.summary().count;
        if scroll_top.item_ix >= item_count {
            scroll_top.item_ix = item_count;
            scroll_top.offset_in_item = px(0.);
        }

        state.logical_scroll_top = Some(scroll_top);
    }

    /// Scroll the list to the given item, such that the item is fully visible.
    pub fn scroll_to_reveal_item(&self, ix: usize) {
        let state = &mut *self.0.borrow_mut();

        let mut scroll_top = state.logical_scroll_top();
        let height = state
            .last_layout_bounds
            .map_or(px(0.), |bounds| bounds.size.height);
        let padding = state.last_padding.unwrap_or_default();

        if ix <= scroll_top.item_ix {
            scroll_top.item_ix = ix;
            scroll_top.offset_in_item = px(0.);
        } else {
            let mut cursor = state.items.cursor::<ListItemSummary>();
            cursor.seek(&Count(ix + 1), Bias::Right, &());
            let bottom = cursor.start().height + padding.top;
            let goal_top = px(0.).max(bottom - height + padding.bottom);

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

    /// Get the bounds for the given item in window coordinates, if it's
    /// been rendered.
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
        if let Some(&ListItem::Measured { size, .. }) = cursor.item() {
            let &(Count(count), Height(top)) = cursor.start();
            if count == ix {
                let top = bounds.top() + top - scroll_top;
                return Some(Bounds::from_corners(
                    point(bounds.left(), top),
                    point(bounds.right(), top + size.height),
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
        // Drop scroll events after a reset, since we can't calculate
        // the new logical scroll top without the item heights
        if self.reset {
            return;
        }

        let padding = self.last_padding.unwrap_or_default();
        let scroll_max =
            (self.items.summary().height + padding.top + padding.bottom - height).max(px(0.));
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
                    is_scrolled: self.logical_scroll_top.is_some(),
                },
                cx,
            );
        }

        cx.refresh();
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

    fn layout_items(
        &mut self,
        available_width: Option<Pixels>,
        available_height: Pixels,
        padding: &Edges<Pixels>,
        cx: &mut WindowContext,
    ) -> LayoutItemsResponse {
        let old_items = self.items.clone();
        let mut measured_items = VecDeque::new();
        let mut item_layouts = VecDeque::new();
        let mut rendered_height = padding.top;
        let mut max_item_width = px(0.);
        let mut scroll_top = self.logical_scroll_top();
        let mut rendered_focused_item = false;

        let available_item_space = size(
            available_width.map_or(AvailableSpace::MinContent, |width| {
                AvailableSpace::Definite(width)
            }),
            AvailableSpace::MinContent,
        );

        let mut cursor = old_items.cursor::<Count>();

        // Render items after the scroll top, including those in the trailing overdraw
        cursor.seek(&Count(scroll_top.item_ix), Bias::Right, &());
        for (ix, item) in cursor.by_ref().enumerate() {
            let visible_height = rendered_height - scroll_top.offset_in_item;
            if visible_height >= available_height + self.overdraw {
                break;
            }

            // Use the previously cached height and focus handle if available
            let mut size = item.size();

            // If we're within the visible area or the height wasn't cached, render and measure the item's element
            if visible_height < available_height || size.is_none() {
                let item_index = scroll_top.item_ix + ix;
                let mut element = (self.render_item)(item_index, cx);
                let element_size = element.layout_as_root(available_item_space, cx);
                size = Some(element_size);
                if visible_height < available_height {
                    item_layouts.push_back(ItemLayout {
                        index: item_index,
                        element,
                        size: element_size,
                    });
                    if item.contains_focused(cx) {
                        rendered_focused_item = true;
                    }
                }
            }

            let size = size.unwrap();
            rendered_height += size.height;
            max_item_width = max_item_width.max(size.width);
            measured_items.push_back(ListItem::Measured {
                size,
                focus_handle: item.focus_handle(),
            });
        }
        rendered_height += padding.bottom;

        // Prepare to start walking upward from the item at the scroll top.
        cursor.seek(&Count(scroll_top.item_ix), Bias::Right, &());

        // If the rendered items do not fill the visible region, then adjust
        // the scroll top upward.
        if rendered_height - scroll_top.offset_in_item < available_height {
            while rendered_height < available_height {
                cursor.prev(&());
                if let Some(item) = cursor.item() {
                    let item_index = cursor.start().0;
                    let mut element = (self.render_item)(item_index, cx);
                    let element_size = element.layout_as_root(available_item_space, cx);
                    let focus_handle = item.focus_handle();
                    rendered_height += element_size.height;
                    measured_items.push_front(ListItem::Measured {
                        size: element_size,
                        focus_handle,
                    });
                    item_layouts.push_front(ItemLayout {
                        index: item_index,
                        element,
                        size: element_size,
                    });
                    if item.contains_focused(cx) {
                        rendered_focused_item = true;
                    }
                } else {
                    break;
                }
            }

            scroll_top = ListOffset {
                item_ix: cursor.start().0,
                offset_in_item: rendered_height - available_height,
            };

            match self.alignment {
                ListAlignment::Top => {
                    scroll_top.offset_in_item = scroll_top.offset_in_item.max(px(0.));
                    self.logical_scroll_top = Some(scroll_top);
                }
                ListAlignment::Bottom => {
                    scroll_top = ListOffset {
                        item_ix: cursor.start().0,
                        offset_in_item: rendered_height - available_height,
                    };
                    self.logical_scroll_top = None;
                }
            };
        }

        // Measure items in the leading overdraw
        let mut leading_overdraw = scroll_top.offset_in_item;
        while leading_overdraw < self.overdraw {
            cursor.prev(&());
            if let Some(item) = cursor.item() {
                let size = if let ListItem::Measured { size, .. } = item {
                    *size
                } else {
                    let mut element = (self.render_item)(cursor.start().0, cx);
                    element.layout_as_root(available_item_space, cx)
                };

                leading_overdraw += size.height;
                measured_items.push_front(ListItem::Measured {
                    size,
                    focus_handle: item.focus_handle(),
                });
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
        self.items = new_items;

        // If none of the visible items are focused, check if an off-screen item is focused
        // and include it to be rendered after the visible items so keyboard interaction continues
        // to work for it.
        if !rendered_focused_item {
            let mut cursor = self
                .items
                .filter::<_, Count>(|summary| summary.has_focus_handles);
            cursor.next(&());
            while let Some(item) = cursor.item() {
                if item.contains_focused(cx) {
                    let item_index = cursor.start().0;
                    let mut element = (self.render_item)(cursor.start().0, cx);
                    let size = element.layout_as_root(available_item_space, cx);
                    item_layouts.push_back(ItemLayout {
                        index: item_index,
                        element,
                        size,
                    });
                    break;
                }
                cursor.next(&());
            }
        }

        LayoutItemsResponse {
            max_item_width,
            scroll_top,
            item_layouts,
        }
    }

    fn prepaint_items(
        &mut self,
        bounds: Bounds<Pixels>,
        padding: Edges<Pixels>,
        autoscroll: bool,
        cx: &mut WindowContext,
    ) -> Result<LayoutItemsResponse, ListOffset> {
        cx.transact(|cx| {
            let mut layout_response =
                self.layout_items(Some(bounds.size.width), bounds.size.height, &padding, cx);

            // Avoid honoring autoscroll requests from elements other than our children.
            cx.take_autoscroll();

            // Only paint the visible items, if there is actually any space for them (taking padding into account)
            if bounds.size.height > padding.top + padding.bottom {
                let mut item_origin = bounds.origin + Point::new(px(0.), padding.top);
                item_origin.y -= layout_response.scroll_top.offset_in_item;
                for item in &mut layout_response.item_layouts {
                    cx.with_content_mask(Some(ContentMask { bounds }), |cx| {
                        item.element.prepaint_at(item_origin, cx);
                    });

                    if let Some(autoscroll_bounds) = cx.take_autoscroll() {
                        if autoscroll {
                            if autoscroll_bounds.top() < bounds.top() {
                                return Err(ListOffset {
                                    item_ix: item.index,
                                    offset_in_item: autoscroll_bounds.top() - item_origin.y,
                                });
                            } else if autoscroll_bounds.bottom() > bounds.bottom() {
                                let mut cursor = self.items.cursor::<Count>();
                                cursor.seek(&Count(item.index), Bias::Right, &());
                                let mut height = bounds.size.height - padding.top - padding.bottom;

                                // Account for the height of the element down until the autoscroll bottom.
                                height -= autoscroll_bounds.bottom() - item_origin.y;

                                // Keep decreasing the scroll top until we fill all the available space.
                                while height > Pixels::ZERO {
                                    cursor.prev(&());
                                    let Some(item) = cursor.item() else { break };

                                    let size = item.size().unwrap_or_else(|| {
                                        let mut item = (self.render_item)(cursor.start().0, cx);
                                        let item_available_size = size(
                                            bounds.size.width.into(),
                                            AvailableSpace::MinContent,
                                        );
                                        item.layout_as_root(item_available_size, cx)
                                    });
                                    height -= size.height;
                                }

                                return Err(ListOffset {
                                    item_ix: cursor.start().0,
                                    offset_in_item: if height < Pixels::ZERO {
                                        -height
                                    } else {
                                        Pixels::ZERO
                                    },
                                });
                            }
                        }
                    }

                    item_origin.y += item.size.height;
                }
            } else {
                layout_response.item_layouts.clear();
            }

            Ok(layout_response)
        })
    }
}

impl std::fmt::Debug for ListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unmeasured { .. } => write!(f, "Unrendered"),
            Self::Measured { size, .. } => f.debug_struct("Rendered").field("size", size).finish(),
        }
    }
}

/// An offset into the list's items, in terms of the item index and the number
/// of pixels off the top left of the item.
#[derive(Debug, Clone, Copy, Default)]
pub struct ListOffset {
    /// The index of an item in the list
    pub item_ix: usize,
    /// The number of pixels to offset from the item index.
    pub offset_in_item: Pixels,
}

impl Element for List {
    type RequestLayoutState = ();
    type PrepaintState = ListPrepaintState;

    fn id(&self) -> Option<crate::ElementId> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        cx: &mut crate::WindowContext,
    ) -> (crate::LayoutId, Self::RequestLayoutState) {
        let layout_id = match self.sizing_behavior {
            ListSizingBehavior::Infer => {
                let mut style = Style::default();
                style.overflow.y = Overflow::Scroll;
                style.refine(&self.style);
                cx.with_text_style(style.text_style().cloned(), |cx| {
                    let state = &mut *self.state.0.borrow_mut();

                    let available_height = if let Some(last_bounds) = state.last_layout_bounds {
                        last_bounds.size.height
                    } else {
                        // If we don't have the last layout bounds (first render),
                        // we might just use the overdraw value as the available height to layout enough items.
                        state.overdraw
                    };
                    let padding = style.padding.to_pixels(
                        state.last_layout_bounds.unwrap_or_default().size.into(),
                        cx.rem_size(),
                    );

                    let layout_response = state.layout_items(None, available_height, &padding, cx);
                    let max_element_width = layout_response.max_item_width;

                    let summary = state.items.summary();
                    let total_height = summary.height;

                    cx.request_measured_layout(
                        style,
                        move |known_dimensions, available_space, _cx| {
                            let width =
                                known_dimensions
                                    .width
                                    .unwrap_or(match available_space.width {
                                        AvailableSpace::Definite(x) => x,
                                        AvailableSpace::MinContent | AvailableSpace::MaxContent => {
                                            max_element_width
                                        }
                                    });
                            let height = match available_space.height {
                                AvailableSpace::Definite(height) => total_height.min(height),
                                AvailableSpace::MinContent | AvailableSpace::MaxContent => {
                                    total_height
                                }
                            };
                            size(width, height)
                        },
                    )
                })
            }
            ListSizingBehavior::Auto => {
                let mut style = Style::default();
                style.refine(&self.style);
                cx.with_text_style(style.text_style().cloned(), |cx| {
                    cx.request_layout(style, None)
                })
            }
        };
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) -> ListPrepaintState {
        let state = &mut *self.state.0.borrow_mut();
        state.reset = false;

        let mut style = Style::default();
        style.refine(&self.style);

        let hitbox = cx.insert_hitbox(bounds, false);

        // If the width of the list has changed, invalidate all cached item heights
        if state.last_layout_bounds.map_or(true, |last_bounds| {
            last_bounds.size.width != bounds.size.width
        }) {
            let new_items = SumTree::from_iter(
                state.items.iter().map(|item| ListItem::Unmeasured {
                    focus_handle: item.focus_handle(),
                }),
                &(),
            );

            state.items = new_items;
        }

        let padding = style.padding.to_pixels(bounds.size.into(), cx.rem_size());
        let layout = match state.prepaint_items(bounds, padding, true, cx) {
            Ok(layout) => layout,
            Err(autoscroll_request) => {
                state.logical_scroll_top = Some(autoscroll_request);
                state.prepaint_items(bounds, padding, false, cx).unwrap()
            }
        };

        state.last_layout_bounds = Some(bounds);
        state.last_padding = Some(padding);
        ListPrepaintState { hitbox, layout }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        bounds: Bounds<crate::Pixels>,
        _: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        cx: &mut crate::WindowContext,
    ) {
        cx.with_content_mask(Some(ContentMask { bounds }), |cx| {
            for item in &mut prepaint.layout.item_layouts {
                item.element.paint(cx);
            }
        });

        let list_state = self.state.clone();
        let height = bounds.size.height;
        let scroll_top = prepaint.layout.scroll_top;
        let hitbox_id = prepaint.hitbox.id;
        cx.on_mouse_event(move |event: &ScrollWheelEvent, phase, cx| {
            if phase == DispatchPhase::Bubble && hitbox_id.is_hovered(cx) {
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
            ListItem::Unmeasured { focus_handle } => ListItemSummary {
                count: 1,
                rendered_count: 0,
                unrendered_count: 1,
                height: px(0.),
                has_focus_handles: focus_handle.is_some(),
            },
            ListItem::Measured {
                size, focus_handle, ..
            } => ListItemSummary {
                count: 1,
                rendered_count: 1,
                unrendered_count: 0,
                height: size.height,
                has_focus_handles: focus_handle.is_some(),
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
        self.has_focus_handles |= summary.has_focus_handles;
    }
}

impl<'a> sum_tree::Dimension<'a, ListItemSummary> for Count {
    fn add_summary(&mut self, summary: &'a ListItemSummary, _: &()) {
        self.0 += summary.count;
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
mod test {

    use gpui::{ScrollDelta, ScrollWheelEvent};

    use crate::{self as gpui, TestAppContext};

    #[gpui::test]
    fn test_reset_after_paint_before_scroll(cx: &mut TestAppContext) {
        use crate::{div, list, point, px, size, Element, ListState, Styled};

        let cx = cx.add_empty_window();

        let state = ListState::new(5, crate::ListAlignment::Top, px(10.), |_, _| {
            div().h(px(10.)).w_full().into_any()
        });

        // Ensure that the list is scrolled to the top
        state.scroll_to(gpui::ListOffset {
            item_ix: 0,
            offset_in_item: px(0.0),
        });

        // Paint
        cx.draw(point(px(0.), px(0.)), size(px(100.), px(20.)), |_| {
            list(state.clone()).w_full().h_full()
        });

        // Reset
        state.reset(5);

        // And then receive a scroll event _before_ the next paint
        cx.simulate_event(ScrollWheelEvent {
            position: point(px(1.), px(1.)),
            delta: ScrollDelta::Pixels(point(px(0.), px(-500.))),
            ..Default::default()
        });

        // Scroll position should stay at the top of the list
        assert_eq!(state.logical_scroll_top().item_ix, 0);
        assert_eq!(state.logical_scroll_top().offset_in_item, px(0.));
    }
}
