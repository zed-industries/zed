use gpui2::style::{StyleHelpers, Styleable};
use gpui2::{Element, IntoElement, ParentElement, ViewContext};

use crate::{h_stack, theme};

#[derive(Element)]
pub struct Breadcrumb {}

impl Breadcrumb {
    pub fn new() -> Self {
        Self {}
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        h_stack()
            .px_1()
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
