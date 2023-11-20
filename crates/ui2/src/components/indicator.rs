use crate::prelude::*;
use gpui::{px, Div, RenderOnce};

#[derive(RenderOnce)]
pub struct UnreadIndicator;

impl Component for UnreadIndicator {
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        div()
            .rounded_full()
            .border_2()
            .border_color(cx.theme().colors().surface_background)
            .w(px(9.0))
            .h(px(9.0))
            .z_index(2)
            .bg(cx.theme().status().info)
    }
}

impl UnreadIndicator {
    pub fn new() -> Self {
        Self
    }

    fn render(self, cx: &mut WindowContext) -> impl Element {
        div()
            .rounded_full()
            .border_2()
            .border_color(cx.theme().colors().surface_background)
            .w(px(9.0))
            .h(px(9.0))
            .z_index(2)
            .bg(cx.theme().status().info)
    }
}
