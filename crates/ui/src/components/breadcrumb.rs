use gpui2::elements::div;
use gpui2::style::{StyleHelpers, Styleable};
use gpui2::{Element, IntoElement, ParentElement, ViewContext};

use crate::theme;

#[derive(Element)]
pub struct Breadcrumb {}

pub fn breadcrumb() -> Breadcrumb {
    Breadcrumb {}
}

impl Breadcrumb {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        div()
            .px_1()
            .flex()
            .flex_row()
            // TODO: Read font from theme (or settings?).
            .font("Zed Mono Extended")
            .text_sm()
            .text_color(theme.middle.base.default.foreground)
            .rounded_md()
            .hover()
            .fill(theme.highest.base.hovered.background)
            // TODO: Replace hardcoded breadcrumbs.
            .child("crates/ui/src/components/toolbar.rs")
            .child(" › ")
            .child("impl Breadcrumb")
            .child(" › ")
            .child("fn render")
    }
}
