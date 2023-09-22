use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{Element, IntoElement, ParentElement, ViewContext};

use crate::{icon, icon_button, theme, IconAsset};

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
            .flex()
            .justify_between()
            .child(
                // TODO(maxdeviant): Replace this with real breadcrumbs.
                div()
                    .child("crates/ui/src/components/toolbar.rs")
                    .text_color(theme.middle.base.default.foreground),
            )
            .child(
                div()
                    .flex()
                    .child(icon_button("icons/inlay_hint.svg"))
                    .child(icon_button("icons/magnifying_glass.svg"))
                    .child(icon_button("icons/magic-wand.svg")),
            )
    }
}
