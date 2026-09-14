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

/// Like [`replace_control_characters`], but also moves the given byte offsets
/// into the replaced string, and returns `None` when nothing was replaced.
///
/// Callers that carry byte offsets into the text — highlight positions, for
/// instance — cannot use [`replace_control_characters`] on its own: every
/// stand-in is wider in bytes than the character it replaces, so offsets past
/// the first replacement would land inside a character. Indexing a `str` at
/// such an offset panics in release builds too, so remapping is not optional.
///
/// Each offset must sit on a character boundary. Offsets past the end of the
/// text are clamped to the end of the replaced string.
///
/// # Examples
///
/// ```
/// use ui::utils::replace_control_characters_remapping_offsets;
///
/// // The offset of "b" moves from 2 to 4, the stand-in being three bytes wide.
/// let mut offsets = vec![0, 2];
/// let replaced = replace_control_characters_remapping_offsets("a\tb", &mut offsets);
/// assert_eq!(replaced.as_deref(), Some("a␉b"));
/// assert_eq!(offsets, vec![0, 4]);
/// ```
pub fn replace_control_characters_remapping_offsets(
    text: &str,
    offsets: &mut [usize],
) -> Option<String> {
    // Control characters are single bytes below 0x80, so scanning raw bytes
    // cannot mistake a UTF-8 continuation byte for one.
    if !text.bytes().any(|byte| byte.is_ascii_control()) {
        return None;
    }

    // Maps every byte offset in `text` to its offset in the replaced string.
    // Only entries on character boundaries are filled in, which is all callers
    // are allowed to ask about.
    let mut moved = vec![0; text.len() + 1];
    let mut replaced = String::with_capacity(text.len());
    for (offset, character) in text.char_indices() {
        moved[offset] = replaced.len();
        match printable_substitute(character) {
            Some(substitute) => replaced.push(substitute),
            None => replaced.push(character),
        }
    }
    moved[text.len()] = replaced.len();

    for offset in offsets {
        debug_assert!(
            *offset > text.len() || text.is_char_boundary(*offset),
            "offset {offset} is not on a character boundary of {text:?}",
        );
        *offset = moved.get(*offset).copied().unwrap_or(replaced.len());
    }

    Some(replaced)
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

    #[test]
    fn test_printable_text_is_left_alone_when_remapping() {
        let mut offsets = vec![0, 3];
        assert_eq!(
            replace_control_characters_remapping_offsets("main.rs", &mut offsets),
            None,
        );
        assert_eq!(offsets, vec![0, 3], "offsets must not move");
    }

    #[test]
    fn test_remaps_offsets_after_a_replacement() {
        // Each stand-in is two bytes wider than what it replaces, so offsets
        // shift by two per replacement that precedes them.
        let mut offsets = vec![0, 1, 2];
        let replaced = replace_control_characters_remapping_offsets("a\tb", &mut offsets).unwrap();
        assert_eq!(replaced, "a␉b");
        assert_eq!(offsets, vec![0, 1, 4]);
    }

    #[test]
    fn test_remaps_offsets_across_several_replacements() {
        let mut offsets = vec![0, 2, 4];
        let replaced =
            replace_control_characters_remapping_offsets("a\tb\tc", &mut offsets).unwrap();
        assert_eq!(replaced, "a␉b␉c");
        assert_eq!(offsets, vec![0, 4, 8]);
    }

    #[test]
    fn test_remaps_offsets_around_multi_byte_characters() {
        // "é" is already two bytes, so the mapping must count real byte widths
        // rather than assume one byte per character.
        let mut offsets = vec![0, 2, 5];
        let replaced = replace_control_characters_remapping_offsets("é\tü", &mut offsets).unwrap();
        assert_eq!(replaced, "é␉ü");
        assert_eq!(offsets, vec![0, 2, 5 + 2]);
    }

    #[test]
    fn test_remapped_offsets_stay_on_character_boundaries() {
        for text in ["a\nb", "\r\n\t", "é\nü", "🚀\t🚀", "a\x7fb"] {
            let mut offsets: Vec<usize> = text.char_indices().map(|(ix, _)| ix).collect();
            let replaced =
                replace_control_characters_remapping_offsets(text, &mut offsets).unwrap();
            for offset in offsets {
                assert!(
                    replaced.is_char_boundary(offset),
                    "offset {offset} is not a boundary of {replaced:?} (from {text:?})",
                );
            }
        }
    }

    #[test]
    fn test_remaps_the_end_offset() {
        // Highlight ranges may end at the very end of the text.
        let mut offsets = vec!["a\tb".len()];
        let replaced = replace_control_characters_remapping_offsets("a\tb", &mut offsets).unwrap();
        assert_eq!(offsets, vec![replaced.len()]);
    }

    #[test]
    fn test_clamps_offsets_past_the_end() {
        let mut offsets = vec![999];
        let replaced = replace_control_characters_remapping_offsets("a\tb", &mut offsets).unwrap();
        assert_eq!(offsets, vec![replaced.len()]);
    }

    #[test]
    fn test_remapping_agrees_with_the_borrowing_variant() {
        for text in ["main.rs", "a\nb", "\r\n\t", "é\nü", "🚀\t🚀"] {
            let remapped = replace_control_characters_remapping_offsets(text, &mut []);
            let borrowed = replace_control_characters(text);
            match remapped {
                Some(replaced) => assert_eq!(replaced, borrowed.as_ref()),
                None => assert!(matches!(borrowed, Cow::Borrowed(_))),
            }
        }
    }
}
