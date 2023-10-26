use std::marker::PhantomData;

use gpui2::{relative, rems, Size};

use crate::prelude::*;
use crate::{Icon, IconButton, Pane, Tab};

#[derive(Element)]
pub struct Terminal<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync + Clone> Terminal<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
        }
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl IntoAnyElement<S> {
        let theme = theme(cx);

        let can_navigate_back = true;
        let can_navigate_forward = false;

        div()
            .flex()
            .flex_col()
            .w_full()
            .child(
                // Terminal Tabs.
                div()
                    .w_full()
                    .flex()
                    .bg(theme.surface)
                    .child(
                        div().px_1().flex().flex_none().gap_2().child(
                            div()
                                .flex()
                                .items_center()
                                .gap_px()
                                .child(
                                    IconButton::new("arrow_left", Icon::ArrowLeft).state(
                                        InteractionState::Enabled.if_enabled(can_navigate_back),
                                    ),
                                )
                                .child(IconButton::new("arrow_right", Icon::ArrowRight).state(
                                    InteractionState::Enabled.if_enabled(can_navigate_forward),
                                )),
                        ),
                    )
                    .child(
                        div().w_0().flex_1().h_full().child(
                            div()
                                .flex()
                                .child(
                                    Tab::new(1)
                                        .title("zed — fish".to_string())
                                        .icon(Icon::Terminal)
                                        .close_side(IconSide::Right)
                                        .current(true),
                                )
                                .child(
                                    Tab::new(2)
                                        .title("zed — fish".to_string())
                                        .icon(Icon::Terminal)
                                        .close_side(IconSide::Right)
                                        .current(false),
                                ),
                        ),
                    ),
            )
            // Terminal Pane.
            .child(
                Pane::new(
                    "terminal",
                    Size {
                        width: relative(1.).into(),
                        height: rems(36.).into(),
                    },
                )
                .child(crate::static_data::terminal_buffer(&theme)),
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
    pub struct TerminalStory<S: 'static + Send + Sync + Clone> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync + Clone> TerminalStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl IntoAnyElement<S> {
            Story::container(cx)
                .child(Story::title_for::<_, Terminal<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(Terminal::new())
        }
    }
}
