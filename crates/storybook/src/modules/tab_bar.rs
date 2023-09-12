use std::marker::PhantomData;

use crate::components::{icon_button, tab, ButtonVariant};
use crate::theme::theme;
use gpui2::elements::div::ScrollState;
use gpui2::style::StyleHelpers;
use gpui2::{elements::div, IntoElement};
use gpui2::{Element, ParentElement, ViewContext};

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
                            .child(icon_button("icons/arrow_left.svg", ButtonVariant::Filled))
                            .child(icon_button("icons/arrow_right.svg", ButtonVariant::Ghost)),
                    ),
            )
            .child(
                div().w_0().flex_1().h_full().child(
                    div()
                        .flex()
                        .gap_px()
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
                            .child(icon_button("icons/plus.svg", ButtonVariant::Ghost))
                            .child(icon_button("icons/split.svg", ButtonVariant::Ghost)),
                    ),
            )
    }
}
