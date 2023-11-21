use gpui::Div;

use crate::prelude::*;

pub struct Story {}

impl Story {
    pub fn container(cx: &mut gpui::WindowContext) -> Div {
        div()
            .size_full()
            .flex()
            .flex_col()
            .pt_2()
            .px_4()
            .bg(cx.theme().colors().background)
    }

    pub fn title(cx: &mut WindowContext, title: impl Into<SharedString>) -> impl Element {
        div()
            .text_xl()
            .text_color(cx.theme().colors().text)
            .child(title.into())
    }

    pub fn title_for<T>(cx: &mut WindowContext) -> impl Element {
        Self::title(cx, std::any::type_name::<T>())
    }

    pub fn label(cx: &mut WindowContext, label: impl Into<SharedString>) -> impl Element {
        div()
            .mt_4()
            .mb_2()
            .text_xs()
            .text_color(cx.theme().colors().text)
            .child(label.into())
    }
}
