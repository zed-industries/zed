//! Helper functions and rules for enforcing the ABNF grammar for
//! RFC 7468-flavored PEM as described in Section 3.
//!
//! The grammar described below is intended to follow the "ABNF (Strict)"
//! subset of the grammar as described in Section 3 Figure 3.

use crate::{Error, Result, PRE_ENCAPSULATION_BOUNDARY};
use core::str;

/// NUL char
pub(crate) const CHAR_NUL: u8 = 0x00;

/// Horizontal tab
pub(crate) const CHAR_HT: u8 = 0x09;

/// Space
pub(crate) const CHAR_SP: u8 = 0x20;

/// Carriage return
pub(crate) const CHAR_CR: u8 = 0x0d;

/// Line feed
pub(crate) const CHAR_LF: u8 = 0x0a;

/// Colon ':'
pub(crate) const CHAR_COLON: u8 = 0x3A;

/// Any printable character except hyphen-minus, as defined in the
/// 'labelchar' production in the RFC 7468 ABNF grammar
pub(crate) fn is_labelchar(char: u8) -> bool {
    matches!(char, 0x21..=0x2C | 0x2E..=0x7E)
}

/// Does the provided byte match a character allowed in a label?
// TODO: allow hyphen-minus to match the 'label' production in the ABNF grammar
pub(crate) fn is_allowed_in_label(char: u8) -> bool {
    is_labelchar(char) || matches!(char, CHAR_HT | CHAR_SP)
}

/// Does the provided byte match the "WSP" ABNF production from Section 3?
///
/// > The common ABNF production WSP is congruent with "blank";
/// > a new production W is used for "whitespace"
pub(crate) fn is_wsp(char: u8) -> bool {
    matches!(char, CHAR_HT | CHAR_SP)
}

/// Strip the "preamble", i.e. data that appears before the PEM
/// pre-encapsulation boundary.
///
/// Presently no attempt is made to ensure the preamble decodes successfully
/// under any particular character encoding. The only byte which is disallowed
/// is the NUL byte. This restriction does not appear in RFC7468, but rather
/// is inspired by the OpenSSL PEM decoder.
///
/// Returns a slice which starts at the beginning of the encapsulated text.
///
/// From RFC7468:
/// > Data before the encapsulation boundaries are permitted, and
/// > parsers MUST NOT malfunction when processing such data.
pub(crate) fn strip_preamble(mut bytes: &[u8]) -> Result<&[u8]> {
    if bytes.starts_with(PRE_ENCAPSULATION_BOUNDARY) {
        return Ok(bytes);
    }

    while let Some((byte, remaining)) = bytes.split_first() {
        match *byte {
            CHAR_NUL => {
                return Err(Error::Preamble);
            }
            CHAR_LF if remaining.starts_with(PRE_ENCAPSULATION_BOUNDARY) => {
                return Ok(remaining);
            }
            _ => (),
        }

        bytes = remaining;
    }

    Err(Error::Preamble)
}

/// Strip a newline (`eol`) from the beginning of the provided byte slice.
///
/// The newline is considered mandatory and a decoding error will occur if it
/// is not present.
///
/// From RFC 7468 Section 3:
/// > lines are divided with CRLF, CR, or LF.
pub(crate) fn strip_leading_eol(bytes: &[u8]) -> Option<&[u8]> {
    match bytes {
        [CHAR_LF, rest @ ..] => Some(rest),
        [CHAR_CR, CHAR_LF, rest @ ..] => Some(rest),
        [CHAR_CR, rest @ ..] => Some(rest),
        _ => None,
    }
}

/// Strip a newline (`eol`) from the end of the provided byte slice.
///
/// The newline is considered mandatory and a decoding error will occur if it
/// is not present.
///
/// From RFC 7468 Section 3:
/// > lines are divided with CRLF, CR, or LF.
pub(crate) fn strip_trailing_eol(bytes: &[u8]) -> Option<&[u8]> {
    match bytes {
        [head @ .., CHAR_CR, CHAR_LF] => Some(head),
        [head @ .., CHAR_LF] => Some(head),
        [head @ .., CHAR_CR] => Some(head),
        _ => None,
    }
}

