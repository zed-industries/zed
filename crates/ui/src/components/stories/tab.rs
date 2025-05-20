use std::cmp::Ordering;

use gpui::Render;
use story::Story;

use crate::{IconButtonShape, TabPosition, prelude::*};
use crate::{Indicator, Tab};

pub struct TabStory;

impl Render for TabStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Story::container(cx)
            .child(Story::title_for::<Tab>(cx))
            .child(Story::label("Default", cx))
            .child(h_flex().child(Tab::new("tab_1").child("Tab 1")))
            .child(Story::label("With indicator", cx))
            .child(
                h_flex().child(
                    Tab::new("tab_1")
                        .start_slot(Indicator::dot().color(Color::Warning))
                        .child("Tab 1"),
                ),
            )
            .child(Story::label("With close button", cx))
            .child(
                h_flex().child(
                    Tab::new("tab_1")
                        .end_slot(
                            IconButton::new("close_button", IconName::Close)
                                .visible_on_hover("")
                                .shape(IconButtonShape::Square)
                                .icon_color(Color::Muted)
                                .size(ButtonSize::None)
                                .icon_size(IconSize::XSmall),
                        )
                        .child("Tab 1"),
                ),
            )
            .child(Story::label("List of tabs", cx))
            .child(
                h_flex()
                    .child(Tab::new("tab_1").child("Tab 1"))
                    .child(Tab::new("tab_2").child("Tab 2")),
            )
            .child(Story::label("List of tabs with first tab selected", cx))
            .child(
                h_flex()
                    .child(
                        Tab::new("tab_1")
                            .toggle_state(true)
                            .position(TabPosition::First)
                            .child("Tab 1"),
                    )
                    .child(
                        Tab::new("tab_2")
                            .position(TabPosition::Middle(Ordering::Greater))
                            .child("Tab 2"),
                    )
                    .child(
                        Tab::new("tab_3")
                            .position(TabPosition::Middle(Ordering::Greater))
                            .child("Tab 3"),
                    )
                    .child(Tab::new("tab_4").position(TabPosition::Last).child("Tab 4")),
            )
            .child(Story::label("List of tabs with last tab selected", cx))
            .child(
                h_flex()
                    .child(
                        Tab::new("tab_1")
                            .position(TabPosition::First)
                            .child("Tab 1"),
                    )
                    .child(
                        Tab::new("tab_2")
                            .position(TabPosition::Middle(Ordering::Less))
                            .child("Tab 2"),
                    )
                    .child(
                        Tab::new("tab_3")
                            .position(TabPosition::Middle(Ordering::Less))
                            .child("Tab 3"),
                    )
                    .child(
                        Tab::new("tab_4")
                            .position(TabPosition::Last)
                            .toggle_state(true)
                            .child("Tab 4"),
                    ),
            )
            .child(Story::label("List of tabs with second tab selected", cx))
            .child(
                h_flex()
                    .child(
                        Tab::new("tab_1")
                            .position(TabPosition::First)
                            .child("Tab 1"),
                    )
                    .child(
                        Tab::new("tab_2")
                            .position(TabPosition::Middle(Ordering::Equal))
                            .toggle_state(true)
                            .child("Tab 2"),
                    )
                    .child(
                        Tab::new("tab_3")
                            .position(TabPosition::Middle(Ordering::Greater))
                            .child("Tab 3"),
                    )
                    .child(Tab::new("tab_4").position(TabPosition::Last).child("Tab 4")),
            )
    }
}
