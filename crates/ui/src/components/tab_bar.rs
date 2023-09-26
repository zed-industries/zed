use std::marker::PhantomData;

use gpui2::elements::div::div;
use gpui2::elements::div::ScrollState;
use gpui2::style::StyleHelpers;
use gpui2::{Element, IntoElement, ParentElement, ViewContext};

use crate::prelude::*;
use crate::{icon_button, theme, IconAsset, Tab};

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
            .fill(theme.middle.base.default.background)
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
                                icon_button()
                                    .icon(IconAsset::ArrowLeft)
                                    .state(InteractionState::Enabled.if_enabled(can_navigate_back)),
                            )
                            .child(
                                icon_button().icon(IconAsset::ArrowRight).state(
                                    InteractionState::Enabled.if_enabled(can_navigate_forward),
                                ),
                            ),
                    ),
            )
            .child(
                div().w_0().flex_1().h_full().child(
                    div()
                        .flex()
                        .overflow_x_scroll(self.scroll_state.clone())
                        .child(
                            Tab::new()
                                .title("Cargo.toml")
                                .current(false)
                                .git_status(GitStatus::Modified),
                        )
                        .child(Tab::new().title("Channels Panel").current(false))
                        .child(
                            Tab::new()
                                .title("channels_panel.rs")
                                .current(true)
                                .git_status(GitStatus::Modified),
                        )
                        .child(
                            Tab::new()
                                .title("workspace.rs")
                                .current(false)
                                .git_status(GitStatus::Modified),
                        )
                        .child(Tab::new().title("icon_button.rs").current(false))
                        .child(
                            Tab::new()
                                .title("storybook.rs")
                                .current(false)
                                .git_status(GitStatus::Created),
                        )
                        .child(Tab::new().title("theme.rs").current(false))
                        .child(Tab::new().title("theme_registry.rs").current(false))
                        .child(Tab::new().title("styleable_helpers.rs").current(false)),
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
                            .child(icon_button().icon(IconAsset::Plus))
                            .child(icon_button().icon(IconAsset::Split)),
                    ),
            )
    }
}
