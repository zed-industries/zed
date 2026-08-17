//! Holds functions to determine if a character belongs to a specific character set.

/// Check whether the character is nil (`\0`).
#[inline]
pub(crate) fn is_z(c: char) -> bool {
    c == '\0'
}

/// Check whether the character is a line break (`\r` or `\n`).
#[inline]
pub(crate) fn is_break(c: char) -> bool {
    c == '\n' || c == '\r'
}

/// Check whether the character is nil or a line break (`\0`, `\r`, `\n`).
#[inline]
pub(crate) fn is_breakz(c: char) -> bool {
    is_break(c) || is_z(c)
}

/// Check whether the character is a whitespace (` ` or `\t`).
#[inline]
pub(crate) fn is_blank(c: char) -> bool {
    c == ' ' || c == '\t'
}

/// Check whether the character is nil, a linebreak or a whitespace.
///
/// `\0`, ` `, `\t`, `\n`, `\r`
#[inline]
pub(crate) fn is_blank_or_breakz(c: char) -> bool {
    is_blank(c) || is_breakz(c)
}

/// Check whether the character is an ascii digit.
#[inline]
pub(crate) fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

/// Check whether the character is a digit, letter, `_` or `-`.
#[inline]
pub(crate) fn is_alpha(c: char) -> bool {
    matches!(c, '0'..='9' | 'a'..='z' | 'A'..='Z' | '_' | '-')
}

/// Check whether the character is a hexadecimal character (case insensitive).
#[inline]
pub(crate) fn is_hex(c: char) -> bool {
    c.is_ascii_digit() || ('a'..='f').contains(&c) || ('A'..='F').contains(&c)
}

/// Convert the hexadecimal digit to an integer.
#[inline]
pub(crate) fn as_hex(c: char) -> u32 {
    match c {
        '0'..='9' => (c as u32) - ('0' as u32),
        'a'..='f' => (c as u32) - ('a' as u32) + 10,
        'A'..='F' => (c as u32) - ('A' as u32) + 10,
        _ => unreachable!(),
    }
}

/// Check whether the character is a YAML flow character (one of `,[]{}`).
#[inline]
pub(crate) fn is_flow(c: char) -> bool {
    matches!(c, ',' | '[' | ']' | '{' | '}')
}

/// Check whether the character is the BOM character.
#[inline]
pub(crate) fn is_bom(c: char) -> bool {
    c == '\u{FEFF}'
}

/// Check whether the character is a YAML non-breaking character.
#[inline]
pub(crate) fn is_yaml_non_break(c: char) -> bool {
    // TODO(ethiraric, 28/12/2023): is_printable
    !is_break(c) && !is_bom(c)
}

/// Check whether the character is NOT a YAML whitespace (` ` / `\t`).
#[inline]
pub(crate) fn is_yaml_non_space(c: char) -> bool {
    is_yaml_non_break(c) && !is_blank(c)
}

/// Check whether the character is a valid YAML anchor name character.
#[inline]
pub(crate) fn is_anchor_char(c: char) -> bool {
    is_yaml_non_space(c) && !is_flow(c) && !is_z(c)
}

/// Check whether the character is a valid word character.
#[inline]
pub(crate) fn is_word_char(c: char) -> bool {
    is_alpha(c) && c != '_'
}

/// Check whether the character is a valid URI character.
#[inline]
pub(crate) fn is_uri_char(c: char) -> bool {
    is_word_char(c) || "#;/?:@&=+$,_.!~*\'()[]%".contains(c)
}

/// Check whether the character is a valid tag character.
#[inline]
pub(crate) fn is_tag_char(c: char) -> bool {
    is_uri_char(c) && !is_flow(c) && c != '!'
}

/// Check if the string can be expressed a valid literal block scalar.
/// The YAML spec supports all of the following in block literals except `#xFEFF`:
/// ```no_compile
///     #x9 | #xA | [#x20-#x7E]                /* 8 bit */
///   | #x85 | [#xA0-#xD7FF] | [#xE000-#xFFFD] /* 16 bit */
///   | [#x10000-#x10FFFF]                     /* 32 bit */
/// ```
#[inline]
pub(crate) fn is_valid_literal_block_scalar(string: &str) -> bool {
    string.chars().all(|character: char|
        matches!(character, '\t' | '\n' | '\x20'..='\x7e' | '\u{0085}' | '\u{00a0}'..='\u{d7fff}'))
}
