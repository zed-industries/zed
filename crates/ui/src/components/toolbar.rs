use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{Element, IntoElement, ParentElement, ViewContext};

use crate::{breadcrumb, icon_button, theme, IconAsset};

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
                    .child(icon_button().icon(IconAsset::InlayHint))
                    .child(icon_button().icon(IconAsset::MagnifyingGlass))
                    .child(icon_button().icon(IconAsset::MagicWand)),
            )
    }
}
