use std::rc::Rc;

use crate::{
    Action, AsKeystroke, DummyKeyboardMapper, InvalidKeystrokeError, KeyBindingContextPredicate,
    KeybindingKeystroke, Keystroke, Modifiers, MouseButton, MouseStroke, PlatformKeyboardMapper,
    ScrollDirection, ScrollStroke, SharedString,
};
use smallvec::SmallVec;

/// The type of input that triggers a binding
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BindingInputKind {
    /// A sequence of keystrokes (e.g., "ctrl-k ctrl-c")
    Keystrokes(SmallVec<[KeybindingKeystroke; 2]>),
    /// A mouse click with modifiers
    Mouse(MouseStroke),
    /// A scroll event with modifiers
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
    /// Construct a new keybinding from the given data. Panics on parse error.
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

    /// Construct a new mouse binding. Panics on parse error.
    pub fn new_mouse<A: Action>(input: &str, action: A, context: Option<&str>) -> Self {
        let context_predicate =
            context.map(|context| KeyBindingContextPredicate::parse(context).unwrap().into());
        let mouse_stroke = MouseStroke::parse(input).expect("Invalid mouse stroke");
        Self {
            action: Box::new(action),
            input: BindingInputKind::Mouse(mouse_stroke),
            context_predicate,
            meta: None,
            action_input: None,
        }
    }

    /// Construct a new scroll binding. Panics on parse error.
    pub fn new_scroll<A: Action>(input: &str, action: A, context: Option<&str>) -> Self {
        let context_predicate =
            context.map(|context| KeyBindingContextPredicate::parse(context).unwrap().into());
        let scroll_stroke = ScrollStroke::parse(input).expect("Invalid scroll stroke");
        Self {
            action: Box::new(action),
            input: BindingInputKind::Scroll(scroll_stroke),
            context_predicate,
            meta: None,
            action_input: None,
        }
    }

    /// Load a keybinding from the given raw data.
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

    /// Load a mouse binding from the given raw data.
    pub fn load_mouse(
        input: &str,
        action: Box<dyn Action>,
        context_predicate: Option<Rc<KeyBindingContextPredicate>>,
        action_input: Option<SharedString>,
    ) -> std::result::Result<Self, crate::InvalidMouseStrokeError> {
        let mouse_stroke = MouseStroke::parse(input)?;
        Ok(Self {
            input: BindingInputKind::Mouse(mouse_stroke),
            action,
            context_predicate,
            meta: None,
            action_input,
        })
    }

    /// Load a scroll binding from the given raw data.
    pub fn load_scroll(
        input: &str,
        action: Box<dyn Action>,
        context_predicate: Option<Rc<KeyBindingContextPredicate>>,
        action_input: Option<SharedString>,
    ) -> std::result::Result<Self, crate::InvalidScrollStrokeError> {
        let scroll_stroke = ScrollStroke::parse(input)?;
        Ok(Self {
            input: BindingInputKind::Scroll(scroll_stroke),
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

    /// Returns true if this is a keyboard binding
    pub fn is_key_binding(&self) -> bool {
        matches!(self.input, BindingInputKind::Keystrokes(_))
    }

    /// Returns true if this is a mouse binding
    pub fn is_mouse_binding(&self) -> bool {
        matches!(self.input, BindingInputKind::Mouse(_))
    }

    /// Returns true if this is a scroll binding
    pub fn is_scroll_binding(&self) -> bool {
        matches!(self.input, BindingInputKind::Scroll(_))
    }

    /// Check if the given keystrokes match this binding.
    /// Returns None if this is not a keystroke binding or no match.
    /// Returns Some(true) if partial match (more keystrokes needed).
    /// Returns Some(false) if complete match.
    pub fn match_keystrokes(&self, typed: &[impl AsKeystroke]) -> Option<bool> {
        let keystrokes = match &self.input {
            BindingInputKind::Keystrokes(ks) => ks,
            _ => return None,
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

    /// Check if a mouse event matches this binding.
    pub fn match_mouse(
        &self,
        button: MouseButton,
        modifiers: Modifiers,
        click_count: usize,
    ) -> bool {
        match &self.input {
            BindingInputKind::Mouse(mouse_stroke) => {
                mouse_stroke.matches(button, modifiers, click_count)
            }
            _ => false,
        }
    }

    /// Check if a scroll event matches this binding.
    pub fn match_scroll(&self, direction: ScrollDirection, modifiers: Modifiers) -> bool {
        match &self.input {
            BindingInputKind::Scroll(scroll_stroke) => scroll_stroke.matches(direction, modifiers),
            _ => false,
        }
    }

    /// Get the keystrokes associated with this binding (if it's a keystroke binding)
    pub fn keystrokes(&self) -> &[KeybindingKeystroke] {
        match &self.input {
            BindingInputKind::Keystrokes(ks) => ks.as_slice(),
            _ => &[],
        }
    }

    /// Get the mouse stroke if this is a mouse binding
    pub fn mouse_stroke(&self) -> Option<&MouseStroke> {
        match &self.input {
            BindingInputKind::Mouse(ms) => Some(ms),
            _ => None,
        }
    }

    /// Get the scroll stroke if this is a scroll binding
    pub fn scroll_stroke(&self) -> Option<&ScrollStroke> {
        match &self.input {
            BindingInputKind::Scroll(ss) => Some(ss),
            _ => None,
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
