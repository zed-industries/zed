use gpui::{Entity, MouseDownEvent};
use ui::{
    ActiveTheme, App, Color, Div, IntoElement, Label, LabelCommon, LabelSize, ParentElement,
    Styled, Window, div, h_flex, px, v_flex,
};

// use gpui::{DragMoveEvent, Entity, MouseButton, MouseDownEvent};
// use ui::{
//     ActiveTheme, App, Color, Context, Div, FluentBuilder, InteractiveElement, IntoElement, Label,
//     LabelCommon, LabelSize, ParentElement, StatefulInteractiveElement, Styled, Window, div, h_flex,
//     px, v_flex,
// };

use crate::{
    preview::{
        EditorPreview,
        state::{StackedLayout, TelescopeLayout},
    },
    // render::window_controls::{
    //     ResizeSide, TelescopeHeightResizeDrag, TelescopePreviewResizeDrag, clear_resize_highlight,
    //     highlighted_drag_preview,
    // },
};

use super::state::LayoutMode;
use crate::render::window_controls;

mod on_drag;

impl EditorPreview {
    pub(crate) fn render(
        &self,
        layout: LayoutMode,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        match layout {
            LayoutMode::Stacked(stacked) => self
                .render_stacked_preview(stacked, window, cx)
                .into_any_element(),
            LayoutMode::Telescope(telescope) => self
                .render_telescope_preview(telescope, window, cx)
                .into_any_element(),
        }
    }
}

