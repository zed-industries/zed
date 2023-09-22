use crate::theme::theme;
use crate::{label, LabelColor, LabelSize};
use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{Element, IntoElement};
use gpui2::{ParentElement, ViewContext};

#[derive(Element)]
pub struct PaletteItem {
    pub label: &'static str,
    pub keybinding: Option<&'static str>,
}

pub fn palette_item(label: &'static str, keybinding: Option<&'static str>) -> PaletteItem {
    PaletteItem { label, keybinding }
}

impl PaletteItem {
    pub fn label(mut self, label: &'static str) -> Self {
        self.label = label;
        self
    }

    pub fn keybinding(mut self, keybinding: Option<&'static str>) -> Self {
        self.keybinding = keybinding;
        self
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        let keybinding_label = match self.keybinding {
            Some(keybind) => label(keybind)
                .color(LabelColor::Muted)
                .size(LabelSize::Small),
            None => label(""),
        };

        div()
            .flex()
            .flex_row()
            .grow()
            .justify_between()
            .child(label(self.label))
            .child(
                self.keybinding
                    .map(|_| {
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .px_1()
                            .py_0()
                            .my_0p5()
                            .rounded_md()
                            .text_sm()
                            .fill(theme.lowest.on.default.background)
                            .child(keybinding_label)
                    })
                    .unwrap_or_else(|| div()),
            )
    }
}
