use std::ops::Range;

use crate::{LabelLike, prelude::*};
use gpui::{HighlightStyle, StyleRefinement, StyledText};

/// A struct representing a label element in the UI.
///
/// The `Label` struct stores the label text and common properties for a label element.
/// It provides methods for modifying these properties.
///
/// # Examples
///
/// ```
/// use ui::prelude::*;
///
/// Label::new("Hello, World!");
/// ```
///
/// **A colored label**, for example labeling a dangerous action:
///
/// ```
/// use ui::prelude::*;
///
/// let my_label = Label::new("Delete").color(Color::Error);
/// ```
///
/// **A label with a strikethrough**, for example labeling something that has been deleted:
///
/// ```
/// use ui::prelude::*;
///
/// let my_label = Label::new("Deleted").strikethrough();
/// ```
#[derive(IntoElement, RegisterComponent)]
pub struct Label {
    base: LabelLike,
    label: SharedString,
    render_code_spans: bool,
}

impl Label {
    /// Creates a new [`Label`] with the given text.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!");
    /// ```
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            base: LabelLike::new(),
            label: label.into(),
            render_code_spans: false,
        }
    }

    /// When enabled, text wrapped in backticks (e.g. `` `code` ``) will be
    /// rendered in the buffer (monospace) font.
    pub fn render_code_spans(mut self) -> Self {
        self.render_code_spans = true;
        self
    }

    /// Sets the text of the [`Label`].
    pub fn set_text(&mut self, text: impl Into<SharedString>) {
        self.label = text.into();
    }

    /// Truncates the label from the start, keeping the end visible.
    pub fn truncate_start(mut self) -> Self {
        self.base = self.base.truncate_start();
        self
    }
}

// Style methods.
impl Label {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.base.style()
    }

    gpui::margin_style_methods!({
        visibility: pub
    });

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

impl LabelCommon for Label {
    /// Sets the size of the label using a [`LabelSize`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!").size(LabelSize::Small);
    /// ```
    fn size(mut self, size: LabelSize) -> Self {
        self.base = self.base.size(size);
        self
    }

    /// Sets the weight of the label using a [`FontWeight`].
    ///
    /// # Examples
    ///
    /// ```
    /// use gpui::FontWeight;
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!").weight(FontWeight::BOLD);
    /// ```
    fn weight(mut self, weight: gpui::FontWeight) -> Self {
        self.base = self.base.weight(weight);
        self
    }

    /// Sets the line height style of the label using a [`LineHeightStyle`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!").line_height_style(LineHeightStyle::UiLabel);
    /// ```
    fn line_height_style(mut self, line_height_style: LineHeightStyle) -> Self {
        self.base = self.base.line_height_style(line_height_style);
        self
    }

    /// Sets the color of the label using a [`Color`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!").color(Color::Accent);
    /// ```
    fn color(mut self, color: Color) -> Self {
        self.base = self.base.color(color);
        self
    }

    /// Sets the strikethrough property of the label.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!").strikethrough();
    /// ```
    fn strikethrough(mut self) -> Self {
        self.base = self.base.strikethrough();
        self
    }

    /// Sets the italic property of the label.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!").italic();
    /// ```
    fn italic(mut self) -> Self {
        self.base = self.base.italic();
        self
    }

    /// Sets the alpha property of the color of label.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!").alpha(0.5);
    /// ```
    fn alpha(mut self, alpha: f32) -> Self {
        self.base = self.base.alpha(alpha);
        self
    }

    fn underline(mut self) -> Self {
        self.base = self.base.underline();
        self
    }

    /// Truncates overflowing text with an ellipsis (`…`) if needed.
    fn truncate(mut self) -> Self {
        self.base = self.base.truncate();
        self
    }

    fn single_line(mut self) -> Self {
        self.label = SharedString::from(self.label.replace('\n', "⏎"));
        self.base = self.base.single_line();
        self
    }

    fn buffer_font(mut self, cx: &App) -> Self {
        self.base = self.base.buffer_font(cx);
        self
    }

    /// Styles the label to look like inline code.
    fn inline_code(mut self, cx: &App) -> Self {
        self.base = self.base.inline_code(cx);
        self
    }
}

impl RenderOnce for Label {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        if self.render_code_spans {
            if let Some((stripped, code_ranges)) = parse_backtick_spans(&self.label) {
                let buffer_font_family = theme::theme_settings(cx).buffer_font(cx).family.clone();
                let background_color = cx.theme().colors().element_background;

                let highlights = code_ranges.iter().map(|range| {
                    (
                        range.clone(),
                        HighlightStyle {
                            background_color: Some(background_color),
                            ..Default::default()
                        },
                    )
                });

                let font_overrides = code_ranges
                    .iter()
                    .map(|range| (range.clone(), buffer_font_family.clone()));

                return self.base.child(
                    StyledText::new(stripped)
                        .with_highlights(highlights)
                        .with_font_family_overrides(font_overrides),
                );
            }
        }
        self.base.child(self.label)
    }
}

