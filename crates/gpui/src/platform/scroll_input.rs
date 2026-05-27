use std::error::Error;
use std::fmt::Display;

use crate::Modifiers;

/// The direction of scroll for a scroll binding
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ScrollDirection {
    /// Scrolling up (or zooming in)
    Up,
    /// Scrolling down (or zooming out)
    Down,
}

impl ScrollDirection {
    /// Converts a scroll delta into a dominant vertical scroll direction.
    pub fn from_delta(delta: &crate::ScrollDelta) -> Option<Self> {
        match delta {
            crate::ScrollDelta::Pixels(point) => {
                if point.y.0.abs() > point.x.0.abs() {
                    if point.y.0 > 0.0 {
                        Some(Self::Up)
                    } else if point.y.0 < 0.0 {
                        Some(Self::Down)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            crate::ScrollDelta::Lines(point) => {
                if point.y.abs() > point.x.abs() {
                    if point.y > 0.0 {
                        Some(Self::Up)
                    } else if point.y < 0.0 {
                        Some(Self::Down)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }
}

/// A scroll input that can be bound to an action.
///
/// Note: This is only for modifier+scroll combinations that trigger discrete actions
/// (like ctrl+scroll for font size). Normal scrolling behavior is not bindable.
///
/// Syntax: `<modifiers->scroll-<up|down>`
///
/// Examples:
/// - `ctrl-scroll-up` - ctrl + scroll up (e.g., increase font size)
/// - `ctrl-scroll-down` - ctrl + scroll down (e.g., decrease font size)
/// - `cmd-scroll-up` - cmd + scroll up (macOS)
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ScrollInput {
    /// The modifier keys that must be held
    pub modifiers: Modifiers,
    /// The scroll direction
    pub direction: ScrollDirection,
}

/// Error type for `ScrollInput::parse`
#[derive(Debug)]
pub struct InvalidScrollInputError {
    /// The invalid input string
    pub input: String,
}

impl Error for InvalidScrollInputError {}

impl Display for InvalidScrollInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid scroll input \"{}\". {}",
            self.input, SCROLL_INPUT_PARSE_EXPECTED_MESSAGE
        )
    }
}

/// Help message for scroll input parsing errors
pub const SCROLL_INPUT_PARSE_EXPECTED_MESSAGE: &str = "Expected format: \
    <modifiers->scroll-<up|down>. \
    At least one modifier is required. \
    Modifiers: ctrl, alt, shift, fn, cmd/super/win.";

impl ScrollInput {
    /// Create a new scroll input
    pub fn new(direction: ScrollDirection, modifiers: Modifiers) -> Self {
        Self {
            modifiers,
            direction,
        }
    }

    /// Parse a scroll input string.
    ///
    /// Syntax: `<modifiers->scroll-<up|down>`
    ///
    /// Examples:
    /// - `ctrl-scroll-up` - ctrl + scroll up
    /// - `cmd-scroll-down` - cmd + scroll down
    pub fn parse(source: &str) -> Result<Self, InvalidScrollInputError> {
        let mut modifiers = Modifiers::none();
        let mut direction = None;

        let components: Vec<&str> = source.split('-').collect();
        let mut i = 0;

        while i < components.len() {
            let component_lower = components[i].to_ascii_lowercase();

            if modifiers.parse_modifier_component(components[i]) {
                i += 1;
                continue;
            }

            // Check for "scroll-up" or "scroll-down"
            if component_lower == "scroll" && i + 1 < components.len() {
                let dir_component = components[i + 1].to_ascii_lowercase();
                direction = match dir_component.as_str() {
                    "up" => Some(ScrollDirection::Up),
                    "down" => Some(ScrollDirection::Down),
                    _ => {
                        return Err(InvalidScrollInputError {
                            input: source.to_owned(),
                        });
                    }
                };
                i += 2;
                continue;
            }

            // Unknown component
            return Err(InvalidScrollInputError {
                input: source.to_owned(),
            });
        }

        let direction = direction.ok_or_else(|| InvalidScrollInputError {
            input: source.to_owned(),
        })?;
        if !modifiers.modified() {
            return Err(InvalidScrollInputError {
                input: source.to_owned(),
            });
        }

        Ok(ScrollInput {
            modifiers,
            direction,
        })
    }

    /// Attempt to parse `source` as a scroll input.
    ///
    /// Returns `Ok(None)` if `source` is not in the scroll-input grammar at
    /// all (it does not end with `scroll-<direction>`). Returns `Err` if it
    /// is recognizably a scroll input but malformed.
    pub fn try_parse(source: &str) -> Result<Option<Self>, InvalidScrollInputError> {
        let mut tail = source.rsplit('-');
        let Some(direction) = tail.next() else {
            return Ok(None);
        };
        let Some(scroll_keyword) = tail.next() else {
            return Ok(None);
        };
        if !scroll_keyword.eq_ignore_ascii_case("scroll")
            || !matches!(direction.to_ascii_lowercase().as_str(), "up" | "down")
        {
            return Ok(None);
        }
        Self::parse(source).map(Some)
    }

    /// Check if a scroll event matches this input
    pub fn matches(&self, direction: ScrollDirection, modifiers: Modifiers) -> bool {
        self.direction == direction && self.modifiers == modifiers
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

        let dir_str = match self.direction {
            ScrollDirection::Up => "scroll-up",
            ScrollDirection::Down => "scroll-down",
        };
        parts.push(dir_str);

        parts.join("-")
    }
}

impl Display for ScrollInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.unparse())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_scroll() {
        let stroke = ScrollInput::parse("ctrl-scroll-down").unwrap();
        assert_eq!(stroke.direction, ScrollDirection::Down);
        assert!(stroke.modifiers.control);
    }

    #[test]
    fn test_parse_with_modifiers() {
        let stroke = ScrollInput::parse("ctrl-scroll-up").unwrap();
        assert_eq!(stroke.direction, ScrollDirection::Up);
        assert!(stroke.modifiers.control);

        let stroke = ScrollInput::parse("alt-scroll-down").unwrap();
        assert_eq!(stroke.direction, ScrollDirection::Down);
        assert!(stroke.modifiers.alt);

        let stroke = ScrollInput::parse("ctrl-shift-scroll-up").unwrap();
        assert!(stroke.modifiers.control);
        assert!(stroke.modifiers.shift);
    }

    #[test]
    fn test_parse_case_insensitive() {
        let stroke = ScrollInput::parse("CTRL-SCROLL-UP").unwrap();
        assert_eq!(stroke.direction, ScrollDirection::Up);
        assert!(stroke.modifiers.control);

        let stroke = ScrollInput::parse("CTRL-Scroll-Down").unwrap();
        assert!(stroke.modifiers.control);
        assert_eq!(stroke.direction, ScrollDirection::Down);
    }

    #[test]
    fn test_parse_invalid() {
        assert!(ScrollInput::parse("scroll").is_err());
        assert!(ScrollInput::parse("scroll-up").is_err());
        assert!(ScrollInput::parse("scroll-left").is_err());
        assert!(ScrollInput::parse("scroll-").is_err());
        assert!(ScrollInput::parse("").is_err());
        assert!(ScrollInput::parse("ctrl-scroll").is_err());
    }

    #[test]
    fn test_matches() {
        let stroke = ScrollInput::parse("ctrl-scroll-up").unwrap();

        let mut mods = Modifiers::none();
        mods.control = true;

        assert!(stroke.matches(ScrollDirection::Up, mods));
        assert!(!stroke.matches(ScrollDirection::Down, mods));
        assert!(!stroke.matches(ScrollDirection::Up, Modifiers::none()));
    }

    #[test]
    fn test_unparse() {
        let stroke = ScrollInput::parse("ctrl-scroll-up").unwrap();
        assert_eq!(stroke.unparse(), "ctrl-scroll-up");

        let stroke = ScrollInput::parse("alt-shift-scroll-down").unwrap();
        assert_eq!(stroke.unparse(), "alt-shift-scroll-down");
    }
}
