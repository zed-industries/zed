use crate::{Action, KeyBindingContextPredicate, Keystroke, PlatformKeyboard};
use anyhow::Result;
use smallvec::SmallVec;

/// A keybinding and its associated metadata, from the keymap.
pub struct KeyBinding {
    pub(crate) action: Box<dyn Action>,
    pub(crate) keystrokes: SmallVec<[Keystroke; 2]>,
    pub(crate) context_predicate: Option<KeyBindingContextPredicate>,
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
    /// Construct a new keybinding from the given data.
    pub fn new<A: Action>(
        keystrokes: &str,
        action: A,
        context_predicate: Option<&str>,
        keyboard_manager: &dyn PlatformKeyboard,
    ) -> Self {
        Self::load(
            keystrokes,
            Box::new(action),
            context_predicate,
            keyboard_manager,
        )
        .unwrap()
    }

    /// Load a keybinding from the given raw data.
    pub fn load(
        keystrokes: &str,
        action: Box<dyn Action>,
        context: Option<&str>,
        keyboard_manager: &dyn PlatformKeyboard,
    ) -> Result<Self> {
        let context = if let Some(context) = context {
            Some(KeyBindingContextPredicate::parse(context)?)
        } else {
            None
        };

        let keystrokes = keystrokes
            .split_whitespace()
            .map(|keystroke| Keystroke::parse(keystroke, keyboard_manager))
            .collect::<Result<_>>()?;

        Ok(Self {
            keystrokes,
            action,
            context_predicate: context,
        })
    }

    /// Check if the given keystrokes match this binding.
    pub fn match_keystrokes(&self, typed: &[Keystroke]) -> Option<bool> {
        if self.keystrokes.len() < typed.len() {
            return None;
        }

        // TODO:
        // should_match?
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
