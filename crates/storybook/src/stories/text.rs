use gpui::{
    App, AppContext as _, Context, Entity, HighlightStyle, InteractiveText, IntoElement,
    ParentElement, Render, Styled, StyledText, Window, div, green, red,
};
use indoc::indoc;
use story::*;

pub struct TextStory;

impl TextStory {
    pub fn model(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self)
    }
}

impl Render for TextStory {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Story::container(cx)
            .child(Story::title("Text", cx))
            .children(vec![
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
                        StoryItem::new(
                            "Wrapping Text",
                            div().max_w_96().child(concat!(
                                "The quick brown fox jumps over the lazy dog. ",
                                "Meanwhile, the lazy dog decided it was time for a change. ",
                                "He started daily workout routines, ate healthier and became the fastest dog in town.",
                            )),
                        )
                        .description("Set a width or max-width to enable text wrapping.")
                        .usage(indoc! {r##"
                            div()
                                .max_w_96()
                                .child("Some text that you want to wrap.")
                            "##
                        }),
                    )
                    .child(
                        StoryItem::new(
                            "tbd",
                            div().flex().w_96().child(
                                div().overflow_hidden().child(concat!(
                                    "flex-row. width 96. overflow-hidden. The quick brown fox jumps over the lazy dog. ",
                                    "Meanwhile, the lazy dog decided it was time for a change. ",
                                    "He started daily workout routines, ate healthier and became the fastest dog in town.",
                                )),
                            ),
                        ),
                    )
                    .child(
                        StoryItem::new(
                            "Text in Horizontal Flex",
                            div().flex().w_96().bg(red()).child(concat!(
                                "flex-row. width 96. The quick brown fox jumps over the lazy dog. ",
                                "Meanwhile, the lazy dog decided it was time for a change. ",
                                "He started daily workout routines, ate healthier and became the fastest dog in town.",
                            )),
                        )
                        .usage(indoc! {r##"
                            // NOTE: When rendering text in a horizontal flex container,
                            // Taffy will not pass width constraints down from the parent.
                            // To fix this, render text in a parent with overflow: hidden

                            div()
                                .max_w_96()
                                .child("Some text that you want to wrap.")
                            "##
                        }),
                    )
                    .child(
                        StoryItem::new(
                            "Interactive Text",
                            InteractiveText::new(
                                "interactive",
                                StyledText::new("Hello world, how is it going?").with_default_highlights(
                                    &window.text_style(),
                                    [
                                        (
                                            6..11,
                                            HighlightStyle {
                                                background_color: Some(green()),
                                                ..Default::default()
                                            },
                                        ),
                                    ],
                                ),
                            )
                            .on_click(vec![2..4, 1..3, 7..9], |range_ix, _, _cx| {
                                println!("Clicked range {range_ix}");
                            }),
                        )
                        .usage(indoc! {r##"
                            InteractiveText::new(
                                "interactive",
                                StyledText::new("Hello world, how is it going?").with_highlights(&window.text_style(), [
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
                        }),
                    ),
            ])
            .into_element()
    }
}
