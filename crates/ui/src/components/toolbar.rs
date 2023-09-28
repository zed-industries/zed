use crate::prelude::*;
use crate::{theme, Breadcrumb, Icon, IconButton};

#[derive(Clone)]
pub struct ToolbarItem {}

#[derive(Element, Clone)]
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
                    .child(IconButton::new(Icon::InlayHint))
                    .child(IconButton::new(Icon::MagnifyingGlass))
                    .child(IconButton::new(Icon::MagicWand)),
            )
    }
}
