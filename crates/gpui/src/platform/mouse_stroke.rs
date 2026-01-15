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
/// Represents a mouse button press with optional modifiers and click count.
/// Syntax: `[modifiers-][clickcount-]mouse<1-5>`
///
/// Examples:
/// - `mouse1` - left click
/// - `mouse2` - right click
/// - `mouse3` - middle click
/// - `mouse4` - back button
/// - `mouse5` - forward button
/// - `alt-mouse1` - alt + left click
/// - `double-mouse1` - double left click
/// - `ctrl-double-mouse1` - ctrl + double left click
#[derive(Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct MouseStroke {
    /// The modifier keys that must be held
    pub modifiers: Modifiers,
    /// The mouse button
    pub button: MouseButton,
    /// The number of clicks required
    pub click_count: ClickCount,
}

/// Error type for `MouseStroke::parse`
#[derive(Debug)]
pub struct InvalidMouseStrokeError {
    /// The invalid input string
    pub input: String,
}

impl Error for InvalidMouseStrokeError {}

impl Display for InvalidMouseStrokeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid mouse stroke \"{}\". {}",
            self.input, MOUSE_STROKE_PARSE_EXPECTED_MESSAGE
        )
    }
}

/// Help message for mouse stroke parsing errors
pub const MOUSE_STROKE_PARSE_EXPECTED_MESSAGE: &str = "Expected format: \
    [modifiers-][clickcount-]mouse<1-5>. \
    Modifiers: ctrl, alt, shift, fn, cmd/super/win. \
    Click count: double, triple. \
    Mouse buttons: mouse1 (left), mouse2 (right), mouse3 (middle), mouse4 (back), mouse5 (forward).";

impl MouseStroke {
    /// Create a new mouse stroke
    pub fn new(button: MouseButton, modifiers: Modifiers, click_count: ClickCount) -> Self {
        Self {
            modifiers,
            button,
            click_count,
        }
    }