/// Parses backtick-delimited code spans from a string.
///
/// Returns `None` if there are no matched backtick pairs.
/// Otherwise returns the text with backticks stripped and the byte ranges
/// of the code spans in the stripped string.
fn parse_backtick_spans(text: &str) -> Option<(SharedString, Vec<Range<usize>>)> {
    if !text.contains('`') {
        return None;
    }

    let mut stripped = String::with_capacity(text.len());
    let mut code_ranges = Vec::new();
    let mut in_code = false;
    let mut code_start = 0;

    for ch in text.chars() {
        if ch == '`' {
            if in_code {
                code_ranges.push(code_start..stripped.len());
            } else {
                code_start = stripped.len();
            }
            in_code = !in_code;
        } else {
            stripped.push(ch);
        }
    }

    if code_ranges.is_empty() {
        return None;
    }

    Some((SharedString::from(stripped), code_ranges))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_backtick_spans_no_backticks() {
        assert_eq!(parse_backtick_spans("plain text"), None);
    }

    #[test]
    fn test_parse_backtick_spans_single_span() {
        let (text, ranges) = parse_backtick_spans("use `zed` to open").unwrap();
        assert_eq!(text.as_ref(), "use zed to open");
        assert_eq!(ranges, vec![4..7]);
    }

    #[test]
    fn test_parse_backtick_spans_multiple_spans() {
        let (text, ranges) = parse_backtick_spans("flags `-e` or `-n`").unwrap();
        assert_eq!(text.as_ref(), "flags -e or -n");
        assert_eq!(ranges, vec![6..8, 12..14]);
    }

    #[test]
    fn test_parse_backtick_spans_unmatched_backtick() {
        // A trailing unmatched backtick should not produce a code range
        assert_eq!(parse_backtick_spans("trailing `backtick"), None);
    }

    #[test]
    fn test_parse_backtick_spans_empty_span() {
        let (text, ranges) = parse_backtick_spans("empty `` span").unwrap();
        assert_eq!(text.as_ref(), "empty  span");
        assert_eq!(ranges, vec![6..6]);
    }
}

impl Component for Label {
    fn scope() -> ComponentScope {
        ComponentScope::Typography
    }

    fn description() -> Option<&'static str> {
        Some("A text label component that supports various styles, sizes, and formatting options.")
    }

    fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    example_group_with_title(
                        "Sizes",
                        vec![
                            single_example("Default", Label::new("Project Explorer").into_any_element()),
                            single_example("Small", Label::new("File: main.rs").size(LabelSize::Small).into_any_element()),
                            single_example("Large", Label::new("Welcome to Zed").size(LabelSize::Large).into_any_element()),
                        ],
                    ),
                    example_group_with_title(
                        "Colors",
                        vec![
                            single_example("Default", Label::new("Status: Ready").into_any_element()),
                            single_example("Accent", Label::new("New Update Available").color(Color::Accent).into_any_element()),
                            single_example("Error", Label::new("Build Failed").color(Color::Error).into_any_element()),
                        ],
                    ),
                    example_group_with_title(
                        "Styles",
                        vec![
                            single_example("Default", Label::new("Normal Text").into_any_element()),
                            single_example("Bold", Label::new("Important Notice").weight(gpui::FontWeight::BOLD).into_any_element()),
                            single_example("Italic", Label::new("Code Comment").italic().into_any_element()),
                            single_example("Strikethrough", Label::new("Deprecated Feature").strikethrough().into_any_element()),
                            single_example("Underline", Label::new("Clickable Link").underline().into_any_element()),
                            single_example("Inline Code", Label::new("fn main() {}").inline_code(cx).into_any_element()),
                        ],
                    ),
                    example_group_with_title(
                        "Line Height Styles",
                        vec![
                            single_example("Default", Label::new("Multi-line\nText\nExample").into_any_element()),
                            single_example("UI Label", Label::new("Compact\nUI\nLabel").line_height_style(LineHeightStyle::UiLabel).into_any_element()),
                        ],
                    ),
                    example_group_with_title(
                        "Special Cases",
                        vec![
                            single_example("Single Line", Label::new("Line 1\nLine 2\nLine 3").single_line().into_any_element()),
                            single_example("Regular Truncation", div().max_w_24().child(Label::new("This is a very long file name that should be truncated: very_long_file_name_with_many_words.rs").truncate()).into_any_element()),
                            single_example("Start Truncation", div().max_w_24().child(Label::new("zed/crates/ui/src/components/label/truncate/label/label.rs").truncate_start()).into_any_element()),
                        ],
                    ),
                ])
                .into_any_element()
        )
    }
}
