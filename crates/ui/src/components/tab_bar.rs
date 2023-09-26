use std::marker::PhantomData;

use gpui2::elements::div::{div, ScrollState};
use gpui2::{Element, IntoElement, ParentElement, ViewContext};

use crate::prelude::*;
use crate::{theme, IconAsset, IconButton, Tab};

#[derive(Element)]
pub struct TabBar<V: 'static> {
    view_type: PhantomData<V>,
    scroll_state: ScrollState,
}

impl<V: 'static> TabBar<V> {
    pub fn new(scroll_state: ScrollState) -> Self {
        Self {
            view_type: PhantomData,
            scroll_state,
        }
    }

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
                                IconButton::new(IconAsset::ArrowLeft)
                                    .state(InteractionState::Enabled.if_enabled(can_navigate_back)),
                            )
                            .child(
                                IconButton::new(IconAsset::ArrowRight).state(
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
                            .child(IconButton::new(IconAsset::Plus))
                            .child(IconButton::new(IconAsset::Split)),
                    ),
            )
    }
}
