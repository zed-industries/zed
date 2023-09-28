use chrono::DateTime;
use gpui2::geometry::{relative, rems, Size};

use crate::{
    prelude::*, v_stack, ChatMessage, ChatPanel, Panel, PanelAllowedSides, PanelSide, ProjectPanel,
    Terminal,
};
use crate::{theme, Pane, PaneGroup, SplitDirection, StatusBar, TitleBar};

#[derive(Element, Default)]
pub struct WorkspaceElement {
    left_panel_scroll_state: ScrollState,
    right_panel_scroll_state: ScrollState,
    tab_bar_scroll_state: ScrollState,
    bottom_panel_scroll_state: ScrollState,
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
                            |_, _| vec![Terminal::new().into_any()],
                            Box::new(()),
                        ),
                        Pane::new(
                            ScrollState::default(),
                            Size {
                                width: relative(1.).into(),
                                height: temp_size,
                            },
                            |_, _| vec![Terminal::new().into_any()],
                            Box::new(()),
                        ),
                    ],
                    SplitDirection::Vertical,
                ),
                PaneGroup::new_panes(
                    vec![Pane::new(
                        ScrollState::default(),
                        Size {
                            width: relative(1.).into(),
                            height: relative(1.).into(),
                        },
                        |_, _| vec![Terminal::new().into_any()],
                        Box::new(()),
                    )],
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
                    .border_t()
                    .border_b()
                    .border_color(theme.lowest.base.default.border)
                    .child(
                        ProjectPanel::new(self.left_panel_scroll_state.clone())
                            .side(PanelSide::Left),
                    )
                    .child(
                        v_stack()
                            .flex_1()
                            .h_full()
                            .child(
                                div()
                                    .flex()
                                    .flex_1()
                                    // CSS Hack: Flex 1 has to have a set height to properly fill the space
                                    // Or it will give you a height of 0
                                    .h_px()
                                    .child(root_group),
                            )
                            .child(
                                Panel::new(
                                    self.bottom_panel_scroll_state.clone(),
                                    |_, _| vec![Terminal::new().into_any()],
                                    Box::new(()),
                                )
                                .allowed_sides(PanelAllowedSides::BottomOnly)
                                .side(PanelSide::Bottom),
                            ),
                    )
                    .child(ChatPanel::new(ScrollState::default()).with_messages(vec![
                                ChatMessage::new(
                                    "osiewicz".to_string(),
                                    "is this thing on?".to_string(),
                                    DateTime::parse_from_rfc3339(
                                        "2023-09-27T15:40:52.707Z",
                                    )
                                    .unwrap()
                                    .naive_local(),
                                ),
                                ChatMessage::new(
                                    "maxdeviant".to_string(),
                                    "Reading you loud and clear!".to_string(),
                                    DateTime::parse_from_rfc3339(
                                        "2023-09-28T15:40:52.707Z",
                                    )
                                    .unwrap()
                                    .naive_local(),
                                ),
                            ])),
            )
            .child(StatusBar::new())
    }
}
