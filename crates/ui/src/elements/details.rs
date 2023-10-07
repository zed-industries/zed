use crate::prelude::*;
use crate::theme;

#[derive(Element, Clone)]
pub struct Details {
    text: &'static str,
    meta: Option<&'static str>,
}

impl Details {
    pub fn new(text: &'static str) -> Self {
        Self { text, meta: None }
    }

    pub fn meta_text(mut self, meta: &'static str) -> Self {
        self.meta = Some(meta);
        self
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        div()
            // .flex()
            // .w_full()
            .p_1()
            .gap_0p5()
            .text_xs()
            .text_color(theme.lowest.base.default.foreground)
            .child(self.text)
            .children(self.meta.map(|m| m))
    }
}
