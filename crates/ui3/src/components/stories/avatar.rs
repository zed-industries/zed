use gpui::Render;
use story::{Story, StoryItem, StorySection};

use crate::{prelude::*, AudioStatus, Availability, AvatarAvailabilityIndicator};
use crate::{Avatar, AvatarAudioStatusIndicator};

pub struct AvatarStory;

impl Render for AvatarStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        Story::container()
            .child(Story::title_for::<Avatar>())
            .child(
                StorySection::new()
                    .child(StoryItem::new(
                        "Default",
                        Avatar::new("https://avatars.githubusercontent.com/u/1714999?v=4"),
                    ))
                    .child(StoryItem::new(
                        "Default",
                        Avatar::new("https://avatars.githubusercontent.com/u/326587?v=4"),
                    )),
            )
            .child(
                StorySection::new()
                    .child(StoryItem::new(
                        "With free availability indicator",
                        Avatar::new("https://avatars.githubusercontent.com/u/326587?v=4")
                            .indicator(AvatarAvailabilityIndicator::new(Availability::Free)),
                    ))
                    .child(StoryItem::new(
                        "With busy availability indicator",
                        Avatar::new("https://avatars.githubusercontent.com/u/326587?v=4")
                            .indicator(AvatarAvailabilityIndicator::new(Availability::Busy)),
                    )),
            )
            .child(
                StorySection::new()
                    .child(StoryItem::new(
                        "With info border",
                        Avatar::new("https://avatars.githubusercontent.com/u/326587?v=4")
                            .border_color(cx.theme().status().info_border),
                    ))
                    .child(StoryItem::new(
                        "With error border",
                        Avatar::new("https://avatars.githubusercontent.com/u/326587?v=4")
                            .border_color(cx.theme().status().error_border),
                    )),
            )
            .child(
                StorySection::new()
                    .child(StoryItem::new(
                        "With muted audio indicator",
                        Avatar::new("https://avatars.githubusercontent.com/u/326587?v=4")
                            .indicator(AvatarAudioStatusIndicator::new(AudioStatus::Muted)),
                    ))
                    .child(StoryItem::new(
                        "With deafened audio indicator",
                        Avatar::new("https://avatars.githubusercontent.com/u/326587?v=4")
                            .indicator(AvatarAudioStatusIndicator::new(AudioStatus::Deafened)),
                    )),
            )
    }
}
