use editor::DisplayPoint;
use gpui::{HighlightStyle, KeyContext};

use crate::perm::{Trie, TrimResult};

#[derive(Debug, Default)]
pub(crate) struct OverlayState {
    pub style: HighlightStyle,
    pub point: DisplayPoint,
}

#[derive(Debug, Default)]
pub(crate) enum EditorState {
    #[default]
    None,
    NCharInput(NCharInput),
    Selection(Selection),
}

impl EditorState {
    pub(crate) fn is_none(&self) -> bool {
        matches!(self, EditorState::None)
    }

    pub(crate) fn clear(&mut self) {
        *self = EditorState::None;
    }

    pub(crate) fn easy_motion_controlled(&self) -> bool {
        !self.is_none()
    }

    pub(crate) fn keymap_context_layer(&self) -> KeyContext {
        let mut context = KeyContext::new_with_defaults();
        if self.easy_motion_controlled() {
            context.add("EasyMotionControlled");
            context.add("menu");
        }
        return context;
    }
}

#[derive(Debug)]
pub(crate) struct Selection {
    trie: Trie<OverlayState>,
}

impl Selection {
    pub(crate) fn new(trie: Trie<OverlayState>) -> Selection {
        Selection { trie }
    }

    #[allow(dead_code)]
    pub(crate) fn trie(&self) -> &Trie<OverlayState> {
        &self.trie
    }

    pub(crate) fn record_char(&mut self, character: char) -> TrimResult<DisplayPoint> {
        match self.trie.trim(character) {
            TrimResult::NoChange => TrimResult::NoChange,
            TrimResult::Changed => TrimResult::Changed,
            TrimResult::Found(overlay) => return TrimResult::Found(overlay.point.clone()),
            TrimResult::Err => return TrimResult::Err,
        }
    }

    pub(crate) fn record_str(mut self, characters: &str) -> (Self, TrimResult<DisplayPoint>) {
        let mut changed = false;
        for character in characters.chars() {
            let ret = self.record_char(character);
            match ret {
                TrimResult::NoChange => {}
                TrimResult::Changed => changed = true,
                TrimResult::Found(point) => return (self, TrimResult::Found(point)),
                TrimResult::Err => return (self, TrimResult::Err),
            };
        }
        if changed {
            (self, TrimResult::Changed)
        } else {
            (self, TrimResult::NoChange)
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct NCharInput {
    n: usize,
    chars: String,
}

#[derive(Debug)]
pub(crate) enum InputResult {
    Recording(NCharInput),
    ShowTrie(String),
}

impl NCharInput {
    pub(crate) fn record_str(mut self, characters: &str) -> InputResult {
        if self.chars.len() + characters.len() >= self.n {
            self.chars
                .push_str(&characters[0..self.n - self.chars.len()]);
            InputResult::ShowTrie(self.chars)
        } else {
            self.chars.push_str(characters);
            InputResult::Recording(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{InputResult, NCharInput};

    #[test]
    fn test_record_str() {
        let char_input = NCharInput {
            n: 4,
            chars: "a".to_string(),
        };
        let res = char_input.record_str("b");
        let char_input = match res {
            InputResult::Recording(char_input) => {
                assert_eq!(&char_input.chars, "ab");
                char_input
            }
            InputResult::ShowTrie(_) => panic!("incorrect keystroke resule"),
        };

        let res = char_input.record_str("characters");
        match res {
            InputResult::Recording(_) => panic!("incorrect keystroke resule"),
            InputResult::ShowTrie(str) => assert_eq!(str, "abch"),
        }
    }
}
