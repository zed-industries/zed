use gpui::{Div, Render};
use story::Story;

use ui::prelude::*;

pub struct ViewportUnitsStory;

impl Render for ViewportUnitsStory {
    type Element = Div;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        Story::container().child(
            div()
                .flex()
                .flex_row()
                .child(
                    div()
                        .w_vw(0.5, cx)
                        .h_vh(0.8, cx)
                        .bg(gpui::red())
                        .text_color(gpui::white())
                        .child("50vw, 80vh"),
                )
                .child(
                    div()
                        .w_vw(0.25, cx)
                        .h_vh(0.33, cx)
                        .bg(gpui::green())
                        .text_color(gpui::white())
                        .child("25vw, 33vh"),
                ),
        )
    }
}
