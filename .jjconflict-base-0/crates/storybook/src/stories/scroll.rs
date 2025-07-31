use gpui::{App, Entity, Render, SharedString, Styled, Window, div, prelude::*, px};
use ui::Tooltip;
use ui::prelude::*;

pub struct ScrollStory;

impl ScrollStory {
    pub fn model(cx: &mut App) -> Entity<ScrollStory> {
        cx.new(|_| ScrollStory)
    }
}

impl Render for ScrollStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                            .id(id.clone())
                            .tooltip(Tooltip::text(id))
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
            }))
    }
}
