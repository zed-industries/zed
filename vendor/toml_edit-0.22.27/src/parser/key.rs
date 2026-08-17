use std::ops::RangeInclusive;

use winnow::combinator::peek;
use winnow::combinator::separated;
use winnow::combinator::trace;
use winnow::token::any;
use winnow::token::take_while;

use crate::key::Key;
use crate::parser::error::CustomError;
use crate::parser::prelude::*;
use crate::parser::strings::{basic_string, literal_string};
use crate::parser::trivia::{from_utf8_unchecked, ws};
use crate::repr::{Decor, Repr};
use crate::InternalString;
use crate::RawString;

// key = simple-key / dotted-key
// dotted-key = simple-key 1*( dot-sep simple-key )
pub(crate) fn key(input: &mut Input<'_>) -> ModalResult<Vec<Key>> {
    let mut key_path = trace(
        "dotted-key",
        separated(
            1..,
            (ws.span(), simple_key, ws.span()).map(|(pre, (raw, key), suffix)| {
                Key::new(key)
                    .with_repr_unchecked(Repr::new_unchecked(raw))
                    .with_dotted_decor(Decor::new(
                        RawString::with_span(pre),
                        RawString::with_span(suffix),
                    ))
            }),
            DOT_SEP,
        )
        .context(StrContext::Label("key"))
        .try_map(|k: Vec<_>| {
            // Inserting the key will require recursion down the line
            RecursionCheck::check_depth(k.len())?;
            Ok::<_, CustomError>(k)
        }),
    )
    .parse_next(input)?;

    let mut leaf_decor = Decor::new("", "");
    {
        let first_dotted_decor = key_path
            .first_mut()
            .expect("always at least one key")
            .dotted_decor_mut();
        if let Some(prefix) = first_dotted_decor.prefix().cloned() {
            leaf_decor.set_prefix(prefix);
            first_dotted_decor.set_prefix("");
        }
    }
    let last_key = &mut key_path.last_mut().expect("always at least one key");
    {
        let last_dotted_decor = last_key.dotted_decor_mut();
        if let Some(suffix) = last_dotted_decor.suffix().cloned() {
            leaf_decor.set_suffix(suffix);
            last_dotted_decor.set_suffix("");
        }
    }

    *last_key.leaf_decor_mut() = leaf_decor;

    Ok(key_path)
}

// simple-key = quoted-key / unquoted-key
// quoted-key = basic-string / literal-string
pub(crate) fn simple_key(input: &mut Input<'_>) -> ModalResult<(RawString, InternalString)> {
    trace(
        "simple-key",
        dispatch! {peek(any);
            crate::parser::strings::QUOTATION_MARK => basic_string
                .map(|s: std::borrow::Cow<'_, str>| s.as_ref().into()),
            crate::parser::strings::APOSTROPHE => literal_string.map(|s: &str| s.into()),
            _ => unquoted_key.map(|s: &str| s.into()),
        }
        .with_span()
        .map(|(k, span)| {
            let raw = RawString::with_span(span);
            (raw, k)
        }),
    )
    .parse_next(input)
}

// unquoted-key = 1*( ALPHA / DIGIT / %x2D / %x5F ) ; A-Z / a-z / 0-9 / - / _
fn unquoted_key<'i>(input: &mut Input<'i>) -> ModalResult<&'i str> {
    trace(
        "unquoted-key",
        take_while(1.., UNQUOTED_CHAR)
            .map(|b| unsafe { from_utf8_unchecked(b, "`is_unquoted_char` filters out on-ASCII") }),
    )
    .parse_next(input)
}

const UNQUOTED_CHAR: (
    RangeInclusive<u8>,
    RangeInclusive<u8>,
    RangeInclusive<u8>,
    u8,
    u8,
) = (b'A'..=b'Z', b'a'..=b'z', b'0'..=b'9', b'-', b'_');

// dot-sep   = ws %x2E ws  ; . Period
const DOT_SEP: u8 = b'.';

#[cfg(test)]
#[cfg(feature = "parse")]
#[cfg(feature = "display")]
mod test {
    use super::*;

    #[test]
    fn keys() {
        let cases = [
            ("a", "a"),
            (r#""hello\n ""#, "hello\n "),
            (r"'hello\n '", "hello\\n "),
        ];

        for (input, expected) in cases {
            dbg!(input);
            let parsed = simple_key.parse(new_input(input));
            assert_eq!(
                parsed,
                Ok((RawString::with_span(0..(input.len())), expected.into())),
                "Parsing {input:?}"
            );
        }
    }
}
