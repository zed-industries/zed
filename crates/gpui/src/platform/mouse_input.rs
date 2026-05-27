use std::error::Error;
use std::fmt::Display;

use crate::{Modifiers, MouseButton, NavigationDirection};

/// The number of clicks for a mouse binding
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Default)]
pub enum ClickCount {
    /// Single click (default)
    #[default]
    Single = 1,
    /// Double click
    Double = 2,
    /// Triple click
    Triple = 3,
}

impl ClickCount {
    /// Returns the numeric value of the click count
    pub fn as_usize(self) -> usize {
        self as usize
    }

    /// Check if an event click count matches this binding click count
    pub fn matches(self, event_click_count: usize) -> bool {
        event_click_count == self.as_usize()
    }
}

/// A mouse input that can be bound to an action.
///
/// Represents a mouse button press with required modifiers and optional click count.
/// Syntax: `<modifiers->[clickcount-]mouse<1-5>`
///
/// Examples:
/// - `alt-mouse1` - alt + left click
/// - `shift-mouse2` - shift + right click
/// - `ctrl-mouse3` - ctrl + middle click
/// - `cmd-mouse4` - platform modifier + back button
/// - `cmd-mouse5` - platform modifier + forward button
/// - `ctrl-double-mouse1` - ctrl + double left click
#[derive(Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct MouseInput {
    /// The modifier keys that must be held
    pub modifiers: Modifiers,
    /// The mouse button
    pub button: MouseButton,
    /// The number of clicks required
    pub click_count: ClickCount,
}

/// Error type for `MouseInput::parse`
#[derive(Debug)]
pub struct InvalidMouseInputError {
    /// The invalid input string
    pub input: String,
}

impl Error for InvalidMouseInputError {}

impl Display for InvalidMouseInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid mouse input \"{}\". {}",
            self.input, MOUSE_INPUT_PARSE_EXPECTED_MESSAGE
        )
    }
}

/// Help message for mouse input parsing errors
pub const MOUSE_INPUT_PARSE_EXPECTED_MESSAGE: &str = "Expected format: \
    <modifiers->[clickcount-]mouse<1-5>. \
    At least one modifier is required. \
    Modifiers: ctrl, alt, shift, fn, cmd/super/win. \
    Click count: double, triple. \
    Mouse buttons: mouse1 (left), mouse2 (right), mouse3 (middle), mouse4 (back), mouse5 (forward).";

impl MouseInput {
    /// Create a new mouse input
    pub fn new(button: MouseButton, modifiers: Modifiers, click_count: ClickCount) -> Self {
        Self {
            modifiers,
            button,
            click_count,
        }
    }

    /// Parse a mouse input string.
    ///
    /// Syntax: `<modifiers->[clickcount-]mouse<1-5>`
    ///
    /// Examples:
    /// - `alt-mouse1` - alt + left click
    /// - `shift-mouse2` - shift + right click
    /// - `ctrl-double-mouse1` - ctrl + double left click
    pub fn parse(source: &str) -> Result<Self, InvalidMouseInputError> {
        let mut modifiers = Modifiers::none();
        let mut click_count = ClickCount::Single;
        let mut button = None;

        let components: Vec<&str> = source.split('-').collect();

        for (i, component) in components.iter().enumerate() {
            let component_lower = component.to_ascii_lowercase();

            if modifiers.parse_modifier_component(component) {
                continue;
            }

            // Check for click count
            if component_lower == "double" {
                click_count = ClickCount::Double;
                continue;
            }
            if component_lower == "triple" {
                click_count = ClickCount::Triple;
                continue;
            }

            // Check for mouse button - must be the last component
            if i == components.len() - 1 {
                button = parse_mouse_button(&component_lower);
                if button.is_some() {
                    continue;
                }
            }

            // Unknown component
            return Err(InvalidMouseInputError {
                input: source.to_owned(),
            });
        }

        let button = button.ok_or_else(|| InvalidMouseInputError {
            input: source.to_owned(),
        })?;
        if !modifiers.modified() {
            return Err(InvalidMouseInputError {
                input: source.to_owned(),
            });
        }

        Ok(MouseInput {
            modifiers,
            button,
            click_count,
        })
    }

    /// Attempt to parse `source` as a mouse input.
    ///
    /// Returns `Ok(None)` if `source` is not in the mouse-input grammar at all
    /// (its final `-`-separated component is not a mouse button token like
    /// `mouse1`..`mouse5`). Returns `Err` if it is recognizably a mouse input
    /// but malformed.
    pub fn try_parse(source: &str) -> Result<Option<Self>, InvalidMouseInputError> {
        let last = source.rsplit('-').next().unwrap_or("").to_ascii_lowercase();
        if parse_mouse_button(&last).is_none() {
            return Ok(None);
        }
        Self::parse(source).map(Some)
    }

    /// Check if a mouse event matches this input
    pub fn matches(
        &self,
        button: MouseButton,
        modifiers: Modifiers,
        event_click_count: usize,
    ) -> bool {
        self.button == button
            && self.modifiers == modifiers
            && self.click_count.matches(event_click_count)
    }

