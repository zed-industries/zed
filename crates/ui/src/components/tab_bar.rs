use std::marker::PhantomData;

use crate::prelude::*;
use crate::{theme, Icon, IconButton, Tab};

#[derive(Element)]
pub struct TabBar<V: 'static> {
    view_type: PhantomData<V>,
    scroll_state: ScrollState,
}

impl<V: 'static> TabBar<V> {
    pub fn new() -> Self {
        Self {
            view_type: PhantomData,
            scroll_state: ScrollState::default(),
        }
    }

    pub fn bind_scroll_state(&mut self, scroll_state: ScrollState) {
        self.scroll_state = scroll_state;
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
                                IconButton::new(Icon::ArrowLeft)
                                    .state(InteractionState::Enabled.if_enabled(can_navigate_back)),
                            )
                            .child(
                                IconButton::new(Icon::ArrowRight).state(
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
                                .title("Cargo.toml".to_string())
                                .current(false)
                                .git_status(GitStatus::Modified),
                        )
                        .child(
                            Tab::new()
                                .title("Channels Panel".to_string())
                                .current(false),
                        )
                        .child(
                            Tab::new()
                                .title("channels_panel.rs".to_string())
                                .current(true)
                                .git_status(GitStatus::Modified),
                        )
                        .child(
                            Tab::new()
                                .title("workspace.rs".to_string())
                                .current(false)
                                .git_status(GitStatus::Modified),
                        )
                        .child(
                            Tab::new()
                                .title("icon_button.rs".to_string())
                                .current(false),
                        )
                        .child(
                            Tab::new()
                                .title("storybook.rs".to_string())
                                .current(false)
                                .git_status(GitStatus::Created),
                        )
                        .child(Tab::new().title("theme.rs".to_string()).current(false))
                        .child(
                            Tab::new()
                                .title("theme_registry.rs".to_string())
                                .current(false),
                        )
                        .child(
                            Tab::new()
                                .title("styleable_helpers.rs".to_string())
                                .current(false),
                        ),
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
                            .child(IconButton::new(Icon::Plus))
                            .child(IconButton::new(Icon::Split)),
                    ),
            )
    }
}
