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

    /// Pushes a keystroke onto the matcher.
    /// The result of the new keystroke is returned:
    /// - KeyMatch::None =>
    ///         No match is valid for this key given any pending keystrokes.
    /// - KeyMatch::Pending =>
    ///         There exist bindings which are still waiting for more keys.
    /// - KeyMatch::Complete(matches) =>
    ///         One or more bindings have received the necessary key presses.
    ///         Bindings added later will take precedence over earlier bindings.
    pub(crate) fn match_keystroke(
        &mut self,
        keystroke: &Keystroke,
        context_stack: &[KeyContext],
    ) -> KeymatchResult {
        let keymap = self.keymap.borrow();

        // Clear pending keystrokes if the keymap has changed since the last matched keystroke.
        if keymap.version() != self.keymap_version {
            self.keymap_version = keymap.version();
            self.pending_keystrokes.clear();
        }

        let mut pending_key = None;
        let mut bindings = SmallVec::new();

        for binding in keymap.bindings().rev() {
            if !keymap.binding_enabled(binding, context_stack) {
                continue;
            }

            for candidate in keystroke.match_candidates() {
                self.pending_keystrokes.push(candidate.clone());
                match binding.match_keystrokes(&self.pending_keystrokes) {
                    KeyMatch::Matched => {
                        bindings.push(binding.clone());
                    }
                    KeyMatch::Pending => {
                        pending_key.get_or_insert(candidate);
                    }
                    KeyMatch::None => {}
                }
                self.pending_keystrokes.pop();
            }
        }

        if bindings.is_empty() && pending_key.is_none() && !self.pending_keystrokes.is_empty() {
            drop(keymap);
            self.pending_keystrokes.remove(0);
            return self.match_keystroke(keystroke, context_stack);
        }

        let pending = if let Some(pending_key) = pending_key {
            self.pending_keystrokes.push(pending_key);
            true
        } else {
            self.pending_keystrokes.clear();
            false
        };

        KeymatchResult { bindings, pending }
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
