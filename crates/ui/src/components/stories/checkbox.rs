use gpui::{Render, ViewContext};
use story::Story;

use crate::prelude::*;
use crate::{h_flex, Checkbox};

pub struct CheckboxStory;

impl Render for CheckboxStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        Story::container()
            .child(Story::title_for::<Checkbox>())
            .child(Story::label("Default"))
            .child(
                h_flex()
                    .p_2()
                    .gap_2()
                    .rounded_md()
                    .border()
                    .border_color(cx.theme().colors().border)
                    .child(Checkbox::new("checkbox-enabled", Selection::Unselected))
                    .child(Checkbox::new(
                        "checkbox-intermediate",
                        Selection::Indeterminate,
                    ))
                    .child(Checkbox::new("checkbox-selected", Selection::Selected)),
            )
            .child(Story::label("Disabled"))
            .child(
                h_flex()
                    .p_2()
                    .gap_2()
                    .rounded_md()
                    .border()
                    .border_color(cx.theme().colors().border)
                    .child(Checkbox::new("checkbox-disabled", Selection::Unselected).disabled(true))
                    .child(
                        Checkbox::new("checkbox-disabled-intermediate", Selection::Indeterminate)
                            .disabled(true),
                    )
                    .child(
                        Checkbox::new("checkbox-disabled-selected", Selection::Selected)
                            .disabled(true),
                    ),
            )
    }
}
