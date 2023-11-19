use gpui::Div;

use crate::prelude::*;

pub struct Story {}

impl Story {
    pub fn container<V: 'static>(cx: &mut ViewContext<V>) -> Div<V> {
        div()
            .size_full()
            .flex()
            .flex_col()
            .pt_2()
            .px_4()
            .bg(cx.theme().colors().background)
    }

    pub fn title<V: 'static>(
        cx: &mut ViewContext<V>,
        title: impl Into<SharedString>,
    ) -> impl Element<V> {
        div()
            .text_xl()
            .text_color(cx.theme().colors().text)
            .child(title.into())
    }

    pub fn title_for<V: 'static, T>(cx: &mut ViewContext<V>) -> impl Element<V> {
        Self::title(cx, std::any::type_name::<T>())
    }

    pub fn label<V: 'static>(
        cx: &mut ViewContext<V>,
        label: impl Into<SharedString>,
    ) -> impl Element<V> {
        div()
            .mt_4()
            .mb_2()
            .text_xs()
            .text_color(cx.theme().colors().text)
            .child(label.into())
    }
}
