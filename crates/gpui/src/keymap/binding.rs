use std::rc::Rc;

use crate::{
    Action, EmptyKeyboardMapper, InvalidKeystrokeError, KeyBindingContextPredicate, KeyboardMapper,
    Keystroke,
};
use smallvec::SmallVec;

/// A keybinding and its associated metadata, from the keymap.
pub struct KeyBinding {
    pub(crate) action: Box<dyn Action>,
    pub(crate) keystrokes: SmallVec<[Keystroke; 2]>,
    pub(crate) context_predicate: Option<Rc<KeyBindingContextPredicate>>,
}

impl Clone for KeyBinding {
    fn clone(&self) -> Self {
        KeyBinding {
            action: self.action.boxed_clone(),
            keystrokes: self.keystrokes.clone(),
            context_predicate: self.context_predicate.clone(),
        }
    }
}

impl KeyBinding {
    /// Construct a new keybinding from the given data. Panics on parse error.
    pub fn new<A: Action>(keystrokes: &str, action: A, context: Option<&str>) -> Self {
        let context_predicate = if let Some(context) = context {
            Some(KeyBindingContextPredicate::parse(context).unwrap().into())
        } else {
            None
        };
        Self::load(
            keystrokes,
            Box::new(action),
            context_predicate,
            false,
            &EmptyKeyboardMapper,
        )
        .unwrap()
    }

    /// Load a keybinding from the given raw data.
    pub fn load(
        keystrokes: &str,
        action: Box<dyn Action>,
        context_predicate: Option<Rc<KeyBindingContextPredicate>>,
        // key_equivalents: Option<&HashMap<char, char>>,
        use_key_equivalents: bool,
        keyboard_mapper: &dyn KeyboardMapper,
    ) -> std::result::Result<Self, InvalidKeystrokeError> {
        let keystrokes: SmallVec<[Keystroke; 2]> = keystrokes
            .split_whitespace()
            .map(|source| {
                Keystroke::parse(source)
                    .map(|keystroke| keyboard_mapper.map_keystroke(keystroke, use_key_equivalents))
            })
            .collect::<std::result::Result<_, _>>()?;

        // if let Some(equivalents) = key_equivalents {
        //     for keystroke in keystrokes.iter_mut() {
        //         if keystroke.key.chars().count() == 1 {
        //             if let Some(key) = equivalents.get(&keystroke.key.chars().next().unwrap()) {
        //                 keystroke.key = key.to_string();
        //             }
        //         }
        //     }
        // }

        Ok(Self {
            keystrokes,
            action,
            context_predicate,
        })
    }

    /// Check if the given keystrokes match this binding.
    pub fn match_keystrokes(&self, typed: &[Keystroke]) -> Option<bool> {
        if self.keystrokes.len() < typed.len() {
            return None;
        }

        for (target, typed) in self.keystrokes.iter().zip(typed.iter()) {
            if !typed.should_match(target) {
                return None;
            }
        }

        Some(self.keystrokes.len() > typed.len())
    }

    /// Get the keystrokes associated with this binding
    pub fn keystrokes(&self) -> &[Keystroke] {
        self.keystrokes.as_slice()
    }

    /// Get the action associated with this binding
    pub fn action(&self) -> &dyn Action {
        self.action.as_ref()
    }

    /// Get the predicate used to match this binding
    pub fn predicate(&self) -> Option<Rc<KeyBindingContextPredicate>> {
        self.context_predicate.as_ref().map(|rc| rc.clone())
    }
}

impl std::fmt::Debug for KeyBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyBinding")
            .field("keystrokes", &self.keystrokes)
            .field("context_predicate", &self.context_predicate)
            .field("action", &self.action.name())
            .finish()
    }
}
