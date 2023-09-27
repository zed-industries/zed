use gpui2::geometry::{relative, rems, Size};

use crate::{prelude::*, Pane};
use crate::{theme, Icon, IconButton, Tab};

#[derive(Element)]
pub struct Terminal {}

impl Terminal {
    pub fn new() -> Self {
        Self {}
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        let can_navigate_back = true;
        let can_navigate_forward = false;

        div()
            .flex()
            .flex_col()
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
                                // .overflow_x_scroll(self.scroll_state.clone())
                                .child(
                                    Tab::new()
                                        .title("zed — fish")
                                        .icon(Icon::Terminal)
                                        .close_side(IconSide::Right)
                                        .current(true),
                                )
                                .child(
                                    Tab::new()
                                        .title("zed — fish")
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
            ))
    }
}
