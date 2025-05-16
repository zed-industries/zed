use std::{any::Any, cell::Cell, fmt::Debug, ops::Range, rc::Rc, sync::Arc};

use crate::{IntoElement, prelude::*, px, relative};
use gpui::{
    Along, App, Axis as ScrollbarAxis, BorderStyle, Bounds, ContentMask, Corners, Edges, Element,
    ElementId, Entity, EntityId, GlobalElementId, Hitbox, Hsla, IsZero, LayoutId, ListState,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point, ScrollHandle, ScrollWheelEvent,
    Size, Style, UniformListScrollHandle, Window, quad,
};

pub struct Scrollbar {
    thumb: Range<f32>,
    state: ScrollbarState,
    kind: ScrollbarAxis,
}

impl ScrollableHandle for UniformListScrollHandle {
    fn content_size(&self) -> Size<Pixels> {
        self.0.borrow().base_handle.content_size()
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
}

impl ScrollableHandle for ListState {
    fn content_size(&self) -> Size<Pixels> {
        self.content_size_for_scrollbar()
    }

    fn set_offset(&self, point: Point<Pixels>) {
        self.set_offset_from_scrollbar(point);
    }

    fn offset(&self) -> Point<Pixels> {
        self.scroll_px_offset_for_scrollbar()
    }

    fn drag_started(&self) {
        self.scrollbar_drag_started();
    }

    fn drag_ended(&self) {
        self.scrollbar_drag_ended();
    }

    fn viewport(&self) -> Bounds<Pixels> {
        self.viewport_bounds()
    }
}

impl ScrollableHandle for ScrollHandle {
    fn content_size(&self) -> Size<Pixels> {
        self.padded_content_size()
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
}

pub trait ScrollableHandle: Any + Debug {
    fn content_size(&self) -> Size<Pixels>;
    fn set_offset(&self, point: Point<Pixels>);
    fn offset(&self) -> Point<Pixels>;
    fn viewport(&self) -> Bounds<Pixels>;
    fn drag_started(&self) {}
    fn drag_ended(&self) {}
}

/// A scrollbar state that should be persisted across frames.
#[derive(Clone, Debug)]
pub struct ScrollbarState {
    // If Some(), there's an active drag, offset by percentage from the origin of a thumb.
    drag: Rc<Cell<Option<Pixels>>>,
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

    /// Set a parent model which should be notified whenever this Scrollbar gets a scroll event.
    pub fn parent_entity<V: 'static>(mut self, v: &Entity<V>) -> Self {
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
        const MINIMUM_THUMB_SIZE: Pixels = px(25.);
        let content_size = self.scroll_handle.content_size().along(axis);
        let viewport_size = self.scroll_handle.viewport().size.along(axis);
        if content_size.is_zero() || viewport_size.is_zero() || content_size < viewport_size {
            return None;
        }

        let max_offset = content_size - viewport_size;
        let current_offset = self
            .scroll_handle
            .offset()
            .along(axis)
            .clamp(-max_offset, Pixels::ZERO)
            .abs();

        let visible_percentage = viewport_size / content_size;
        let thumb_size = MINIMUM_THUMB_SIZE.max(viewport_size * visible_percentage);
        if thumb_size > viewport_size {
            return None;
        }
        let start_offset = (current_offset / max_offset) * (viewport_size - thumb_size);
        let thumb_percentage_start = start_offset / viewport_size;
        let thumb_percentage_end = (start_offset + thumb_size) / viewport_size;
        Some(thumb_percentage_start..thumb_percentage_end)
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

