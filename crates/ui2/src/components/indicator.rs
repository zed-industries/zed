use gpui::px;

use crate::prelude::*;

#[derive(Component)]
pub struct UnreadIndicator;

impl UnreadIndicator {
    pub fn new() -> Self {
        Self
    }

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
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
