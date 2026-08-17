use crate::{
    err::{perr, ParseErrorKind::*},
    parse::{check_suffix, hex_digit_value},
    ParseError,
};


/// Must start with `\`. Returns the unscaped value as `E` and the number of
/// input bytes the escape is long.
///
/// `unicode` and `byte_escapes` specify which types of escapes are
/// supported. [Quote escapes] are always unescaped, [Unicode escapes] only if
/// `unicode` is true. If `byte_escapes` is false, [ASCII escapes] are
/// used, if it's true, [Byte escapes] are (the only difference being that the
/// latter supports \xHH escapes > 0x7f).
///
/// [Quote escapes]: https://doc.rust-lang.org/reference/tokens.html#quote-escapes
/// [Unicode escapes]: https://doc.rust-lang.org/reference/tokens.html#unicode-escapes
/// [Ascii escapes]: https://doc.rust-lang.org/reference/tokens.html#ascii-escapes
/// [Byte escapes]: https://doc.rust-lang.org/reference/tokens.html#byte-escapes
pub(crate) fn unescape(
    input: &str,
    unicode: bool,
    byte_escapes: bool,
    allow_nul: bool,
) -> Result<(Unescape, usize), ParseError> {
    let first = input.as_bytes().get(1).ok_or(perr(0, UnterminatedEscape))?;
    let out = match first {
        // Quote escapes
        b'\'' => (Unescape::Byte(b'\''), 2),
        b'"' => (Unescape::Byte(b'"'), 2),

        // Ascii escapes
        b'n' => (Unescape::Byte(b'\n'), 2),
        b'r' => (Unescape::Byte(b'\r'), 2),
        b't' => (Unescape::Byte(b'\t'), 2),
        b'\\' => (Unescape::Byte(b'\\'), 2),
        b'0' => if allow_nul {
            (Unescape::Byte(b'\0'), 2)
        } else {
            return Err(perr(0..2, DisallowedNulEscape))
        },
        b'x' => {
            let hex_string = input.get(2..4)
                .ok_or(perr(0..input.len(), UnterminatedEscape))?
                .as_bytes();
            let first = hex_digit_value(hex_string[0]).ok_or(perr(0..4, InvalidXEscape))?;
            let second = hex_digit_value(hex_string[1]).ok_or(perr(0..4, InvalidXEscape))?;
            let value = second + 16 * first;

            if !byte_escapes && value > 0x7F {
                return Err(perr(0..4, NonAsciiXEscape));
            }

            if !allow_nul && value == 0 {
                return Err(perr(0..4, DisallowedNulEscape));
            }

            (Unescape::Byte(value), 4)
        }

        // Unicode escape
        b'u' => {
            if !unicode {
                return Err(perr(0..2, UnicodeEscapeInByteLiteral));
            }

            if input.as_bytes().get(2) != Some(&b'{') {
                return Err(perr(0..2, UnicodeEscapeWithoutBrace));
            }

            let closing_pos = input.bytes().position(|b| b == b'}')
                .ok_or(perr(0..input.len(), UnterminatedUnicodeEscape))?;

            let inner = &input[3..closing_pos];
            if inner.as_bytes().first() == Some(&b'_') {
                return Err(perr(3, InvalidStartOfUnicodeEscape));
            }

            let mut v: u32 = 0;
            let mut digit_count = 0;
            for (i, b) in inner.bytes().enumerate() {
                if b == b'_' {
                    continue;
                }

                let digit = hex_digit_value(b).ok_or(perr(3 + i, NonHexDigitInUnicodeEscape))?;

                if digit_count == 6 {
                    return Err(perr(3 + i, TooManyDigitInUnicodeEscape));
                }
                digit_count += 1;
                v = 16 * v + digit as u32;
            }

            if !allow_nul && v == 0 {
                return Err(perr(0..closing_pos + 1, DisallowedNulEscape));
            }

            let c = std::char::from_u32(v)
                .ok_or(perr(0..closing_pos + 1, InvalidUnicodeEscapeChar))?;

            (Unescape::Unicode(c), closing_pos + 1)
        }

        _ => return Err(perr(0..2, UnknownEscape)),
    };

    Ok(out)
}

/// Result of unescaping an escape-sequence in a string.
pub(crate) enum Unescape {
    Byte(u8),
    Unicode(char),
}

impl Unescape {
    /// Returns this value as `char`, panicking if it's a byte with a value > 0x7f.
    pub(crate) fn unwrap_char(self) -> char {
        match self {
            Self::Byte(b) => {
                assert!(b <= 0x7F, "non ASCII byte");
                b.into()
            }
            Self::Unicode(c) => c,
        }
    }

    /// Returns this value as `u8`, panicking if it was `Unicode`.
    pub(crate) fn unwrap_byte(self) -> u8 {
        match self {
            Self::Byte(b) => b,
            Self::Unicode(_) => panic!("unexpected unicode escape value"),
        }
    }
}

pub(crate) trait EscapeContainer {
    fn new() -> Self;
    fn is_empty(&self) -> bool;
    fn push(&mut self, v: Unescape);
    fn push_str(&mut self, s: &str);
}