    /// Parse a mouse stroke string.
    ///
    /// Syntax: `[modifiers-][clickcount-]mouse<1-5>`
    ///
    /// Examples:
    /// - `mouse1` - left click
    /// - `alt-mouse1` - alt + left click
    /// - `double-mouse1` - double left click
    /// - `ctrl-double-mouse1` - ctrl + double left click
    pub fn parse(source: &str) -> Result<Self, InvalidMouseStrokeError> {
        let mut modifiers = Modifiers::none();
        let mut click_count = ClickCount::Single;
        let mut button = None;

        let components: Vec<&str> = source.split('-').collect();

        for (i, component) in components.iter().enumerate() {
            let component_lower = component.to_ascii_lowercase();

            // Check for modifiers
            if component_lower == "ctrl" {
                modifiers.control = true;
                continue;
            }
            if component_lower == "alt" {
                modifiers.alt = true;
                continue;
            }
            if component_lower == "shift" {
                modifiers.shift = true;
                continue;
            }
            if component_lower == "fn" {
                modifiers.function = true;
                continue;
            }
            if component_lower == "cmd"
                || component_lower == "super"
                || component_lower == "win"
            {
                modifiers.platform = true;
                continue;
            }
            if component_lower == "secondary" {
                if cfg!(target_os = "macos") {
                    modifiers.platform = true;
                } else {
                    modifiers.control = true;
                }
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
            return Err(InvalidMouseStrokeError {
                input: source.to_owned(),
            });
        }

        let button = button.ok_or_else(|| InvalidMouseStrokeError {
            input: source.to_owned(),
        })?;

        Ok(MouseStroke {
            modifiers,
            button,
            click_count,
        })
    }

    /// Check if a mouse event matches this stroke
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

/// Check if a string looks like a mouse stroke (contains "mouse")
pub fn looks_like_mouse_stroke(s: &str) -> bool {
    s.to_ascii_lowercase().contains("mouse")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_mouse_buttons() {
        let stroke = MouseStroke::parse("mouse1").unwrap();
        assert_eq!(stroke.button, MouseButton::Left);
        assert_eq!(stroke.modifiers, Modifiers::none());
        assert_eq!(stroke.click_count, ClickCount::Single);

        let stroke = MouseStroke::parse("mouse2").unwrap();
        assert_eq!(stroke.button, MouseButton::Right);

        let stroke = MouseStroke::parse("mouse3").unwrap();
        assert_eq!(stroke.button, MouseButton::Middle);

        let stroke = MouseStroke::parse("mouse4").unwrap();
        assert_eq!(stroke.button, MouseButton::Navigate(NavigationDirection::Back));

        let stroke = MouseStroke::parse("mouse5").unwrap();
        assert_eq!(stroke.button, MouseButton::Navigate(NavigationDirection::Forward));
    }

    #[test]
    fn test_parse_with_modifiers() {
        let stroke = MouseStroke::parse("alt-mouse1").unwrap();
        assert_eq!(stroke.button, MouseButton::Left);
        assert!(stroke.modifiers.alt);
        assert!(!stroke.modifiers.control);

        let stroke = MouseStroke::parse("ctrl-mouse1").unwrap();
        assert!(stroke.modifiers.control);

        let stroke = MouseStroke::parse("shift-mouse1").unwrap();
        assert!(stroke.modifiers.shift);

        let stroke = MouseStroke::parse("ctrl-alt-shift-mouse1").unwrap();
        assert!(stroke.modifiers.control);
        assert!(stroke.modifiers.alt);
        assert!(stroke.modifiers.shift);
    }

    #[test]
    fn test_parse_with_click_count() {
        let stroke = MouseStroke::parse("double-mouse1").unwrap();
        assert_eq!(stroke.click_count, ClickCount::Double);

        let stroke = MouseStroke::parse("triple-mouse1").unwrap();
        assert_eq!(stroke.click_count, ClickCount::Triple);

        let stroke = MouseStroke::parse("alt-double-mouse1").unwrap();
        assert_eq!(stroke.click_count, ClickCount::Double);
        assert!(stroke.modifiers.alt);

        let stroke = MouseStroke::parse("ctrl-shift-triple-mouse3").unwrap();
        assert_eq!(stroke.click_count, ClickCount::Triple);
        assert_eq!(stroke.button, MouseButton::Middle);
        assert!(stroke.modifiers.control);
        assert!(stroke.modifiers.shift);
    }

    #[test]
    fn test_parse_case_insensitive() {
        let stroke = MouseStroke::parse("MOUSE1").unwrap();
        assert_eq!(stroke.button, MouseButton::Left);

        let stroke = MouseStroke::parse("ALT-Mouse1").unwrap();
        assert!(stroke.modifiers.alt);

        let stroke = MouseStroke::parse("Double-MOUSE1").unwrap();
        assert_eq!(stroke.click_count, ClickCount::Double);
    }

    #[test]
    fn test_parse_invalid() {
        assert!(MouseStroke::parse("mouse0").is_err());
        assert!(MouseStroke::parse("mouse6").is_err());
        assert!(MouseStroke::parse("click").is_err());
        assert!(MouseStroke::parse("").is_err());
        assert!(MouseStroke::parse("alt-").is_err());
        assert!(MouseStroke::parse("alt-click").is_err());
    }

    #[test]
    fn test_matches() {
        let stroke = MouseStroke::parse("alt-mouse1").unwrap();
        
        let mut mods = Modifiers::none();
        mods.alt = true;
        
        assert!(stroke.matches(MouseButton::Left, mods, 1));
        assert!(!stroke.matches(MouseButton::Right, mods, 1));
        assert!(!stroke.matches(MouseButton::Left, Modifiers::none(), 1));
        assert!(!stroke.matches(MouseButton::Left, mods, 2));
    }

    #[test]
    fn test_unparse() {
        let stroke = MouseStroke::parse("alt-double-mouse1").unwrap();
        assert_eq!(stroke.unparse(), "alt-double-mouse1");

        let stroke = MouseStroke::parse("ctrl-shift-mouse3").unwrap();
        assert_eq!(stroke.unparse(), "ctrl-shift-mouse3");
    }
}
