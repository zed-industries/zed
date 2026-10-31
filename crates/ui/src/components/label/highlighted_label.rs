use std::ops::Range;

use gpui::{FontWeight, HighlightStyle, StyleRefinement, StyledText};
use gpui_util::debug_panic;

use crate::utils::replace_control_characters_remapping_offsets;
use crate::{LabelCommon, LabelLike, LabelSize, LineHeightStyle, prelude::*};

#[derive(IntoElement, RegisterComponent)]
pub struct HighlightedLabel {
    base: LabelLike,
    label: SharedString,
    highlight_indices: Vec<usize>,
}

impl HighlightedLabel {
    /// Constructs a label with the given characters highlighted.
    /// Characters are identified by UTF-8 byte position.
    #[track_caller]
    pub fn new(label: impl Into<SharedString>, mut highlight_indices: Vec<usize>) -> Self {
        let label = label.into();

        if let Some(index) = highlight_indices
            .iter()
            .find(|&i| !label.is_char_boundary(*i))
        {
            let location = std::panic::Location::caller();
            debug_panic!(
                "highlight index {index} is not a valid UTF-8 boundary (called from {location})"
            );
            highlight_indices.clear();
        }

        Self {
            base: LabelLike::new(),
            label,
            highlight_indices,
        }
    }

    /// Constructs a label with the given byte ranges highlighted.
    /// Assumes that the highlight ranges are valid UTF-8 byte positions.
    pub fn from_ranges(
        label: impl Into<SharedString>,
        highlight_ranges: Vec<Range<usize>>,
    ) -> Self {
        let label = label.into();
        let highlight_indices = highlight_ranges
            .iter()
            .flat_map(|range| {
                let mut indices = Vec::new();
                let mut index = range.start;
                while index < range.end {
                    indices.push(index);
                    index += label[index..].chars().next().map_or(0, |c| c.len_utf8());
                }
                indices
            })
            .collect();

        Self {
            base: LabelLike::new(),
            label,
            highlight_indices,
        }
    }

    pub fn text(&self) -> &str {
        self.label.as_str()
    }

    pub fn highlight_indices(&self) -> &[usize] {
        &self.highlight_indices
    }

    /// Truncates the label from the start, keeping the end visible.
    pub fn truncate_start(mut self) -> Self {
        self.base = self.base.truncate_start();
        self
    }

    /// Truncates overflowing text with an ellipsis (`…`) in the middle if needed.
    pub fn truncate_middle(mut self) -> Self {
        self.base = self.base.truncate_middle();
        self
    }
}

impl HighlightedLabel {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.base.style()
    }

    pub fn flex_1(mut self) -> Self {
        self.style().flex_grow = Some(1.);
        self.style().flex_shrink = Some(1.);
        self.style().flex_basis = Some(gpui::relative(0.).into());
        self
    }

    pub fn flex_none(mut self) -> Self {
        self.style().flex_grow = Some(0.);
        self.style().flex_shrink = Some(0.);
        self
    }

    pub fn flex_grow(mut self) -> Self {
        self.style().flex_grow = Some(1.);
        self
    }

    pub fn flex_shrink(mut self) -> Self {
        self.style().flex_shrink = Some(1.);
        self
    }

    pub fn flex_shrink_0(mut self) -> Self {
        self.style().flex_shrink = Some(0.);
        self
    }
}

impl LabelCommon for HighlightedLabel {
    fn size(mut self, size: LabelSize) -> Self {
        self.base = self.base.size(size);
        self
    }

    fn weight(mut self, weight: FontWeight) -> Self {
        self.base = self.base.weight(weight);
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

    fn strikethrough(mut self) -> Self {
        self.base = self.base.strikethrough();
        self
    }

    fn italic(mut self) -> Self {
        self.base = self.base.italic();
        self
    }

    fn alpha(mut self, alpha: f32) -> Self {
        self.base = self.base.alpha(alpha);
        self
    }

    fn underline(mut self) -> Self {
        self.base = self.base.underline();
        self
    }

    fn truncate(mut self) -> Self {
        self.base = self.base.truncate();
        self
    }

    fn single_line(mut self) -> Self {
        // The highlight indices are byte offsets into the label, and every
        // stand-in is wider in bytes than the character it replaces, so they
        // have to move with the text. Left alone they would index into the
        // middle of a character, which panics when the highlight ranges are
        // built — in release builds too.
        if let Some(replaced) =
            replace_control_characters_remapping_offsets(&self.label, &mut self.highlight_indices)
        {
            self.label = SharedString::from(replaced);
        }
        self.base = self.base.single_line();
        self
    }

    fn buffer_font(mut self, cx: &App) -> Self {
        self.base = self.base.buffer_font(cx);
        self
    }

    fn inline_code(mut self, cx: &App) -> Self {
        self.base = self.base.inline_code(cx);
        self
    }
}

pub fn highlight_ranges(
    text: &str,
    indices: &[usize],
    style: HighlightStyle,
) -> Vec<(Range<usize>, HighlightStyle)> {
    let mut highlight_indices = indices.iter().copied().peekable();
    let mut highlights: Vec<(Range<usize>, HighlightStyle)> = Vec::new();

    while let Some(start_ix) = highlight_indices.next() {
        let mut end_ix = start_ix;

        loop {
            end_ix += text[end_ix..].chars().next().map_or(0, |c| c.len_utf8());
            if highlight_indices.next_if(|&ix| ix == end_ix).is_none() {
                break;
            }
        }

        highlights.push((start_ix..end_ix, style));
    }

    highlights
}

impl RenderOnce for HighlightedLabel {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let highlight_color = cx.theme().colors().text_accent;

