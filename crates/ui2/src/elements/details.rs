use std::marker::PhantomData;

use crate::prelude::*;
use crate::theme;

#[derive(Element, Clone)]
pub struct Details<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
    text: &'static str,
    meta: Option<&'static str>,
}

impl<S: 'static + Send + Sync + Clone> Details<S> {
    pub fn new(text: &'static str) -> Self {
        Self {
            state_type: PhantomData,
            text,
            meta: None,
        }
    }

    pub fn meta_text(mut self, meta: &'static str) -> Self {
        self.meta = Some(meta);
        self
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
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
