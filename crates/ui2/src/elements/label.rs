use std::marker::PhantomData;

use gpui2::{relative, Hsla, WindowContext};
use smallvec::SmallVec;

use crate::prelude::*;
use crate::theme::old_theme;

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
        let theme = theme(cx);
        // TODO: Remove
        let old_theme = old_theme(cx);

        match self {
            Self::Default => theme.text,
            Self::Muted => theme.text_muted,
            Self::Created => old_theme.middle.positive.default.foreground,
            Self::Modified => old_theme.middle.warning.default.foreground,
            Self::Deleted => old_theme.middle.negative.default.foreground,
            Self::Disabled => theme.text_disabled,
            Self::Hidden => old_theme.middle.variant.default.foreground,
            Self::Placeholder => theme.text_placeholder,
            Self::Accent => old_theme.middle.accent.default.foreground,
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

#[derive(Element)]
pub struct Label<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
    label: SharedString,
    line_height_style: LineHeightStyle,
    color: LabelColor,
    strikethrough: bool,
}

impl<S: 'static + Send + Sync> Label<S> {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            state_type: PhantomData,
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

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
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

#[derive(Element)]
pub struct HighlightedLabel<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
    label: SharedString,
    color: LabelColor,
    highlight_indices: Vec<usize>,
    strikethrough: bool,
}

impl<S: 'static + Send + Sync> HighlightedLabel<S> {
    pub fn new(label: impl Into<SharedString>, highlight_indices: Vec<usize>) -> Self {
        Self {
            state_type: PhantomData,
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

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let theme = theme(cx);

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
    use crate::Story;

    use super::*;

    #[derive(Element)]
    pub struct LabelStory<S: 'static + Send + Sync> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync> LabelStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(
            &mut self,
            _view: &mut S,
            cx: &mut ViewContext<S>,
        ) -> impl Element<ViewState = S> {
            Story::container(cx)
                .child(Story::title_for::<_, Label<S>>(cx))
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
