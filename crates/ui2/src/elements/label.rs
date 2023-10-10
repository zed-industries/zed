use std::marker::PhantomData;

use gpui3::{Hsla, WindowContext};
use smallvec::SmallVec;

use crate::prelude::*;
use crate::theme::theme;

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

        match self {
            Self::Default => theme.middle.base.default.foreground,
            Self::Muted => theme.middle.variant.default.foreground,
            Self::Created => theme.middle.positive.default.foreground,
            Self::Modified => theme.middle.warning.default.foreground,
            Self::Deleted => theme.middle.negative.default.foreground,
            Self::Disabled => theme.middle.base.disabled.foreground,
            Self::Hidden => theme.middle.variant.default.foreground,
            Self::Placeholder => theme.middle.base.disabled.foreground,
            Self::Accent => theme.middle.accent.default.foreground,
        }
    }
}

#[derive(Default, PartialEq, Copy, Clone)]
pub enum LabelSize {
    #[default]
    Default,
    Small,
}

#[derive(Element, Clone)]
pub struct Label<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
    label: String,
    color: LabelColor,
    size: LabelSize,
    highlight_indices: Vec<usize>,
    strikethrough: bool,
}

impl<S: 'static + Send + Sync + Clone> Label<S> {
    pub fn new<L>(label: L) -> Self
    where
        L: Into<String>,
    {
        Self {
            state_type: PhantomData,
            label: label.into(),
            color: LabelColor::Default,
            size: LabelSize::Default,
            highlight_indices: Vec::new(),
            strikethrough: false,
        }
    }

    pub fn color(mut self, color: LabelColor) -> Self {
        self.color = color;
        self
    }

    pub fn size(mut self, size: LabelSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_highlights(mut self, indices: Vec<usize>) -> Self {
        self.highlight_indices = indices;
        self
    }

    pub fn set_strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = strikethrough;
        self
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        let theme = theme(cx);

        let highlight_color = theme.lowest.accent.default.foreground;

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
                        .fill(LabelColor::Hidden.hsla(cx)),
                )
            })
            .children(runs.into_iter().map(|run| {
                let mut div = div();

                if self.size == LabelSize::Small {
                    div = div.text_xs();
                } else {
                    div = div.text_sm();
                }

                div.text_color(run.color).child(run.text)
            }))
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
    pub struct LabelStory<S: 'static + Send + Sync + Clone> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync + Clone> LabelStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
            Story::container(cx)
                .child(Story::title_for::<_, Label<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(Label::new("Hello, world!"))
                .child(Story::label(cx, "Highlighted"))
                .child(Label::new("Hello, world!").with_highlights(vec![0, 1, 2, 7, 8, 12]))
        }
    }
}
