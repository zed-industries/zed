use std::{error::Error, fmt::Display};

use super::mouse_stroke::{append_modifiers, try_parse_modifier_token};
use crate::Modifiers;

/// Error type for `ScrollStroke::parse`.
#[derive(Debug)]
pub struct InvalidScrollStrokeError {
    /// The invalid scroll stroke string.
    pub stroke: String,
}

impl Error for InvalidScrollStrokeError {}

impl Display for InvalidScrollStrokeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid scroll stroke \"{}\". {}",
            self.stroke, SCROLL_STROKE_PARSE_EXPECTED_MESSAGE
        )
    }
}

/// Sentence explaining what scroll stroke parser expects.
pub const SCROLL_STROKE_PARSE_EXPECTED_MESSAGE: &str = "Expected a sequence of optional modifiers \
    (`ctrl`, `alt`, `shift`, `fn`, `cmd`, `super`, or `win`) followed by `scroll-up` or `scroll-down` \
    (aliases: `mousescrollup`, `mousescrolldown`), \
    separated by `-`.";

/// The direction of a scroll wheel event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScrollDirection {
    /// Scrolling upward.
    Up,
    /// Scrolling downward.
    Down,
}

/// A scroll wheel binding.
/// Syntax: `[modifiers-]scroll-<up|down>`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScrollStroke {
    /// Modifier keys held during the scroll.
    pub modifiers: Modifiers,
    /// The direction of scroll.
    pub direction: ScrollDirection,
}

fn match_direction_token(part: &str) -> Option<ScrollDirection> {
    if part.eq_ignore_ascii_case("mousescrollup") || part.eq_ignore_ascii_case("scrollup") {
        Some(ScrollDirection::Up)
    } else if part.eq_ignore_ascii_case("mousescrolldown")
        || part.eq_ignore_ascii_case("scrolldown")
    {
        Some(ScrollDirection::Down)
    } else {
        None
    }
}

impl ScrollStroke {
    /// Parse a scroll stroke from a string like "ctrl-scroll-up".
    pub fn parse(source: &str) -> std::result::Result<Self, InvalidScrollStrokeError> {
        let invalid = || InvalidScrollStrokeError {
            stroke: source.to_owned(),
        };
        let mut modifiers = Modifiers::none();
        let mut direction: Option<ScrollDirection> = None;

        let mut parts = source.split('-').peekable();
        while let Some(part) = parts.next() {
            if try_parse_modifier_token(part, &mut modifiers) {
                continue;
            }

            if part.eq_ignore_ascii_case("scroll") {
                let next = parts.next().ok_or_else(invalid)?;
                direction = Some(if next.eq_ignore_ascii_case("up") {
                    ScrollDirection::Up
                } else if next.eq_ignore_ascii_case("down") {
                    ScrollDirection::Down
                } else {
                    return Err(invalid());
                });
            } else if let Some(d) = match_direction_token(part) {
                direction = Some(d);
            } else {
                return Err(invalid());
            }

            // Direction must be the last token.
            if parts.peek().is_some() {
                return Err(invalid());
            }
        }

        Ok(ScrollStroke {
            modifiers,
            direction: direction.ok_or_else(invalid)?,
        })
    }

    /// Returns true if this stroke matches the given event parameters.
    pub fn matches(&self, direction: ScrollDirection, modifiers: &Modifiers) -> bool {
        self.direction == direction && &self.modifiers == modifiers
    }

    /// Produces a string representation that `parse` can understand.
    pub fn unparse(&self) -> String {
        let mut result = String::new();
        append_modifiers(&mut result, &self.modifiers);
        match self.direction {
            ScrollDirection::Up => result.push_str("scroll-up"),
            ScrollDirection::Down => result.push_str("scroll-down"),
        }
        result
    }
}

/// Returns true if the given string looks like a scroll stroke.
pub fn looks_like_scroll_stroke(s: &str) -> bool {
    let tokens: Vec<&str> = s.split('-').collect();
    tokens.iter().enumerate().any(|(i, tok)| {
        if match_direction_token(tok).is_some() {
            true
        } else if tok.eq_ignore_ascii_case("scroll") {
            tokens
                .get(i + 1)
                .is_some_and(|n| n.eq_ignore_ascii_case("up") || n.eq_ignore_ascii_case("down"))
        } else {
            false
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mousescroll_aliases() {
        let up = ScrollStroke::parse("ctrl-mousescrollup").unwrap();
        assert_eq!(up.direction, ScrollDirection::Up);
        assert!(up.modifiers.control);

        let down = ScrollStroke::parse("alt-mousescrolldown").unwrap();
        assert_eq!(down.direction, ScrollDirection::Down);
        assert!(down.modifiers.alt);
    }

    #[test]
    fn test_looks_like_scroll_stroke() {
        assert!(looks_like_scroll_stroke("ctrl-scroll-down"));
        assert!(looks_like_scroll_stroke("ctrl-mousescrollup"));
        assert!(looks_like_scroll_stroke("ALT-MOUSESCROLLDOWN"));
        assert!(!looks_like_scroll_stroke("scrolllock"));
        assert!(!looks_like_scroll_stroke("alt-mouse1"));
    }

    #[test]
    fn test_parse_rejects_trailing_tokens() {
        assert!(ScrollStroke::parse("scroll-up-alt").is_err());
        assert!(ScrollStroke::parse("alt-scroll-up").is_ok());
    }
}
