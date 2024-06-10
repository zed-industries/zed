use std::ops::Range;

use crate::{DisplayPoint, DisplayRow};
use gpui::{AnyElement, HighlightStyle, IntoElement, StyledText, TextStyle};

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
        style: &TextStyle,
        visible_display_row_range: Range<DisplayRow>,
    ) -> Option<(DisplayPoint, f32, AnyElement)> {
        if !visible_display_row_range.contains(&self.point.row()) {
            return None;
        }
        let iter = self.highlights.iter().cloned();

        let el = StyledText::new(self.text.clone())
            .with_highlights(style, iter)
            .into_any_element();
        Some((self.point, self.offset, el))
    }
}
