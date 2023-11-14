use gpui::{relative, Hsla, Text, TextRun, WindowContext};

use crate::prelude::*;
use crate::styled_ext::StyledExt;

#[derive(Default, PartialEq, Copy, Clone)]
pub enum LabelColor {
    #[default]
    Default,
    Accent,
    Created,
    Deleted,
    Disabled,
    Error,
    Hidden,
    Info,
    Modified,
    Muted,
    Placeholder,
    Player(u32),
    Selected,
    Success,
    Warning,
}

impl LabelColor {
    pub fn hsla(&self, cx: &WindowContext) -> Hsla {
        match self {
            LabelColor::Default => cx.theme().colors().text,
            LabelColor::Muted => cx.theme().colors().text_muted,
            LabelColor::Created => cx.theme().status().created,
            LabelColor::Modified => cx.theme().status().modified,
            LabelColor::Deleted => cx.theme().status().deleted,
            LabelColor::Disabled => cx.theme().colors().text_disabled,
            LabelColor::Hidden => cx.theme().status().hidden,
            LabelColor::Info => cx.theme().status().info,
            LabelColor::Placeholder => cx.theme().colors().text_placeholder,
            LabelColor::Accent => cx.theme().colors().text_accent,
            LabelColor::Player(i) => cx.theme().styles.player.0[i.clone() as usize].cursor,
            LabelColor::Error => cx.theme().status().error,
            LabelColor::Selected => cx.theme().colors().text_accent,
            LabelColor::Success => cx.theme().status().success,
            LabelColor::Warning => cx.theme().status().warning,
        }
    }
}

#[derive(Default, PartialEq, Copy, Clone)]
pub enum LineHeightStyle {
    #[default]
    TextLabel,
    /// Sets the line height to 1
    UILabel,
}

#[derive(Component)]
pub struct Label {
    label: SharedString,
    line_height_style: LineHeightStyle,
    color: LabelColor,
    strikethrough: bool,
}

impl Label {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            line_height_style: LineHeightStyle::default(),
            color: LabelColor::Default,
            strikethrough: false,
        }
    }

    pub fn color(mut self, color: LabelColor) -> Self {
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

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        div()
            .when(self.strikethrough, |this| {
                this.relative().child(
                    div()
                        .absolute()
                        .top_1_2()
                        .w_full()
                        .h_px()
                        .bg(LabelColor::Hidden.hsla(cx)),
                )
            })
            .text_ui()
            .when(self.line_height_style == LineHeightStyle::UILabel, |this| {
                this.line_height(relative(1.))
            })
            .text_color(self.color.hsla(cx))
            .child(self.label.clone())
    }
}

#[derive(Component)]
pub struct HighlightedLabel {
    label: SharedString,
    color: LabelColor,
    highlight_indices: Vec<usize>,
    strikethrough: bool,
}

impl HighlightedLabel {
    /// shows a label with the given characters highlighted.
    /// characters are identified by utf8 byte position.
    pub fn new(label: impl Into<SharedString>, highlight_indices: Vec<usize>) -> Self {
        Self {
            label: label.into(),
            color: LabelColor::Default,
            highlight_indices,
            strikethrough: false,
        }
    }

    pub fn color(mut self, color: LabelColor) -> Self {
        self.color = color;
        self
    }

    pub fn set_strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = strikethrough;
        self
    }

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        let highlight_color = cx.theme().colors().text_accent;
        let mut text_style = cx.text_style().clone();

        let mut highlight_indices = self.highlight_indices.iter().copied().peekable();

        let mut runs: Vec<TextRun> = Vec::new();

        for (char_ix, char) in self.label.char_indices() {
            let mut color = self.color.hsla(cx);

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
                        .bg(LabelColor::Hidden.hsla(cx)),
                )
            })
            .child(Text::styled(self.label, runs))
    }
}

/// A run of text that receives the same style.
struct Run {
    pub text: String,
    pub color: Hsla,
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::Story;
    use gpui::{Div, Render};

    pub struct LabelStory;

    impl Render for LabelStory {
        type Element = Div<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<_, Label>(cx))
                .child(Story::label(cx, "Default"))
                .child(Label::new("Hello, world!"))
                .child(Story::label(cx, "Highlighted"))
                .child(HighlightedLabel::new(
                    "Hello, world!",
                    vec![0, 1, 2, 7, 8, 12],
                ))
                .child(HighlightedLabel::new(
                    "Héllo, world!",
                    vec![0, 1, 3, 8, 9, 13],
                ))
        }
    }
}
