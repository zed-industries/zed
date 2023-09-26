use gpui2::elements::div;
use gpui2::elements::div::ScrollState;
use gpui2::geometry::{relative, rems, Size};
use gpui2::{hsla, Element, IntoElement, ParentElement, ViewContext};

use crate::prelude::*;
use crate::{theme, ChatPanel, CollabPanel, Pane, PaneGroup, SplitDirection, StatusBar, TitleBar};

#[derive(Element, Default)]
pub struct WorkspaceElement {
    left_scroll_state: ScrollState,
    right_scroll_state: ScrollState,
    tab_bar_scroll_state: ScrollState,
}

impl WorkspaceElement {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let temp_size = rems(36.).into();

        let root_group = PaneGroup::new_groups(
            vec![
                PaneGroup::new_panes(
                    vec![
                        Pane::new(
                            ScrollState::default(),
                            Size {
                                width: relative(1.).into(),
                                height: temp_size,
                            },
                        )
                        .fill(hsla(0.6, 0.5, 0.5, 1.)),
                        Pane::new(
                            ScrollState::default(),
                            Size {
                                width: relative(1.).into(),
                                height: temp_size,
                            },
                        )
                        .fill(hsla(0.5, 0.5, 0.5, 1.)),
                    ],
                    SplitDirection::Vertical,
                ),
                PaneGroup::new_groups(
                    vec![
                        PaneGroup::new_panes(
                            vec![Pane::new(
                                ScrollState::default(),
                                Size {
                                    width: relative(1.).into(),
                                    height: temp_size,
                                },
                            )
                            .fill(hsla(0.6, 0.5, 0.5, 1.))],
                            SplitDirection::Horizontal,
                        ),
                        PaneGroup::new_panes(
                            vec![
                                Pane::new(
                                    ScrollState::default(),
                                    Size {
                                        width: relative(1.).into(),
                                        height: temp_size,
                                    },
                                )
                                .fill(hsla(0.3, 0.2, 0.6, 1.)),
                                Pane::new(
                                    ScrollState::default(),
                                    Size {
                                        width: relative(1.).into(),
                                        height: temp_size,
                                    },
                                )
                                .fill(hsla(0.7, 0.2, 0.6, 1.)),
                                Pane::new(
                                    ScrollState::default(),
                                    Size {
                                        width: relative(1.).into(),
                                        height: temp_size,
                                    },
                                )
                                .fill(hsla(0.9, 0.2, 0.6, 1.)),
                                Pane::new(
                                    ScrollState::default(),
                                    Size {
                                        width: relative(1.).into(),
                                        height: temp_size,
                                    },
                                )
                                .fill(hsla(0.7, 0.5, 0.5, 1.)),
                            ],
                            SplitDirection::Horizontal,
                        ),
                    ],
                    SplitDirection::Vertical,
                ),
            ],
            SplitDirection::Horizontal,
        );

        let theme = theme(cx).clone();

        div()
            .size_full()
            .flex()
            .flex_col()
            .font("Zed Sans Extended")
            .gap_0()
            .justify_start()
            .items_start()
            .text_color(theme.lowest.base.default.foreground)
            .fill(theme.lowest.base.default.background)
            .child(TitleBar::new(cx))
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .flex()
                    .flex_row()
                    .overflow_hidden()
                    .child(CollabPanel::new(self.left_scroll_state.clone()))
                    .child(root_group)
                    .child(ChatPanel::new(self.right_scroll_state.clone())),
            )
            .child(StatusBar::new())
    }
}