    fn source(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _debug_id: Option<&gpui::DebugElementId>,
        window: &mut Window,
        cx: &mut App,
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

        (window.request_layout(style, None, cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _debug_id: Option<&gpui::DebugElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        _: &mut App,
    ) -> Self::PrepaintState {
        window.with_content_mask(Some(ContentMask { bounds }), |window| {
            window.insert_hitbox(bounds, false)
        })
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _debug_id: Option<&gpui::DebugElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        const EXTRA_PADDING: Pixels = px(5.0);
        window.with_content_mask(Some(ContentMask { bounds }), |window| {
            let axis = self.kind;
            let colors = cx.theme().colors();
            let thumb_background = colors
                .surface_background
                .blend(colors.scrollbar_thumb_background);

            let padded_bounds = Bounds::from_corners(
                bounds
                    .origin
                    .apply_along(axis, |origin| origin + EXTRA_PADDING),
                bounds
                    .bottom_right()
                    .apply_along(axis, |track_end| track_end - 3.0 * EXTRA_PADDING),
            );

            let thumb_offset = self.thumb.start * padded_bounds.size.along(axis);
            let thumb_end = self.thumb.end * padded_bounds.size.along(axis);

            let thumb_bounds = Bounds::new(
                padded_bounds
                    .origin
                    .apply_along(axis, |origin| origin + thumb_offset),
                padded_bounds
                    .size
                    .apply_along(axis, |_| thumb_end - thumb_offset)
                    .apply_along(axis.invert(), |width| width / 1.5),
            );

            let corners = Corners::all(thumb_bounds.size.along(axis.invert()) / 2.0);

            window.paint_quad(quad(
                thumb_bounds,
                corners,
                thumb_background,
                Edges::default(),
                Hsla::transparent_black(),
                BorderStyle::default(),
            ));

            let scroll = self.state.scroll_handle.clone();

            enum ScrollbarMouseEvent {
                GutterClick,
                ThumbDrag(Pixels),
            }

            let compute_click_offset =
                move |event_position: Point<Pixels>,
                      item_size: Size<Pixels>,
                      event_type: ScrollbarMouseEvent| {
                    let viewport_size = padded_bounds.size.along(axis);

                    let thumb_size = thumb_bounds.size.along(axis);

                    let thumb_offset = match event_type {
                        ScrollbarMouseEvent::GutterClick => thumb_size / 2.,
                        ScrollbarMouseEvent::ThumbDrag(thumb_offset) => thumb_offset,
                    };

                    let thumb_start = (event_position.along(axis)
                        - padded_bounds.origin.along(axis)
                        - thumb_offset)
                        .clamp(px(0.), viewport_size - thumb_size);

                    let max_offset = (item_size.along(axis) - viewport_size).max(px(0.));
                    let percentage = if viewport_size > thumb_size {
                        thumb_start / (viewport_size - thumb_size)
                    } else {
                        0.
                    };

                    -max_offset * percentage
                };

            window.on_mouse_event({
                let scroll = scroll.clone();
                let state = self.state.clone();
                move |event: &MouseDownEvent, phase, _, _| {
                    if !(phase.bubble() && bounds.contains(&event.position)) {
                        return;
                    }

                    scroll.drag_started();

                    if thumb_bounds.contains(&event.position) {
                        let offset = event.position.along(axis) - thumb_bounds.origin.along(axis);
                        state.drag.set(Some(offset));
                    } else {
                        let click_offset = compute_click_offset(
                            event.position,
                            scroll.content_size(),
                            ScrollbarMouseEvent::GutterClick,
                        );
                        scroll.set_offset(scroll.offset().apply_along(axis, |_| click_offset));
                    }
                }
            });

            window.on_mouse_event({
                let scroll = scroll.clone();
                move |event: &ScrollWheelEvent, phase, window, _| {
                    if phase.bubble() && bounds.contains(&event.position) {
                        let current_offset = scroll.offset();
                        scroll.set_offset(
                            current_offset + event.delta.pixel_delta(window.line_height()),
                        );
                    }
                }
            });

            let state = self.state.clone();
            window.on_mouse_event(move |event: &MouseMoveEvent, _, window, cx| {
                if let Some(drag_state) = state.drag.get().filter(|_| event.dragging()) {
                    let drag_offset = compute_click_offset(
                        event.position,
                        scroll.content_size(),
                        ScrollbarMouseEvent::ThumbDrag(drag_state),
                    );
                    scroll.set_offset(scroll.offset().apply_along(axis, |_| drag_offset));
                    window.refresh();
                    if let Some(id) = state.parent_id {
                        cx.notify(id);
                    }
                } else {
                    state.drag.set(None);
                }
            });
            let state = self.state.clone();
            let scroll = self.state.scroll_handle.clone();
            window.on_mouse_event(move |_event: &MouseUpEvent, phase, _, cx| {
                if phase.bubble() {
                    state.drag.take();
                    scroll.drag_ended();
                    if let Some(id) = state.parent_id {
                        cx.notify(id);
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
