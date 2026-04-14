use std::rc::Rc;

use crate::{
    Action, AsKeystroke, DummyKeyboardMapper, InvalidKeystrokeError, InvalidMouseStrokeError,
    InvalidScrollStrokeError, KeyBindingContextPredicate, KeybindingKeystroke, Keystroke,
    Modifiers, MouseButton, MouseStroke, PlatformKeyboardMapper, ScrollDirection, ScrollStroke,
    SharedString,
};
use smallvec::SmallVec;

/// The kind of input that triggers a binding.
#[derive(Clone, Debug, PartialEq)]
pub enum BindingInputKind {
    /// A sequence of keystrokes (standard keyboard binding).
    Keystrokes(SmallVec<[KeybindingKeystroke; 2]>),
    /// A mouse button click.
    Mouse(MouseStroke),
    /// A scroll wheel event.
    Scroll(ScrollStroke),
}

/// A keybinding and its associated metadata, from the keymap.
pub struct KeyBinding {
    pub(crate) action: Box<dyn Action>,
    pub(crate) input: BindingInputKind,
    pub(crate) context_predicate: Option<Rc<KeyBindingContextPredicate>>,
    pub(crate) meta: Option<KeyBindingMetaIndex>,
    /// The json input string used when building the keybinding, if any
    pub(crate) action_input: Option<SharedString>,
}

impl Clone for KeyBinding {
    fn clone(&self) -> Self {
        KeyBinding {
            action: self.action.boxed_clone(),
            input: self.input.clone(),
            context_predicate: self.context_predicate.clone(),
            meta: self.meta,
            action_input: self.action_input.clone(),
        }
    }
}

impl KeyBinding {
    /// Construct a new keystroke keybinding from the given data. Panics on parse error.
    pub fn new<A: Action>(keystrokes: &str, action: A, context: Option<&str>) -> Self {
        let context_predicate =
            context.map(|context| KeyBindingContextPredicate::parse(context).unwrap().into());
        Self::load(
            keystrokes,
            Box::new(action),
            context_predicate,
            false,
            None,
            &DummyKeyboardMapper,
        )
        .unwrap()
    }

    /// Load a keystroke keybinding from the given raw data.
    pub fn load(
        keystrokes: &str,
        action: Box<dyn Action>,
        context_predicate: Option<Rc<KeyBindingContextPredicate>>,
        use_key_equivalents: bool,
        action_input: Option<SharedString>,
        keyboard_mapper: &dyn PlatformKeyboardMapper,
    ) -> std::result::Result<Self, InvalidKeystrokeError> {
        let keystrokes: SmallVec<[KeybindingKeystroke; 2]> = keystrokes
            .split_whitespace()
            .map(|source| {
                let keystroke = Keystroke::parse(source)?;
                Ok(KeybindingKeystroke::new_with_mapper(
                    keystroke,
                    use_key_equivalents,
                    keyboard_mapper,
                ))
            })
            .collect::<std::result::Result<_, _>>()?;

        Ok(Self {
            input: BindingInputKind::Keystrokes(keystrokes),
            action,
            context_predicate,
            meta: None,
            action_input,
        })
    }

    /// Load a mouse binding from a string like "alt-mouse1".
    pub fn load_mouse(
        input: &str,
        action: Box<dyn Action>,
        context_predicate: Option<Rc<KeyBindingContextPredicate>>,
        action_input: Option<SharedString>,
    ) -> std::result::Result<Self, InvalidMouseStrokeError> {
        let stroke = MouseStroke::parse(input)?;
        Ok(Self {
            input: BindingInputKind::Mouse(stroke),
            action,
            context_predicate,
            meta: None,
            action_input,
        })
    }

    /// Load a scroll binding from a string like "ctrl-scroll-up".
    pub fn load_scroll(
        input: &str,
        action: Box<dyn Action>,
        context_predicate: Option<Rc<KeyBindingContextPredicate>>,
        action_input: Option<SharedString>,
    ) -> std::result::Result<Self, InvalidScrollStrokeError> {
        let stroke = ScrollStroke::parse(input)?;
        Ok(Self {
            input: BindingInputKind::Scroll(stroke),
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

    /// Returns true if this is a mouse binding.
    pub fn is_mouse_binding(&self) -> bool {
        matches!(self.input, BindingInputKind::Mouse(_))
    }

    /// Returns true if this is a scroll binding.
    pub fn is_scroll_binding(&self) -> bool {
        matches!(self.input, BindingInputKind::Scroll(_))
    }

    /// Check if the given keystrokes match this binding.
    pub fn match_keystrokes(&self, typed: &[impl AsKeystroke]) -> Option<bool> {
        let BindingInputKind::Keystrokes(keystrokes) = &self.input else {
            return None;
        };

        if keystrokes.len() < typed.len() {
            return None;
        }

        for (target, typed) in keystrokes.iter().zip(typed.iter()) {
            if !typed.as_keystroke().should_match(target) {
                return None;
            }
        }

        Some(keystrokes.len() > typed.len())
    }

    /// Check if this binding matches the given mouse event.
    pub fn match_mouse(
        &self,
        button: &MouseButton,
        modifiers: &Modifiers,
        click_count: usize,
    ) -> bool {
        if let BindingInputKind::Mouse(stroke) = &self.input {
            stroke.matches(button, modifiers, click_count)
        } else {
            false
        }
    }

    /// Check if this binding matches the given scroll event.
    pub fn match_scroll(&self, direction: ScrollDirection, modifiers: &Modifiers) -> bool {
        if let BindingInputKind::Scroll(stroke) = &self.input {
            stroke.matches(direction, modifiers)
        } else {
            false
        }
    }

    /// Get the keystrokes associated with this binding (empty slice for non-keystroke bindings).
    pub fn keystrokes(&self) -> &[KeybindingKeystroke] {
        if let BindingInputKind::Keystrokes(keystrokes) = &self.input {
            keystrokes.as_slice()
        } else {
            &[]
        }
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
            .field("input", &self.input)
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
