use crate::{
    div::div,
    element::{Element, IntoElement, ParentElement},
    style::StyleHelpers,
};
use gpui::{geometry::pixels, ViewContext};
use playground_macros::Element;

use crate as playground;
#[derive(Element)]
struct WorkspaceElement;

pub fn workspace<V: 'static>() -> impl Element<V> {
    WorkspaceElement
}

impl WorkspaceElement {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        // let theme = &cx.theme::<Theme>().colors;
        div()
            .full()
            .flex()
            .flex_col()
            // .fill(theme.base(0.5))
            .child(self.title_bar(cx))
            .child(self.stage(cx))
            .child(self.status_bar(cx))
    }

    fn title_bar<V: 'static>(&mut self, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        // let colors = &theme(cx).colors;
        div().h(pixels(cx.titlebar_height())) //.fill(colors.base(0.))
    }

    fn status_bar<V: 'static>(&mut self, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        div().h(pixels(cx.titlebar_height())) //.fill(colors.base(0.))
    }

    fn stage<V: 'static>(&mut self, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        div().flex_grow()
    }
}
