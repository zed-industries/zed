use crate::prelude::*;
use gpui::{px, Div, RenderOnce};

#[derive(RenderOnce)]
pub struct UnreadIndicator;

impl<V: 'static> Component<V> for UnreadIndicator {
    type Rendered = Div<V>;

    fn render(self, view: &mut V, cx: &mut ViewContext<V>) -> Self::Rendered {
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

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Element<V> {
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
