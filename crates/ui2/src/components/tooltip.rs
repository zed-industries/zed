use gpui::{div, Div, ParentElement, Render, SharedString, Styled, ViewContext};
use theme2::ActiveTheme;

use crate::StyledExt;

#[derive(Clone, Debug)]
pub struct TextTooltip {
    title: SharedString,
}

impl TextTooltip {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
        }
    }
}

impl Render for TextTooltip {
    type Element = Div<Self>;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        div()
            .elevation_2(cx)
            .font("Zed Sans")
            .text_ui()
            .text_color(cx.theme().colors().text)
            .py_1()
            .px_2()
            .child(self.title.clone())
    }
}