/// Split a slice beginning with a type label as located in an encapsulation
/// boundary. Returns the label as a `&str`, and slice beginning with the
/// encapsulated text with leading `-----` and newline removed.
///
/// This implementation follows the rules put forth in Section 2, which are
/// stricter than those found in the ABNF grammar:
///
/// > Labels are formally case-sensitive, uppercase, and comprised of zero or more
/// > characters; they do not contain consecutive spaces or hyphen-minuses,
/// > nor do they contain spaces or hyphen-minuses at either end.
///
/// We apply a slightly stricter interpretation:
/// - Labels MAY be empty
/// - Non-empty labels MUST start with an upper-case letter: `'A'..='Z'`
/// - The only allowable characters subsequently are `'A'..='Z'` or WSP.
///   (NOTE: this is an overly strict initial implementation and should be relaxed)
/// - Whitespace MUST NOT contain more than one consecutive WSP character
// TODO(tarcieri): evaluate whether this is too strict; support '-'
pub(crate) fn split_label(bytes: &[u8]) -> Option<(&str, &[u8])> {
    let mut n = 0usize;

    // TODO(tarcieri): handle hyphens in labels as well as spaces
    let mut last_was_wsp = false;

    for &char in bytes {
        // Validate character
        if is_labelchar(char) {
            last_was_wsp = false;
        } else if char == b'-' {
            // Possible start of encapsulation boundary delimiter
            break;
        } else if n != 0 && is_wsp(char) {
            // Repeated whitespace disallowed
            if last_was_wsp {
                return None;
            }

            last_was_wsp = true;
        } else {
            return None;
        }

        n = n.checked_add(1)?;
    }

    let (raw_label, rest) = bytes.split_at(n);
    let label = str::from_utf8(raw_label).ok()?;

    match rest {
        [b'-', b'-', b'-', b'-', b'-', body @ ..] => Some((label, strip_leading_eol(body)?)),
        _ => None,
    }
}

/// Validate that the given bytes are allowed as a PEM type label, i.e. the
/// label encoded in the `BEGIN` and `END` encapsulation boundaries.
pub(crate) fn validate_label(label: &[u8]) -> Result<()> {
    // TODO(tarcieri): handle hyphens in labels as well as spaces
    let mut last_was_wsp = false;

    for &char in label {
        if !is_allowed_in_label(char) {
            return Err(Error::Label);
        }

        if is_wsp(char) {
            // Double sequential whitespace characters disallowed
            if last_was_wsp {
                return Err(Error::Label);
            }

            last_was_wsp = true;
        } else {
            last_was_wsp = false;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Empty label is OK.
    #[test]
    fn split_label_empty() {
        let (label, body) = split_label(b"-----\nBODY").unwrap();
        assert_eq!(label, "");
        assert_eq!(body, b"BODY");
    }

    /// Label containing text.
    #[test]
    fn split_label_with_text() {
        let (label, body) = split_label(b"PRIVATE KEY-----\nBODY").unwrap();
        assert_eq!(label, "PRIVATE KEY");
        assert_eq!(body, b"BODY");
    }

    /// Reject labels containing repeated spaces
    #[test]
    fn split_label_with_repeat_wsp_is_err() {
        assert!(split_label(b"PRIVATE  KEY-----\nBODY").is_none());
    }

    /// Basic validation of a label
    #[test]
    fn validate_private_key_label() {
        assert_eq!(validate_label(b"PRIVATE KEY"), Ok(()));
    }

    /// Reject labels with double spaces
    #[test]
    fn validate_private_key_label_reject_double_space() {
        assert_eq!(validate_label(b"PRIVATE  KEY"), Err(Error::Label));
    }
}
