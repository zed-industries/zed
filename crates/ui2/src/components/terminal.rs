use std::marker::PhantomData;
use std::sync::Arc;

use gpui3::{relative, rems, Size};

use crate::prelude::*;
use crate::{theme, Icon, IconButton, Pane, Tab};

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

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
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
                    .fill(theme.middle.base.default.background)
                    .child(
                        div().px_1().flex().flex_none().gap_2().child(
                            div()
                                .flex()
                                .items_center()
                                .gap_px()
                                .child(
                                    IconButton::new(Icon::ArrowLeft).state(
                                        InteractionState::Enabled.if_enabled(can_navigate_back),
                                    ),
                                )
                                .child(IconButton::new(Icon::ArrowRight).state(
                                    InteractionState::Enabled.if_enabled(can_navigate_forward),
                                )),
                        ),
                    )
                    .child(
                        div().w_0().flex_1().h_full().child(
                            div()
                                .flex()
                                .child(
                                    Tab::new()
                                        .title("zed — fish".to_string())
                                        .icon(Icon::Terminal)
                                        .close_side(IconSide::Right)
                                        .current(true),
                                )
                                .child(
                                    Tab::new()
                                        .title("zed — fish".to_string())
                                        .icon(Icon::Terminal)
                                        .close_side(IconSide::Right)
                                        .current(false),
                                ),
                        ),
                    ),
            )
            // Terminal Pane.
            .child(Pane::new(
                ScrollState::default(),
                Size {
                    width: relative(1.).into(),
                    height: rems(36.).into(),
                },
                |_, payload| {
                    let theme = payload.downcast_ref::<Arc<Theme>>().unwrap();

                    vec![crate::static_data::terminal_buffer(&theme).into_any()]
                },
                Box::new(theme),
            ))
    }
}
