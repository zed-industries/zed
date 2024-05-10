use gpui::Hsla;
use serde_derive::Deserialize;

use crate::{try_parse_color, IndentAwareColorContent};

#[derive(Debug, Clone, Copy, Deserialize, Default)]
pub struct IndentAwareColor {
    pub line: Hsla,
    pub active_line: Hsla,
    pub background: Hsla,
    pub active_background: Hsla,
}

/// A collection of colors that are used to color indent aware lines in the editor.
#[derive(Clone, Deserialize)]
pub struct IndentAwareColors(pub Vec<IndentAwareColor>);

impl Default for IndentAwareColors {
    /// Don't use this!
    /// We have to have a default to be `[refineable::Refinable]`.
    /// TODO "Find a way to not need this for Refinable"
    fn default() -> Self {
        Self::dark()
    }
}

impl IndentAwareColors {
    pub fn dark() -> Self {
        //TODO
        Self(vec![])
    }

    pub fn light() -> Self {
        //TODO
        Self(vec![])
    }
}

impl IndentAwareColors {
    pub fn color_for_indent(&self, indent: u32) -> Option<IndentAwareColor> {
        let len = self.0.len();
        if len > 0 {
            self.0.get(indent as usize % len).cloned()
        } else {
            None
        }
    }

    /// Merges the given player colors into this [`PlayerColors`] instance.
    pub fn merge(&mut self, indent_aware_colors: &[IndentAwareColorContent]) {
        if indent_aware_colors.is_empty() {
            return;
        }

        for (idx, indent_aware_color) in indent_aware_colors.iter().enumerate() {
            let line = indent_aware_color
                .line
                .as_ref()
                .and_then(|color| try_parse_color(color).ok());
            let active_line = indent_aware_color
                .active_line
                .as_ref()
                .and_then(|color| try_parse_color(color).ok());
            let background = indent_aware_color
                .background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok());
            let active_background = indent_aware_color
                .active_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok());

            if let Some(indent_aware_color) = self.0.get_mut(idx) {
                *indent_aware_color = IndentAwareColor {
                    line: line.unwrap_or(indent_aware_color.line),
                    active_line: active_line.unwrap_or(indent_aware_color.active_line),
                    background: background.unwrap_or(indent_aware_color.background),
                    active_background: active_background
                        .unwrap_or(indent_aware_color.active_background),
                };
            } else {
                self.0.push(IndentAwareColor {
                    line: line.unwrap_or_default(),
                    active_line: active_line.unwrap_or_default(),
                    background: background.unwrap_or_default(),
                    active_background: active_background.unwrap_or_default(),
                })
            }
        }
    }
}
