use std::{cell::Cell, ops::Range, rc::Rc};

use gpui::{
    point, quad, Bounds, ContentMask, Corners, Edges, EntityId, Hitbox, Hsla, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, ScrollWheelEvent, Style, UniformListScrollHandle,
};
use ui::{prelude::*, px, relative, IntoElement};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ScrollbarKind {
    Horizontal,
    Vertical,
}

pub(crate) struct ProjectPanelScrollbar {
    thumb: Range<f32>,
    scroll: UniformListScrollHandle,
    // If Some(), there's an active drag, offset by percentage from the top of thumb.
    scrollbar_drag_state: Rc<Cell<Option<f32>>>,
    kind: ScrollbarKind,
    parent_id: EntityId,
}

impl ProjectPanelScrollbar {
    pub(crate) fn vertical(
        thumb: Range<f32>,
        scroll: UniformListScrollHandle,
        scrollbar_drag_state: Rc<Cell<Option<f32>>>,
        parent_id: EntityId,
    ) -> Self {
        Self {
            thumb,
            scroll,
            scrollbar_drag_state,
            kind: ScrollbarKind::Vertical,
            parent_id,
        }
    }

    pub(crate) fn horizontal(
        thumb: Range<f32>,
        scroll: UniformListScrollHandle,
        scrollbar_drag_state: Rc<Cell<Option<f32>>>,
        parent_id: EntityId,
    ) -> Self {
        Self {
            thumb,
            scroll,
            scrollbar_drag_state,
            kind: ScrollbarKind::Horizontal,
            parent_id,
        }
    }
}

impl gpui::Element for ProjectPanelScrollbar {
    type RequestLayoutState = ();

    type PrepaintState = Hitbox;

    fn id(&self) -> Option<ui::ElementId> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        cx: &mut ui::WindowContext,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.flex_grow = 1.;
        style.flex_shrink = 1.;
        if self.kind == ScrollbarKind::Vertical {
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
        _id: Option<&gpui::GlobalElementId>,
        bounds: Bounds<ui::Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        cx: &mut ui::WindowContext,
    ) -> Self::PrepaintState {
        cx.with_content_mask(Some(ContentMask { bounds }), |cx| {
            cx.insert_hitbox(bounds, false)
        })
    }

    fn paint(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        bounds: Bounds<ui::Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        cx: &mut ui::WindowContext,
    ) {
        cx.with_content_mask(Some(ContentMask { bounds }), |cx| {
            let colors = cx.theme().colors();
            let thumb_background = colors.scrollbar_thumb_background;
            let is_vertical = self.kind == ScrollbarKind::Vertical;
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

            let scroll = self.scroll.clone();
            let kind = self.kind;
            let thumb_percentage_size = self.thumb.end - self.thumb.start;

            cx.on_mouse_event({
                let scroll = self.scroll.clone();
                let is_dragging = self.scrollbar_drag_state.clone();
                move |event: &MouseDownEvent, phase, _cx| {
                    if phase.bubble() && bounds.contains(&event.position) {
                        if !thumb_bounds.contains(&event.position) {
                            let scroll = scroll.0.borrow();
                            if let Some(item_size) = scroll.last_item_size {
                                match kind {
                                    ScrollbarKind::Horizontal => {
                                        let percentage = (event.position.x - bounds.origin.x)
                                            / bounds.size.width;
                                        let max_offset = item_size.contents.width;
                                        let percentage = percentage.min(1. - thumb_percentage_size);
                                        scroll.base_handle.set_offset(point(
                                            -max_offset * percentage,
                                            scroll.base_handle.offset().y,
                                        ));
                                    }
                                    ScrollbarKind::Vertical => {
                                        let percentage = (event.position.y - bounds.origin.y)
                                            / bounds.size.height;
                                        let max_offset = item_size.contents.height;
                                        let percentage = percentage.min(1. - thumb_percentage_size);
                                        scroll.base_handle.set_offset(point(
                                            scroll.base_handle.offset().x,
                                            -max_offset * percentage,
                                        ));
                                    }
                                }
                            }
                        } else {
                            let thumb_offset = if is_vertical {
                                (event.position.y - thumb_bounds.origin.y) / bounds.size.height
                            } else {
                                (event.position.x - thumb_bounds.origin.x) / bounds.size.width
                            };
                            is_dragging.set(Some(thumb_offset));
                        }
                    }
                }
            });
            cx.on_mouse_event({
                let scroll = self.scroll.clone();
                move |event: &ScrollWheelEvent, phase, cx| {
                    if phase.bubble() && bounds.contains(&event.position) {
                        let scroll = scroll.0.borrow_mut();
                        let current_offset = scroll.base_handle.offset();

                        scroll
                            .base_handle
                            .set_offset(current_offset + event.delta.pixel_delta(cx.line_height()));
                    }
                }
            });
            let drag_state = self.scrollbar_drag_state.clone();
            let view_id = self.parent_id;
            let kind = self.kind;
            cx.on_mouse_event(move |event: &MouseMoveEvent, _, cx| {
                if let Some(drag_state) = drag_state.get().filter(|_| event.dragging()) {
                    let scroll = scroll.0.borrow();
                    if let Some(item_size) = scroll.last_item_size {
                        match kind {
                            ScrollbarKind::Horizontal => {
                                let max_offset = item_size.contents.width;
                                let percentage = (event.position.x - bounds.origin.x)
                                    / bounds.size.width
                                    - drag_state;

                                let percentage = percentage.min(1. - thumb_percentage_size);
                                scroll.base_handle.set_offset(point(
                                    -max_offset * percentage,
                                    scroll.base_handle.offset().y,
                                ));
                            }
                            ScrollbarKind::Vertical => {
                                let max_offset = item_size.contents.height;
                                let percentage = (event.position.y - bounds.origin.y)
                                    / bounds.size.height
                                    - drag_state;

                                let percentage = percentage.min(1. - thumb_percentage_size);
                                scroll.base_handle.set_offset(point(
                                    scroll.base_handle.offset().x,
                                    -max_offset * percentage,
                                ));
                            }
                        };

                        cx.notify(view_id);
                    }
                } else {
                    drag_state.set(None);
                }
            });
            let is_dragging = self.scrollbar_drag_state.clone();
            cx.on_mouse_event(move |_event: &MouseUpEvent, phase, cx| {
                if phase.bubble() {
                    is_dragging.set(None);
                    cx.notify(view_id);
                }
            });
        })
    }
}

impl IntoElement for ProjectPanelScrollbar {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
