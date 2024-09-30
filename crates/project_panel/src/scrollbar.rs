use std::{cell::Cell, ops::Range, rc::Rc};

use gpui::{
    point, AnyView, Bounds, ContentMask, Hitbox, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    ScrollWheelEvent, Style, UniformListScrollHandle,
};
use ui::{prelude::*, px, relative, IntoElement};

pub(crate) struct ProjectPanelScrollbar {
    thumb: Range<f32>,
    scroll: UniformListScrollHandle,
    // If Some(), there's an active drag, offset by percentage from the top of thumb.
    scrollbar_drag_state: Rc<Cell<Option<f32>>>,
    item_count: usize,
    view: AnyView,
}

impl ProjectPanelScrollbar {
    pub(crate) fn new(
        thumb: Range<f32>,
        scroll: UniformListScrollHandle,
        scrollbar_drag_state: Rc<Cell<Option<f32>>>,
        view: AnyView,
        item_count: usize,
    ) -> Self {
        Self {
            thumb,
            scroll,
            scrollbar_drag_state,
            item_count,
            view,
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
        style.size.width = px(12.).into();
        style.size.height = relative(1.).into();
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

            let thumb_offset = self.thumb.start * bounds.size.height;
            let thumb_end = self.thumb.end * bounds.size.height;

            let thumb_percentage_size = self.thumb.end - self.thumb.start;
            let thumb_bounds = {
                let thumb_upper_left = point(bounds.origin.x, bounds.origin.y + thumb_offset);
                let thumb_lower_right = point(
                    bounds.origin.x + bounds.size.width,
                    bounds.origin.y + thumb_end,
                );
                Bounds::from_corners(thumb_upper_left, thumb_lower_right)
            };
            cx.paint_quad(gpui::fill(thumb_bounds, thumb_background));
            let scroll = self.scroll.clone();
            let item_count = self.item_count;
            cx.on_mouse_event({
                let scroll = self.scroll.clone();
                let is_dragging = self.scrollbar_drag_state.clone();
                move |event: &MouseDownEvent, phase, _cx| {
                    if phase.bubble() && bounds.contains(&event.position) {
                        if !thumb_bounds.contains(&event.position) {
                            let scroll = scroll.0.borrow();
                            if let Some(last_height) = scroll.last_item_height {
                                let max_offset = item_count as f32 * last_height;
                                let percentage =
                                    (event.position.y - bounds.origin.y) / bounds.size.height;

                                let percentage = percentage.min(1. - thumb_percentage_size);
                                scroll
                                    .base_handle
                                    .set_offset(point(px(0.), -max_offset * percentage));
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
                        let scroll = scroll.0.borrow_mut();
                        let current_offset = scroll.base_handle.offset();
                        scroll
                            .base_handle
                            .set_offset(current_offset + event.delta.pixel_delta(cx.line_height()));
                    }
                }
            });
            let drag_state = self.scrollbar_drag_state.clone();
            let view_id = self.view.entity_id();
            cx.on_mouse_event(move |event: &MouseMoveEvent, _, cx| {
                if let Some(drag_state) = drag_state.get().filter(|_| event.dragging()) {
                    let scroll = scroll.0.borrow();
                    if let Some(last_height) = scroll.last_item_height {
                        let max_offset = item_count as f32 * last_height;
                        let percentage =
                            (event.position.y - bounds.origin.y) / bounds.size.height - drag_state;

                        let percentage = percentage.min(1. - thumb_percentage_size);
                        scroll
                            .base_handle
                            .set_offset(point(px(0.), -max_offset * percentage));
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
