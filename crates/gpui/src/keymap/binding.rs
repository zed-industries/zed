use std::rc::Rc;

use crate::{
    Action, AsKeystroke, DummyKeyboardMapper, InvalidKeystrokeError, InvalidMouseInputError,
    InvalidScrollInputError, KeyBindingContextPredicate, KeybindingKeystroke, Keystroke,
    Modifiers, MouseButton, MouseInput, PlatformKeyboardMapper, ScrollDirection, ScrollInput,
    SharedString,
};
use smallvec::SmallVec;

/// The input that triggers a binding: a keystroke sequence, a single modified
/// mouse click, or a single modified scroll event.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BindingInput {
    /// A sequence of one or more keystrokes (e.g. `ctrl-k ctrl-c`).
    Keystrokes(SmallVec<[KeybindingKeystroke; 2]>),
    /// A single modified mouse click.
    Mouse(MouseInput),
    /// A single modified scroll event.
    Scroll(ScrollInput),
}

/// Error returned when a binding input source string fails to parse.
#[derive(Debug)]
pub enum BindingInputParseError {
    /// One of the tokens in a keystroke chord failed to parse.
    Keystroke(InvalidKeystrokeError),
    /// The source was recognized as a mouse input but was malformed.
    Mouse(InvalidMouseInputError),
    /// The source was recognized as a scroll input but was malformed.
    Scroll(InvalidScrollInputError),
}

impl std::fmt::Display for BindingInputParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Keystroke(e) => std::fmt::Display::fmt(e, f),
            Self::Mouse(e) => std::fmt::Display::fmt(e, f),
            Self::Scroll(e) => std::fmt::Display::fmt(e, f),
        }
    }
}

impl std::error::Error for BindingInputParseError {}

impl From<InvalidKeystrokeError> for BindingInputParseError {
    fn from(e: InvalidKeystrokeError) -> Self {
        Self::Keystroke(e)
    }
}

impl From<InvalidMouseInputError> for BindingInputParseError {
    fn from(e: InvalidMouseInputError) -> Self {
        Self::Mouse(e)
    }
}

impl From<InvalidScrollInputError> for BindingInputParseError {
    fn from(e: InvalidScrollInputError) -> Self {
        Self::Scroll(e)
    }
}

impl BindingInput {
    /// Parse a binding input source string from a keymap. The kind of input is
    /// determined by the source's grammar:
    ///
    /// * Whitespace-separated tokens always denote a keystroke chord.
    /// * A single token whose trailing component is a mouse button (e.g.
    ///   `cmd-mouse1`) is a [`MouseInput`].
    /// * A single token of the form `<modifiers->scroll-<direction>` is a
    ///   [`ScrollInput`].
    /// * Anything else is parsed as a single-keystroke chord.
    pub fn parse(
        source: &str,
        use_key_equivalents: bool,
        keyboard_mapper: &dyn PlatformKeyboardMapper,
    ) -> Result<Self, BindingInputParseError> {
        if !source.contains(char::is_whitespace) {
            if let Some(input) = MouseInput::try_parse(source)? {
                return Ok(Self::Mouse(input));
            }
            if let Some(input) = ScrollInput::try_parse(source)? {
                return Ok(Self::Scroll(input));
            }
        }
        let keystrokes: SmallVec<[KeybindingKeystroke; 2]> = source
            .split_whitespace()
            .map(|token| {
                let keystroke = Keystroke::parse(token)?;
                Ok(KeybindingKeystroke::new_with_mapper(
                    keystroke,
                    use_key_equivalents,
                    keyboard_mapper,
                ))
            })
            .collect::<Result<_, InvalidKeystrokeError>>()?;
        Ok(Self::Keystrokes(keystrokes))
    }
}

/// A keybinding and its associated metadata, from the keymap.
pub struct KeyBinding {
    pub(crate) action: Box<dyn Action>,
    pub(crate) input: BindingInput,
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
    /// Construct a new binding from the given source string. Panics on parse
    /// error. Intended for test code and inline definitions.
    pub fn new<A: Action>(source: &str, action: A, context: Option<&str>) -> Self {
        let context_predicate = context.map(|context| {
            Rc::new(
                KeyBindingContextPredicate::parse(context).expect("invalid context predicate"),
            )
        });
        Self::load(
            source,
            Box::new(action),
            context_predicate,
            false,
            None,
            &DummyKeyboardMapper,
        )
        .expect("invalid binding input")
    }

    /// Load a binding from the given raw data. The source is auto-classified
    /// as a keystroke chord, a mouse input, or a scroll input via
    /// [`BindingInput::parse`].
    pub fn load(
        source: &str,
        action: Box<dyn Action>,
        context_predicate: Option<Rc<KeyBindingContextPredicate>>,
        use_key_equivalents: bool,
        action_input: Option<SharedString>,
        keyboard_mapper: &dyn PlatformKeyboardMapper,
    ) -> Result<Self, BindingInputParseError> {
        Ok(Self {
            input: BindingInput::parse(source, use_key_equivalents, keyboard_mapper)?,
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
        matches!(self.input, BindingInput::Keystrokes(_))
    }

    /// Returns true if this is a mouse binding
    pub fn is_mouse_binding(&self) -> bool {
        matches!(self.input, BindingInput::Mouse(_))
    }

    /// Returns true if this is a scroll binding
    pub fn is_scroll_binding(&self) -> bool {
        matches!(self.input, BindingInput::Scroll(_))
    }

    /// Get a display string for the binding input (works for all binding types)
    pub fn input_display_string(&self) -> String {
        match &self.input {
            BindingInput::Keystrokes(keystrokes) => keystrokes
                .iter()
                .map(|k| k.to_string())
                .collect::<Vec<_>>()
                .join(" "),
            BindingInput::Mouse(mouse_input) => mouse_input.to_string(),
            BindingInput::Scroll(scroll_input) => scroll_input.to_string(),
        }
    }

    /// Check if the given keystrokes match this binding.
    /// Returns None if this is not a keystroke binding or no match.
    /// Returns Some(true) if partial match (more keystrokes needed).
    /// Returns Some(false) if complete match.
    pub fn match_keystrokes(&self, typed: &[impl AsKeystroke]) -> Option<bool> {
        let keystrokes = match &self.input {
            BindingInput::Keystrokes(ks) => ks,
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
            BindingInput::Mouse(mouse_input) => {
                mouse_input.matches(button, modifiers, click_count)
            }
            _ => false,
        }
    }

    /// Check if a scroll event matches this binding.
    pub fn match_scroll(&self, direction: ScrollDirection, modifiers: Modifiers) -> bool {
        match &self.input {
            BindingInput::Scroll(scroll_input) => scroll_input.matches(direction, modifiers),
            _ => false,
        }
    }

    /// Get the input kind for this binding
    pub fn input(&self) -> &BindingInput {
        &self.input
    }

    /// Get the keystrokes associated with this binding (if it's a keystroke binding)
    pub fn keystrokes(&self) -> &[KeybindingKeystroke] {
        match &self.input {
            BindingInput::Keystrokes(ks) => ks.as_slice(),
            _ => &[],
        }
    }

    /// Get the mouse input if this is a mouse binding
    pub fn mouse_input(&self) -> Option<&MouseInput> {
        match &self.input {
            BindingInput::Mouse(ms) => Some(ms),
            _ => None,
        }
    }

    /// Get the scroll input if this is a scroll binding
    pub fn scroll_input(&self) -> Option<&ScrollInput> {
        match &self.input {
            BindingInput::Scroll(ss) => Some(ss),
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
