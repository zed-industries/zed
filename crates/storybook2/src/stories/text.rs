use gpui::{
    blue, div, red, white, Div, ParentComponent, Render, Styled, View, VisualContext, WindowContext,
};
use ui::v_stack;

pub struct TextStory;

impl TextStory {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.build_view(|cx| Self)
    }
}

impl Render for TextStory {
    type Element = Div<Self>;

    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> Self::Element {
        v_stack()
            .bg(blue())
            .child(
                div()
                    .flex()
                    .child(div().max_w_96().bg(white()).child(concat!(
        "max-width: 96. The quick brown fox jumps over the lazy dog. ",
        "Meanwhile, the lazy dog decided it was time for a change. ",
        "He started daily workout routines, ate healthier and became the fastest dog in town.",
    ))),
            )
            .child(div().h_5())
            .child(div().flex().flex_col().w_96().bg(white()).child(concat!(
        "flex-col. width: 96; The quick brown fox jumps over the lazy dog. ",
        "Meanwhile, the lazy dog decided it was time for a change. ",
        "He started daily workout routines, ate healthier and became the fastest dog in town.",
    )))
            .child(div().h_5())
            .child(
                div()
                    .flex()
                    .child(div().min_w_96().bg(white()).child(concat!(
    "min-width: 96. The quick brown fox jumps over the lazy dog. ",
    "Meanwhile, the lazy dog decided it was time for a change. ",
    "He started daily workout routines, ate healthier and became the fastest dog in town.",
))))
            .child(div().h_5())
            .child(div().flex().w_96().bg(white()).child(div().overflow_hidden().child(concat!(
        "flex-row. width 96. overflow-hidden. The quick brown fox jumps over the lazy dog. ",
        "Meanwhile, the lazy dog decided it was time for a change. ",
        "He started daily workout routines, ate healthier and became the fastest dog in town.",
    ))))
            // NOTE: When rendering text in a horizonal flex container,
            // Taffy will not pass width constraints down from the parent.
            // To fix this, render text in a praent with overflow: hidden, which
                    .child(div().h_5())
                    .child(div().flex().w_96().bg(red()).child(concat!(
                "flex-row. width 96. The quick brown fox jumps over the lazy dog. ",
                "Meanwhile, the lazy dog decided it was time for a change. ",
                "He started daily workout routines, ate healthier and became the fastest dog in town.",
            )))
    }
}
