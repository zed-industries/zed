use gpui2::Div;

use crate::prelude::*;

pub struct Story {}

impl Story {
    pub fn container<S: 'static + Send + Sync>(cx: &mut ViewContext<S>) -> Div<S> {
        let color = ThemeColor::new(cx);

        div()
            .size_full()
            .flex()
            .flex_col()
            .pt_2()
            .px_4()
            .font("Zed Mono Extended")
            .bg(color.background)
    }

    pub fn title<S: 'static + Send + Sync>(
        cx: &mut ViewContext<S>,
        title: &str,
    ) -> impl Element<ViewState = S> {
        let color = ThemeColor::new(cx);

        div()
            .text_xl()
            .text_color(color.text)
            .child(title.to_owned())
    }

    pub fn title_for<S: 'static + Send + Sync, T>(
        cx: &mut ViewContext<S>,
    ) -> impl Element<ViewState = S> {
        Self::title(cx, std::any::type_name::<T>())
    }

    pub fn label<S: 'static + Send + Sync>(
        cx: &mut ViewContext<S>,
        label: &str,
    ) -> impl Element<ViewState = S> {
        let color = ThemeColor::new(cx);

        div()
            .mt_4()
            .mb_2()
            .text_xs()
            .text_color(color.text)
            .child(label.to_owned())
    }
}
