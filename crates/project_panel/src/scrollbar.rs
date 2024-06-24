use std::{cell::Cell, ops::Range, rc::Rc};

use gpui::{
    point, AnyView, Bounds, ContentMask, Hitbox, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    ScrollWheelEvent, Style, UniformListScrollHandle,
};
use ui::{prelude::*, px, relative, IntoElement};

pub(crate) struct ProjectPanelScrollbar {
    thumb: Range<f32>,
    scroll: UniformListScrollHandle,
    is_dragging_scrollbar: Rc<Cell<bool>>,
    item_count: usize,
    view: AnyView,
}

impl ProjectPanelScrollbar {
    pub(crate) fn new(
        thumb: Range<f32>,
        scroll: UniformListScrollHandle,
        is_dragging_scrollbar: Rc<Cell<bool>>,
        view: AnyView,
        item_count: usize,
    ) -> Self {
        Self {
            thumb,
            scroll,
            is_dragging_scrollbar,
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
            let scrollbar_background = colors.scrollbar_track_border;
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
                let is_dragging = self.is_dragging_scrollbar.clone();
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
                            is_dragging.set(true);
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
            let is_dragging = self.is_dragging_scrollbar.clone();
            let view_id = self.view.entity_id();
            cx.on_mouse_event(move |event: &MouseMoveEvent, _, cx| {
                if event.dragging() && is_dragging.get() {
                    let scroll = scroll.0.borrow();
                    if let Some(last_height) = scroll.last_item_height {
                        let max_offset = item_count as f32 * last_height;
                        let percentage = (event.position.y - bounds.origin.y) / bounds.size.height;

                        let percentage = percentage.min(1. - thumb_percentage_size);
                        scroll
                            .base_handle
                            .set_offset(point(px(0.), -max_offset * percentage));
                        cx.notify(view_id);
                    }
                } else {
                    is_dragging.set(false);
                }
            });
            let is_dragging = self.is_dragging_scrollbar.clone();
            cx.on_mouse_event(move |_event: &MouseUpEvent, phase, cx| {
                if phase.bubble() {
                    is_dragging.set(false);
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
