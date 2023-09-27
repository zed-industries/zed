use crate::prelude::*;
use crate::Icon;
use crate::{theme, Breadcrumb, IconButton};

pub struct ToolbarItem {}

#[derive(Element)]
pub struct Toolbar {
    items: Vec<ToolbarItem>,
}

impl Toolbar {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        div()
            .p_2()
            .flex()
            .justify_between()
            .child(Breadcrumb::new())
            .child(
                div()
                    .flex()
                    .child(IconButton::new(Icon::inlay_hint()))
                    .child(IconButton::new(Icon::magnifying_glass()))
                    .child(IconButton::new(Icon::magic_wand())),
            )
    }
}
