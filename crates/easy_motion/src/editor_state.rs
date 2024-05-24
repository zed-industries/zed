use editor::DisplayPoint;
use gpui::{HighlightStyle, KeyContext};

use crate::perm::{Trie, TrimResult};

#[derive(Debug, Default)]
pub(crate) struct OverlayState {
    pub style: HighlightStyle,
    pub point: DisplayPoint,
}

#[derive(Debug, Default)]
pub(crate) struct EditorState {
    pub control: bool,
    pub current_trie: Option<Trie<OverlayState>>,
}

impl EditorState {
    #[allow(dead_code)]
    pub(crate) fn trie(&self) -> Option<&Trie<OverlayState>> {
        self.current_trie.as_ref()
    }

    pub(crate) fn easy_motion_controlled(&self) -> bool {
        return self.control;
    }

    pub(crate) fn keymap_context_layer(&self) -> KeyContext {
        let mut context = KeyContext::new_with_defaults();
        if self.easy_motion_controlled() {
            context.add("EasyMotionControlled");
            context.add("menu");
        }
        return context;
    }

    pub(crate) fn record_char(&mut self, character: char) -> TrimResult<DisplayPoint> {
        let Some(trie) = &mut self.current_trie else {
            return TrimResult::Err;
        };
        match trie.trim(character) {
            TrimResult::NoChange => TrimResult::NoChange,
            TrimResult::Changed => TrimResult::Changed,
            TrimResult::Found(overlay) => return TrimResult::Found(overlay.point.clone()),
            TrimResult::Err => return TrimResult::Err,
        }
    }

    pub(crate) fn record_str(&mut self, characters: &str) -> TrimResult<DisplayPoint> {
        let mut changed = false;
        for character in characters.chars() {
            let ret = self.record_char(character);
            match ret {
                TrimResult::NoChange => {}
                TrimResult::Changed => changed = true,
                TrimResult::Found(point) => return TrimResult::Found(point),
                TrimResult::Err => return TrimResult::Err,
            };
        }
        if changed {
            TrimResult::Changed
        } else {
            TrimResult::NoChange
        }
    }
}
