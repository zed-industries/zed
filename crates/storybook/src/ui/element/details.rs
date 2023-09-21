use crate::theme::theme;
use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{Element, ViewContext};
use gpui2::{IntoElement, ParentElement};

#[derive(Element, Clone)]
pub struct Details {
    text: &'static str,
    meta: Option<&'static str>,
}

pub fn details(text: &'static str) -> Details {
    Details { text, meta: None }
}

impl Details {
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
            .child(self.text.clone())
            .children(self.meta.map(|m| m))
    }
}
