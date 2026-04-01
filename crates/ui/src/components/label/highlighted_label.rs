use std::ops::Range;

use gpui::{FontWeight, HighlightStyle, StyleRefinement, StyledText};

use crate::{LabelCommon, LabelLike, LabelSize, prelude::*};

#[derive(IntoElement, RegisterComponent)]
pub struct HighlightedLabel {
    base: LabelLike,
    label: SharedString,
    highlight_indices: Vec<usize>,
}

impl HighlightedLabel {
    /// Constructs a label with the given characters highlighted.
    /// Characters are identified by UTF-8 byte position.
    pub fn new(label: impl Into<SharedString>, highlight_indices: Vec<usize>) -> Self {
        let label = label.into();
        for &run in &highlight_indices {
            assert!(
                label.is_char_boundary(run),
                "highlight index {run} is not a valid UTF-8 boundary"
            );
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

impl_label_common!(HighlightedLabel);

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

    fn description() -> Option<&'static str> {
        Some("A label with highlighted characters based on specified indices.")
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    example_group_with_title(
                        "Basic Usage",
                        vec![
                            single_example(
                                "Default",
                                HighlightedLabel::new("Highlighted Text", vec![0, 1, 2, 3]).into_any_element(),
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
                                HighlightedLabel::new("Single Line Highlight\nWith Newline", vec![0, 1, 7, 8, 9])
                                    .single_line()
                                    .into_any_element(),
                            ),
                            single_example(
                                "Truncate",
                                HighlightedLabel::new("This is a very long text that should be truncated with highlights", vec![0, 1, 2, 3, 4, 5])
                                    .truncate()
                                    .into_any_element(),
                            ),
                        ],
                    ),
                ])
                .into_any_element()
        )
    }
}
