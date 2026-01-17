use std::error::Error;
use std::fmt::Display;

use crate::{
    Keystroke, Modifiers, MouseStroke, ScrollStroke,
    looks_like_mouse_stroke, looks_like_scroll_stroke,
};

/// A single input that can trigger a binding - either keyboard, mouse click, or scroll.
///
/// This enum unifies keyboard, mouse, and scroll inputs so they can be configured
/// in the same keymap file with a consistent syntax.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum BindingInput {
    /// A keyboard keystroke (e.g., "ctrl-s", "cmd-shift-p")
    Keystroke(Keystroke),
    /// A mouse click (e.g., "mouse1", "alt-mouse1", "double-mouse1")
    Mouse(MouseStroke),
    /// A scroll event with modifiers (e.g., "ctrl-scroll-up")
    Scroll(ScrollStroke),
}

/// Error type for `BindingInput::parse`
#[derive(Debug)]
pub struct InvalidBindingInputError {
    /// The invalid input string
    pub input: String,
    /// A description of what went wrong
    pub message: String,
}

impl Error for InvalidBindingInputError {}

impl Display for InvalidBindingInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid binding input \"{}\": {}", self.input, self.message)
    }
}

impl BindingInput {
    /// Parse an input string, auto-detecting the type (keystroke, mouse, or scroll).
    ///
    /// The parser checks for mouse and scroll patterns first, then falls back to
    /// keyboard keystroke parsing.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Keyboard
    /// BindingInput::parse("ctrl-s") // -> Keystroke
    /// BindingInput::parse("cmd-shift-p") // -> Keystroke
    ///
    /// // Mouse
    /// BindingInput::parse("mouse1") // -> Mouse (left click)
    /// BindingInput::parse("alt-mouse1") // -> Mouse (alt + left click)
    /// BindingInput::parse("double-mouse1") // -> Mouse (double click)
    ///
    /// // Scroll
    /// BindingInput::parse("ctrl-scroll-up") // -> Scroll
    /// BindingInput::parse("cmd-scroll-down") // -> Scroll
    /// ```
    pub fn parse(source: &str) -> Result<Self, InvalidBindingInputError> {
        // Check for mouse input first (contains "mouse")
        if looks_like_mouse_stroke(source) {
            return MouseStroke::parse(source)
                .map(BindingInput::Mouse)
                .map_err(|e| InvalidBindingInputError {
                    input: source.to_owned(),
                    message: e.to_string(),
                });
        }

        // Check for scroll input (contains "scroll-")
        if looks_like_scroll_stroke(source) {
            return ScrollStroke::parse(source)
                .map(BindingInput::Scroll)
                .map_err(|e| InvalidBindingInputError {
                    input: source.to_owned(),
                    message: e.to_string(),
                });
        }

        // Default to keystroke parsing
        Keystroke::parse(source)
            .map(BindingInput::Keystroke)
            .map_err(|e| InvalidBindingInputError {
                input: source.to_owned(),
                message: e.to_string(),
            })
    }

    /// Returns true if this is a keyboard keystroke
    pub fn is_keystroke(&self) -> bool {
        matches!(self, BindingInput::Keystroke(_))
    }

    /// Returns true if this is a mouse click
    pub fn is_mouse(&self) -> bool {
        matches!(self, BindingInput::Mouse(_))
    }

    /// Returns true if this is a scroll input
    pub fn is_scroll(&self) -> bool {
        matches!(self, BindingInput::Scroll(_))
    }

    /// Returns the keystroke if this is a keyboard input
    pub fn as_keystroke(&self) -> Option<&Keystroke> {
        match self {
            BindingInput::Keystroke(k) => Some(k),
            _ => None,
        }
    }

    /// Returns the mouse stroke if this is a mouse input
    pub fn as_mouse(&self) -> Option<&MouseStroke> {
        match self {
            BindingInput::Mouse(m) => Some(m),
            _ => None,
        }
    }

