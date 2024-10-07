use gpui::ScrollHandle;
use gpui::{div, prelude::*, px, Render, SharedString, Styled, View, WindowContext};
use ui::ScrollbarState;
use ui::Tooltip;
use ui::{prelude::*, Scrollbar};

pub struct ScrollStory {
    scroll_state: ScrollbarState,
    scroll_handle: ScrollHandle,
}

impl ScrollStory {
    pub fn view(cx: &mut WindowContext) -> View<ScrollStory> {
        cx.new_view(|cx| {
            let scroll_handle = ScrollHandle::new();

            ScrollStory {
                scroll_state: ScrollbarState::for_scrollable(cx.view(), scroll_handle.clone()),
                scroll_handle,
            }
        })
    }
}

impl Render for ScrollStory {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let color_1 = theme.status().created;
        let color_2 = theme.status().modified;

        h_flex()
            .child(
                v_flex()
                    .justify_start()
                    .id("parent")
                    .track_scroll(&self.scroll_handle)
                    .overflow_scroll()
                    .children((0..10).map(|row| {
                        div()
                            .bg(theme.colors().background)
                            .size_full()
                            .w(px(1000.))
                            .h(px(100.))
                            .flex()
                            .flex_row()
                            .children((0..10).map(|column| {
                                let id = SharedString::from(format!("{}, {}", row, column));
                                let bg = if row % 2 == column % 2 {
                                    color_1
                                } else {
                                    color_2
                                };
                                div()
                                    .id(id)
                                    .tooltip(move |cx| {
                                        Tooltip::text(format!("{}, {}", row, column), cx)
                                    })
                                    .bg(bg)
                                    .size(px(100_f32))
                                    .when(row >= 5 && column >= 5, |d| {
                                        d.overflow_scroll()
                                            .child(div().size(px(50.)).bg(color_1))
                                            .child(div().size(px(50.)).bg(color_2))
                                            .child(div().size(px(50.)).bg(color_1))
                                            .child(div().size(px(50.)).bg(color_2))
                                    })
                            }))
                    })),
            )
            .child(
                div()
                    .absolute()
                    .h_full()
                    .right_1()
                    .top_1()
                    .bottom_1()
                    .occlude()
                    .on_scroll_wheel(cx.listener(|_, _, cx| {
                        cx.notify();
                        cx.stop_propagation();
                    }))
                    .w_8()
                    .p_2()
                    .children(Scrollbar::vertical(self.scroll_state.clone())),
            )
    }
}
