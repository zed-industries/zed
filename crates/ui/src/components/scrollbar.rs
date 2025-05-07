use std::{any::Any, cell::Cell, fmt::Debug, ops::Range, rc::Rc, sync::Arc};

use crate::{IntoElement, prelude::*, px, relative};
use gpui::{
    Along, App, Axis as ScrollbarAxis, BorderStyle, Bounds, ContentMask, Corners, Edges, Element,
    ElementId, Entity, EntityId, GlobalElementId, Hitbox, Hsla, LayoutId, ListState, MouseButton,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point, ScrollHandle, ScrollWheelEvent,
    Size, Stateful, Style, UniformListScrollHandle, Window, point, quad,
};

#[track_caller]
pub fn scrollbar(state: ScrollbarState, window: &mut Window) -> Stateful<Div> {
    let id = window.current_view();
    let location = core::panic::Location::caller();
    div()
        .id(*location)
        .occlude()
        .absolute()
        .right_1()
        .top_0()
        .bottom_0()
        .pb_6()
        .w_3()
        .h_full()
        .cursor_default()
        .on_mouse_move(move |_, _window, cx| {
            cx.notify(id);
            cx.stop_propagation()
        })
        .on_hover(|_, _window, cx| cx.stop_propagation())
        .on_any_mouse_down(|_, _window, cx| cx.stop_propagation())
        .on_scroll_wheel(move |_, _window, cx| cx.notify(id))
        .on_mouse_up(MouseButton::Left, |_, _, cx| cx.stop_propagation())
        .children(Scrollbar::vertical(state.clone()))
}

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
}

impl ScrollableHandle for ListState {
    fn content_size(&self) -> Option<ContentSize> {
        Some(ContentSize {
            size: self.content_size_for_scrollbar(),
            scroll_adjustment: None,
        })
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
}

#[derive(Debug)]
pub struct ContentSize {
    pub size: Size<Pixels>,
    pub scroll_adjustment: Option<Point<Pixels>>,
}

pub trait ScrollableHandle: Any + Debug {
    fn content_size(&self) -> Option<ContentSize>;
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
        const MINIMUM_THUMB_SIZE: f32 = 25.;
        let ContentSize {
            size: main_dimension_size,
            scroll_adjustment,
        } = self.scroll_handle.content_size()?;
        let content_size = main_dimension_size.along(axis).0;
        let mut current_offset = self.scroll_handle.offset().along(axis).min(px(0.)).abs().0;
        if let Some(adjustment) = scroll_adjustment.and_then(|adjustment| {
            let adjust = adjustment.along(axis).0;
            if adjust < 0.0 { Some(adjust) } else { None }
        }) {
            current_offset -= adjustment;
        }
        let viewport_size = self.scroll_handle.viewport().size.along(axis).0;
        if content_size < viewport_size {
            return None;
        }
        let visible_percentage = viewport_size / content_size;
        let thumb_size = MINIMUM_THUMB_SIZE.max(viewport_size * visible_percentage);
        if thumb_size > viewport_size {
            return None;
        }
        let max_offset = content_size - viewport_size;
        current_offset = current_offset.clamp(0., max_offset);
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

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
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
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        window.with_content_mask(Some(ContentMask { bounds }), |window| {
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
            window.paint_quad(quad(
                thumb_bounds,
                corners,
                thumb_background,
                Edges::default(),
                Hsla::transparent_black(),
                BorderStyle::default(),
            ));

            let scroll = self.state.scroll_handle.clone();
            let axis = self.kind;

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
                    } else if let Some(ContentSize {
                        size: item_size, ..
                    }) = scroll.content_size()
                    {
                        let click_offset = {
                            let viewport_size = padded_bounds.size.along(axis);

                            let thumb_size = thumb_bounds.size.along(axis);
                            let thumb_start = (event.position.along(axis)
                                - padded_bounds.origin.along(axis)
                                - (thumb_size / 2.))
                                .clamp(px(0.), viewport_size - thumb_size);

                            let max_offset = (item_size.along(axis) - viewport_size).max(px(0.));
                            let percentage = if viewport_size > thumb_size {
                                thumb_start / (viewport_size - thumb_size)
                            } else {
                                0.
                            };

                            -max_offset * percentage
                        };
                        match axis {
                            ScrollbarAxis::Horizontal => {
                                scroll.set_offset(point(click_offset, scroll.offset().y));
                            }
                            ScrollbarAxis::Vertical => {
                                scroll.set_offset(point(scroll.offset().x, click_offset));
                            }
                        }
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
            let axis = self.kind;
            window.on_mouse_event(move |event: &MouseMoveEvent, _, window, cx| {
                if let Some(drag_state) = state.drag.get().filter(|_| event.dragging()) {
                    if let Some(ContentSize {
                        size: item_size, ..
                    }) = scroll.content_size()
                    {
                        let drag_offset = {
                            let viewport_size = padded_bounds.size.along(axis);

                            let thumb_size = thumb_bounds.size.along(axis);
                            let thumb_start = (event.position.along(axis)
                                - padded_bounds.origin.along(axis)
                                - drag_state)
                                .clamp(px(0.), viewport_size - thumb_size);

                            let max_offset = (item_size.along(axis) - viewport_size).max(px(0.));
                            let percentage = if viewport_size > thumb_size {
                                thumb_start / (viewport_size - thumb_size)
                            } else {
                                0.
                            };

                            -max_offset * percentage
                        };
                        match axis {
                            ScrollbarAxis::Horizontal => {
                                scroll.set_offset(point(drag_offset, scroll.offset().y));
                            }
                            ScrollbarAxis::Vertical => {
                                scroll.set_offset(point(scroll.offset().x, drag_offset));
                            }
                        };
                        window.refresh();
                        if let Some(id) = state.parent_id {
                            cx.notify(id);
                        }
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
