use std::{error::Error, fmt::Display};

use crate::{Modifiers, MouseButton, NavigationDirection};

/// Error type for `MouseStroke::parse`.
#[derive(Debug)]
pub struct InvalidMouseStrokeError {
    /// The invalid mouse stroke string.
    pub stroke: String,
}

impl Error for InvalidMouseStrokeError {}

impl Display for InvalidMouseStrokeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid mouse stroke \"{}\". {}",
            self.stroke, MOUSE_STROKE_PARSE_EXPECTED_MESSAGE
        )
    }
}

/// Sentence explaining what mouse stroke parser expects.
pub const MOUSE_STROKE_PARSE_EXPECTED_MESSAGE: &str = "Expected a sequence of optional modifiers \
    (`ctrl`, `alt`, `shift`, `fn`, `cmd`, `super`, or `win`), an optional click count \
    (`double` or `triple`), followed by a mouse button (`mouse1` through `mouse5`), separated by `-`.";

const BUTTON_TOKENS: &[&str] = &["mouse1", "mouse2", "mouse3", "mouse4", "mouse5"];

pub(super) fn try_parse_modifier_token(part: &str, modifiers: &mut Modifiers) -> bool {
    if part.eq_ignore_ascii_case("ctrl") {
        modifiers.control = true;
    } else if part.eq_ignore_ascii_case("alt") {
        modifiers.alt = true;
    } else if part.eq_ignore_ascii_case("shift") {
        modifiers.shift = true;
    } else if part.eq_ignore_ascii_case("fn") {
        modifiers.function = true;
    } else if part.eq_ignore_ascii_case("secondary") {
        if cfg!(target_os = "macos") {
            modifiers.platform = true;
        } else {
            modifiers.control = true;
        }
    } else if part.eq_ignore_ascii_case("cmd")
        || part.eq_ignore_ascii_case("super")
        || part.eq_ignore_ascii_case("win")
    {
        modifiers.platform = true;
    } else {
        return false;
    }
    true
}

pub(super) fn append_modifiers(result: &mut String, modifiers: &Modifiers) {
    if modifiers.function {
        result.push_str("fn-");
    }
    if modifiers.control {
        result.push_str("ctrl-");
    }
    if modifiers.alt {
        result.push_str("alt-");
    }
    if modifiers.platform {
        #[cfg(target_os = "macos")]
        result.push_str("cmd-");
        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        result.push_str("super-");
        #[cfg(target_os = "windows")]
        result.push_str("win-");
    }
    if modifiers.shift {
        result.push_str("shift-");
    }
}

/// The click count for a mouse binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClickCount {
    /// Single click.
    Single,
    /// Double click.
    Double,
    /// Triple click.
    Triple,
}

impl ClickCount {
    fn as_usize(self) -> usize {
        match self {
            ClickCount::Single => 1,
            ClickCount::Double => 2,
            ClickCount::Triple => 3,
        }
    }
}

/// A mouse button stroke that can be bound in a keymap.
/// Syntax: `[modifiers-][clickcount-]mouse<1-5>`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MouseStroke {
    /// Modifier keys held during the click.
    pub modifiers: Modifiers,
    /// The mouse button that was pressed.
    pub button: MouseButton,
    /// The click count.
    pub click_count: ClickCount,
}

impl MouseStroke {
    /// Parse a mouse stroke from a string like "alt-mouse1" or "double-mouse3".
    pub fn parse(source: &str) -> std::result::Result<Self, InvalidMouseStrokeError> {
        let invalid = || InvalidMouseStrokeError {
            stroke: source.to_owned(),
        };
        let mut modifiers = Modifiers::none();
        let mut click_count = ClickCount::Single;
        let mut button: Option<MouseButton> = None;

        let mut parts = source.split('-').peekable();
        while let Some(part) = parts.next() {
            if try_parse_modifier_token(part, &mut modifiers) {
                continue;
            }
            if part.eq_ignore_ascii_case("double") {
                click_count = ClickCount::Double;
                continue;
            }
            if part.eq_ignore_ascii_case("triple") {
                click_count = ClickCount::Triple;
                continue;
            }
            button = Some(match part {
                p if p.eq_ignore_ascii_case("mouse1") => MouseButton::Left,
                p if p.eq_ignore_ascii_case("mouse2") => MouseButton::Right,
                p if p.eq_ignore_ascii_case("mouse3") => MouseButton::Middle,
                p if p.eq_ignore_ascii_case("mouse4") => {
                    MouseButton::Navigate(NavigationDirection::Back)
                }
                p if p.eq_ignore_ascii_case("mouse5") => {
                    MouseButton::Navigate(NavigationDirection::Forward)
                }
                _ => return Err(invalid()),
            });
            // Button must be the last token.
            if parts.peek().is_some() {
                return Err(invalid());
            }
        }

        Ok(MouseStroke {
            modifiers,
            button: button.ok_or_else(invalid)?,
            click_count,
        })
    }

    /// Returns true if this stroke matches the given event parameters.
    pub fn matches(&self, button: &MouseButton, modifiers: &Modifiers, click_count: usize) -> bool {
        self.click_count.as_usize() == click_count
            && &self.button == button
            && &self.modifiers == modifiers
    }

    /// Produces a string representation that `parse` can understand.
    pub fn unparse(&self) -> String {
        let mut result = String::new();
        append_modifiers(&mut result, &self.modifiers);
        match self.click_count {
            ClickCount::Double => result.push_str("double-"),
            ClickCount::Triple => result.push_str("triple-"),
            ClickCount::Single => {}
        }
        let button_str = match &self.button {
            MouseButton::Left => "mouse1",
            MouseButton::Right => "mouse2",
            MouseButton::Middle => "mouse3",
            MouseButton::Navigate(NavigationDirection::Back) => "mouse4",
            MouseButton::Navigate(NavigationDirection::Forward) => "mouse5",
        };
        result.push_str(button_str);
        result
    }
}

/// Returns true if the given string looks like a mouse button stroke.
pub fn looks_like_mouse_stroke(s: &str) -> bool {
    s.split('-')
        .any(|tok| BUTTON_TOKENS.iter().any(|b| tok.eq_ignore_ascii_case(b)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_looks_like_mouse_stroke() {
        assert!(looks_like_mouse_stroke("alt-mouse1"));
        assert!(looks_like_mouse_stroke("double-mouse3"));
        assert!(looks_like_mouse_stroke("ALT-MOUSE4"));
        assert!(!looks_like_mouse_stroke("alt-mousescrolldown"));
        assert!(!looks_like_mouse_stroke("ctrl-mousescrollup"));
        assert!(!looks_like_mouse_stroke("mouse1foo"));
    }

    #[test]
    fn test_parse_rejects_trailing_tokens() {
        assert!(MouseStroke::parse("mouse1-alt").is_err());
        assert!(MouseStroke::parse("alt-mouse1").is_ok());
    }
}