        let highlights = highlight_ranges(
            &self.label,
            &self.highlight_indices,
            HighlightStyle {
                color: Some(highlight_color),
                ..Default::default()
            },
        );

        let mut text_style = window.text_style();
        text_style.color = self.base.color.color(cx);

        self.base
            .child(StyledText::new(self.label).with_default_highlights(&text_style, highlights))
    }
}

impl Component for HighlightedLabel {
    fn scope() -> ComponentScope {
        ComponentScope::Typography
    }

    fn name() -> &'static str {
        "HighlightedLabel"
    }

    fn description() -> &'static str {
        "A label with highlighted characters based on specified indices."
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> AnyElement {
        v_flex()
            .gap_6()
            .children(vec![
                example_group_with_title(
                    "Basic Usage",
                    vec![
                        single_example(
                            "Default",
                            HighlightedLabel::new("Highlighted Text", vec![0, 1, 2, 3])
                                .into_any_element(),
                        ),
                        single_example(
                            "Custom Color",
                            HighlightedLabel::new("Colored Highlight", vec![0, 1, 7, 8, 9])
                                .color(Color::Accent)
                                .into_any_element(),
                        ),
                    ],
                ),
                example_group_with_title(
                    "Styles",
                    vec![
                        single_example(
                            "Bold",
                            HighlightedLabel::new("Bold Highlight", vec![0, 1, 2, 3])
                                .weight(FontWeight::BOLD)
                                .into_any_element(),
                        ),
                        single_example(
                            "Italic",
                            HighlightedLabel::new("Italic Highlight", vec![0, 1, 6, 7, 8])
                                .italic()
                                .into_any_element(),
                        ),
                        single_example(
                            "Underline",
                            HighlightedLabel::new("Underlined Highlight", vec![0, 1, 10, 11, 12])
                                .underline()
                                .into_any_element(),
                        ),
                    ],
                ),
                example_group_with_title(
                    "Sizes",
                    vec![
                        single_example(
                            "Small",
                            HighlightedLabel::new("Small Highlight", vec![0, 1, 5, 6, 7])
                                .size(LabelSize::Small)
                                .into_any_element(),
                        ),
                        single_example(
                            "Large",
                            HighlightedLabel::new("Large Highlight", vec![0, 1, 5, 6, 7])
                                .size(LabelSize::Large)
                                .into_any_element(),
                        ),
                    ],
                ),
                example_group_with_title(
                    "Special Cases",
                    vec![
                        single_example(
                            "Single Line",
                            HighlightedLabel::new(
                                "Single Line Highlight\nWith Newline",
                                vec![0, 1, 7, 8, 9],
                            )
                            .single_line()
                            .into_any_element(),
                        ),
                        single_example(
                            "Truncate",
                            HighlightedLabel::new(
                                "This is a very long text that should be truncated with highlights",
                                vec![0, 1, 2, 3, 4, 5],
                            )
                            .truncate()
                            .into_any_element(),
                        ),
                    ],
                ),
            ])
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line_replaces_control_characters() {
        let label = HighlightedLabel::new("a\nb\rc\td", Vec::new()).single_line();
        assert_eq!(label.text(), "a⏎b␍c␉d");
    }

    #[test]
    fn test_single_line_moves_highlights_with_the_text() {
        // Highlighting "a" and "b" in "a\tb": "b" sits at byte 2 before the
        // replacement and at byte 4 after it, the stand-in being three bytes.
        let label = HighlightedLabel::new("a\tb", vec![0, 2]).single_line();
        assert_eq!(label.text(), "a␉b");
        assert_eq!(label.highlight_indices(), &[0, 4]);
    }

    #[test]
    fn test_single_line_keeps_highlights_on_character_boundaries() {
        // Highlighting every character of a name full of control characters is
        // what would panic while building the highlight ranges if the offsets
        // were left pointing into the old text.
        let text = "a\tb\nc\rd";
        let indices: Vec<usize> = text.char_indices().map(|(ix, _)| ix).collect();
        let label = HighlightedLabel::new(text, indices).single_line();
        for index in label.highlight_indices() {
            assert!(
                label.text().is_char_boundary(*index),
                "index {index} is not a boundary of {:?}",
                label.text(),
            );
        }
    }

    #[test]
    fn test_single_line_leaves_printable_text_alone() {
        let label = HighlightedLabel::new("main.rs", vec![0, 5]).single_line();
        assert_eq!(label.text(), "main.rs");
        assert_eq!(label.highlight_indices(), &[0, 5]);
    }

    #[test]
    fn test_replacement_only_happens_on_single_line() {
        // Multi-line labels are a legitimate use, so nothing is substituted
        // unless the caller asked for a single line.
        let label = HighlightedLabel::new("a\nb", vec![0]);
        assert_eq!(label.text(), "a\nb");
    }
}
