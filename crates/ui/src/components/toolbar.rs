use gpui2::elements::div;
use gpui2::{Element, IntoElement, ViewContext};

use crate::theme;

#[derive(Element)]
pub struct Toolbar {}

impl Toolbar {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        div()
    }
}
