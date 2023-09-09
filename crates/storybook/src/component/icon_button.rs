use crate::theme::theme;
use gpui2::elements::svg;
use gpui2::style::{StyleHelpers, Styleable};
use gpui2::{elements::div, IntoElement};
use gpui2::{Element, ParentElement, ViewContext};

#[derive(Element)]
struct IconButton {
    path: &'static str,
    variant: Variant,
}

#[derive(PartialEq)]
pub enum Variant {
    Ghost,
    Filled,
}

pub fn icon_button<V: 'static>(path: &'static str, variant: Variant) -> impl Element<V> {
    IconButton { path, variant }
}

impl IconButton {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        let mut div = div();

        if self.variant == Variant::Filled {
            div = div.fill(theme.middle.base.default.background);
        }

        div.w_7()
            .h_6()
            .flex()
            .items_center()
            .justify_center()
            .rounded_md()
            .border()
            .border_color(theme.middle.base.default.background)
            .hover()
            .fill(theme.middle.base.hovered.background)
            .border_color(theme.middle.variant.hovered.border)
            .active()
            .fill(theme.middle.base.pressed.background)
            .border_color(theme.middle.variant.pressed.border)
            .child(
                svg()
                    .path(self.path)
                    .w_4()
                    .h_4()
                    .fill(theme.middle.variant.default.foreground),
            )
    }
}
