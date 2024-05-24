use std::ops::Range;

use crate::{DisplayPoint, DisplayRow, Editor, EditorSnapshot, EditorStyle};
use gpui::{AnyElement, HighlightStyle, IntoElement, StyledText, ViewContext};

#[derive(Debug, Clone, Default)]
pub struct Overlay {
    pub text: String,
    pub highlights: Vec<(Range<usize>, HighlightStyle)>,
    pub point: DisplayPoint,
    pub offset: f32,
}

impl Overlay {
    pub fn render(
        &self,
        style: &EditorStyle,
        _snapshot: &EditorSnapshot,
        visible_display_row_range: Range<DisplayRow>,
        _cx: &mut ViewContext<Editor>,
    ) -> Option<(DisplayPoint, f32, AnyElement)> {
        if !visible_display_row_range.contains(&self.point.row()) {
            return None;
        }
        let iter = self.highlights.iter().cloned();

        let el = StyledText::new(self.text.clone())
            .with_highlights(&style.text, iter)
            .into_any_element();
        Some((self.point, self.offset, el))
        // -- same rendering as player names --
        // let text_size = style.text.font_size;
        // let high = iter.next().unwrap().1;
        // let el = div()
        //     .bg(high.background_color.unwrap())
        //     .text_size(text_size)
        //     .px_0p5()
        //     .line_height(text_size.to_pixels(cx.rem_size()) + px(2.))
        //     .text_color(Hsla::white())
        //     .font_family(style.text.font_family.clone())
        //     .child(self.text.clone())
        //     .into_any_element();
    }
}