    /// Produces a string representation that can be parsed back
    pub fn unparse(&self) -> String {
        let mut parts = Vec::new();

        if self.modifiers.control {
            parts.push("ctrl");
        }
        if self.modifiers.alt {
            parts.push("alt");
        }
        if self.modifiers.shift {
            parts.push("shift");
        }
        if self.modifiers.platform {
            #[cfg(target_os = "macos")]
            parts.push("cmd");
            #[cfg(target_os = "windows")]
            parts.push("win");
            #[cfg(not(any(target_os = "macos", target_os = "windows")))]
            parts.push("super");
        }
        if self.modifiers.function {
            parts.push("fn");
        }

        match self.click_count {
            ClickCount::Single => {}
            ClickCount::Double => parts.push("double"),
            ClickCount::Triple => parts.push("triple"),
        }

        let button_str = match self.button {
            MouseButton::Left => "mouse1",
            MouseButton::Right => "mouse2",
            MouseButton::Middle => "mouse3",
            MouseButton::Navigate(NavigationDirection::Back) => "mouse4",
            MouseButton::Navigate(NavigationDirection::Forward) => "mouse5",
        };
        parts.push(button_str);

        parts.join("-")
    }
}

impl Display for MouseInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.unparse())
    }
}

fn parse_mouse_button(s: &str) -> Option<MouseButton> {
    match s {
        "mouse1" => Some(MouseButton::Left),
        "mouse2" => Some(MouseButton::Right),
        "mouse3" => Some(MouseButton::Middle),
        "mouse4" => Some(MouseButton::Navigate(NavigationDirection::Back)),
        "mouse5" => Some(MouseButton::Navigate(NavigationDirection::Forward)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mouse_buttons() {
        let stroke = MouseInput::parse("alt-mouse1").unwrap();
        assert_eq!(stroke.button, MouseButton::Left);
        assert!(stroke.modifiers.alt);
        assert_eq!(stroke.click_count, ClickCount::Single);

        let stroke = MouseInput::parse("alt-mouse2").unwrap();
        assert_eq!(stroke.button, MouseButton::Right);

        let stroke = MouseInput::parse("alt-mouse3").unwrap();
        assert_eq!(stroke.button, MouseButton::Middle);

        let stroke = MouseInput::parse("alt-mouse4").unwrap();
        assert_eq!(
            stroke.button,
            MouseButton::Navigate(NavigationDirection::Back)
        );

        let stroke = MouseInput::parse("alt-mouse5").unwrap();
        assert_eq!(
            stroke.button,
            MouseButton::Navigate(NavigationDirection::Forward)
        );
    }

    #[test]
    fn test_parse_with_modifiers() {
        let stroke = MouseInput::parse("alt-mouse1").unwrap();
        assert_eq!(stroke.button, MouseButton::Left);
        assert!(stroke.modifiers.alt);
        assert!(!stroke.modifiers.control);

        let stroke = MouseInput::parse("ctrl-mouse1").unwrap();
        assert!(stroke.modifiers.control);

        let stroke = MouseInput::parse("shift-mouse1").unwrap();
        assert!(stroke.modifiers.shift);

        let stroke = MouseInput::parse("ctrl-alt-shift-mouse1").unwrap();
        assert!(stroke.modifiers.control);
        assert!(stroke.modifiers.alt);
        assert!(stroke.modifiers.shift);
    }

    #[test]
    fn test_parse_with_click_count() {
        let stroke = MouseInput::parse("alt-double-mouse1").unwrap();
        assert_eq!(stroke.click_count, ClickCount::Double);
        assert!(stroke.modifiers.alt);

        let stroke = MouseInput::parse("alt-triple-mouse1").unwrap();
        assert_eq!(stroke.click_count, ClickCount::Triple);
        assert!(stroke.modifiers.alt);

        let stroke = MouseInput::parse("ctrl-shift-triple-mouse3").unwrap();
        assert_eq!(stroke.click_count, ClickCount::Triple);
        assert_eq!(stroke.button, MouseButton::Middle);
        assert!(stroke.modifiers.control);
        assert!(stroke.modifiers.shift);
    }

    #[test]
    fn test_parse_case_insensitive() {
        let stroke = MouseInput::parse("ALT-MOUSE1").unwrap();
        assert_eq!(stroke.button, MouseButton::Left);

        let stroke = MouseInput::parse("ALT-Mouse1").unwrap();
        assert!(stroke.modifiers.alt);

        let stroke = MouseInput::parse("Alt-Double-MOUSE1").unwrap();
        assert_eq!(stroke.click_count, ClickCount::Double);
    }

    #[test]
    fn test_parse_invalid() {
        assert!(MouseInput::parse("mouse0").is_err());
        assert!(MouseInput::parse("mouse1").is_err());
        assert!(MouseInput::parse("mouse6").is_err());
        assert!(MouseInput::parse("double-mouse1").is_err());
        assert!(MouseInput::parse("click").is_err());
        assert!(MouseInput::parse("").is_err());
        assert!(MouseInput::parse("alt-").is_err());
        assert!(MouseInput::parse("alt-click").is_err());
    }

    #[test]
    fn test_matches() {
        let stroke = MouseInput::parse("alt-mouse1").unwrap();

        let mut mods = Modifiers::none();
        mods.alt = true;

        assert!(stroke.matches(MouseButton::Left, mods, 1));
        assert!(!stroke.matches(MouseButton::Right, mods, 1));
        assert!(!stroke.matches(MouseButton::Left, Modifiers::none(), 1));
        assert!(!stroke.matches(MouseButton::Left, mods, 2));
    }

    #[test]
    fn test_unparse() {
        let stroke = MouseInput::parse("alt-double-mouse1").unwrap();
        assert_eq!(stroke.unparse(), "alt-double-mouse1");

        let stroke = MouseInput::parse("ctrl-shift-mouse3").unwrap();
        assert_eq!(stroke.unparse(), "ctrl-shift-mouse3");
    }
}
