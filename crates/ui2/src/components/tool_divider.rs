use crate::prelude::*;
use gpui::{Div, RenderOnce};

#[derive(RenderOnce)]
pub struct ToolDivider;

impl<V: 'static> Component<V> for ToolDivider {
    type Rendered = Div<V>;

    fn render(self, view: &mut V, cx: &mut ViewContext<V>) -> Self::Rendered {
        div().w_px().h_3().bg(cx.theme().colors().border)
    }
}

impl ToolDivider {
    pub fn new() -> Self {
        Self
    }

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Element<V> {
        div().w_px().h_3().bg(cx.theme().colors().border)
    }
}
