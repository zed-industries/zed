use crate::theme::theme;
use gpui2::{elements::div, style::StyleHelpers, Element, IntoElement, ParentElement, ViewContext};
use std::marker::PhantomData;

#[derive(Element)]
pub struct CollabPanelElement<V: 'static> {
    view_type: PhantomData<V>,
}

pub fn collab_panel<V: 'static>() -> CollabPanelElement<V> {
    CollabPanelElement {
        view_type: PhantomData,
    }
}

impl<V: 'static> CollabPanelElement<V> {
    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        div()
            .full()
            .font("Zed Mono")
            .text_color(theme.middle.variant.default.foreground)
            .fill(theme.middle.base.default.background)
            .py_2()
            .child(
                div()
                    .px_2()
                    .flex()
                    .justify_between()
                    .child("#CRDB")
                    .child("V"),
            )
    }
}
