use std::marker::PhantomData;
use std::sync::Arc;

use chrono::DateTime;
use gpui3::{relative, rems, Size};

use crate::prelude::*;
use crate::{theme, v_stack, Panel, PanelAllowedSides, PanelSide, ProjectPanel, StatusBar};

#[derive(Element)]
pub struct WorkspaceElement<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
    left_panel_scroll_state: ScrollState,
    right_panel_scroll_state: ScrollState,
    tab_bar_scroll_state: ScrollState,
    bottom_panel_scroll_state: ScrollState,
}

impl<S: 'static + Send + Sync + Clone> WorkspaceElement<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
            left_panel_scroll_state: ScrollState::default(),
            right_panel_scroll_state: ScrollState::default(),
            tab_bar_scroll_state: ScrollState::default(),
            bottom_panel_scroll_state: ScrollState::default(),
        }
    }

    pub fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        let theme = theme(cx).clone();

        // let temp_size = rems(36.).into();

        // let root_group = PaneGroup::new_groups(
        //     vec![
        //         PaneGroup::new_panes(
        //             vec![
        //                 Pane::new(
        //                     ScrollState::default(),
        //                     Size {
        //                         width: relative(1.).into(),
        //                         height: temp_size,
        //                     },
        //                     |_, payload| {
        //                         let theme = payload.downcast_ref::<Arc<Theme>>().unwrap();

        //                         vec![EditorPane::new(hello_world_rust_editor_with_status_example(
        //                             &theme,
        //                         ))
        //                         .into_any()]
        //                     },
        //                     Box::new(theme.clone()),
        //                 ),
        //                 Pane::new(
        //                     ScrollState::default(),
        //                     Size {
        //                         width: relative(1.).into(),
        //                         height: temp_size,
        //                     },
        //                     |_, _| vec![Terminal::new().into_any()],
        //                     Box::new(()),
        //                 ),
        //             ],
        //             SplitDirection::Vertical,
        //         ),
        //         PaneGroup::new_panes(
        //             vec![Pane::new(
        //                 ScrollState::default(),
        //                 Size {
        //                     width: relative(1.).into(),
        //                     height: relative(1.).into(),
        //                 },
        //                 |_, payload| {
        //                     let theme = payload.downcast_ref::<Arc<Theme>>().unwrap();

        //                     vec![EditorPane::new(hello_world_rust_editor_with_status_example(
        //                         &theme,
        //                     ))
        //                     .into_any()]
        //                 },
        //                 Box::new(theme.clone()),
        //             )],
        //             SplitDirection::Vertical,
        //         ),
        //     ],
        //     SplitDirection::Horizontal,
        // );

        div()
            .relative()
            .size_full()
            .flex()
            .flex_col()
            .font("Zed Sans Extended")
            .gap_0()
            .justify_start()
            .items_start()
            .text_color(theme.lowest.base.default.foreground)
            .fill(theme.lowest.base.default.background)
            // .child(TitleBar::new(cx).set_livestream(Some(Livestream {
            //     players: random_players_with_call_status(7),
            //     channel: Some("gpui2-ui".to_string()),
            // })))
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
                        Panel::new(
                            self.left_panel_scroll_state.clone(),
                            |_, payload| vec![ProjectPanel::new(ScrollState::default()).into_any()],
                            Box::new(()),
                        )
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
                                    .h_px(), // .child(root_group),
                            )
                            .child(
                                Panel::new(
                                    self.bottom_panel_scroll_state.clone(),
                                    |_, _| {
                                        vec![
                                            // Terminal::new().into_any()
                                        ]
                                    },
                                    Box::new(()),
                                )
                                .allowed_sides(PanelAllowedSides::BottomOnly)
                                .side(PanelSide::Bottom),
                            ),
                    )
                    .child(
                        Panel::new(
                            self.right_panel_scroll_state.clone(),
                            |_, payload| vec![ProjectPanel::new(ScrollState::default()).into_any()],
                            Box::new(()),
                        )
                        .side(PanelSide::Right),
                    ), // .child(
                       //     Panel::new(
                       //         self.right_panel_scroll_state.clone(),
                       //         |_, payload| {
                       //             vec![ChatPanel::new(ScrollState::default())
                       //                 .with_messages(vec![
                       //                     ChatMessage::new(
                       //                         "osiewicz".to_string(),
                       //                         "is this thing on?".to_string(),
                       //                         DateTime::parse_from_rfc3339(
                       //                             "2023-09-27T15:40:52.707Z",
                       //                         )
                       //                         .unwrap()
                       //                         .naive_local(),
                       //                     ),
                       //                     ChatMessage::new(
                       //                         "maxdeviant".to_string(),
                       //                         "Reading you loud and clear!".to_string(),
                       //                         DateTime::parse_from_rfc3339(
                       //                             "2023-09-28T15:40:52.707Z",
                       //                         )
                       //                         .unwrap()
                       //                         .naive_local(),
                       //                     ),
                       //                 ])
                       //                 .into_any()]
                       //         },
                       //         Box::new(()),
                       //     )
                       //     .side(PanelSide::Right),
                       // ),
            )
            .child(StatusBar::new())
        // An example of a toast is below
        // Currently because of stacking order this gets obscured by other elements

        // .child(Toast::new(
        //     ToastOrigin::Bottom,
        //     |_, payload| {
        //         let theme = payload.downcast_ref::<Arc<Theme>>().unwrap();

        //         vec![Label::new("label").into_any()]
        //     },
        //     Box::new(theme.clone()),
        // ))
    }
}
