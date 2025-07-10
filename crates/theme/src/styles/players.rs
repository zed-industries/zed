#![allow(missing_docs)]

use gpui::Hsla;
use serde_derive::Deserialize;

use crate::{PlayerColorContent, default::default_dark_theme, try_parse_color};

#[derive(Debug, Clone, Copy, Deserialize, Default, PartialEq)]
pub struct PlayerColor {
    pub cursor: Hsla,
    pub background: Hsla,
    pub selection: Hsla,
}

/// A collection of colors that are used to color players in the editor.
///
/// The first color is always the local player's color, usually a blue.
///
/// The rest of the default colors crisscross back and forth on the
/// color wheel so that the colors are as distinct as possible.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct PlayerColors(pub Vec<PlayerColor>);

impl Default for PlayerColors {
    /// Don't use this!
    /// We have to have a default to be `[refineable::Refinable]`.
    /// TODO "Find a way to not need this for Refinable"
    fn default() -> Self {
        default_dark_theme().players().clone()
    }
}

impl PlayerColors {
    pub fn local(&self) -> PlayerColor {
        *self.0.first().unwrap()
    }

    pub fn agent(&self) -> PlayerColor {
        *self.0.last().unwrap()
    }

    pub fn absent(&self) -> PlayerColor {
        *self.0.last().unwrap()
    }

    pub fn read_only(&self) -> PlayerColor {
        let local = self.local();
        PlayerColor {
            cursor: local.cursor.grayscale(),
            background: local.background.grayscale(),
            selection: local.selection.grayscale(),
        }
    }

    pub fn color_for_participant(&self, participant_index: u32) -> PlayerColor {
        let len = self.0.len() - 1;
        self.0[(participant_index as usize % len) + 1]
    }

    /// Merges the given player colors into this [`PlayerColors`] instance.
    pub fn merge(&mut self, user_player_colors: &[PlayerColorContent]) {
        if user_player_colors.is_empty() {
            return;
        }

        for (idx, player) in user_player_colors.iter().enumerate() {
            let cursor = player
                .cursor
                .as_ref()
                .and_then(|color| try_parse_color(color).ok());
            let background = player
                .background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok());
            let selection = player
                .selection
                .as_ref()
                .and_then(|color| try_parse_color(color).ok());

            if let Some(player_color) = self.0.get_mut(idx) {
                *player_color = PlayerColor {
                    cursor: cursor.unwrap_or(player_color.cursor),
                    background: background.unwrap_or(player_color.background),
                    selection: selection.unwrap_or(player_color.selection),
                };
            } else {
                self.0.push(PlayerColor {
                    cursor: cursor.unwrap_or_default(),
                    background: background.unwrap_or_default(),
                    selection: selection.unwrap_or_default(),
                });
            }
        }
    }
}
