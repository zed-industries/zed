use crate::{Action, KeyContext, Keymap, KeymapVersion, Keystroke};
use parking_lot::Mutex;
use smallvec::SmallVec;
use std::sync::Arc;

pub(crate) struct KeystrokeMatcher {
    pending_keystrokes: Vec<Keystroke>,
    keymap: Arc<Mutex<Keymap>>,
    keymap_version: KeymapVersion,
}

pub struct KeymatchResult {
    pub actions: SmallVec<[Box<dyn Action>; 1]>,
    pub pending: bool,
}

impl KeystrokeMatcher {
    pub fn new(keymap: Arc<Mutex<Keymap>>) -> Self {
        let keymap_version = keymap.lock().version();
        Self {
            pending_keystrokes: Vec::new(),
            keymap_version,
            keymap,
        }
    }

    pub fn clear_pending(&mut self) {
        self.pending_keystrokes.clear();
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
        let keymap = self.keymap.lock();
        // Clear pending keystrokes if the keymap has changed since the last matched keystroke.
        if keymap.version() != self.keymap_version {
            self.keymap_version = keymap.version();
            self.pending_keystrokes.clear();
        }

        let mut pending_key = None;
        let mut actions = SmallVec::new();

        for binding in keymap.bindings().rev() {
            if !keymap.binding_enabled(binding, context_stack) {
                continue;
            }

            for candidate in keystroke.match_candidates() {
                self.pending_keystrokes.push(candidate.clone());
                match binding.match_keystrokes(&self.pending_keystrokes) {
                    KeyMatch::Matched => {
                        actions.push(binding.action.boxed_clone());
                    }
                    KeyMatch::Pending => {
                        pending_key.get_or_insert(candidate);
                    }
                    KeyMatch::None => {}
                }
                self.pending_keystrokes.pop();
            }
        }

        let pending = if let Some(pending_key) = pending_key {
            self.pending_keystrokes.push(pending_key);
            true
        } else {
            self.pending_keystrokes.clear();
            false
        };

        KeymatchResult { actions, pending }
    }
}

/// The result of matching a keystroke against a given keybinding.
/// - KeyMatch::None => No match is valid for this key given any pending keystrokes.
/// - KeyMatch::Pending => There exist bindings that is still waiting for more keys.
/// - KeyMatch::Some(matches) => One or more bindings have received the necessary key presses.
#[derive(Debug)]
pub enum KeyMatch {
    None,
    Pending,
    Matched,
}

