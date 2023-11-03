use std::time::Duration;

use gpui2::{
    div, px, Component, Div, ParentElement, Render, SharedString, Styled, View, ViewContext,
    VisualContext, WindowContext,
};
use theme2::ActiveTheme;

const DELAY: Duration = Duration::from_millis(500);

#[derive(Clone, Debug)]
pub struct TextTooltip {
    title: SharedString,
    visible: bool,
}

impl TextTooltip {
    pub fn new(str: SharedString) -> Self {
        Self {
            title: str,
            visible: false,
        }
    }

    pub fn build_view(str: SharedString, cx: &mut WindowContext) -> View<Self> {
        let view = cx.build_view(|cx| TextTooltip::new(str));

        let handle = view.downgrade();
        cx.spawn(|mut cx| async move {
            cx.background_executor().timer(DELAY).await;

            handle
                .update(&mut cx, |this, cx| {
                    this.visible = true;
                    cx.notify();
                })
                .ok();
        })
        .detach();

        view
    }
}

impl Render for TextTooltip {
    type Element = Div<Self>;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        let theme = cx.theme();
        div()
            .when(!self.visible, |this| this.invisible())
            .bg(theme.colors().background)
            .rounded(px(8.))
            .border()
            .font("Zed Sans")
            .border_color(theme.colors().border)
            .text_color(theme.colors().text)
            .pl_2()
            .pr_2()
            .child(self.title.clone())
    }
}
