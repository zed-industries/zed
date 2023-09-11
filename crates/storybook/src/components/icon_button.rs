use crate::theme::theme;
use gpui2::elements::svg;
use gpui2::style::{StyleHelpers, Styleable};
use gpui2::{elements::div, IntoElement};
use gpui2::{Element, ParentElement, ViewContext};

#[derive(Element)]
pub(crate) struct IconButton {
    path: &'static str,
    variant: ButtonVariant,
}

#[derive(PartialEq)]
pub enum ButtonVariant {
    Ghost,
    Filled,
}

pub fn icon_button<V: 'static>(path: &'static str, variant: ButtonVariant) -> impl Element<V> {
    IconButton { path, variant }
}

impl IconButton {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        let mut div = div();

        if self.variant == ButtonVariant::Filled {
            div = div.fill(theme.highest.negative.default.background);
        }

        div.w_7()
            .h_6()
            .flex()
            .items_center()
            .justify_center()
            .rounded_md()
            .hover()
            .fill(theme.highest.base.hovered.background)
            .active()
            .fill(theme.highest.base.pressed.background)
            .child(
                svg()
                    .path(self.path)
                    .w_4()
                    .h_4()
                    .fill(theme.highest.variant.default.foreground),
            )
    }
}
