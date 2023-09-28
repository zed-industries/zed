use gpui2::geometry::{relative, rems, Size};
use gpui2::hsla;

use crate::{prelude::*, v_stack, Icon, IconElement, Panel, PanelAllowedSides, PanelSide};
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
                        ),
                        Pane::new(
                            ScrollState::default(),
                            Size {
                                width: relative(1.).into(),
                                height: temp_size,
                            },
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
                        Panel::new(
                            self.left_panel_scroll_state.clone(),
                            vec![IconElement::new(Icon::ExclamationTriangle).into_any()],
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
                                    .h_px()
                                    .child(root_group),
                            )
                            .child(
                                Panel::new(
                                    self.bottom_panel_scroll_state.clone(),
                                    vec![IconElement::new(Icon::MagicWand).into_any()],
                                )
                                .allowed_sides(PanelAllowedSides::BottomOnly)
                                .side(PanelSide::Bottom),
                            ),
                    )
                    .child(
                        Panel::new(
                            self.right_panel_scroll_state.clone(),
                            vec![IconElement::new(Icon::Ai).into_any()],
                        )
                        .side(PanelSide::Right),
                    ),
            )
            .child(StatusBar::new())
    }
}
