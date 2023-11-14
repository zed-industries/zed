use gpui::{div, Node, ParentComponent, Render, SharedString, Styled, ViewContext};
use theme2::ActiveTheme;

#[derive(Clone, Debug)]
pub struct TextTooltip {
    title: SharedString,
}

impl TextTooltip {
    pub fn new(str: SharedString) -> Self {
        Self { title: str }
    }
}

impl Render for TextTooltip {
    type Element = Node<Self>;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        let theme = cx.theme();
        div()
            .bg(theme.colors().background)
            .rounded_lg()
            .border()
            .font("Zed Sans")
            .border_color(theme.colors().border)
            .text_color(theme.colors().text)
            .pl_2()
            .pr_2()
            .child(self.title.clone())
    }
}
