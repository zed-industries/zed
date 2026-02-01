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

/// A scroll input that can be bound to an action.
///
/// Note: This is only for modifier+scroll combinations that trigger discrete actions
/// (like ctrl+scroll for font size). Normal scrolling behavior is not bindable.
///
/// Syntax: `[modifiers-]scroll-<up|down>`
///
/// Examples:
/// - `ctrl-scroll-up` - ctrl + scroll up (e.g., increase font size)
/// - `ctrl-scroll-down` - ctrl + scroll down (e.g., decrease font size)
/// - `cmd-scroll-up` - cmd + scroll up (macOS)
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ScrollStroke {
    /// The modifier keys that must be held
    pub modifiers: Modifiers,
    /// The scroll direction
    pub direction: ScrollDirection,
}

/// Error type for `ScrollStroke::parse`
#[derive(Debug)]
pub struct InvalidScrollStrokeError {
    /// The invalid input string
    pub input: String,
}

impl Error for InvalidScrollStrokeError {}

impl Display for InvalidScrollStrokeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid scroll stroke \"{}\". {}",
            self.input, SCROLL_STROKE_PARSE_EXPECTED_MESSAGE
        )
    }
}

/// Help message for scroll stroke parsing errors
pub const SCROLL_STROKE_PARSE_EXPECTED_MESSAGE: &str = "Expected format: \
    [modifiers-]scroll-<up|down>. \
    Modifiers: ctrl, alt, shift, fn, cmd/super/win.";

impl ScrollStroke {
    /// Create a new scroll stroke
    pub fn new(direction: ScrollDirection, modifiers: Modifiers) -> Self {
        Self {
            modifiers,
            direction,
        }
    }

    /// Parse a scroll stroke string.
    ///
    /// Syntax: `[modifiers-]scroll-<up|down>`
    ///
    /// Examples:
    /// - `scroll-up` - scroll up (no modifiers)
    /// - `ctrl-scroll-up` - ctrl + scroll up
    /// - `cmd-scroll-down` - cmd + scroll down
    pub fn parse(source: &str) -> Result<Self, InvalidScrollStrokeError> {
        let mut modifiers = Modifiers::none();
        let mut direction = None;

        let components: Vec<&str> = source.split('-').collect();
        let mut i = 0;

        while i < components.len() {
            let component_lower = components[i].to_ascii_lowercase();

            // Check for modifiers
            if component_lower == "ctrl" {
                modifiers.control = true;
                i += 1;
                continue;
            }
            if component_lower == "alt" {
                modifiers.alt = true;
                i += 1;
                continue;
            }
            if component_lower == "shift" {
                modifiers.shift = true;
                i += 1;
                continue;
            }
            if component_lower == "fn" {
                modifiers.function = true;
                i += 1;
                continue;
            }
            if component_lower == "cmd"
                || component_lower == "super"
                || component_lower == "win"
            {
                modifiers.platform = true;
                i += 1;
                continue;
            }
            if component_lower == "secondary" {
                if cfg!(target_os = "macos") {
                    modifiers.platform = true;
                } else {
                    modifiers.control = true;
                }
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
                        return Err(InvalidScrollStrokeError {
                            input: source.to_owned(),
                        })
                    }
                };
                i += 2;
                continue;
            }

            // Unknown component
            return Err(InvalidScrollStrokeError {
                input: source.to_owned(),
            });
        }

        let direction = direction.ok_or_else(|| InvalidScrollStrokeError {
            input: source.to_owned(),
        })?;

        Ok(ScrollStroke {
            modifiers,
            direction,
        })
    }

    /// Check if a scroll event matches this stroke
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

/// Check if a string looks like a scroll stroke (contains "scroll-")
pub fn looks_like_scroll_stroke(s: &str) -> bool {
    s.to_ascii_lowercase().contains("scroll-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_scroll() {
        let stroke = ScrollStroke::parse("scroll-up").unwrap();
        assert_eq!(stroke.direction, ScrollDirection::Up);
        assert_eq!(stroke.modifiers, Modifiers::none());

        let stroke = ScrollStroke::parse("scroll-down").unwrap();
        assert_eq!(stroke.direction, ScrollDirection::Down);
        assert_eq!(stroke.modifiers, Modifiers::none());
    }

    #[test]
    fn test_parse_with_modifiers() {
        let stroke = ScrollStroke::parse("ctrl-scroll-up").unwrap();
        assert_eq!(stroke.direction, ScrollDirection::Up);
        assert!(stroke.modifiers.control);

        let stroke = ScrollStroke::parse("alt-scroll-down").unwrap();
        assert_eq!(stroke.direction, ScrollDirection::Down);
        assert!(stroke.modifiers.alt);

        let stroke = ScrollStroke::parse("ctrl-shift-scroll-up").unwrap();
        assert!(stroke.modifiers.control);
        assert!(stroke.modifiers.shift);
    }

    #[test]
    fn test_parse_case_insensitive() {
        let stroke = ScrollStroke::parse("SCROLL-UP").unwrap();
        assert_eq!(stroke.direction, ScrollDirection::Up);

        let stroke = ScrollStroke::parse("CTRL-Scroll-Down").unwrap();
        assert!(stroke.modifiers.control);
        assert_eq!(stroke.direction, ScrollDirection::Down);
    }

    #[test]
    fn test_parse_invalid() {
        assert!(ScrollStroke::parse("scroll").is_err());
        assert!(ScrollStroke::parse("scroll-left").is_err());
        assert!(ScrollStroke::parse("scroll-").is_err());
        assert!(ScrollStroke::parse("").is_err());
        assert!(ScrollStroke::parse("ctrl-scroll").is_err());
    }

    #[test]
    fn test_matches() {
        let stroke = ScrollStroke::parse("ctrl-scroll-up").unwrap();

        let mut mods = Modifiers::none();
        mods.control = true;

        assert!(stroke.matches(ScrollDirection::Up, mods));
        assert!(!stroke.matches(ScrollDirection::Down, mods));
        assert!(!stroke.matches(ScrollDirection::Up, Modifiers::none()));
    }

    #[test]
    fn test_unparse() {
        let stroke = ScrollStroke::parse("ctrl-scroll-up").unwrap();
        assert_eq!(stroke.unparse(), "ctrl-scroll-up");

        let stroke = ScrollStroke::parse("alt-shift-scroll-down").unwrap();
        assert_eq!(stroke.unparse(), "alt-shift-scroll-down");
    }
}
