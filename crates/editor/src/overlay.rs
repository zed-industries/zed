use std::ops::Range;

use crate::{DisplayPoint, DisplayRow, Editor, EditorSnapshot, EditorStyle};
use gpui::{AnyElement, HighlightStyle, IntoElement, StyledText, ViewContext};

pub struct Overlay {
    pub text: String,
    pub highlight: HighlightStyle,
    pub point: DisplayPoint,
}

impl Overlay {
    pub fn render(
        &self,
        style: &EditorStyle,
        _snapshot: &EditorSnapshot,
        visible_display_row_range: Range<DisplayRow>,
        _cx: &mut ViewContext<Editor>,
    ) -> Option<(DisplayPoint, AnyElement)> {
        if !visible_display_row_range.contains(&self.point.row()) {
            return None;
        }
        let highlights = vec![(0..self.text.len(), self.highlight)];
        let iter = highlights.iter().cloned();
        Some((
            self.point,
            StyledText::new(self.text.clone())
                .with_highlights(&style.text, iter)
                .into_any_element(),
        ))
    }
}
