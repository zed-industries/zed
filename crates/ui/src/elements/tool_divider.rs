use crate::theme::theme;
use gpui2::style::StyleHelpers;
use gpui2::{elements::div, IntoElement};
use gpui2::{Element, ViewContext};

#[derive(Element)]
pub struct ToolDivider {}

pub fn tool_divider<V: 'static>() -> impl Element<V> {
    ToolDivider {}
}

impl ToolDivider {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        div().w_px().h_3().fill(theme.lowest.base.default.border)
    }
}
