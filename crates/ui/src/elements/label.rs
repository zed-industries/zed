use gpui2::elements::div;
use gpui2::{Element, Hsla, IntoElement, ParentElement, ViewContext};
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

#[derive(Default, PartialEq, Copy, Clone)]
pub enum LabelSize {
    #[default]
    Default,
    Small,
}

#[derive(Element, Clone)]
pub struct Label {
    label: String,
    color: LabelColor,
    size: LabelSize,
    highlight_indices: Vec<usize>,
}

impl Label {
    pub fn new<L>(label: L) -> Self
    where
        L: Into<String>,
    {
        Self {
            label: label.into(),
            color: LabelColor::Default,
            size: LabelSize::Default,
            highlight_indices: Vec::new(),
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

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        let color = match self.color {
            LabelColor::Default => theme.lowest.base.default.foreground,
            LabelColor::Muted => theme.lowest.variant.default.foreground,
            LabelColor::Created => theme.lowest.positive.default.foreground,
            LabelColor::Modified => theme.lowest.warning.default.foreground,
            LabelColor::Deleted => theme.lowest.negative.default.foreground,
            LabelColor::Disabled => theme.lowest.base.disabled.foreground,
            LabelColor::Hidden => theme.lowest.variant.default.foreground,
            LabelColor::Placeholder => theme.lowest.base.disabled.foreground,
            LabelColor::Accent => theme.lowest.accent.default.foreground,
        };

        let highlight_color = theme.lowest.accent.default.foreground;

        let mut highlight_indices = self.highlight_indices.iter().copied().peekable();

        let mut runs: SmallVec<[Run; 8]> = SmallVec::new();

        for (char_ix, char) in self.label.char_indices() {
            let mut color = color;

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

        div().flex().children(runs.into_iter().map(|run| {
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
