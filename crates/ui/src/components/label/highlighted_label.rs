use std::ops::Range;

use gpui::{HighlightStyle, StyledText};

use crate::{prelude::*, LabelCommon, LabelLike, LabelSize, LineHeightStyle};

#[derive(IntoElement)]
pub struct HighlightedLabel {
    base: LabelLike,
    label: SharedString,
    highlight_indices: Vec<usize>,
}

impl HighlightedLabel {
    /// Constructs a label with the given characters highlighted.
    /// Characters are identified by UTF-8 byte position.
    pub fn new(label: impl Into<SharedString>, highlight_indices: Vec<usize>) -> Self {
        Self {
            base: LabelLike::new(),
            label: label.into(),
            highlight_indices,
        }
    }
}

impl LabelCommon for HighlightedLabel {
    fn size(mut self, size: LabelSize) -> Self {
        self.base = self.base.size(size);
        self
    }

    fn line_height_style(mut self, line_height_style: LineHeightStyle) -> Self {
        self.base = self.base.line_height_style(line_height_style);
        self
    }

    fn color(mut self, color: Color) -> Self {
        self.base = self.base.color(color);
        self
    }

    fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.base = self.base.strikethrough(strikethrough);
        self
    }
}

impl RenderOnce for HighlightedLabel {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let highlight_color = cx.theme().colors().text_accent;

        let mut highlight_indices = self.highlight_indices.iter().copied().peekable();
        let mut highlights: Vec<(Range<usize>, HighlightStyle)> = Vec::new();

        while let Some(start_ix) = highlight_indices.next() {
            let mut end_ix = start_ix;

            loop {
                end_ix = end_ix + self.label[end_ix..].chars().next().unwrap().len_utf8();
                if let Some(&next_ix) = highlight_indices.peek() {
                    if next_ix == end_ix {
                        end_ix = next_ix;
                        highlight_indices.next();
                        continue;
                    }
                }
                break;
            }

            highlights.push((
                start_ix..end_ix,
                HighlightStyle {
                    color: Some(highlight_color),
                    ..Default::default()
                },
            ));
        }

        let mut text_style = cx.text_style().clone();
        text_style.color = self.base.color.color(cx);

        self.base
            .child(StyledText::new(self.label).with_highlights(&text_style, highlights))
    }
}
