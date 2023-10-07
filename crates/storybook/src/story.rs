use gpui2::elements::div::Div;
use ui::prelude::*;
use ui::theme;

pub struct Story {}

impl Story {
    pub fn container<V: 'static>(cx: &mut ViewContext<V>) -> Div<V> {
        let theme = theme(cx);

        div()
            .size_full()
            .flex()
            .flex_col()
            .pt_2()
            .px_4()
            .font("Zed Mono Extended")
            .fill(theme.lowest.base.default.background)
    }

    pub fn title<V: 'static>(cx: &mut ViewContext<V>, title: &str) -> impl Element<V> {
        let theme = theme(cx);

        div()
            .text_xl()
            .text_color(theme.lowest.base.default.foreground)
            .child(title.to_owned())
    }

    pub fn title_for<V: 'static, T>(cx: &mut ViewContext<V>) -> impl Element<V> {
        Self::title(cx, std::any::type_name::<T>())
    }

    pub fn label<V: 'static>(cx: &mut ViewContext<V>, label: &str) -> impl Element<V> {
        let theme = theme(cx);

        div()
            .mt_4()
            .mb_2()
            .text_xs()
            .text_color(theme.lowest.base.default.foreground)
            .child(label.to_owned())
    }
}
