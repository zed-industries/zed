use gpui2::{
    div, px, Div, ParentElement, Render, SharedString, Styled, View, ViewContext, VisualContext,
};
use theme2::ActiveTheme;

#[derive(Clone, Debug)]
pub struct TextTooltip {
    title: SharedString,
}

impl TextTooltip {
    pub fn new(str: SharedString) -> Self {
        Self { title: str }
    }

    pub fn build_view<C: VisualContext>(str: SharedString, cx: &mut C) -> C::Result<View<Self>> {
        cx.build_view(|cx| TextTooltip::new(str))
    }
}

impl Render for TextTooltip {
    type Element = Div<Self>;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        let theme = cx.theme();
        div()
            .bg(theme.colors().background)
            .rounded(px(8.))
            .border()
            .border_color(theme.colors().border)
            .text_color(theme.colors().text)
            .pl_2()
            .pr_2()
            .child(self.title.clone())
    }
}
