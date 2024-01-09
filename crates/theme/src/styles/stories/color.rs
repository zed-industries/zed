use gpui::prelude::*;
use gpui::{div, px, ViewContext};
use story::Story;

use crate::{default_color_scales, ColorScaleStep};

pub struct ColorsStory;

impl Render for ColorsStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let color_scales = default_color_scales();

        Story::container().child(Story::title("Colors")).child(
            div()
                .id("colors")
                .flex()
                .flex_col()
                .gap_1()
                .overflow_y_scroll()
                .text_color(gpui::white())
                .children(color_scales.into_iter().map(|scale| {
                    div()
                        .flex()
                        .child(
                            div()
                                .w(px(75.))
                                .line_height(px(24.))
                                .child(scale.name().clone()),
                        )
                        .child(
                            div().flex().gap_1().children(
                                ColorScaleStep::ALL
                                    .map(|step| div().flex().size_6().bg(scale.step(cx, step))),
                            ),
                        )
                })),
        )
    }
}
