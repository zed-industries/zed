use gpui::{
    div, green, red, HighlightStyle, InteractiveText, IntoElement, ParentElement, Render, Styled,
    StyledText, View, VisualContext, WindowContext,
};
use indoc::indoc;
use story::*;

pub struct TextStory;

impl TextStory {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self)
    }
}

impl Render for TextStory {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        StoryContainer::new("Text Story", "crates/storybook/src/stories/text.rs")
            .children(
                vec![

            StorySection::new()
                .child(
                    StoryItem::new("Default", div().bg(gpui::blue()).child("Hello World!"))
                        .usage(indoc! {r##"
                            div()
                                .child("Hello World!")
                            "##
                        }),
                )
                .child(
                    StoryItem::new("Wrapping Text",
                        div().max_w_96()
                            .child(
                                concat!(
                                    "The quick brown fox jumps over the lazy dog. ",
                                    "Meanwhile, the lazy dog decided it was time for a change. ",
                                    "He started daily workout routines, ate healthier and became the fastest dog in town.",
                                )
                            )
                    )
                    .description("Set a width or max-width to enable text wrapping.")
                    .usage(indoc! {r##"
                        div()
                            .max_w_96()
                            .child("Some text that you want to wrap.")
                        "##
                    })
                )
                .child(
                    StoryItem::new("tbd",
                    div().flex().w_96().child(div().overflow_hidden().child(concat!(
                            "flex-row. width 96. overflow-hidden. The quick brown fox jumps over the lazy dog. ",
                            "Meanwhile, the lazy dog decided it was time for a change. ",
                            "He started daily workout routines, ate healthier and became the fastest dog in town.",
                        )))
                    )
                )
                .child(
                    StoryItem::new("Text in Horizontal Flex",
                        div().flex().w_96().bg(red()).child(concat!(
                                        "flex-row. width 96. The quick brown fox jumps over the lazy dog. ",
                                        "Meanwhile, the lazy dog decided it was time for a change. ",
                                        "He started daily workout routines, ate healthier and became the fastest dog in town.",
                                    ))
                    )
                    .usage(indoc! {r##"
                        // NOTE: When rendering text in a horizontal flex container,
                        // Taffy will not pass width constraints down from the parent.
                        // To fix this, render text in a parent with overflow: hidden

                        div()
                            .max_w_96()
                            .child("Some text that you want to wrap.")
                        "##
                    })
                )
                .child(
                    StoryItem::new("Interactive Text",
                        InteractiveText::new(
                            "interactive",
                            StyledText::new("Hello world, how is it going?").with_highlights(&cx.text_style(), [
                                (6..11, HighlightStyle {
                                    background_color: Some(green()),
                                    ..Default::default()
                                }),
                            ]),
                        )
                        .on_click(vec![2..4, 1..3, 7..9], |range_ix, _cx| {
                            println!("Clicked range {range_ix}");
                        })
                    )
                    .usage(indoc! {r##"
                        InteractiveText::new(
                            "interactive",
                            StyledText::new("Hello world, how is it going?").with_highlights(&cx.text_style(), [
                                (6..11, HighlightStyle {
                                    background_color: Some(green()),
                                    ..Default::default()
                                }),
                            ]),
                        )
                        .on_click(vec![2..4, 1..3, 7..9], |range_ix, _cx| {
                            println!("Clicked range {range_ix}");
                        })
                        "##
                    })
                )
        ]
            ).into_element()
    }
}
