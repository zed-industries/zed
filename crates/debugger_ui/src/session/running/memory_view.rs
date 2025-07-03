use std::time::Duration;

use gpui::{
    FocusHandle, ListHorizontalSizingBehavior, ListState, MouseButton, ScrollHandle, Stateful,
    Task, list,
};
use ui::{
    Context, Div, Divider, Element, InteractiveElement, Label, LabelCommon, LineHeightStyle,
    ParentElement, Render, Scrollbar, ScrollbarState, StatefulInteractiveElement, Styled, Window,
    div, h_flex, px, v_flex,
};
use util::ResultExt;

pub(crate) struct MemoryView {
    state: ListState,
    line_width: usize,
    scroll_state: ScrollbarState,
    show_scrollbar: bool,
    hide_scrollbar_task: Option<Task<()>>,
    focus_handle: FocusHandle,
}

impl MemoryView {
    pub(crate) fn new(cx: &mut Context<Self>) -> Self {
        let line_width = 16;
        let scroll_handle = ScrollHandle::new();
        let state = ListState::new(
            Self::list_rows(line_width),
            gpui::ListAlignment::Top,
            px(1000.),
            move |ix, window, cx| {
                h_flex()
                    .w_full()
                    .gap_2()
                    .child(
                        Label::new(format!("{:08X}", ix * line_width))
                            .buffer_font(cx)
                            .size(ui::LabelSize::Small),
                    )
                    .child(Divider::vertical())
                    .child(
                        h_flex()
                            .id(("memory-view-row", ix * line_width))
                            .w_full()
                            .px_1()
                            .gap_1()
                            .children((0..line_width).map(|cell_ix| {
                                Label::new(format!(
                                    "{:02X}",
                                    (ix * line_width + cell_ix) % u8::MAX as usize
                                ))
                                .buffer_font(cx)
                                .size(ui::LabelSize::Small)
                                .line_height_style(LineHeightStyle::UiLabel)
                            }))
                            .overflow_x_scroll(),
                    )
                    .into_any()
            },
        );
        Self {
            scroll_state: ScrollbarState::new(state.clone()),
            state,
            line_width,

            show_scrollbar: false,
            hide_scrollbar_task: None,
            focus_handle: cx.focus_handle(),
        }
    }

    fn list_rows(bytes_per_row: usize) -> usize {
        4096 / bytes_per_row
    }
    fn hide_scrollbar(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        const SCROLLBAR_SHOW_INTERVAL: Duration = Duration::from_secs(1);
        self.hide_scrollbar_task = Some(cx.spawn_in(window, async move |panel, cx| {
            cx.background_executor()
                .timer(SCROLLBAR_SHOW_INTERVAL)
                .await;
            panel
                .update(cx, |panel, cx| {
                    panel.show_scrollbar = false;
                    cx.notify();
                })
                .log_err();
        }))
    }

    fn render_vertical_scrollbar(&self, cx: &mut Context<Self>) -> Option<Stateful<Div>> {
        if !(self.show_scrollbar || self.scroll_state.is_dragging()) {
            return None;
        }
        Some(
            div()
                .occlude()
                .id("memory-view-vertical-scrollbar")
                .on_mouse_move(cx.listener(|_, _, _, cx| {
                    cx.notify();
                    cx.stop_propagation()
                }))
                .on_hover(|_, _, cx| {
                    cx.stop_propagation();
                })
                .on_any_mouse_down(|_, _, cx| {
                    cx.stop_propagation();
                })
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(|_, _, _, cx| {
                        cx.stop_propagation();
                    }),
                )
                .on_scroll_wheel(cx.listener(|_, _, _, cx| {
                    cx.notify();
                }))
                .h_full()
                .absolute()
                .right_1()
                .top_1()
                .bottom_0()
                .w(px(12.))
                .cursor_default()
                .children(Scrollbar::vertical(self.scroll_state.clone())),
        )
    }
}

impl Render for MemoryView {
    fn render(
        &mut self,
        window: &mut ui::Window,
        cx: &mut ui::Context<Self>,
    ) -> impl ui::IntoElement {
        v_flex()
            .id("Memory-view")
            .p_1()
            .size_full()
            .on_hover(cx.listener(|this, hovered, window, cx| {
                if *hovered {
                    this.show_scrollbar = true;
                    this.hide_scrollbar_task.take();
                    cx.notify();
                } else if !this.focus_handle.contains_focused(window, cx) {
                    this.hide_scrollbar(window, cx);
                }
            }))
            .child(list(self.state.clone()).size_full())
            .children(self.render_vertical_scrollbar(cx))
    }
}
