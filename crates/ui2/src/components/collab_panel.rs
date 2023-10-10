use std::marker::PhantomData;

use gpui3::{img, svg, ArcCow};

use crate::prelude::*;
use crate::theme::{theme, Theme};
use crate::{
    static_collab_panel_channels, static_collab_panel_current_call, v_stack, Icon, List,
    ListHeader, ToggleState,
};

#[derive(Element)]
pub struct CollabPanel<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
    scroll_state: ScrollState,
}

impl<S: 'static + Send + Sync + Clone> CollabPanel<S> {
    pub fn new(scroll_state: ScrollState) -> Self {
        Self {
            state_type: PhantomData,
            scroll_state,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        let theme = theme(cx);

        v_stack()
            .w_64()
            .h_full()
            .fill(theme.middle.base.default.background)
            .child(
                v_stack()
                    .w_full()
                    .overflow_y_scroll(self.scroll_state.clone())
                    .child(
                        div()
                            .fill(theme.lowest.base.default.background)
                            .pb_1()
                            .border_color(theme.lowest.base.default.border)
                            .border_b()
                            .child(
                                List::new(static_collab_panel_current_call())
                                    .header(
                                        ListHeader::new("CRDB")
                                            .left_icon(Icon::Hash.into())
                                            .set_toggle(ToggleState::Toggled),
                                    )
                                    .set_toggle(ToggleState::Toggled),
                            ),
                    )
                    .child(
                        v_stack().py_1().child(
                            List::new(static_collab_panel_channels())
                                .header(
                                    ListHeader::new("CHANNELS").set_toggle(ToggleState::Toggled),
                                )
                                .empty_message("No channels yet. Add a channel to get started.")
                                .set_toggle(ToggleState::Toggled),
                        ),
                    )
                    .child(
                        v_stack().py_1().child(
                            List::new(static_collab_panel_current_call())
                                .header(
                                    ListHeader::new("CONTACTS – ONLINE")
                                        .set_toggle(ToggleState::Toggled),
                                )
                                .set_toggle(ToggleState::Toggled),
                        ),
                    )
                    .child(
                        v_stack().py_1().child(
                            List::new(static_collab_panel_current_call())
                                .header(
                                    ListHeader::new("CONTACTS – OFFLINE")
                                        .set_toggle(ToggleState::NotToggled),
                                )
                                .set_toggle(ToggleState::NotToggled),
                        ),
                    ),
            )
            .child(
                div()
                    .h_7()
                    .px_2()
                    .border_t()
                    .border_color(theme.middle.variant.default.border)
                    .flex()
                    .items_center()
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.middle.variant.default.foreground)
                            .child("Find..."),
                    ),
            )
    }

    fn list_section_header(
        &self,
        label: impl Into<ArcCow<'static, str>>,
        expanded: bool,
        theme: &Theme,
    ) -> impl Element<State = S> {
        div()
            .h_7()
            .px_2()
            .flex()
            .justify_between()
            .items_center()
            .child(div().flex().gap_1().text_sm().child(label.into()))
            .child(
                div().flex().h_full().gap_1().items_center().child(
                    svg()
                        .path(if expanded {
                            "icons/caret_down.svg"
                        } else {
                            "icons/caret_up.svg"
                        })
                        .w_3p5()
                        .h_3p5()
                        .fill(theme.middle.variant.default.foreground),
                ),
            )
    }

    fn list_item(
        &self,
        avatar_uri: impl Into<ArcCow<'static, str>>,
        label: impl Into<ArcCow<'static, str>>,
        theme: &Theme,
    ) -> impl Element<State = S> {
        div()
            .h_7()
            .px_2()
            .flex()
            .items_center()
            .hover()
            .fill(theme.lowest.variant.hovered.background)
            // .active()
            // .fill(theme.lowest.variant.pressed.background)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .text_sm()
                    .child(
                        img()
                            .uri(avatar_uri)
                            .size_3p5()
                            .rounded_full()
                            .fill(theme.middle.positive.default.foreground),
                    )
                    .child(label.into()),
            )
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use crate::Story;

    use super::*;

    #[derive(Element)]
    pub struct CollabPanelStory<S: 'static + Send + Sync + Clone> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync + Clone> CollabPanelStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
            Story::container(cx)
                .child(Story::title_for::<_, CollabPanel<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(CollabPanel::new(ScrollState::default()))
        }
    }
}
