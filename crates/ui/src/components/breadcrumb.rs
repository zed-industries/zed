use gpui2::elements::div;
use gpui2::style::StyleHelpers;
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
            .flex()
            .flex_row()
            // TODO: Read font from theme.
            .font("Zed Mono Extended")
            .text_sm()
            .text_color(theme.middle.base.default.foreground)
            // TODO: Replace hardcoded breadcrumbs.
            .child("crates/ui/src/components/toolbar.rs")
            .child(" › ")
            .child("impl Breadcrumb")
            .child(" › ")
            .child("fn render")
    }
}
