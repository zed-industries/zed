use crate::{prelude::*, HighlightedLabel, Label};
use gpui::Render;
use story::Story;

pub struct LabelStory;

impl Render for LabelStory {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        Story::container()
            .child(Story::title_for::<Label>())
            .child(Story::label("Default"))
            .child(Label::new("Hello, world!"))
            .child(Story::label("Highlighted"))
            .child(HighlightedLabel::new(
                "Hello, world!",
                vec![0, 1, 2, 7, 8, 12],
            ))
            .child(HighlightedLabel::new(
                "Héllo, world!",
                vec![0, 1, 3, 8, 9, 13],
            ))
            .child(Story::label("Highlighted with `color`"))
            .child(
                HighlightedLabel::new("Hello, world!", vec![0, 1, 2, 7, 8, 12]).color(Color::Error),
            )
    }
}
