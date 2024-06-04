use editor::DisplayPoint;
use gpui::{EntityId, HighlightStyle, KeyContext};

use crate::{
    trie::{Trie, TrimResult},
    Direction,
};

#[derive(Debug, Default, Clone)]
pub(crate) struct OverlayState {
    pub style: HighlightStyle,
    pub editor_id: EntityId,
    pub point: DisplayPoint,
}

#[derive(Debug, Default)]
pub(crate) enum EditorState {
    #[default]
    PendingSearch,
    NCharInput(NCharInput),
    Selection(Selection),
    Pattern(Pattern),
}

impl EditorState {
    pub(crate) fn new_selection(trie: Trie<OverlayState>) -> EditorState {
        EditorState::Selection(Selection::new(trie))
    }

    pub(crate) fn new_n_char(n: usize, direction: Direction) -> EditorState {
        EditorState::NCharInput(NCharInput::new(n, direction))
    }

    pub(crate) fn new_pattern(direction: Direction) -> EditorState {
        EditorState::Pattern(Pattern::new(direction))
    }

    pub(crate) fn keymap_context_layer(&self) -> KeyContext {
        let mut context = KeyContext::new_with_defaults();
        context.add("VimWaiting");
        return context;
    }
}

#[derive(Debug)]
pub(crate) struct Selection {
    selection: String,
    trie: Trie<OverlayState>,
}

impl Selection {
    pub(crate) fn new(trie: Trie<OverlayState>) -> Selection {
        Selection {
            selection: String::new(),
            trie,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn trie(&self) -> &Trie<OverlayState> {
        &self.trie
    }

    pub(crate) fn selection(&self) -> &str {
        self.selection.as_str()
    }

    pub(crate) fn record_char(&mut self, character: char) -> TrimResult<OverlayState> {
        self.selection.push(character);
        self.trie.trim(character).cloned()
    }

    pub(crate) fn record_str(mut self, characters: &str) -> (Self, TrimResult<OverlayState>) {
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
    direction: Direction,
    chars: String,
}

#[derive(Debug)]
pub(crate) enum InputResult {
    Recording(NCharInput),
    ShowTrie(String),
}

impl NCharInput {
    pub(crate) fn new(n: usize, direction: Direction) -> Self {
        Self {
            n,
            direction,
            chars: String::new(),
        }
    }

    pub(crate) fn direction(&self) -> Direction {
        self.direction
    }

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

    pub(crate) fn chars(&self) -> &str {
        self.chars.as_str()
    }
}

#[derive(Debug, Default)]
pub(crate) struct Pattern {
    direction: Direction,
    chars: String,
}

impl Pattern {
    pub(crate) fn new(direction: Direction) -> Self {
        Self {
            direction,
            chars: String::new(),
        }
    }

    pub(crate) fn direction(&self) -> Direction {
        self.direction
    }

    pub(crate) fn record_str(mut self, keys: &str) -> Self {
        self.chars.push_str(keys);
        self
    }

    pub(crate) fn chars(&self) -> &str {
        self.chars.as_str()
    }
}

#[cfg(test)]
mod tests {
    use crate::Direction;

    use super::{InputResult, NCharInput};

    #[test]
    fn test_record_str() {
        let char_input = NCharInput {
            n: 4,
            direction: Direction::BiDirectional,
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
