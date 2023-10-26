use gpui2::Div;

use crate::prelude::*;

pub struct Story {}

impl Story {
    pub fn container<S: 'static + Send + Sync>(cx: &mut ViewContext<S>) -> Div<S> {
        let theme = theme(cx);

        div()
            .size_full()
            .flex()
            .flex_col()
            .pt_2()
            .px_4()
            .font("Zed Mono")
            .bg(theme.background)
    }

    pub fn title<S: 'static + Send + Sync>(
        cx: &mut ViewContext<S>,
        title: &str,
    ) -> impl IntoAnyElement<S> {
        let theme = theme(cx);

        div()
            .text_xl()
            .text_color(theme.text)
            .child(title.to_owned())
    }

    pub fn title_for<S: 'static + Send + Sync, T>(
        cx: &mut ViewContext<S>,
    ) -> impl IntoAnyElement<S> {
        Self::title(cx, std::any::type_name::<T>())
    }

    pub fn label<S: 'static + Send + Sync>(
        cx: &mut ViewContext<S>,
        label: &str,
    ) -> impl IntoAnyElement<S> {
        let theme = theme(cx);

        div()
            .mt_4()
            .mb_2()
            .text_xs()
            .text_color(theme.text)
            .child(label.to_owned())
    }
}
