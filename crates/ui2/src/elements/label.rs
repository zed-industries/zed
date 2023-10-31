use gpui2::{relative, Hsla, WindowContext};
use smallvec::SmallVec;

use crate::prelude::*;
#[derive(Default, PartialEq, Copy, Clone)]
pub enum LabelColor {
    #[default]
    Default,
    Muted,
    Created,
    Modified,
    Deleted,
    Disabled,
    Hidden,
    Placeholder,
    Accent,
}

impl LabelColor {
    pub fn hsla(&self, cx: &WindowContext) -> Hsla {
        let theme = old_theme(cx);

        match self {
            Self::Default => theme.text,
            Self::Muted => theme.text_muted,
            Self::Created => gpui2::red(),
            Self::Modified => gpui2::red(),
            Self::Deleted => gpui2::red(),
            Self::Disabled => theme.text_disabled,
            Self::Hidden => gpui2::red(),
            Self::Placeholder => theme.text_placeholder,
            Self::Accent => gpui2::red(),
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
                        .top_px()
                        .my_auto()
                        .w_full()
                        .h_px()
                        .bg(LabelColor::Hidden.hsla(cx)),
                )
            })
            .text_size(ui_size(cx, 1.))
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
        let theme = old_theme(cx);

        let highlight_color = theme.text_accent;

        let mut highlight_indices = self.highlight_indices.iter().copied().peekable();

        let mut runs: SmallVec<[Run; 8]> = SmallVec::new();

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
                    last_run.text.push(char);
                    false
                } else {
                    true
                }
            } else {
                true
            };

            if start_new_run {
                runs.push(Run {
                    text: char.to_string(),
                    color,
                });
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
            .children(
                runs.into_iter()
                    .map(|run| div().text_color(run.color).child(run.text)),
            )
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
    use gpui2::{Div, Render};

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
        }
    }
}
