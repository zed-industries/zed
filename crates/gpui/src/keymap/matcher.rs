use crate::{KeyBinding, KeyContext, Keymap, KeymapVersion, Keystroke};
use smallvec::SmallVec;
use std::{cell::RefCell, rc::Rc};

pub(crate) struct KeystrokeMatcher {
    pending_keystrokes: Vec<Keystroke>,
    keymap: Rc<RefCell<Keymap>>,
    keymap_version: KeymapVersion,
}

pub struct KeymatchResult {
    pub bindings: SmallVec<[KeyBinding; 1]>,
    pub pending: bool,
}

impl KeystrokeMatcher {
    pub fn new(keymap: Rc<RefCell<Keymap>>) -> Self {
        let keymap_version = keymap.borrow().version();
        Self {
            pending_keystrokes: Vec::new(),
            keymap_version,
            keymap,
        }
    }

    pub fn has_pending_keystrokes(&self) -> bool {
        !self.pending_keystrokes.is_empty()
    }
}

/// The result of matching a keystroke against a given keybinding.
/// - KeyMatch::None => No match is valid for this key given any pending keystrokes.
/// - KeyMatch::Pending => There exist bindings that is still waiting for more keys.
/// - KeyMatch::Some(matches) => One or more bindings have received the necessary key presses.
#[derive(Debug, PartialEq)]
pub enum KeyMatch {
    None,
    Pending,
    Matched,
}
