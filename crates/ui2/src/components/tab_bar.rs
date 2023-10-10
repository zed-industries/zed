use std::marker::PhantomData;

use crate::prelude::*;
use crate::{theme, Icon, IconButton, Tab};

#[derive(Element)]
pub struct TabBar<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
    scroll_state: ScrollState,
    tabs: Vec<Tab<S>>,
}

impl<S: 'static + Send + Sync + Clone> TabBar<S> {
    pub fn new(tabs: Vec<Tab<S>>) -> Self {
        Self {
            state_type: PhantomData,
            scroll_state: ScrollState::default(),
            tabs,
        }
    }

    pub fn bind_scroll_state(&mut self, scroll_state: ScrollState) {
        self.scroll_state = scroll_state;
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
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
                        .children(self.tabs.clone()),
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

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use crate::Story;

    use super::*;

    #[derive(Element)]
    pub struct TabBarStory<S: 'static + Send + Sync + Clone> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync + Clone> TabBarStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
            Story::container(cx)
                .child(Story::title_for::<_, TabBar<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(TabBar::new(vec![
                    Tab::new()
                        .title("Cargo.toml".to_string())
                        .current(false)
                        .git_status(GitStatus::Modified),
                    Tab::new()
                        .title("Channels Panel".to_string())
                        .current(false),
                    Tab::new()
                        .title("channels_panel.rs".to_string())
                        .current(true)
                        .git_status(GitStatus::Modified),
                    Tab::new()
                        .title("workspace.rs".to_string())
                        .current(false)
                        .git_status(GitStatus::Modified),
                    Tab::new()
                        .title("icon_button.rs".to_string())
                        .current(false),
                    Tab::new()
                        .title("storybook.rs".to_string())
                        .current(false)
                        .git_status(GitStatus::Created),
                    Tab::new().title("theme.rs".to_string()).current(false),
                    Tab::new()
                        .title("theme_registry.rs".to_string())
                        .current(false),
                    Tab::new()
                        .title("styleable_helpers.rs".to_string())
                        .current(false),
                ]))
        }
    }
}
