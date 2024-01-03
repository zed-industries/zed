use gpui::Render;
use story::Story;

use ui::prelude::*;

pub struct ViewportUnitsStory;

impl Render for ViewportUnitsStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        Story::container().child(
            div()
                .flex()
                .flex_row()
                .child(
                    div()
                        .w(vw(0.5, cx))
                        .h(vh(0.8, cx))
                        .bg(gpui::red())
                        .text_color(gpui::white())
                        .child("50vw, 80vh"),
                )
                .child(
                    div()
                        .w(vw(0.25, cx))
                        .h(vh(0.33, cx))
                        .bg(gpui::green())
                        .text_color(gpui::white())
                        .child("25vw, 33vh"),
                ),
        )
    }
}
