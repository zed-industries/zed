#![expect(missing_docs)]
use std::{cell::Cell, ops::Range, rc::Rc};

use crate::{prelude::*, px, relative, IntoElement};
use gpui::{
    point, quad, Along, Axis as ScrollbarAxis, Bounds, ContentMask, Corners, Edges, Element,
    ElementId, Entity, EntityId, GlobalElementId, Hitbox, Hsla, LayoutId, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, Pixels, Point, ScrollHandle, ScrollWheelEvent, Size, Style,
    UniformListScrollHandle, View, WindowContext,
};

pub struct Scrollbar {
    thumb: Range<f32>,
    state: ScrollbarState,
    kind: ScrollbarAxis,
}

#[derive(Clone)]
pub enum ScrollableHandle {
    Uniform(UniformListScrollHandle),
    NonUniform(ScrollHandle),
}

impl ScrollableHandle {
    fn content_size(&self) -> Option<Size<Pixels>> {
        match self {
            ScrollableHandle::Uniform(handle) => {
                handle.0.borrow().last_item_size.map(|size| size.contents)
            }
            ScrollableHandle::NonUniform(handle) => {
                let last_children_index = handle.children_count().checked_sub(1)?;
                // todo: PO: this is slightly wrong for horizontal scrollbar, as the last item is not necessarily the longest one.
                let mut last_item = handle.bounds_for_item(last_children_index)?;
                last_item.size.height += last_item.origin.y;
                last_item.size.width += last_item.origin.x;
                Some(last_item.size)
            }
        }
    }
    fn set_offset(&self, point: Point<Pixels>) {
        let base_handle = match self {
            ScrollableHandle::Uniform(handle) => &handle.0.borrow().base_handle,
            ScrollableHandle::NonUniform(handle) => &handle,
        };
        base_handle.set_offset(point);
    }
    fn offset(&self) -> Point<Pixels> {
        let base_handle = match self {
            ScrollableHandle::Uniform(handle) => &handle.0.borrow().base_handle,
            ScrollableHandle::NonUniform(handle) => &handle,
        };
        base_handle.offset()
    }
    fn bounds(&self) -> Bounds<Pixels> {
        let base_handle = match self {
            ScrollableHandle::Uniform(handle) => &handle.0.borrow().base_handle,
            ScrollableHandle::NonUniform(handle) => &handle,
        };
        base_handle.bounds()
    }
}
impl From<UniformListScrollHandle> for ScrollableHandle {
    fn from(value: UniformListScrollHandle) -> Self {
        Self::Uniform(value)
    }
}

impl From<ScrollHandle> for ScrollableHandle {
    fn from(value: ScrollHandle) -> Self {
        Self::NonUniform(value)
    }
}

#[derive(Clone)]
pub struct ScrollbarState {
    // If Some(), there's an active drag, offset by percentage from the top of thumb.
    drag: Rc<Cell<Option<f32>>>,
    parent_id: EntityId,
    scroll_handle: ScrollableHandle,
}

impl ScrollbarState {
    pub fn for_scrollable<V: 'static>(view: &View<V>, scroll: impl Into<ScrollableHandle>) -> Self {
        Self {
            drag: Default::default(),
            parent_id: view.entity_id(),
            scroll_handle: scroll.into(),
        }
    }

    pub fn is_dragging(&self) -> bool {
        self.drag.get().is_some()
    }

    fn thumb_bounds(&self, axis: ScrollbarAxis) -> Option<Range<f32>> {
        const MINIMUM_SCROLLBAR_PERCENTAGE_SIZE: f32 = 0.005;
        let main_dimension_size = self.scroll_handle.content_size()?.along(axis).0;
        let current_offset = self.scroll_handle.offset().along(axis).min(px(0.)).abs().0;
        let mut percentage = current_offset / main_dimension_size;
        let viewport_size = self.scroll_handle.bounds().size;
        let end_offset = (current_offset + viewport_size.along(axis).0) / main_dimension_size;
        // Scroll handle might briefly report an offset greater than the length of a list;
        // in such case we'll adjust the starting offset as well to keep the scrollbar thumb length stable.
        let overshoot = (end_offset - 1.).clamp(0., 1.);
        if overshoot > 0. {
            percentage -= overshoot;
        }
        if percentage + MINIMUM_SCROLLBAR_PERCENTAGE_SIZE > 1.0 || end_offset > main_dimension_size
        {
            return None;
        }
        if main_dimension_size < viewport_size.along(axis).0 {
            return None;
        }
        let end_offset = end_offset.clamp(percentage + MINIMUM_SCROLLBAR_PERCENTAGE_SIZE, 1.);
        Some(percentage..end_offset)
    }
}

impl Scrollbar {
    pub fn vertical(state: ScrollbarState) -> Option<Self> {
        Self::new(state, ScrollbarAxis::Vertical)
    }

    pub fn horizontal(state: ScrollbarState) -> Option<Self> {
        Self::new(state, ScrollbarAxis::Horizontal)
    }
    fn new(state: ScrollbarState, kind: ScrollbarAxis) -> Option<Self> {
        let thumb = state.thumb_bounds(kind)?;
        Some(Self { thumb, state, kind })
    }
}

impl Element for Scrollbar {
    type RequestLayoutState = ();

    type PrepaintState = Hitbox;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.flex_grow = 1.;
        style.flex_shrink = 1.;
        if self.kind == ScrollbarAxis::Vertical {
            style.size.width = px(12.).into();
            style.size.height = relative(1.).into();
        } else {
            style.size.width = relative(1.).into();
            style.size.height = px(12.).into();
        }

