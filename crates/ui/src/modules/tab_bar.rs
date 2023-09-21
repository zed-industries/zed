use std::marker::PhantomData;

use gpui2::elements::div;
use gpui2::elements::div::ScrollState;
use gpui2::style::StyleHelpers;
use gpui2::{Element, IntoElement, ParentElement, ViewContext};

use crate::prelude::InteractionState;
use crate::theme::theme;
use crate::{icon_button, tab};

#[derive(Element)]
pub struct TabBar<V: 'static> {
    view_type: PhantomData<V>,
    scroll_state: ScrollState,
}

pub fn tab_bar<V: 'static>(scroll_state: ScrollState) -> TabBar<V> {
    TabBar {
        view_type: PhantomData,
        scroll_state,
    }
}

impl<V: 'static> TabBar<V> {
    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        let can_navigate_back = true;
        let can_navigate_forward = false;
        div()
            .w_full()
            .flex()
            // Left Side
            .child(
                div()
                    .px_1()
                    .flex()
                    .flex_none()
                    .gap_2()
                    // Nav Buttons
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_px()
                            .child(
                                icon_button("icons/arrow_left.svg")
                                    .state(InteractionState::Enabled.if_enabled(can_navigate_back)),
                            )
                            .child(
                                icon_button("icons/arrow_right.svg").state(
                                    InteractionState::Enabled.if_enabled(can_navigate_forward),
                                ),
                            ),
                    ),
            )
            .child(
                div().w_0().flex_1().h_full().child(
                    div()
                        .flex()
                        .gap_1()
                        .overflow_x_scroll(self.scroll_state.clone())
                        .child(tab("Cargo.toml", false))
                        .child(tab("Channels Panel", true))
                        .child(tab("channels_panel.rs", false))
                        .child(tab("workspace.rs", false))
                        .child(tab("icon_button.rs", false))
                        .child(tab("storybook.rs", false))
                        .child(tab("theme.rs", false))
                        .child(tab("theme_registry.rs", false))
                        .child(tab("styleable_helpers.rs", false)),
                ),
            )
            // Right Side
            .child(
                div()
                    .px_1()
                    .flex()
                    .flex_none()
                    .gap_2()
                    // Nav Buttons
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_px()
                            .child(icon_button("icons/plus.svg"))
                            .child(icon_button("icons/split.svg")),
                    ),
            )
    }
}