    /// Returns the scroll stroke if this is a scroll input
    pub fn as_scroll(&self) -> Option<&ScrollStroke> {
        match self {
            BindingInput::Scroll(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the modifiers for this input
    pub fn modifiers(&self) -> Modifiers {
        match self {
            BindingInput::Keystroke(k) => k.modifiers,
            BindingInput::Mouse(m) => m.modifiers,
            BindingInput::Scroll(s) => s.modifiers,
        }
    }

    /// Produces a string representation that can be parsed back
    pub fn unparse(&self) -> String {
        match self {
            BindingInput::Keystroke(k) => k.unparse(),
            BindingInput::Mouse(m) => m.unparse(),
            BindingInput::Scroll(s) => s.unparse(),
        }
    }
}

impl From<Keystroke> for BindingInput {
    fn from(keystroke: Keystroke) -> Self {
        BindingInput::Keystroke(keystroke)
    }
}

impl From<MouseStroke> for BindingInput {
    fn from(mouse_stroke: MouseStroke) -> Self {
        BindingInput::Mouse(mouse_stroke)
    }
}

impl From<ScrollStroke> for BindingInput {
    fn from(scroll_stroke: ScrollStroke) -> Self {
        BindingInput::Scroll(scroll_stroke)
    }
}

/// Validate that a sequence of binding inputs is valid.
///
/// Rules:
/// - Mouse and scroll bindings cannot be chained (must be single inputs)
/// - Keyboard keystrokes can be chained (e.g., "ctrl-k ctrl-c")
/// - Cannot mix mouse/scroll with keyboard in a sequence
pub fn validate_binding_inputs(inputs: &[BindingInput]) -> Result<(), String> {
    if inputs.is_empty() {
        return Err("Binding must have at least one input".to_string());
    }

    let has_mouse = inputs.iter().any(|i| i.is_mouse());
    let has_scroll = inputs.iter().any(|i| i.is_scroll());
    let has_keystroke = inputs.iter().any(|i| i.is_keystroke());

    // Mouse and scroll bindings must be single inputs
    if has_mouse && inputs.len() > 1 {
        return Err("Mouse bindings cannot be part of a sequence".to_string());
    }

    if has_scroll && inputs.len() > 1 {
        return Err("Scroll bindings cannot be part of a sequence".to_string());
    }

    // Cannot mix types
    if (has_mouse || has_scroll) && has_keystroke {
        return Err("Cannot mix mouse/scroll with keyboard inputs in a binding".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ScrollDirection;

    #[test]
    fn test_parse_keystroke() {
        let input = BindingInput::parse("ctrl-s").unwrap();
        assert!(input.is_keystroke());
        assert!(!input.is_mouse());
        assert!(!input.is_scroll());
    }

    #[test]
    fn test_parse_mouse() {
        let input = BindingInput::parse("mouse1").unwrap();
        assert!(input.is_mouse());
        assert!(!input.is_keystroke());

        let input = BindingInput::parse("alt-mouse1").unwrap();
        assert!(input.is_mouse());

        let input = BindingInput::parse("double-mouse1").unwrap();
        assert!(input.is_mouse());
        let mouse = input.as_mouse().unwrap();
        assert_eq!(mouse.click_count, crate::ClickCount::Double);
    }

    #[test]
    fn test_parse_scroll() {
        let input = BindingInput::parse("scroll-up").unwrap();
        assert!(input.is_scroll());

        let input = BindingInput::parse("ctrl-scroll-down").unwrap();
        assert!(input.is_scroll());
        let scroll = input.as_scroll().unwrap();
        assert!(scroll.modifiers.control);
        assert_eq!(scroll.direction, ScrollDirection::Down);
    }

    #[test]
    fn test_validate_single_inputs() {
        let inputs = vec![BindingInput::parse("ctrl-s").unwrap()];
        assert!(validate_binding_inputs(&inputs).is_ok());

        let inputs = vec![BindingInput::parse("mouse1").unwrap()];
        assert!(validate_binding_inputs(&inputs).is_ok());

        let inputs = vec![BindingInput::parse("ctrl-scroll-up").unwrap()];
        assert!(validate_binding_inputs(&inputs).is_ok());
    }

    #[test]
    fn test_validate_keystroke_sequence() {
        let inputs = vec![
            BindingInput::parse("ctrl-k").unwrap(),
            BindingInput::parse("ctrl-c").unwrap(),
        ];
        assert!(validate_binding_inputs(&inputs).is_ok());
    }

    #[test]
    fn test_validate_invalid_mouse_sequence() {
        let inputs = vec![
            BindingInput::parse("mouse1").unwrap(),
            BindingInput::parse("mouse2").unwrap(),
        ];
        assert!(validate_binding_inputs(&inputs).is_err());
    }

    #[test]
    fn test_validate_invalid_mixed() {
        let inputs = vec![
            BindingInput::parse("ctrl-k").unwrap(),
            BindingInput::parse("mouse1").unwrap(),
        ];
        assert!(validate_binding_inputs(&inputs).is_err());
    }

    #[test]
    fn test_modifiers() {
        let input = BindingInput::parse("ctrl-alt-s").unwrap();
        let mods = input.modifiers();
        assert!(mods.control);
        assert!(mods.alt);

        let input = BindingInput::parse("shift-mouse1").unwrap();
        let mods = input.modifiers();
        assert!(mods.shift);
    }
}
