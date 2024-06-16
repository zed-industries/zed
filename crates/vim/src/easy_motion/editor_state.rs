use gpui::{HighlightStyle, KeyContext};

use crate::easy_motion::trie::{Trie, TrimResult};

#[derive(Debug, Default, Clone)]
pub(crate) struct OverlayState {
    pub style: HighlightStyle,
    pub offset: usize,
}

#[derive(Debug)]
pub(crate) struct EasyMotionState {
    selection: String,
    trie: Trie<OverlayState>,
}

impl EasyMotionState {
    pub(crate) fn new(trie: Trie<OverlayState>) -> Self {
        Self {
            selection: String::new(),
            trie,
        }
    }

    pub(crate) fn keymap_context_layer(&self) -> KeyContext {
        let mut context = KeyContext::new_with_defaults();
        context.add("VimWaiting");
        return context;
    }

    #[allow(dead_code)]
    pub(crate) fn trie(&self) -> &Trie<OverlayState> {
        &self.trie
    }

    #[allow(dead_code)]
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
