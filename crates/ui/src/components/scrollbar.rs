#![allow(missing_docs)]
use std::{any::Any, cell::Cell, fmt::Debug, ops::Range, rc::Rc, sync::Arc};

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

impl ScrollableHandle for UniformListScrollHandle {
    fn content_size(&self) -> Option<ContentSize> {
        Some(ContentSize {
            size: self.0.borrow().last_item_size.map(|size| size.contents)?,
            scroll_adjustment: None,
        })
    }

    fn set_offset(&self, point: Point<Pixels>) {
        self.0.borrow().base_handle.set_offset(point);
    }

    fn offset(&self) -> Point<Pixels> {
        self.0.borrow().base_handle.offset()
    }

    fn viewport(&self) -> Bounds<Pixels> {
        self.0.borrow().base_handle.bounds()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ScrollableHandle for ScrollHandle {
    fn content_size(&self) -> Option<ContentSize> {
        let last_children_index = self.children_count().checked_sub(1)?;

        let mut last_item = self.bounds_for_item(last_children_index)?;
        let mut scroll_adjustment = None;

        if last_children_index != 0 {
            // todo: PO: this is slightly wrong for horizontal scrollbar, as the last item is not necessarily the longest one.
            let first_item = self.bounds_for_item(0)?;
            last_item.size.height += last_item.origin.y;
            last_item.size.width += last_item.origin.x;

            scroll_adjustment = Some(first_item.origin);
            last_item.size.height -= first_item.origin.y;
            last_item.size.width -= first_item.origin.x;
        }

        Some(ContentSize {
            size: last_item.size,
            scroll_adjustment,
        })
    }

    fn set_offset(&self, point: Point<Pixels>) {
        self.set_offset(point);
    }

    fn offset(&self) -> Point<Pixels> {
        self.offset()
    }

    fn viewport(&self) -> Bounds<Pixels> {
        self.bounds()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct ContentSize {
    pub size: Size<Pixels>,
    pub scroll_adjustment: Option<Point<Pixels>>,
}

pub trait ScrollableHandle: Debug + 'static {
    fn content_size(&self) -> Option<ContentSize>;
    fn set_offset(&self, point: Point<Pixels>);
    fn offset(&self) -> Point<Pixels>;
    fn viewport(&self) -> Bounds<Pixels>;
    fn as_any(&self) -> &dyn Any;
}

/// A scrollbar state that should be persisted across frames.
#[derive(Clone, Debug)]
pub struct ScrollbarState {
    // If Some(), there's an active drag, offset by percentage from the origin of a thumb.
    drag: Rc<Cell<Option<f32>>>,
    parent_id: Option<EntityId>,
    scroll_handle: Arc<dyn ScrollableHandle>,
}

impl ScrollbarState {
    pub fn new(scroll: impl ScrollableHandle) -> Self {
        Self {
            drag: Default::default(),
            parent_id: None,
            scroll_handle: Arc::new(scroll),
        }
    }

    /// Set a parent view which should be notified whenever this Scrollbar gets a scroll event.
    pub fn parent_view<V: 'static>(mut self, v: &View<V>) -> Self {
        self.parent_id = Some(v.entity_id());
        self
    }

    pub fn scroll_handle(&self) -> &Arc<dyn ScrollableHandle> {
        &self.scroll_handle
    }

    pub fn is_dragging(&self) -> bool {
        self.drag.get().is_some()
    }

    fn thumb_range(&self, axis: ScrollbarAxis) -> Option<Range<f32>> {
        const MINIMUM_SCROLLBAR_PERCENTAGE_SIZE: f32 = 0.005;
        let ContentSize {
            size: main_dimension_size,
            scroll_adjustment,
        } = self.scroll_handle.content_size()?;
        let main_dimension_size = main_dimension_size.along(axis).0;
        let mut current_offset = self.scroll_handle.offset().along(axis).min(px(0.)).abs().0;
        if let Some(adjustment) = scroll_adjustment.and_then(|adjustment| {
            let adjust = adjustment.along(axis).0;
            if adjust < 0.0 {
                Some(adjust)
            } else {
                None
            }
        }) {
            current_offset -= adjustment;
        }

        let mut percentage = current_offset / main_dimension_size;
        let viewport_size = self.scroll_handle.viewport().size;
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
        let thumb = state.thumb_range(kind)?;
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
            let thumb_background = colors
                .surface_background
                .blend(colors.scrollbar_thumb_background);
            let is_vertical = self.kind == ScrollbarAxis::Vertical;
            let extra_padding = px(5.0);
            let padded_bounds = if is_vertical {
                Bounds::from_corners(
                    bounds.origin + point(Pixels::ZERO, extra_padding),
                    bounds.bottom_right() - point(Pixels::ZERO, extra_padding * 3),
                )
            } else {
                Bounds::from_corners(
                    bounds.origin + point(extra_padding, Pixels::ZERO),
                    bounds.bottom_right() - point(extra_padding * 3, Pixels::ZERO),
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
                    } else if let Some(ContentSize {
                        size: item_size, ..
                    }) = scroll.content_size()
                    {
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
                    if let Some(ContentSize {
                        size: item_size, ..
                    }) = scroll.content_size()
                    {
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

                        if let Some(id) = state.parent_id {
                            cx.notify(Some(id));
                        }
                    }
                } else {
                    state.drag.set(None);
                }
            });
            let state = self.state.clone();
            cx.on_mouse_event(move |_event: &MouseUpEvent, phase, cx| {
                if phase.bubble() {
                    state.drag.take();
                    if let Some(id) = state.parent_id {
                        cx.notify(Some(id));
                    }
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
