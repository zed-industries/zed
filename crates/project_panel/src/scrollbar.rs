use std::{cell::Cell, ops::Range, rc::Rc};

use gpui::{
    point, Bounds, ContentMask, EntityId, Hitbox, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    ScrollWheelEvent, Size, Style, UniformListScrollHandle,
};
use ui::{prelude::*, px, relative, IntoElement};

#[derive(Debug, Clone, Copy)]
pub(crate) enum ScrollbarKind {
    Horizontal { viewport_width: Pixels },
    Vertical { item_count: usize },
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
        item_count: usize,
    ) -> Self {
        Self {
            thumb,
            scroll,
            scrollbar_drag_state,
            kind: ScrollbarKind::Vertical { item_count },
            parent_id,
        }
    }

    pub(crate) fn horizontal(
        thumb: Range<f32>,
        scroll: UniformListScrollHandle,
        scrollbar_drag_state: Rc<Cell<Option<f32>>>,
        parent_id: EntityId,
        viewport_width: Pixels,
    ) -> Self {
        Self {
            thumb,
            scroll,
            scrollbar_drag_state,
            kind: ScrollbarKind::Horizontal { viewport_width },
            parent_id,
        }
    }

    fn is_vertical(&self) -> bool {
        matches!(self.kind, ScrollbarKind::Vertical { .. })
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
        if self.is_vertical() {
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
            let scrollbar_background = colors.scrollbar_track_background;
            let thumb_background = colors.scrollbar_thumb_background;
            cx.paint_quad(gpui::fill(bounds, scrollbar_background));

            let thumb_bounds = if self.is_vertical() {
                let thumb_offset = self.thumb.start * bounds.size.height;
                let thumb_end = self.thumb.end * bounds.size.height;
                let thumb_upper_left = point(bounds.origin.x, bounds.origin.y + thumb_offset);
                let thumb_lower_right = point(
                    bounds.origin.x + bounds.size.width,
                    bounds.origin.y + thumb_end,
                );
                Bounds::from_corners(thumb_upper_left, thumb_lower_right)
            } else {
                let thumb_offset = self.thumb.start * bounds.size.width;
                let thumb_end = self.thumb.end * bounds.size.width;
                let thumb_upper_left = point(bounds.origin.x + thumb_offset, bounds.origin.y);
                let thumb_lower_right = point(
                    bounds.origin.x + thumb_end,
                    bounds.origin.y + bounds.size.height,
                );
                Bounds::from_corners(thumb_upper_left, thumb_lower_right)
            };
            let thumb_percentage_size = self.thumb.end - self.thumb.start;

            cx.paint_quad(gpui::fill(thumb_bounds, thumb_background));
            let scroll = self.scroll.clone();
            let kind = self.kind;

            cx.on_mouse_event({
                let scroll = self.scroll.clone();
                let is_dragging = self.scrollbar_drag_state.clone();
                move |event: &MouseDownEvent, phase, _cx| {
                    if phase.bubble() && bounds.contains(&event.position) {
                        dbg!("mouse down event");
                        if !thumb_bounds.contains(&event.position) {
                            let scroll = scroll.0.borrow();
                            if let Some(Size {
                                height: last_height,
                                ..
                            }) = scroll.last_item_size
                            {
                                let max_offset = match kind {
                                    ScrollbarKind::Horizontal { viewport_width } => {
                                        viewport_width.0
                                    }
                                    ScrollbarKind::Vertical { item_count } => {
                                        item_count as f32 * last_height.0
                                    }
                                };

                                let percentage =
                                    (event.position.y - bounds.origin.y) / bounds.size.height;

                                let percentage = percentage.min(1. - thumb_percentage_size);

                                dbg!("??");
                                scroll
                                    .base_handle
                                    .set_offset(point(px(0.), px(-max_offset * percentage)));
                            }
                        } else {
                            let thumb_top_offset =
                                (event.position.y - thumb_bounds.origin.y) / bounds.size.height;
                            is_dragging.set(Some(thumb_top_offset));
                        }
                    }
                }
            });
            cx.on_mouse_event({
                let scroll = self.scroll.clone();
                move |event: &ScrollWheelEvent, phase, cx| {
                    if phase.bubble() && bounds.contains(&event.position) {
                        dbg!("ScrollWheelEvent");
                        let scroll = scroll.0.borrow_mut();
                        let current_offset = scroll.base_handle.offset();

                        scroll.base_handle.set_offset(
                            dbg!(current_offset) + dbg!(event.delta.pixel_delta(cx.line_height())),
                        );
                    }
                }
            });
            let drag_state = self.scrollbar_drag_state.clone();
            let view_id = self.parent_id;
            let kind = self.kind;
            cx.on_mouse_event(move |event: &MouseMoveEvent, _, cx| {
                if let Some(drag_state) = drag_state.get().filter(|_| event.dragging()) {
                    dbg!("MouseMoveEvent");
                    let scroll = scroll.0.borrow();
                    if let Some(Size {
                        height: last_height,
                        width: last_width,
                    }) = scroll.last_item_size
                    {
                        match kind {
                            ScrollbarKind::Horizontal { .. } => {
                                let max_offset = last_width;
                                let percentage = (event.position.x - bounds.origin.x)
                                    / bounds.size.width
                                    - drag_state;

                                let percentage = percentage.min(1. - thumb_percentage_size);
                                scroll
                                    .base_handle
                                    .set_offset(point(-max_offset * percentage, px(0.)));
                            }
                            ScrollbarKind::Vertical { item_count } => {
                                let max_offset = item_count as f32 * last_height;
                                let percentage = (event.position.y - bounds.origin.y)
                                    / bounds.size.height
                                    - drag_state;

                                let percentage = percentage.min(1. - thumb_percentage_size);
                                scroll
                                    .base_handle
                                    .set_offset(point(px(0.), -max_offset * percentage));
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
                    dbg!("MouseUpEvent");
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
