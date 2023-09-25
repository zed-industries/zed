use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{Element, IntoElement, ViewContext};

use crate::theme;

#[derive(Element)]
pub struct ToolDivider {}

impl ToolDivider {
    pub fn new() -> Self {
        Self {}
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        div().w_px().h_3().fill(theme.lowest.base.default.border)
    }
}
