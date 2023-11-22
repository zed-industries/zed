use gpui::{Div, Render};
use story::Story;

use crate::prelude::*;
use crate::{HighlightedLabel, Label};

pub struct LabelStory;

impl Render for LabelStory {
    type Output = Div;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Output {
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
    }
}
