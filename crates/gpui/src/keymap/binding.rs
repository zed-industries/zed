use std::rc::Rc;

use crate::{
    Action, AsKeystroke, DummyKeyboardMapper, InvalidKeystrokeError, KeyBindingContextPredicate,
    KeybindingKeystroke, Keystroke, PlatformKeyboardMapper, SharedString,
};
use smallvec::SmallVec;

/// A keybinding and its associated metadata, from the keymap.
pub struct KeyBinding {
    pub(crate) action: Box<dyn Action>,
    pub(crate) keystrokes: SmallVec<[KeybindingKeystroke; 2]>,
    pub(crate) context_predicate: Option<Rc<KeyBindingContextPredicate>>,
    pub(crate) meta: Option<KeyBindingMetaIndex>,
    /// The json input string used when building the keybinding, if any
    pub(crate) action_input: Option<SharedString>,
}

impl Clone for KeyBinding {
    fn clone(&self) -> Self {
        KeyBinding {
            action: self.action.boxed_clone(),
            keystrokes: self.keystrokes.clone(),
            context_predicate: self.context_predicate.clone(),
            meta: self.meta,
            action_input: self.action_input.clone(),
        }
    }
}

impl KeyBinding {
    /// Constructs a static keybinding with explicit modifiers. Panics on parse error.
    ///
    /// Use [`Self::try_new`] for fallible parsing, or [`Self::load`] to resolve
    /// `secondary-` using the host OS.
    pub fn new<A: Action>(keystrokes: &'static str, action: A, context: Option<&str>) -> Self {
        Self::try_new(keystrokes, action, context).unwrap()
    }

    /// Constructs a static keybinding with explicit modifiers, returning an error
    /// if the keystrokes or context predicate fail to parse.
    ///
    /// Use [`Self::load`] to resolve `secondary-` using the host OS or apply a keyboard mapper.
    pub fn try_new<A: Action>(
        keystrokes: &str,
        action: A,
        context: Option<&str>,
    ) -> anyhow::Result<Self> {
        let context_predicate = context
            .map(|context| KeyBindingContextPredicate::parse(context).map(Rc::new))
            .transpose()?;
        Ok(Self::load_internal(
            keystrokes,
            Box::new(action),
            context_predicate,
            false,
            None,
            &DummyKeyboardMapper,
            |source| Keystroke::parse_internal(source, None),
        )?)
    }

    /// Load a keybinding from the given raw data.
    pub fn load(
        keystrokes: &str,
        action: Box<dyn Action>,
        context_predicate: Option<Rc<KeyBindingContextPredicate>>,
        use_key_equivalents: bool,
        action_input: Option<SharedString>,
        cx: &crate::App,
    ) -> std::result::Result<Self, InvalidKeystrokeError> {
        Self::load_internal(
            keystrokes,
            action,
            context_predicate,
            use_key_equivalents,
            action_input,
            cx.keyboard_mapper().as_ref(),
            |source| Keystroke::parse_with_os(source, cx.platform_info().os),
        )
    }

    fn load_internal<'a>(
        keystrokes: &'a str,
        action: Box<dyn Action>,
        context_predicate: Option<Rc<KeyBindingContextPredicate>>,
        use_key_equivalents: bool,
        action_input: Option<SharedString>,
        keyboard_mapper: &dyn PlatformKeyboardMapper,
        parse_keystroke: impl Fn(&'a str) -> std::result::Result<Keystroke, InvalidKeystrokeError>,
    ) -> std::result::Result<Self, InvalidKeystrokeError> {
        let keystrokes: SmallVec<[KeybindingKeystroke; 2]> = keystrokes
            .split_whitespace()
            .map(|source| {
                let keystroke = parse_keystroke(source)?;
                Ok(KeybindingKeystroke::new_with_mapper(
                    keystroke,
                    use_key_equivalents,
                    keyboard_mapper,
                ))
            })
            .collect::<std::result::Result<_, _>>()?;

        Ok(Self {
            keystrokes,
            action,
            context_predicate,
            meta: None,
            action_input,
        })
    }

    /// Set the metadata for this binding.
    pub fn with_meta(mut self, meta: KeyBindingMetaIndex) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Set the metadata for this binding.
    pub fn set_meta(&mut self, meta: KeyBindingMetaIndex) {
        self.meta = Some(meta);
    }

    /// Check if the given keystrokes match this binding.
    pub fn match_keystrokes(&self, typed: &[impl AsKeystroke]) -> Option<bool> {
        if self.keystrokes.len() < typed.len() {
            return None;
        }

        for (target, typed) in self.keystrokes.iter().zip(typed.iter()) {
            if !typed.as_keystroke().should_match(target) {
                return None;
            }
        }

        Some(self.keystrokes.len() > typed.len())
    }

    /// Get the keystrokes associated with this binding
    pub fn keystrokes(&self) -> &[KeybindingKeystroke] {
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

    /// Get the metadata for this binding
    pub fn meta(&self) -> Option<KeyBindingMetaIndex> {
        self.meta
    }

    /// Get the action input associated with the action for this binding
    pub fn action_input(&self) -> Option<SharedString> {
        self.action_input.clone()
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

/// A unique identifier for retrieval of metadata associated with a key binding.
/// Intended to be used as an index or key into a user-defined store of metadata
/// associated with the binding, such as the source of the binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyBindingMetaIndex(pub u32);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NoAction;

    #[test]
    fn test_try_new() {
        let dynamic_keystroke = String::from("ctrl-a cmd-b");
        let binding = KeyBinding::try_new(&dynamic_keystroke, NoAction, Some("editor && !menu"))
            .expect("valid keybinding should parse");
        assert_eq!(binding.keystrokes().len(), 2);
        assert!(binding.predicate().is_some());

        // Unknown modifier returns Err instead of panicking.
        assert!(KeyBinding::try_new("foo-b", NoAction, None).is_err());

        // `secondary-` requires an OS via `KeyBinding::load`.
        assert!(KeyBinding::try_new("secondary-a", NoAction, None).is_err());

        // Invalid context predicate returns Err instead of panicking.
        assert!(KeyBinding::try_new("ctrl-a", NoAction, Some("editor &&")).is_err());
    }
}
