use gpui::{
    div, white, Div, ParentElement, Render, Styled, View, VisualContext, WindowContext,
};

pub struct TextStory;

impl TextStory {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.build_view(|cx| Self)
    }
}

impl Render for TextStory {
    type Element = Div<Self>;

    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> Self::Element {
        div().size_full().bg(white()).child(concat!(
            "The quick brown fox jumps over the lazy dog. ",
            "Meanwhile, the lazy dog decided it was time for a change. ",
            "He started daily workout routines, ate healthier and became the fastest dog in town.",
        ))
    }
}