impl EscapeContainer for Vec<u8> {
    fn new() -> Self {
        Self::new()
    }
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
    fn push_str(&mut self, s: &str) {
        self.extend_from_slice(s.as_bytes());
    }
    fn push(&mut self, v: Unescape) {
        match v {
            Unescape::Byte(b) => self.push(b),
            Unescape::Unicode(c) => {
                let start = self.len();
                self.resize(self.len() + c.len_utf8(), 0);
                c.encode_utf8(&mut self[start..]);
            }
        }
    }
}

impl EscapeContainer for String {
    fn new() -> Self {
        Self::new()
    }
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
    fn push_str(&mut self, s: &str) {
        self.push_str(s);
    }
    fn push(&mut self, v: Unescape) {
        self.push(v.unwrap_char());
    }
}


/// Checks whether the character is skipped after a string continue start
/// (unescaped backlash followed by `\n`).
fn is_string_continue_skipable_whitespace(b: u8) -> bool {
    b == b' ' || b == b'\t' || b == b'\n'
}

/// Unescapes a whole string or byte string.
#[inline(never)]
pub(crate) fn unescape_string<C: EscapeContainer>(
    input: &str,
    offset: usize,
    unicode: bool,
    byte_escapes: bool,
    allow_nul: bool,
) -> Result<(Option<C>, usize), ParseError> {
    let mut closing_quote_pos = None;
    let mut i = offset;
    let mut end_last_escape = offset;
    let mut value = C::new();
    while i < input.len() {
        match input.as_bytes()[i] {
            // Handle "string continue".
            b'\\' if input.as_bytes().get(i + 1) == Some(&b'\n') => {
                value.push_str(&input[end_last_escape..i]);

                // Find the first non-whitespace character.
                let end_escape = input[i + 2..].bytes()
                    .position(|b| !is_string_continue_skipable_whitespace(b))
                    .ok_or(perr(None, UnterminatedString))?;

                i += 2 + end_escape;
                end_last_escape = i;
            }
            b'\\' => {
                let rest = &input[i..input.len() - 1];
                let (c, len) = unescape(rest, unicode, byte_escapes, allow_nul)
                    .map_err(|e| e.offset_span(i))?;
                value.push_str(&input[end_last_escape..i]);
                value.push(c);
                i += len;
                end_last_escape = i;
            }
            b'\r' => return Err(perr(i, CarriageReturn)),
            b'"' => {
                closing_quote_pos = Some(i);
                break;
            }
            b'\0' if !allow_nul => return Err(perr(i, NulByte)),
            b if !unicode && !b.is_ascii() => return Err(perr(i, NonAsciiInByteLiteral)),
            _ => i += 1,
        }
    }

    let closing_quote_pos = closing_quote_pos.ok_or(perr(None, UnterminatedString))?;

    let start_suffix = closing_quote_pos + 1;
    let suffix = &input[start_suffix..];
    check_suffix(suffix).map_err(|kind| perr(start_suffix, kind))?;

    // `value` is only empty if there was no escape in the input string
    // (with the special case of the input being empty). This means the
    // string value basically equals the input, so we store `None`.
    let value = if value.is_empty() {
        None
    } else {
        // There was an escape in the string, so we need to push the
        // remaining unescaped part of the string still.
        value.push_str(&input[end_last_escape..closing_quote_pos]);
        Some(value)
    };

    Ok((value, start_suffix))
}

/// Reads and checks a raw (byte) string literal. Returns the number of hashes
/// and the index when the suffix starts.
#[inline(never)]
pub(crate) fn scan_raw_string(
    input: &str,
    offset: usize,
    unicode: bool,
    allow_nul: bool,
) -> Result<(u8, usize), ParseError> {
    // Raw string literal
    let num_hashes = input[offset..].bytes().position(|b| b != b'#')
        .ok_or(perr(None, InvalidLiteral))?;

    if num_hashes > 256 {
        return Err(perr(offset..offset + num_hashes, TooManyHashes));
    }

    if input.as_bytes().get(offset + num_hashes) != Some(&b'"') {
        return Err(perr(None, InvalidLiteral));
    }
    let start_inner = offset + num_hashes + 1;
    let hashes = &input[offset..num_hashes + offset];

    let mut closing_quote_pos = None;
    let mut i = start_inner;
    while i < input.len() {
        let b = input.as_bytes()[i];
        if b == b'"' && input[i + 1..].starts_with(hashes) {
            closing_quote_pos = Some(i);
            break;
        }

        // CR are just always disallowed in all (raw) strings. Rust performs
        // a normalization of CR LF to just LF in a pass prior to lexing. But
        // in lexing, it's disallowed.
        if b == b'\r' {
            return Err(perr(i, CarriageReturn));
        }

        if b == b'\0' && !allow_nul {
            return Err(perr(i, NulByte));
        }

        if !unicode {
            if !b.is_ascii() {
                return Err(perr(i, NonAsciiInByteLiteral));
            }
        }

        i += 1;
    }

    let closing_quote_pos = closing_quote_pos.ok_or(perr(None, UnterminatedRawString))?;

    let start_suffix = closing_quote_pos + num_hashes + 1;
    let suffix = &input[start_suffix..];
    check_suffix(suffix).map_err(|kind| perr(start_suffix, kind))?;

    Ok((num_hashes as u8, start_suffix))
}
