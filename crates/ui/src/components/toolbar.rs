use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{Element, IntoElement, ParentElement, ViewContext};

use crate::{breadcrumb, theme, IconAsset, IconButton};

pub struct ToolbarItem {}

#[derive(Element)]
pub struct Toolbar {
    items: Vec<ToolbarItem>,
}

pub fn toolbar() -> Toolbar {
    Toolbar { items: Vec::new() }
}

impl Toolbar {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        div()
            .p_2()
            .flex()
            .justify_between()
            .child(breadcrumb())
            .child(
                div()
                    .flex()
                    .child(IconButton::new(IconAsset::InlayHint))
                    .child(IconButton::new(IconAsset::MagnifyingGlass))
                    .child(IconButton::new(IconAsset::MagicWand)),
            )
    }
}
