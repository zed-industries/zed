use std::ops::Range;

use crate::{DisplayPoint, DisplayRow, DisplaySnapshot};
use gpui::{AnyElement, HighlightStyle, IntoElement, StyledText, TextStyle};
use multi_buffer::MultiBufferSnapshot;

#[derive(Debug, Clone, Default)]
pub struct Overlay {
    pub text: String,
    pub highlights: Vec<(Range<usize>, HighlightStyle)>,
    pub buffer_offset: usize,
}

impl Overlay {
    pub fn render(
        &self,
        style: &TextStyle,
        visible_display_row_range: Range<DisplayRow>,
        buffer_snapshot: &MultiBufferSnapshot,
        display_snapshot: &DisplaySnapshot,
    ) -> Option<(DisplayPoint, AnyElement)> {
        let point = buffer_snapshot.offset_to_point(self.buffer_offset);
        let display_point = display_snapshot.point_to_display_point(point, text::Bias::Left);
        if !visible_display_row_range.contains(&display_point.row()) {
            return None;
        }
        let iter = self.highlights.iter().cloned();

        let el = StyledText::new(self.text.clone())
            .with_highlights(style, iter)
            .into_any_element();
        Some((display_point, el))
    }
}
