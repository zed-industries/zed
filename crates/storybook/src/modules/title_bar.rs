use std::marker::PhantomData;

use crate::components::{avatar, icon_button, tool_divider};
use crate::prelude::{ButtonVariant, Shape, UIState};
use crate::theme::theme;
use gpui2::style::{StyleHelpers, Styleable};
use gpui2::{elements::div, IntoElement};
use gpui2::{Element, ParentElement, ViewContext};

#[derive(Element)]
pub struct TitleBar<V: 'static> {
    view_type: PhantomData<V>,
}

pub fn title_bar<V: 'static>() -> TitleBar<V> {
    TitleBar {
        view_type: PhantomData,
    }
}

impl<V: 'static> TitleBar<V> {
    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .h_8()
            .fill(theme.lowest.base.default.background)
            .child(
                div()
                    .flex()
                    .items_center()
                    .h_full()
                    .gap_4()
                    .px_2()
                    // === Traffic Lights === //
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .w_3()
                                    .h_3()
                                    .rounded_full()
                                    .fill(theme.lowest.positive.default.foreground),
                            )
                            .child(
                                div()
                                    .w_3()
                                    .h_3()
                                    .rounded_full()
                                    .fill(theme.lowest.warning.default.foreground),
                            )
                            .child(
                                div()
                                    .w_3()
                                    .h_3()
                                    .rounded_full()
                                    .fill(theme.lowest.negative.default.foreground),
                            ),
                    )
                    // === Project Info === //
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(
                                div()
                                    .h_full()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .px_2()
                                    .rounded_md()
                                    .hover()
                                    .fill(theme.lowest.base.hovered.background)
                                    .active()
                                    .fill(theme.lowest.base.pressed.background)
                                    .child(div().text_sm().child("project")),
                            )
                            .child(
                                div()
                                    .h_full()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .px_2()
                                    .rounded_md()
                                    .text_color(theme.lowest.variant.default.foreground)
                                    .hover()
                                    .fill(theme.lowest.base.hovered.background)
                                    .active()
                                    .fill(theme.lowest.base.pressed.background)
                                    .child(div().text_sm().child("branch")),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .child(
                        div()
                            .px_2()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(icon_button(
                                "icons/stop_sharing.svg",
                                ButtonVariant::Ghost,
                                UIState::Default,
                            ))
                            .child(icon_button(
                                "icons/exit.svg",
                                ButtonVariant::Ghost,
                                UIState::Default,
                            )),
                    )
                    .child(tool_divider())
                    .child(
                        div()
                            .px_2()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(icon_button(
                                "icons/radix/mic.svg",
                                ButtonVariant::Ghost,
                                UIState::Default,
                            ))
                            .child(icon_button(
                                "icons/radix/speaker-loud.svg",
                                ButtonVariant::Ghost,
                                UIState::Default,
                            ))
                            .child(icon_button(
                                "icons/radix/desktop.svg",
                                ButtonVariant::Ghost,
                                UIState::Default,
                            )),
                    )
                    .child(div().px_2().flex().items_center().child(avatar(
                        "https://avatars.githubusercontent.com/u/1714999?v=4",
                        Shape::RoundedRectangle,
                    ))),
            )
    }
}
