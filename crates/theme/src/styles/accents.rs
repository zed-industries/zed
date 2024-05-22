use gpui::Hsla;
use serde_derive::Deserialize;

use crate::{
    amber, blue, cyan, gold, grass, indigo, iris, jade, lime, orange, pink, purple, tomato,
    try_parse_color, AccentContent,
};

/// A collection of colors that are used to color indent aware lines in the editor.
#[derive(Clone, Deserialize)]
pub struct AccentColors(pub Vec<Hsla>);

impl Default for AccentColors {
    /// Don't use this!
    /// We have to have a default to be `[refineable::Refinable]`.
    /// TODO "Find a way to not need this for Refinable"
    fn default() -> Self {
        Self::dark()
    }
}

impl AccentColors {
    pub fn dark() -> Self {
        Self(vec![
            blue().dark().step_9(),
            orange().dark().step_9(),
            pink().dark().step_9(),
            lime().dark().step_9(),
            purple().dark().step_9(),
            amber().dark().step_9(),
            jade().dark().step_9(),
            tomato().dark().step_9(),
            cyan().dark().step_9(),
            gold().dark().step_9(),
            grass().dark().step_9(),
            indigo().dark().step_9(),
            iris().dark().step_9(),
        ])
    }

    pub fn light() -> Self {
        Self(vec![
            blue().light().step_9(),
            orange().light().step_9(),
            pink().light().step_9(),
            lime().light().step_9(),
            purple().light().step_9(),
            amber().light().step_9(),
            jade().light().step_9(),
            tomato().light().step_9(),
            cyan().light().step_9(),
            gold().light().step_9(),
            grass().light().step_9(),
            indigo().light().step_9(),
            iris().light().step_9(),
        ])
    }
}

impl AccentColors {
    pub fn color_for_index(&self, index: u32) -> Option<Hsla> {
        let len = self.0.len();
        if len > 0 {
            self.0.get(index as usize % len).cloned()
        } else {
            None
        }
    }

    /// Merges the given accent colors into this [`AccentColors`] instance.
    pub fn merge(&mut self, accent_colors: &[AccentContent]) {
        if accent_colors.is_empty() {
            return;
        }

        for (idx, accent) in accent_colors.iter().enumerate() {
            let accent = accent
                .0
                .as_ref()
                .and_then(|color| try_parse_color(color).ok());

            if let Some(accent_color) = self.0.get_mut(idx) {
                *accent_color = accent.unwrap_or(*accent_color);
            } else {
                self.0.push(accent.unwrap_or_default())
            }
        }
    }
}
