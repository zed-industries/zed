use std::marker::PhantomData;

use crate::prelude::Shape;
use crate::theme::theme;
use crate::ui::{avatar, icon_button, text_button, tool_divider};
use gpui2::style::StyleHelpers;
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
                            .child(text_button("zed"))
                            .child(text_button("nate/gpui2-ui-components")),
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
                            .child(icon_button("icons/stop_sharing.svg"))
                            .child(icon_button("icons/exit.svg")),
                    )
                    .child(tool_divider())
                    .child(
                        div()
                            .px_2()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(icon_button("icons/mic.svg"))
                            .child(icon_button("icons/speaker-loud.svg"))
                            .child(icon_button("icons/desktop.svg")),
                    )
                    .child(
                        div().px_2().flex().items_center().child(
                            avatar("https://avatars.githubusercontent.com/u/1714999?v=4")
                                .shape(Shape::RoundedRectangle),
                        ),
                    ),
            )
    }
}
