use gpui::{Div, Render};
use story::Story;

use ui::prelude::*;

pub struct OverflowScrollStory;

impl Render for OverflowScrollStory {
    type Output = Div;

    fn render(&mut self, _cx: &mut ViewContext<Self>) -> Self::Output {
        Story::container()
            .child(Story::title("Overflow Scroll"))
            .child(Story::label("`overflow_x_scroll`"))
            .child(
                h_stack()
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
            .child(Story::label("`overflow_y_scroll`"))
            .child(
                v_stack()
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
