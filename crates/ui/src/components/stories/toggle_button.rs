use gpui::Render;
use story::{Story, StoryItem, StorySection};

use crate::{ToggleButton, prelude::*};

pub struct ToggleButtonStory;

impl Render for ToggleButtonStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Story::container(cx)
            .child(Story::title_for::<ToggleButton>(cx))
            .child(
                StorySection::new().child(
                    StoryItem::new(
                        "Default",
                        ToggleButton::new("default_toggle_button", "Hello"),
                    )
                    .description("Displays a toggle button.")
                    .usage(""),
                ),
            )
            .child(
                StorySection::new().child(
                    StoryItem::new(
                        "Toggle button group",
                        h_flex()
                            .child(
                                ToggleButton::new(1, "Apple")
                                    .style(ButtonStyle::Filled)
                                    .size(ButtonSize::Large)
                                    .first(),
                            )
                            .child(
                                ToggleButton::new(2, "Banana")
                                    .style(ButtonStyle::Filled)
                                    .size(ButtonSize::Large)
                                    .middle(),
                            )
                            .child(
                                ToggleButton::new(3, "Cherry")
                                    .style(ButtonStyle::Filled)
                                    .size(ButtonSize::Large)
                                    .middle(),
                            )
                            .child(
                                ToggleButton::new(4, "Dragonfruit")
                                    .style(ButtonStyle::Filled)
                                    .size(ButtonSize::Large)
                                    .last(),
                            ),
                    )
                    .description("Displays a group of toggle buttons.")
                    .usage(""),
                ),
            )
            .child(
                StorySection::new().child(
                    StoryItem::new(
                        "Toggle button group with selection",
                        h_flex()
                            .child(
                                ToggleButton::new(1, "Apple")
                                    .style(ButtonStyle::Filled)
                                    .size(ButtonSize::Large)
                                    .first(),
                            )
                            .child(
                                ToggleButton::new(2, "Banana")
                                    .style(ButtonStyle::Filled)
                                    .size(ButtonSize::Large)
                                    .toggle_state(true)
                                    .middle(),
                            )
                            .child(
                                ToggleButton::new(3, "Cherry")
                                    .style(ButtonStyle::Filled)
                                    .size(ButtonSize::Large)
                                    .middle(),
                            )
                            .child(
                                ToggleButton::new(4, "Dragonfruit")
                                    .style(ButtonStyle::Filled)
                                    .size(ButtonSize::Large)
                                    .last(),
                            ),
                    )
                    .description("Displays a group of toggle buttons.")
                    .usage(""),
                ),
            )
            .into_element()
    }
}
