use crate::prelude::*;
use crate::styled_ext::StyledExt;
use gpui::{relative, Div, Hsla, IntoElement, StyledText, TextRun, WindowContext};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum LabelSize {
    #[default]
    Default,
    Small,
}

#[derive(Default, PartialEq, Copy, Clone)]
pub enum LineHeightStyle {
    #[default]
    TextLabel,
    /// Sets the line height to 1
    UILabel,
}

#[derive(IntoElement, Clone)]
pub struct Label {
    label: SharedString,
    size: LabelSize,
    line_height_style: LineHeightStyle,
    color: Color,
    strikethrough: bool,
}

impl RenderOnce for Label {
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        div()
            .when(self.strikethrough, |this| {
                this.relative().child(
                    div()
                        .absolute()
                        .top_1_2()
                        .w_full()
                        .h_px()
                        .bg(Color::Hidden.color(cx)),
                )
            })
            .map(|this| match self.size {
                LabelSize::Default => this.text_ui(),
                LabelSize::Small => this.text_ui_sm(),
            })
            .when(self.line_height_style == LineHeightStyle::UILabel, |this| {
                this.line_height(relative(1.))
            })
            .text_color(self.color.color(cx))
            .child(self.label.clone())
    }
}

impl Label {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            size: LabelSize::Default,
            line_height_style: LineHeightStyle::default(),
            color: Color::Default,
            strikethrough: false,
        }
    }

    pub fn size(mut self, size: LabelSize) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn line_height_style(mut self, line_height_style: LineHeightStyle) -> Self {
        self.line_height_style = line_height_style;
        self
    }

    pub fn set_strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = strikethrough;
        self
    }
}

#[derive(IntoElement)]
pub struct HighlightedLabel {
    label: SharedString,
    size: LabelSize,
    color: Color,
    highlight_indices: Vec<usize>,
    strikethrough: bool,
}

impl RenderOnce for HighlightedLabel {
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        let highlight_color = cx.theme().colors().text_accent;
        let mut text_style = cx.text_style().clone();

        let mut highlight_indices = self.highlight_indices.iter().copied().peekable();

        let mut runs: Vec<TextRun> = Vec::new();

        for (char_ix, char) in self.label.char_indices() {
            let mut color = self.color.color(cx);

            if let Some(highlight_ix) = highlight_indices.peek() {
                if char_ix == *highlight_ix {
                    color = highlight_color;
                    highlight_indices.next();
                }
            }

            let last_run = runs.last_mut();
            let start_new_run = if let Some(last_run) = last_run {
                if color == last_run.color {
                    last_run.len += char.len_utf8();
                    false
                } else {
                    true
                }
            } else {
                true
            };

            if start_new_run {
                text_style.color = color;
                runs.push(text_style.to_run(char.len_utf8()))
            }
        }

        div()
            .flex()
            .when(self.strikethrough, |this| {
                this.relative().child(
                    div()
                        .absolute()
                        .top_px()
                        .my_auto()
                        .w_full()
                        .h_px()
                        .bg(Color::Hidden.color(cx)),
                )
            })
            .map(|this| match self.size {
                LabelSize::Default => this.text_ui(),
                LabelSize::Small => this.text_ui_sm(),
            })
            .child(StyledText::new(self.label, runs))
    }
}

impl HighlightedLabel {
    /// shows a label with the given characters highlighted.
    /// characters are identified by utf8 byte position.
    pub fn new(label: impl Into<SharedString>, highlight_indices: Vec<usize>) -> Self {
        Self {
            label: label.into(),
            size: LabelSize::Default,
            color: Color::Default,
            highlight_indices,
            strikethrough: false,
        }
    }

    pub fn size(mut self, size: LabelSize) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn set_strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = strikethrough;
        self
    }
}

/// A run of text that receives the same style.
struct Run {
    pub text: String,
    pub color: Hsla,
}
