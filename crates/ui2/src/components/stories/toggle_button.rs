use gpui::{Component, Render};
use story::{StoryContainer, StoryItem, StorySection};

use crate::{prelude::*, ToggleButton};

pub struct ToggleButtonStory;

impl Render for ToggleButtonStory {
    type Element = Component<StoryContainer>;

    fn render(&mut self, _cx: &mut ViewContext<Self>) -> Self::Element {
        StoryContainer::new(
            "Toggle Button",
            "crates/ui2/src/components/stories/toggle_button.rs",
        )
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
                    h_stack()
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
                    h_stack()
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
                                .selected(true)
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
