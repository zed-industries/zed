use std::ops::Range;

use gpui::{
    point, Bounds, ContentMask, Hitbox, MouseDownEvent, MouseMoveEvent, ScrollWheelEvent, Style,
    UniformListScrollHandle,
};
use ui::{prelude::*, px, relative, IntoElement};

pub(crate) struct ProjectPanelScrollbar {
    thumb: Range<f32>,
    scroll: UniformListScrollHandle,
    item_count: usize,
}

impl ProjectPanelScrollbar {
    pub(crate) fn new(
        thumb: Range<f32>,
        scroll: UniformListScrollHandle,
        item_count: usize,
    ) -> Self {
        Self {
            thumb,
            scroll,
            item_count,
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
        let hitbox_id = _prepaint.id;
        cx.with_content_mask(Some(ContentMask { bounds }), |cx| {
            let colors = cx.theme().colors();
            let scrollbar_background = colors.scrollbar_track_border;
            let thumb_background = colors.scrollbar_thumb_background;
            cx.paint_quad(gpui::fill(bounds, scrollbar_background));

            let thumb_offset = self.thumb.start * bounds.size.height;
            let thumb_end = self.thumb.end * bounds.size.height;
            let thumb_upper_left = point(bounds.origin.x, bounds.origin.y + thumb_offset);
            let thumb_lower_right = point(
                bounds.origin.x + bounds.size.width,
                bounds.origin.y + thumb_end,
            );
            let thumb_percentage_size = self.thumb.end - self.thumb.start;
            cx.paint_quad(gpui::fill(
                Bounds::from_corners(thumb_upper_left, thumb_lower_right),
                thumb_background,
            ));
            let scroll = self.scroll.clone();
            let item_count = self.item_count;
            cx.on_mouse_event({
                let scroll = self.scroll.clone();
                move |event: &MouseDownEvent, phase, _cx| {
                    if phase.bubble() && bounds.contains(&event.position) {
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

            cx.on_mouse_event(move |event: &MouseMoveEvent, phase, cx| {
                if phase.bubble() && bounds.contains(&event.position) && hitbox_id.is_hovered(cx) {
                    if event.dragging() {
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
                        cx.stop_propagation();
                    }
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
