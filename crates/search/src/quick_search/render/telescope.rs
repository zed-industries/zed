use super::QuickSearch;

use ui::{
    ActiveTheme, FluentBuilder, InteractiveElement, IntoElement, ParentElement, Pixels,
    StatefulInteractiveElement, Styled, div, h_flex, px, v_flex,
};

use crate::quick_search::render::highlighted_drag_preview;
use crate::quick_search::{
    ResizeSide, clear_resize_highlight, handle_resize_mouse_down, resize_hover_handler,
    state::TelescopeLayoutState,
};

use gpui::DragMoveEvent;
use gpui::{Context, MouseButton, Window};

use super::window_controls;

#[derive(Clone, Copy)]
struct TelescopePreviewResizeDrag {
    mouse_start_x: Pixels,
    preview_width_start: Pixels,
}

#[derive(Clone, Copy)]
struct TelescopeHeightResizeDrag {
    side: ResizeSide,
    mouse_start_y: Pixels,
    content_height_start: Pixels,
    offset_start: Pixels,
}

impl QuickSearch {
    pub(crate) fn render_telescope_content(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        v_flex()
            .relative()
            .child(
                h_flex()
                    .h(self.telescope.content_height)
                    .child(
                        div()
                            .flex_1()
                            .h(self.telescope.content_height)
                            .overflow_hidden()
                            .child(self.picker.clone()),
                    )
                    .child(self.render_telescope_preview_resize(window, cx))
                    .child(
                        div()
                            .w(self.telescope.preview_width)
                            .h(self.telescope.content_height)
                            .overflow_hidden()
                            .child(self.render_telescope_preview(window, cx)),
                    ),
            )
            .child(self.render_telescope_height_resize(ResizeSide::End, window, cx))
    }

    pub(crate) fn render_telescope_preview(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        v_flex()
            .size_full()
            .border_l_1()
            .border_color(cx.theme().colors().border)
            .child(self.render_preview_header(window, cx))
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .child(self.preview_editor.clone()),
            )
    }

    pub(crate) fn render_telescope_preview_resize(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_highlighted = window.use_state(cx, |_window, _cx| false);
        let divider_size = px(window_controls::RESIZE_DIVIDER_SIZE);
        let handle_width = px(window_controls::RESIZE_HANDLE_WIDTH);
        let handle_offset = (handle_width - divider_size) / 2.0;

        div()
            .id("telescope-preview-resize-divider")
            .relative()
            .w(divider_size)
            .h_full()
            .bg(cx.theme().colors().border)
            .when(*is_highlighted.read(cx), |this| {
                this.bg(cx.theme().colors().border_focused)
            })
            .child(
                div()
                    .id("telescope-preview-resize-handle")
                    .absolute()
                    .left(-handle_offset)
                    .top_0()
                    .bottom_0()
                    .w(handle_width)
                    .cursor_col_resize()
                    .block_mouse_except_scroll()
                    .on_hover(resize_hover_handler(is_highlighted.clone()))
                    .on_mouse_down(MouseButton::Left, handle_resize_mouse_down)
                    .on_drag(
                        TelescopePreviewResizeDrag {
                            mouse_start_x: window.mouse_position().x,
                            preview_width_start: self.telescope.preview_width,
                        },
                        highlighted_drag_preview(is_highlighted.clone()),
                    )
                    .on_drop::<TelescopePreviewResizeDrag>(clear_resize_highlight(
                        is_highlighted.clone(),
                    )),
            )
            .on_drag_move::<TelescopePreviewResizeDrag>(cx.listener(
                |this, event: &DragMoveEvent<TelescopePreviewResizeDrag>, _window, cx| {
                    let drag = event.drag(cx);
                    let delta = drag.mouse_start_x - event.event.position.x;
                    let new_width = (drag.preview_width_start + delta)
                        .max(px(TelescopeLayoutState::MIN_PREVIEW_WIDTH))
                        .min(px(TelescopeLayoutState::MAX_PREVIEW_WIDTH));
                    this.telescope.preview_width = new_width;
                    cx.notify();
                },
            ))
    }

    pub(crate) fn render_telescope_height_resize(
        &self,
        side: ResizeSide,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_highlighted = window.use_state(cx, |_window, _cx| false);
        let divider_size = px(window_controls::RESIZE_DIVIDER_SIZE);
        let handle_height = px(window_controls::RESIZE_HANDLE_HEIGHT);
        let handle_offset = (handle_height - divider_size) / 2.0;
        let corner_clearance = px(window_controls::RESIZE_CORNER_CLEARANCE);

        div()
            .id(match side {
                ResizeSide::Start => "telescope-top-resize-divider",
                ResizeSide::End => "telescope-bottom-resize-divider",
            })
            .relative()
            .h(divider_size)
            .w_full()
            .when(side == ResizeSide::End, |this| {
                this.bg(cx.theme().colors().border)
            })
            .when(
                side == ResizeSide::End && *is_highlighted.read(cx),
                |this| this.bg(cx.theme().colors().border_focused),
            )
            .map(|this| match side {
                ResizeSide::Start => this
                    .absolute()
                    .top(-(divider_size / 2.0))
                    .left(corner_clearance)
                    .right(corner_clearance),
                ResizeSide::End => this.ml(corner_clearance).mr(corner_clearance),
            })
            .child(
                div()
                    .id(match side {
                        ResizeSide::Start => "telescope-top-resize-handle",
                        ResizeSide::End => "telescope-bottom-resize-handle",
                    })
                    .absolute()
                    .top(-handle_offset)
                    .left_0()
                    .right_0()
                    .h(handle_height)
                    .cursor_row_resize()
                    .block_mouse_except_scroll()
                    .on_hover(resize_hover_handler(is_highlighted.clone()))
                    .on_mouse_down(MouseButton::Left, handle_resize_mouse_down)
                    .on_drag(
                        TelescopeHeightResizeDrag {
                            side,
                            mouse_start_y: window.mouse_position().y,
                            content_height_start: self.telescope.content_height,
                            offset_start: self.offset.y,
                        },
                        highlighted_drag_preview(is_highlighted.clone()),
                    )
                    .on_drop::<TelescopeHeightResizeDrag>(clear_resize_highlight(is_highlighted)),
            )
            .on_drag_move::<TelescopeHeightResizeDrag>(cx.listener(
                move |this, event: &DragMoveEvent<TelescopeHeightResizeDrag>, _window, cx| {
                    let drag = event.drag(cx);
                    let delta = event.event.position.y - drag.mouse_start_y;
                    let height_delta = match drag.side {
                        ResizeSide::Start => -delta,
                        ResizeSide::End => delta,
                    };
                    let new_height = (drag.content_height_start + height_delta)
                        .max(px(TelescopeLayoutState::MIN_CONTENT_HEIGHT))
                        .min(px(TelescopeLayoutState::MAX_CONTENT_HEIGHT));

                    this.telescope.content_height = new_height;

                    if drag.side == ResizeSide::Start {
                        let actual_growth = new_height - drag.content_height_start;
                        this.offset.y = drag.offset_start - actual_growth;
                    }
                    cx.notify();
                },
            ))
    }
}
