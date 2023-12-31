use gpui::{Div, Render};
use story::Story;

use crate::prelude::*;
use crate::Avatar;

pub struct AvatarStory;

impl Render for AvatarStory {
    type Output = Div;

    fn render(&mut self, _cx: &mut ViewContext<Self>) -> Self::Output {
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
                    .availability_indicator(true),
            )
            .child(
                Avatar::new("https://avatars.githubusercontent.com/u/326587?v=4")
                    .availability_indicator(false),
            )
    }
}
