use std::marker::PhantomData;

use crate::prelude::*;
use crate::{Icon, IconButton, Tab};

#[derive(Element)]
pub struct TabBar<S: 'static + Send + Sync + Clone> {
    id: ElementId,
    state_type: PhantomData<S>,
    /// Backwards, Forwards
    can_navigate: (bool, bool),
    tabs: Vec<Tab<S>>,
}

impl<S: 'static + Send + Sync + Clone> TabBar<S> {
    pub fn new(id: impl Into<ElementId>, tabs: Vec<Tab<S>>) -> Self {
        Self {
            id: id.into(),
            state_type: PhantomData,
            can_navigate: (false, false),
            tabs,
        }
    }

    pub fn can_navigate(mut self, can_navigate: (bool, bool)) -> Self {
        self.can_navigate = can_navigate;
        self
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<S> {
        let theme = theme(cx);

        let (can_navigate_back, can_navigate_forward) = self.can_navigate;

        div()
            .id(self.id.clone())
            .w_full()
            .flex()
            .bg(theme.tab_bar)
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
                                IconButton::new("arrow_left", Icon::ArrowLeft)
                                    .state(InteractionState::Enabled.if_enabled(can_navigate_back)),
                            )
                            .child(
                                IconButton::new("arrow_right", Icon::ArrowRight).state(
                                    InteractionState::Enabled.if_enabled(can_navigate_forward),
                                ),
                            ),
                    ),
            )
            .child(
                div().w_0().flex_1().h_full().child(
                    div()
                        .id("tabs")
                        .flex()
                        .overflow_x_scroll()
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
                            .child(IconButton::new("plus", Icon::Plus))
                            .child(IconButton::new("split", Icon::Split)),
                    ),
            )
    }
}

use gpui2::ElementId;
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

        fn render(
            &mut self,
            _view: &mut S,
            cx: &mut ViewContext<S>,
        ) -> impl Element<S> {
            Story::container(cx)
                .child(Story::title_for::<_, TabBar<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(TabBar::new(
                    "tab-bar",
                    vec![
                        Tab::new(1)
                            .title("Cargo.toml".to_string())
                            .current(false)
                            .git_status(GitStatus::Modified),
                        Tab::new(2)
                            .title("Channels Panel".to_string())
                            .current(false),
                        Tab::new(3)
                            .title("channels_panel.rs".to_string())
                            .current(true)
                            .git_status(GitStatus::Modified),
                        Tab::new(4)
                            .title("workspace.rs".to_string())
                            .current(false)
                            .git_status(GitStatus::Modified),
                        Tab::new(5)
                            .title("icon_button.rs".to_string())
                            .current(false),
                        Tab::new(6)
                            .title("storybook.rs".to_string())
                            .current(false)
                            .git_status(GitStatus::Created),
                        Tab::new(7).title("theme.rs".to_string()).current(false),
                        Tab::new(8)
                            .title("theme_registry.rs".to_string())
                            .current(false),
                        Tab::new(9)
                            .title("styleable_helpers.rs".to_string())
                            .current(false),
                    ],
                ))
        }
    }
}
