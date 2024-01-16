use gpui::Render;
use story::Story;

use crate::{prelude::*, AudioStatus, Availability, AvatarAvailabilityIndicator};
use crate::{Avatar, AvatarAudioStatusIndicator};

pub struct AvatarStory;

impl Render for AvatarStory {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        Story::container()
            .child(Story::title_for::<Avatar>())
            .child(Story::label("Default"))
            .child(Avatar::new(
                "https://avatars.githubusercontent.com/u/1714999?v=4",
            ))
            .child(Avatar::new(
                "https://avatars.githubusercontent.com/u/326587?v=4",
            ))
            .child(
                Avatar::new("https://avatars.githubusercontent.com/u/326587?v=4")
                    .indicator(AvatarAvailabilityIndicator::new(Availability::Free)),
            )
            .child(
                Avatar::new("https://avatars.githubusercontent.com/u/326587?v=4")
                    .indicator(AvatarAvailabilityIndicator::new(Availability::Busy)),
            )
            .child(
                Avatar::new("https://avatars.githubusercontent.com/u/326587?v=4")
                    .indicator(AvatarAudioStatusIndicator::new(AudioStatus::Muted)),
            )
            .child(
                Avatar::new("https://avatars.githubusercontent.com/u/326587?v=4")
                    .indicator(AvatarAudioStatusIndicator::new(AudioStatus::Deafened)),
            )
    }
}
