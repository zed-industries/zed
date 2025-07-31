use gpui::Render;
use story::Story;

use ui::prelude::*;

pub struct ViewportUnitsStory;

impl Render for ViewportUnitsStory {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Story::container(cx).child(
            div()
                .flex()
                .flex_row()
                .child(
                    div()
                        .w(vw(0.5, window))
                        .h(vh(0.8, window))
                        .bg(gpui::red())
                        .text_color(gpui::white())
                        .child("50vw, 80vh"),
                )
                .child(
                    div()
                        .w(vw(0.25, window))
                        .h(vh(0.33, window))
                        .bg(gpui::green())
                        .text_color(gpui::white())
                        .child("25vw, 33vh"),
                ),
        )
    }
}