impl EditorPreview {
    pub(crate) fn render_telescope_preview(
        &self,
        _layout: TelescopeLayout,
        window: &mut Window,
        cx: &mut App,
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

    fn render_stacked_preview(
        &self,
        layout: StackedLayout,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        v_flex()
            .child(self.render_preview_header(window, cx))
            .child(
                div()
                    .h(layout.preview_height)
                    .overflow_hidden()
                    .child(self.preview_editor.clone()),
            )
    }

    fn render_preview_header(&self, window: &mut Window, cx: &mut App) -> Div {
        if let Some(path) = &self.current_path {
            let file_name = path
                .file_name()
                .map(|name| name.to_string())
                .unwrap_or_default();
            let directory = path
                .parent()
                .map(|path| path.as_std_path().to_string_lossy().to_string())
                .unwrap_or_default();

            let split_menu_handle = self.split_popover_menu_handle.clone();
            let focus_handle = self.focus_handle.clone();

            h_flex()
                .px_2()
                .py_1()
                .gap_2()
                .border_b_1()
                .border_color(cx.theme().colors().border)
                .bg(cx.theme().colors().editor_background)
                .justify_between()
                .child(
                    h_flex()
                        .gap_2()
                        .min_w(px(0.))
                        .child(Label::new(file_name).size(LabelSize::Small).truncate())
                        .child(
                            Label::new(directory)
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                                .truncate(),
                        ),
                )
                .child(window_controls::render_split_menu(
                    split_menu_handle,
                    focus_handle,
                    window,
                    cx,
                ))
        } else {
            h_flex().h(px(26.0))
        }
    }

    // pub(crate) fn render_telescope_preview_resize(
    //     &self,
    //     window: &mut Window,
    //     cx: &mut Context<Self>,
    // ) -> impl IntoElement {
    //     // TODO!(yara) ought to deduplicate this, actually we ought to do that
    //     // for the whole resize thing!! For now lets just get this rendering again.
    //     const RESIZE_HANDLE_WIDTH: f32 = 8.0;
    //     let is_highlighted = window.use_state(cx, |_window, _cx| false);
    //     let divider_size = px(window_controls::RESIZE_DIVIDER_SIZE);
    //     let handle_width = px(window_controls::RESIZE_HANDLE_WIDTH);
    //     let handle_offset = (handle_width - divider_size) / 2.0;

    //     div()
    //         .id("telescope-preview-resize-divider")
    //         .relative()
    //         .w(divider_size)
    //         .h_full()
    //         .bg(cx.theme().colors().border)
    //         .when(*is_highlighted.read(cx), |this| {
    //             this.bg(cx.theme().colors().border_focused)
    //         })
    //         .child(
    //             div()
    //                 .id("telescope-preview-resize-handle")
    //                 .absolute()
    //                 .left(-handle_offset)
    //                 .top_0()
    //                 .bottom_0()
    //                 .w(handle_width)
    //                 .cursor_col_resize()
    //                 .block_mouse_except_scroll()
    //                 .on_hover(set_highlighted_to(is_highlighted.clone()))
    //                 .on_mouse_down(MouseButton::Left, do_nothing)
    //                 .on_drag(
    //                     TelescopePreviewResizeDrag {
    //                         mouse_start_x: window.mouse_position().x,
    //                         preview_width_start: self.telescope.preview_width,
    //                     },
    //                     highlighted_drag_preview(is_highlighted.clone()),
    //                 )
    //                 .on_drop::<TelescopePreviewResizeDrag>(clear_resize_highlight(
    //                     is_highlighted.clone(),
    //                 )),
    //         )
    //         .on_drag_move::<TelescopePreviewResizeDrag>(cx.listener(
    //             |this, event: &DragMoveEvent<TelescopePreviewResizeDrag>, _window, cx| {
    //                 let drag = event.drag(cx);
    //                 let delta = drag.mouse_start_x - event.event.position.x;
    //                 let new_width = (drag.preview_width_start + delta)
    //                     .max(px(TelescopeLayout::MIN_PREVIEW_WIDTH))
    //                     .min(px(TelescopeLayout::MAX_PREVIEW_WIDTH));
    //                 this.telescope.preview_width = new_width;
    //                 cx.notify();
    //             },
    //         ))
    // }

    // pub(crate) fn render_telescope_height_resize(
    //     &self,
    //     side: ResizeSide,
    //     window: &mut Window,
    //     cx: &mut Context<Self>,
    // ) -> impl IntoElement {
    //     let is_highlighted = window.use_state(cx, |_window, _cx| false);
    //     let divider_size = px(window_controls::RESIZE_DIVIDER_SIZE);
    //     let handle_height = px(window_controls::RESIZE_HANDLE_HEIGHT);
    //     let handle_offset = (handle_height - divider_size) / 2.0;
    //     let corner_clearance = px(window_controls::RESIZE_CORNER_CLEARANCE);

    //     div()
    //         .id(match side {
    //             ResizeSide::Start => "telescope-top-resize-divider",
    //             ResizeSide::End => "telescope-bottom-resize-divider",
    //         })
    //         .relative()
    //         .h(divider_size)
    //         .w_full()
    //         .when(side == ResizeSide::End, |this| {
    //             this.bg(cx.theme().colors().border)
    //         })
    //         .when(
    //             side == ResizeSide::End && *is_highlighted.read(cx),
    //             |this| this.bg(cx.theme().colors().border_focused),
    //         )
    //         .map(|this| match side {
    //             ResizeSide::Start => this
    //                 .absolute()
    //                 .top(-(divider_size / 2.0))
    //                 .left(corner_clearance)
    //                 .right(corner_clearance),
    //             ResizeSide::End => this.ml(corner_clearance).mr(corner_clearance),
    //         })
    //         .child(
    //             div()
    //                 .id(match side {
    //                     ResizeSide::Start => "telescope-top-resize-handle",
    //                     ResizeSide::End => "telescope-bottom-resize-handle",
    //                 })
    //                 .absolute()
    //                 .top(-handle_offset)
    //                 .left_0()
    //                 .right_0()
    //                 .h(handle_height)
    //                 .cursor_row_resize()
    //                 .block_mouse_except_scroll()
    //                 .on_hover(set_highlighted_to(is_highlighted.clone()))
    //                 .on_mouse_down(MouseButton::Left, do_nothing)
    //                 .on_drag(
    //                     TelescopeHeightResizeDrag {
    //                         side,
    //                         mouse_start_y: window.mouse_position().y,
    //                         content_height_start: self.telescope.content_height,
    //                         offset_start: self.offset.y,
    //                     },
    //                     highlighted_drag_preview(is_highlighted.clone()),
    //                 )
    //                 .on_drop::<TelescopeHeightResizeDrag>(clear_resize_highlight(is_highlighted)),
    //         )
    //         .on_drag_move::<TelescopeHeightResizeDrag>(cx.listener(
    //             move |this, event: &DragMoveEvent<TelescopeHeightResizeDrag>, _window, cx| {
    //                 let drag = event.drag(cx);
    //                 let delta = event.event.position.y - drag.mouse_start_y;
    //                 let height_delta = match drag.side {
    //                     ResizeSide::Start => -delta,
    //                     ResizeSide::End => delta,
    //                 };
    //                 let new_height = (drag.content_height_start + height_delta)
    //                     .max(px(TelescopeLayout::MIN_CONTENT_HEIGHT))
    //                     .min(px(TelescopeLayout::MAX_CONTENT_HEIGHT));

    //                 this.telescope.content_height = new_height;

    //                 if drag.side == ResizeSide::Start {
    //                     let actual_growth = new_height - drag.content_height_start;
    //                     this.offset.y = drag.offset_start - actual_growth;
    //                 }
    //                 cx.notify();
    //             },
    //         ))
    // }
}

pub(crate) fn set_highlighted_to(
    is_highlighted: Entity<bool>,
) -> impl Fn(&bool, &mut Window, &mut App) {
    move |&hovered, _window, cx| is_highlighted.write(cx, hovered)
}

pub(crate) fn do_nothing(_: &MouseDownEvent, window: &mut Window, cx: &mut App) {
    window.prevent_default();
    cx.stop_propagation();
}
