use std::time::Duration;

use crate::{prelude::*, HighlightedLabel, Label};
use gpui::{pulsating_between, Animation, AnimationExt, Render};
use story::Story;

pub struct LabelStory;

impl Render for LabelStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        Story::container(cx)
            .child(Story::title_for::<Label>(cx))
            .child(Story::label(cx, "Default"))
            .child(Label::new("Hello, world!"))
            .child(Story::label(cx, "Highlighted"))
            .child(HighlightedLabel::new(
                "Hello, world!",
                vec![0, 1, 2, 7, 8, 12],
            ))
            .child(HighlightedLabel::new(
                "Héllo, world!",
                vec![0, 1, 3, 8, 9, 13],
            ))
            .child(Story::label(cx, "Highlighted with `color`"))
            .child(
                HighlightedLabel::new("Hello, world!", vec![0, 1, 2, 7, 8, 12]).color(Color::Error),
            )
            .child(
                Label::new("This text is pulsating").with_animation(
                    "pulsating-label",
                    Animation::new(Duration::from_secs(2))
                        .repeat()
                        .with_easing(pulsating_between(0.4, 0.8)),
                    |label, delta| label.alpha(delta),
                ),
            )
    }
}
