use crate::prelude::*;
use gpui::{Div, RenderOnce};

#[derive(RenderOnce)]
pub struct ToolDivider;

impl Component for ToolDivider {
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        div().w_px().h_3().bg(cx.theme().colors().border)
    }
}

impl ToolDivider {
    pub fn new() -> Self {
        Self
    }

    fn render(self, cx: &mut WindowContext) -> impl Element {
        div().w_px().h_3().bg(cx.theme().colors().border)
    }
}
