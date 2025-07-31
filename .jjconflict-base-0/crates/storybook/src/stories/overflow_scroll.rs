use gpui::Render;
use story::Story;

use ui::prelude::*;

pub struct OverflowScrollStory;

impl Render for OverflowScrollStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Story::container(cx)
            .child(Story::title("Overflow Scroll", cx))
            .child(Story::label("`overflow_x_scroll`", cx))
            .child(
                h_flex()
                    .id("overflow_x_scroll")
                    .gap_2()
                    .overflow_x_scroll()
                    .children((0..100).map(|i| {
                        div()
                            .p_4()
                            .debug_bg_cyan()
                            .child(SharedString::from(format!("Child {}", i + 1)))
                    })),
            )
            .child(Story::label("`overflow_y_scroll`", cx))
            .child(
                v_flex()
                    .w_full()
                    .flex_1()
                    .id("overflow_y_scroll")
                    .gap_2()
                    .overflow_y_scroll()
                    .children((0..100).map(|i| {
                        div()
                            .p_4()
                            .debug_bg_green()
                            .child(SharedString::from(format!("Child {}", i + 1)))
                    })),
            )
    }
}
