use crate::theme::theme;
use gpui2::{
    elements::div, geometry::pixels, style::StyleHelpers, Element, IntoElement, ParentElement,
    ViewContext,
};

#[derive(Element)]
struct WorkspaceElement;

pub fn workspace<V: 'static>() -> impl Element<V> {
    WorkspaceElement
}

impl WorkspaceElement {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        div()
            .full()
            .flex()
            .flex_col()
            .fill(theme.middle.base.default.background)
            .child(self.title_bar(cx))
            .child(self.stage(cx))
            .child(self.status_bar(cx))
    }

    fn title_bar<V: 'static>(&mut self, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        div()
            .h(pixels(cx.titlebar_height()))
            .fill(theme.lowest.base.default.background)
    }

    fn status_bar<V: 'static>(&mut self, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        div().h(pixels(cx.titlebar_height())) //.fill(colors.base(0.))
    }

    fn stage<V: 'static>(&mut self, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        div().flex_grow()
    }
}
