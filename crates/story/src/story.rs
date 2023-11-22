use gpui::prelude::*;
use gpui::{div, hsla, Div, SharedString};

pub struct Story {}

impl Story {
    pub fn container() -> Div {
        div().size_full().flex().flex_col().pt_2().px_4().bg(hsla(
            0. / 360.,
            0. / 100.,
            100. / 100.,
            1.,
        ))
    }

    pub fn title(title: impl Into<SharedString>) -> impl IntoElement {
        div()
            .text_xl()
            .text_color(hsla(0. / 360., 0. / 100., 0. / 100., 1.))
            .child(title.into())
    }

    pub fn title_for<T>() -> impl IntoElement {
        Self::title(std::any::type_name::<T>())
    }

    pub fn label(label: impl Into<SharedString>) -> impl IntoElement {
        div()
            .mt_4()
            .mb_2()
            .text_xs()
            .text_color(hsla(0. / 360., 0. / 100., 0. / 100., 1.))
            .child(label.into())
    }
}
