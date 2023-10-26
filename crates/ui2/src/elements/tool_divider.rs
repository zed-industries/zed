use crate::prelude::*;

#[derive(Component)]
pub struct ToolDivider;

impl ToolDivider {
    pub fn new() -> Self {
        Self
    }

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        let theme = theme(cx);

        div().w_px().h_3().bg(theme.border)
    }
}
