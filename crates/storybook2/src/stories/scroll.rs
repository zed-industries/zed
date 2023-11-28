use gpui::{div, prelude::*, px, Div, Render, SharedString, Stateful, Styled, View, WindowContext};
use theme2::ActiveTheme;
use ui::Tooltip;

pub struct ScrollStory;

impl ScrollStory {
    pub fn view(cx: &mut WindowContext) -> View<ScrollStory> {
        cx.build_view(|_cx| ScrollStory)
    }
}

impl Render for ScrollStory {
    type Element = Stateful<Div>;

    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> Self::Element {
        let theme = cx.theme();
        let color_1 = theme.status().created;
        let color_2 = theme.status().modified;

        div()
            .id("parent")
            .bg(theme.colors().background)
            .size_full()
            .overflow_scroll()
            .children((0..10).map(|row| {
                div()
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
                            .tooltip(move |cx| Tooltip::text(format!("{}, {}", row, column), cx))
                            .bg(bg)
                            .size(px(100. as f32))
                            .when(row >= 5 && column >= 5, |d| {
                                d.overflow_scroll()
                                    .child(div().size(px(50.)).bg(color_1))
                                    .child(div().size(px(50.)).bg(color_2))
                                    .child(div().size(px(50.)).bg(color_1))
                                    .child(div().size(px(50.)).bg(color_2))
                            })
                    }))
            }))
    }
}
