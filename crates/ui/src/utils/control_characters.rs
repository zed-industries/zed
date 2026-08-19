//! Rendering of control characters that would otherwise be invisible.

use std::borrow::Cow;

/// The stand-in used for a line feed (`\n`).
///
/// This is the symbol Zed has always used for line feeds in single-line labels,
/// so text containing `\n` keeps rendering the way it used to.
const LINE_FEED_SYMBOL: char = '⏎';

/// Start of the Unicode "Control Pictures" block (`U+2400`), which holds one
/// printable stand-in per C0 control character, in code point order. So the
/// stand-in for a control character is `U+2400 + <its code point>`.
const CONTROL_PICTURES_START: u32 = 0x2400;

/// The stand-in for `DEL` (`U+007F`), which sits outside the contiguous range
/// covered by [`CONTROL_PICTURES_START`].
const DELETE_SYMBOL: char = '␡';

/// Returns the printable stand-in for a control character, or `None` if the
/// character is already printable.
///
/// C1 controls (`U+0080..=U+009F`) are deliberately left alone: they have no
/// Control Pictures equivalent to map onto.
fn printable_substitute(character: char) -> Option<char> {
    match character {
        '\n' => Some(LINE_FEED_SYMBOL),
        '\x7f' => Some(DELETE_SYMBOL),
        character if character.is_ascii_control() => {
            char::from_u32(CONTROL_PICTURES_START + character as u32)
        }
        _ => None,
    }
}

/// Replaces control characters with printable stand-ins, so that text
/// containing them stays legible instead of collapsing the line it is rendered
/// on.
///
/// File names are allowed to contain characters like `\n`, `\r` and `\t` on
/// most platforms, and rendering those verbatim breaks the layout of whatever
/// is displaying them. Every stand-in is a single character, so byte offsets
/// change but character offsets are preserved.
///
/// The input is returned untouched when it holds no control characters, which
/// is the overwhelmingly common case.
///
/// # Examples
///
/// ```
/// use ui::utils::replace_control_characters;
///
/// assert_eq!(replace_control_characters("main.rs"), "main.rs");
/// assert_eq!(replace_control_characters("two\nlines.rs"), "two⏎lines.rs");
/// assert_eq!(replace_control_characters("tab\there.rs"), "tab␉here.rs");
/// ```
pub fn replace_control_characters(text: &str) -> Cow<'_, str> {
    let Some(first_control_character) = text.find(|character: char| character.is_ascii_control())
    else {
        return Cow::Borrowed(text);
    };

    let (printable_prefix, rest) = text.split_at(first_control_character);
    let mut replaced = String::with_capacity(text.len());
    replaced.push_str(printable_prefix);
    for character in rest.chars() {
        match printable_substitute(character) {
            Some(substitute) => replaced.push(substitute),
            None => replaced.push(character),
        }
    }

    Cow::Owned(replaced)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_printable_text_is_not_reallocated() {
        assert!(matches!(
            replace_control_characters("main.rs"),
            Cow::Borrowed("main.rs")
        ));
        assert!(matches!(replace_control_characters(""), Cow::Borrowed("")));
    }

    #[test]
    fn test_replaces_control_characters() {
        assert_eq!(replace_control_characters("a\nb"), "a⏎b");
        assert_eq!(replace_control_characters("a\rb"), "a␍b");
        assert_eq!(replace_control_characters("a\tb"), "a␉b");
        assert_eq!(replace_control_characters("a\0b"), "a␀b");
        assert_eq!(replace_control_characters("a\x7fb"), "a␡b");
    }

    #[test]
    fn test_replaces_every_occurrence() {
        assert_eq!(replace_control_characters("\r\n\t"), "␍⏎␉");
        assert_eq!(replace_control_characters("a\nb\nc"), "a⏎b⏎c");
    }

    #[test]
    fn test_preserves_non_control_characters() {
        // Multi-byte characters must survive the scan intact, including when
        // they sit next to a character that is replaced.
        assert_eq!(replace_control_characters("é\nü"), "é⏎ü");
        assert_eq!(replace_control_characters("🚀"), "🚀");
        assert_eq!(replace_control_characters("🚀\t🚀"), "🚀␉🚀");
    }

    #[test]
    fn test_character_count_is_preserved() {
        for text in ["a\nb", "\r\n\t", "a\x7fb", "é\nü"] {
            assert_eq!(
                replace_control_characters(text).chars().count(),
                text.chars().count(),
            );
        }
    }

    #[test]
    fn test_leaves_c1_control_characters_alone() {
        // These have no Control Pictures equivalent, so they are passed through.
        assert_eq!(replace_control_characters("a\u{0085}b"), "a\u{0085}b");
    }
}
