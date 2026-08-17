use crate::{
    err::{
        perr,
        ParseErrorKind::{self, *},
    },
    BoolLit, Buffer, ByteLit, ByteStringLit, CStringLit, CharLit, FloatLit, IntegerLit, Literal,
    ParseError, StringLit,
};


pub fn parse<B: Buffer>(input: B) -> Result<Literal<B>, ParseError> {
    let (first, rest) = input.as_bytes().split_first().ok_or(perr(None, Empty))?;
    let second = input.as_bytes().get(1).copied();

    match first {
        b'f' if &*input == "false" => Ok(Literal::Bool(BoolLit::False)),
        b't' if &*input == "true" => Ok(Literal::Bool(BoolLit::True)),

        // A number literal (integer or float).
        b'0'..=b'9' => {
            // To figure out whether this is a float or integer, we do some
            // quick inspection here. Yes, this is technically duplicate
            // work with what is happening in the integer/float parse
            // methods, but it makes the code way easier for now and won't
            // be a huge performance loss.
            //
            // The first non-decimal char in a float literal must
            // be '.', 'e' or 'E'.
            match input.as_bytes().get(1 + end_dec_digits(rest)) {
                Some(b'.') | Some(b'e') | Some(b'E') => FloatLit::parse(input).map(Literal::Float),

                _ => IntegerLit::parse(input).map(Literal::Integer),
            }
        }

        b'\'' => CharLit::parse(input).map(Literal::Char),
        b'"' | b'r' => StringLit::parse(input).map(Literal::String),

        b'b' if second == Some(b'\'') => ByteLit::parse(input).map(Literal::Byte),
        b'b' if second == Some(b'r') || second == Some(b'"') => {
            ByteStringLit::parse(input).map(Literal::ByteString)
        }

        b'c' => CStringLit::parse(input).map(Literal::CString),

        _ => Err(perr(None, InvalidLiteral)),
    }
}


pub(crate) fn first_byte_or_empty(s: &str) -> Result<u8, ParseError> {
    s.as_bytes().first().copied().ok_or(perr(None, Empty))
}

/// Returns the index of the first non-underscore, non-decimal digit in `input`,
/// or the `input.len()` if all characters are decimal digits.
pub(crate) fn end_dec_digits(input: &[u8]) -> usize {
    input.iter()
        .position(|b| !matches!(b, b'_' | b'0'..=b'9'))
        .unwrap_or(input.len())
}

pub(crate) fn hex_digit_value(digit: u8) -> Option<u8> {
    match digit {
        b'0'..=b'9' => Some(digit - b'0'),
        b'a'..=b'f' => Some(digit - b'a' + 10),
        b'A'..=b'F' => Some(digit - b'A' + 10),
        _ => None,
    }
}

/// Makes sure that `s` is a valid literal suffix.
pub(crate) fn check_suffix(s: &str) -> Result<(), ParseErrorKind> {
    if s.is_empty() {
        return Ok(());
    }

    let mut chars = s.chars();
    let first = chars.next().unwrap();
    let rest = chars.as_str();
    if first == '_' && rest.is_empty() {
        return Err(InvalidSuffix);
    }

    // This is just an extra check to improve the error message. If the first
    // character of the "suffix" is already some invalid ASCII
    // char, "unexpected character" seems like the more fitting error.
    if first.is_ascii() && !(first.is_ascii_alphabetic() || first == '_') {
        return Err(UnexpectedChar);
    }

    // Proper check is optional as it's not really necessary in proc macro
    // context.
    #[cfg(feature = "check_suffix")]
    fn is_valid_suffix(first: char, rest: &str) -> bool {
        use unicode_xid::UnicodeXID;

        (first == '_' || first.is_xid_start())
            && rest.chars().all(|c| c.is_xid_continue())
    }

    // When avoiding the dependency on `unicode_xid`, we just do a best effort
    // to catch the most common errors.
    #[cfg(not(feature = "check_suffix"))]
    fn is_valid_suffix(first: char, rest: &str) -> bool {
        if first.is_ascii() && !(first.is_ascii_alphabetic() || first == '_') {
            return false;
        }
        for c in rest.chars() {
            if c.is_ascii() && !(c.is_ascii_alphanumeric() || c == '_') {
                return false;
            }
        }
        true
    }

    if is_valid_suffix(first, rest) {
        Ok(())
    } else {
        Err(InvalidSuffix)
    }
}
