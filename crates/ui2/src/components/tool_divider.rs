use crate::prelude::*;

#[derive(Component)]
pub struct ToolDivider;

impl ToolDivider {
    pub fn new() -> Self {
        Self
    }

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Element<V> {
        div().w_px().h_3().bg(cx.theme().colors().border)
    }
}
