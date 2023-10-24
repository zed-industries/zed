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

    pub fn title<V>(cx: &mut ViewContext<V>, title: &str) -> impl Element<ViewState = V> {
        let color = ThemeColor::new(cx);

        div()
            .text_xl()
            .text_color(color.text)
            .child(title.to_owned())
    }

    pub fn title_for<V, T>(cx: &mut ViewContext<V>) -> impl Element<ViewState = V> {
        Self::title(cx, std::any::type_name::<T>())
    }

    pub fn label<V>(cx: &mut ViewContext<V>, label: &str) -> impl Element<ViewState = V> {
        let color = ThemeColor::new(cx);

        div()
            .mt_4()
            .mb_2()
            .text_xs()
            .text_color(color.text)
            .child(label.to_owned())
    }
}
