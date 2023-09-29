use std::path::PathBuf;
use std::str::FromStr;

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
            .fill(theme.highest.base.default.background)
            .p_2()
            .flex()
            .justify_between()
            .child(Breadcrumb::new(
                PathBuf::from_str("crates/ui/src/components/toolbar.rs").unwrap(),
            ))
            .child(
                div()
                    .flex()
                    .child(IconButton::new(Icon::InlayHint))
                    .child(IconButton::new(Icon::MagnifyingGlass))
                    .child(IconButton::new(Icon::MagicWand)),
            )
    }
}
