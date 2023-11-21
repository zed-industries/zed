use gpui::{Div, Render};

use crate::prelude::*;
use crate::{HighlightedLabel, Label, Story};

pub struct LabelStory;

impl Render for LabelStory {
    type Element = Div;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
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
    }
}
