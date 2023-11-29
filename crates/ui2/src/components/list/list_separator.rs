use gpui::Div;

use crate::prelude::*;

#[derive(IntoElement, Clone)]
pub struct ListSeparator;

impl ListSeparator {
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for ListSeparator {
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        div().h_px().w_full().bg(cx.theme().colors().border_variant)
    }
}