        (cx.request_layout(style, None), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) -> Self::PrepaintState {
        cx.with_content_mask(Some(ContentMask { bounds }), |cx| {
            cx.insert_hitbox(bounds, false)
        })
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        cx: &mut WindowContext,
    ) {
        cx.with_content_mask(Some(ContentMask { bounds }), |cx| {
            let colors = cx.theme().colors();
            let thumb_background = colors.scrollbar_thumb_background;
            let is_vertical = self.kind == ScrollbarAxis::Vertical;
            let extra_padding = px(5.0);
            let padded_bounds = if is_vertical {
                Bounds::from_corners(
                    bounds.origin + point(Pixels::ZERO, extra_padding),
                    bounds.lower_right() - point(Pixels::ZERO, extra_padding * 3),
                )
            } else {
                Bounds::from_corners(
                    bounds.origin + point(extra_padding, Pixels::ZERO),
                    bounds.lower_right() - point(extra_padding * 3, Pixels::ZERO),
                )
            };

            let mut thumb_bounds = if is_vertical {
                let thumb_offset = self.thumb.start * padded_bounds.size.height;
                let thumb_end = self.thumb.end * padded_bounds.size.height;
                let thumb_upper_left = point(
                    padded_bounds.origin.x,
                    padded_bounds.origin.y + thumb_offset,
                );
                let thumb_lower_right = point(
                    padded_bounds.origin.x + padded_bounds.size.width,
                    padded_bounds.origin.y + thumb_end,
                );
                Bounds::from_corners(thumb_upper_left, thumb_lower_right)
            } else {
                let thumb_offset = self.thumb.start * padded_bounds.size.width;
                let thumb_end = self.thumb.end * padded_bounds.size.width;
                let thumb_upper_left = point(
                    padded_bounds.origin.x + thumb_offset,
                    padded_bounds.origin.y,
                );
                let thumb_lower_right = point(
                    padded_bounds.origin.x + thumb_end,
                    padded_bounds.origin.y + padded_bounds.size.height,
                );
                Bounds::from_corners(thumb_upper_left, thumb_lower_right)
            };
            let corners = if is_vertical {
                thumb_bounds.size.width /= 1.5;
                Corners::all(thumb_bounds.size.width / 2.0)
            } else {
                thumb_bounds.size.height /= 1.5;
                Corners::all(thumb_bounds.size.height / 2.0)
            };
            cx.paint_quad(quad(
                thumb_bounds,
                corners,
                thumb_background,
                Edges::default(),
                Hsla::transparent_black(),
            ));

            let scroll = self.state.scroll_handle.clone();
            let kind = self.kind;
            let thumb_percentage_size = self.thumb.end - self.thumb.start;

            cx.on_mouse_event({
                let scroll = scroll.clone();
                let state = self.state.clone();
                let axis = self.kind;
                move |event: &MouseDownEvent, phase, _cx| {
                    if !(phase.bubble() && bounds.contains(&event.position)) {
                        return;
                    }

                    if thumb_bounds.contains(&event.position) {
                        let thumb_offset = (event.position.along(axis)
                            - thumb_bounds.origin.along(axis))
                            / bounds.size.along(axis);
                        state.drag.set(Some(thumb_offset));
                    } else if let Some(item_size) = scroll.content_size() {
                        match kind {
                            ScrollbarAxis::Horizontal => {
                                let percentage =
                                    (event.position.x - bounds.origin.x) / bounds.size.width;
                                let max_offset = item_size.width;
                                let percentage = percentage.min(1. - thumb_percentage_size);
                                scroll
                                    .set_offset(point(-max_offset * percentage, scroll.offset().y));
                            }
                            ScrollbarAxis::Vertical => {
                                let percentage =
                                    (event.position.y - bounds.origin.y) / bounds.size.height;
                                let max_offset = item_size.height;
                                let percentage = percentage.min(1. - thumb_percentage_size);
                                scroll
                                    .set_offset(point(scroll.offset().x, -max_offset * percentage));
                            }
                        }
                    }
                }
            });
            cx.on_mouse_event({
                let scroll = scroll.clone();
                move |event: &ScrollWheelEvent, phase, cx| {
                    if phase.bubble() && bounds.contains(&event.position) {
                        let current_offset = scroll.offset();

                        scroll
                            .set_offset(current_offset + event.delta.pixel_delta(cx.line_height()));
                    }
                }
            });
            let state = self.state.clone();
            let kind = self.kind;
            cx.on_mouse_event(move |event: &MouseMoveEvent, _, cx| {
                if let Some(drag_state) = state.drag.get().filter(|_| event.dragging()) {
                    if let Some(item_size) = scroll.content_size() {
                        match kind {
                            ScrollbarAxis::Horizontal => {
                                let max_offset = item_size.width;
                                let percentage = (event.position.x - bounds.origin.x)
                                    / bounds.size.width
                                    - drag_state;

                                let percentage = percentage.min(1. - thumb_percentage_size);
                                scroll
                                    .set_offset(point(-max_offset * percentage, scroll.offset().y));
                            }
                            ScrollbarAxis::Vertical => {
                                let max_offset = item_size.height;
                                let percentage = (event.position.y - bounds.origin.y)
                                    / bounds.size.height
                                    - drag_state;

                                let percentage = percentage.min(1. - thumb_percentage_size);
                                scroll
                                    .set_offset(point(scroll.offset().x, -max_offset * percentage));
                            }
                        };

                        cx.notify(state.parent_id);
                    }
                } else {
                    state.drag.set(None);
                }
            });
            let state = self.state.clone();
            cx.on_mouse_event(move |_event: &MouseUpEvent, phase, cx| {
                if phase.bubble() {
                    state.drag.take();
                    cx.notify(state.parent_id);
                }
            });
        })
    }
}

impl IntoElement for Scrollbar {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
