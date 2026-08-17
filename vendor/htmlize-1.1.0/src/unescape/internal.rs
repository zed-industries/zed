//! Internal unescape code.

use std::borrow::Cow;
use std::char;
use std::num::IntErrorKind;
use std::result::Result;
use std::slice;

/// See [`super::unescape_in()`].
///
/// # Panics
///
/// Panics if the unescaped bytes are invalid UTF-8.
pub fn unescape_in<'a, M: Matcher, S: Into<Cow<'a, str>>>(
    _matcher: M,
    escaped: S,
) -> Cow<'a, str> {
    let escaped = escaped.into();
    let bytes = escaped.as_bytes();
    match unescape_in_internal::<M>(bytes) {
        Some(buffer) => String::from_utf8(buffer).unwrap().into(),
        None => escaped,
    }
}

/// See [`super::unescape_bytes_in()`].
pub fn unescape_bytes_in<'a, M: Matcher, S: Into<Cow<'a, [u8]>>>(
    _matcher: M,
    escaped: S,
) -> Cow<'a, [u8]> {
    let escaped = escaped.into();
    match unescape_in_internal::<M>(&escaped) {
        Some(buffer) => buffer.into(),
        None => escaped,
    }
}

/// Code that actually does the unescaping.
///
/// Returns `None` if no changes would be made.
fn unescape_in_internal<M: Matcher>(escaped: &[u8]) -> Option<Vec<u8>> {
    let mut amp_iter = memchr::memchr_iter(b'&', escaped);
    while let Some(i) = amp_iter.next() {
        let mut byte_iter = escaped[i..].iter();
        if let Some(expansion) = M::match_entity(&mut byte_iter) {
            // We know there is at least one expansion.

            // All but two entities are as long or longer than their expansion,
            // so allocating the output buffer to be the same size as the input
            // will usually prevent multiple allocations and generally won’t
            // over-allocate by very much.
            //
            // The two entities are `&nGg;` (≫⃒) and `&nLl;` (≪⃒) which are both
            // five byte entities with six byte expansions.
            let mut buffer = Vec::with_capacity(escaped.len());
            buffer.extend_from_slice(&escaped[..i]);
            buffer.extend_from_slice(&expansion);

            #[allow(
                clippy::arithmetic_side_effects,
                reason = "byte_iter.as_slice().len() has to be < escaped.len()"
            )]
            let mut last_end = escaped.len() - byte_iter.as_slice().len();
            for i in amp_iter {
                let mut byte_iter = escaped[i..].iter();
                #[allow(
                    clippy::arithmetic_side_effects,
                    reason = "byte_iter.as_slice().len() has to be < escaped.len()"
                )]
                if let Some(expansion) = M::match_entity(&mut byte_iter) {
                    buffer.extend_from_slice(&escaped[last_end..i]);
                    buffer.extend_from_slice(&expansion);
                    last_end = escaped.len() - byte_iter.as_slice().len();
                }
            }

            buffer.extend_from_slice(&escaped[last_end..]);
            return Some(buffer);
        }
    }

    None
}

/// A Phf-based matcher.
#[cfg(feature = "unescape")]
pub struct Phf;

/// A matchgen-based matcher.
#[cfg(feature = "unescape_fast")]
pub struct Matchgen;

/// Unescape context: from inside an HTML attribute.
#[derive(Clone, Copy, Debug)]
pub struct ContextAttribute;

/// Unescape context: from anywhere outside of an HTML attribute, i.e. regular
/// text. This is generally what you want.
pub struct ContextGeneral;

/// Interface for an entity matching algorithm.
pub trait Matcher {
    /// Match an entity at the beginning of `iter`. Either:
    ///
    ///   * It finds a match: returns `Some(expansion)` and `iter` is updated to
    ///     point to the next character after the entity.
    ///   * It doesn’t find a match: returns `None` and `iter` is updated to
    ///     point to the next character than could plausibly start an entity
    ///     (not necessarily b'&', though; the only guarantee is that we didn’t
    ///     skip a potential entity).
    ///
    /// This version uses matchgen instead of the `ENTITIES` map. It is faster
    /// at runtime but slower to build.
    fn match_entity<'a>(iter: &'a mut slice::Iter<u8>)
        -> Option<Cow<'a, [u8]>>;
}

// Include function to match entities at the start of an iterator. Used in
// `match_entity()`.
//
// fn entity_matcher<'a, I>(iter: &mut I) -> Option<(bool, &'static [u8])>
// where
//     I: Iterator<Item = &'a u8> + Clone,
#[cfg(feature = "unescape_fast")]
include!(concat!(env!("OUT_DIR"), "/matcher.rs"));

#[cfg(feature = "unescape_fast")]
impl Matcher for (Matchgen, ContextAttribute) {
    fn match_entity<'a>(
        iter: &'a mut slice::Iter<u8>,
    ) -> Option<Cow<'a, [u8]>> {
        assert_peek_eq(iter, Some(b'&'), "match_entity() expected '&'");

        if Some(b'#') == peek_n(iter, 1) {
            // Numeric entity.
            return match_numeric_entity(iter);
        }

        let slice = iter.as_slice();
        let (expansion, rest) = entity_matcher(slice);
        #[allow(
            clippy::arithmetic_side_effects,
            reason = "rest is a subslice of slice"
        )]
        let consumed = slice.len() - rest.len();
        if consumed > 0 {
            #[allow(clippy::arithmetic_side_effects, reason = "checked")]
            iter.nth(consumed - 1); // nth(0) is equivalent to next()
        }

        // In an attribute entities ending with an alphanumeric character or '='
        // instead of ';' are passed through without expansion.
        //
        // See `unescape_in()` documentation for examples.
        //
        // https://html.spec.whatwg.org/multipage/parsing.html#named-character-reference-state
        if let Some((closed, expansion)) = expansion {
            if !closed {
                if let Some(next) = peek(iter) {
                    if next == b'=' || next.is_ascii_alphanumeric() {
                        return None;
                    }
                }
            }

            Some(expansion.into())
        } else {
            // Move past initial b'&'.
            iter.next();
            None
        }
    }
}

#[cfg(feature = "unescape_fast")]
impl Matcher for (Matchgen, ContextGeneral) {
    fn match_entity<'a>(
        iter: &'a mut slice::Iter<u8>,
    ) -> Option<Cow<'a, [u8]>> {
        assert_peek_eq(iter, Some(b'&'), "match_entity() expected '&'");

        if Some(b'#') == peek_n(iter, 1) {
            // Numeric entity.
            return match_numeric_entity(iter);
        }

        let slice = iter.as_slice();
        let (expansion, rest) = entity_matcher(slice);
        #[allow(
            clippy::arithmetic_side_effects,
            reason = "rest is a subslice of slice"
        )]
        let consumed = slice.len() - rest.len();
        if consumed > 0 {
            #[allow(clippy::arithmetic_side_effects, reason = "checked")]
            iter.nth(consumed - 1); // nth(0) is equivalent to next()
        }
        expansion
            .map(|(_, expansion)| expansion.into())
            .or_else(|| {
                // No match; move past initial b'&'.
                iter.next();
                None
            })
    }
}

/// A panic message we use repeatedly.
const PEEK_MATCH_ERROR: &str = "iter.next() did not match previous peek(iter)";

#[cfg(feature = "unescape")]
impl Matcher for (Phf, ContextAttribute) {
    fn match_entity<'a>(
        iter: &'a mut slice::Iter<u8>,
    ) -> Option<Cow<'a, [u8]>> {
        use crate::{ENTITIES, ENTITY_MIN_LENGTH};
        assert_peek_eq(iter, Some(b'&'), "match_entity() expected '&'");

        if Some(b'#') == peek_n(iter, 1) {
            // Numeric entity.
            return match_numeric_entity(iter);
        }

        let raw = &iter.as_slice();

        find_longest_candidate(iter);
        match peek(iter) {
            Some(b';') => {
                // Actually consume the semicolon.
                assert_next_eq(iter, Some(b';'), PEEK_MATCH_ERROR);
            }
            Some(b'=') => {
                // In an attribute, entities ending with an alphanumeric
                // character or '=' instead of ';' are passed through without
                // expansion.
                //
                // See `unescape_in()` documentation for examples.
                //
                // Alphanumeric characters don’t matter here because either:
                //
                //   * The loop above consumed ENTITY_MAX_LENGTH-1 alphanumeric
                //     characters, so it can’t match an entity because it didn’t
                //     find a ';' (the longest entity ends with a ';').
                //   * The loop above consumer fewer than ENTITY_MAX_LENGTH-1
                //     alphanumeric characters, so the next character cannot be
                //     alphanumeric.
                //
                // (The longest entity will always end with a ';' since any bare
                // entity will always have a closed version with a trailing
                // semicolon, which by definition will be longer.)
                //
                // https://html.spec.whatwg.org/multipage/parsing.html#named-character-reference-state
                return None;
            }
            _ => {
                // missing-semicolon-after-character-reference: ignore; continue
                // https://html.spec.whatwg.org/multipage/parsing.html#parse-error-missing-semicolon-after-character-reference
            }
        }

        // `raw` was generated from `iter`, so `iter.as_slice().len()` will
        // always be less than `raw.len()`.
        debug_assert!(raw.len() >= iter.as_slice().len());
        #[allow(clippy::arithmetic_side_effects)]
        let candidate = &raw[..raw.len() - iter.as_slice().len()];

        if candidate.len() < ENTITY_MIN_LENGTH {
            // Couldn’t possibly match. Don’t expand.
            return None;
        }

        // If candidate does not exactly match an entity, then don't expand it.
        // The spec says that entities *in attributes* must be terminated with a
        // semicolon, EOF, or some character *other* than [a-zA-Z0-9=].
        //
        // See `unescape_in()` documentation for examples.
        //
        // https://html.spec.whatwg.org/multipage/parsing.html#named-character-reference-state
        ENTITIES.get(candidate).map(|&expansion| expansion.into())
    }
}

#[cfg(feature = "unescape")]
impl Matcher for (Phf, ContextGeneral) {
    fn match_entity<'a>(
        iter: &'a mut slice::Iter<u8>,
    ) -> Option<Cow<'a, [u8]>> {
        use crate::{BARE_ENTITY_MAX_LENGTH, ENTITIES, ENTITY_MIN_LENGTH};
        use std::cmp::min;

        assert_peek_eq(iter, Some(b'&'), "match_entity() expected '&'");

        if Some(b'#') == peek_n(iter, 1) {
            // Numeric entity.
            return match_numeric_entity(iter);
        }

        let raw = &iter.as_slice();

        // Create a backup of `iter` so we can revert if we need to look ahead
        // further than the length of the entity.
        let original_iter = iter.clone();

        find_longest_candidate(iter);
        let has_semicolon = peek(iter) == Some(b';');
        if has_semicolon {
            // Actually consume the semicolon.
            assert_next_eq(iter, Some(b';'), PEEK_MATCH_ERROR);
        } else {
            // missing-semicolon-after-character-reference: ignore; continue
            // https://html.spec.whatwg.org/multipage/parsing.html#parse-error-missing-semicolon-after-character-reference
        }

        // `raw` was generated from `iter`, so `iter.as_slice().len()` will
        // always be less than `raw.len()`.
        debug_assert!(raw.len() >= iter.as_slice().len());
        #[allow(clippy::arithmetic_side_effects)]
        let candidate = &raw[..raw.len() - iter.as_slice().len()];

        if candidate.len() < ENTITY_MIN_LENGTH {
            // Couldn’t possibly match. Don’t expand.
            return None;
        }

        if has_semicolon {
            #[allow(clippy::len_zero, reason = "clarity")]
            if let Some(&expansion) = ENTITIES.get(candidate) {
                // Found a match. It has to be longer than 1 byte.
                *iter = original_iter;
                debug_assert!(candidate.len() >= 1);
                #[allow(clippy::arithmetic_side_effects)]
                iter.nth(candidate.len() - 1); // Update iter. nth(0) == next().
                return Some(expansion.into());
            }
        }

        // No semicolon or it didn’t match.
        //
        // No entity without a semicolon is a prefix for another entity without
        // a semicolon. So, `&times` is a prefix for `&timesbar;`, but never for
        // another bare entity. See test `bare_entity_prefix_rule()`.
        //
        // The implication is that we can search for the shortest match first,
        // since if it matches there can be no other match.
        for check_len in
            ENTITY_MIN_LENGTH..=min(candidate.len(), BARE_ENTITY_MAX_LENGTH)
        {
            if let Some(&expansion) = ENTITIES.get(&candidate[..check_len]) {
                // Found a match. It has to be longer than 1 byte.
                *iter = original_iter;
                debug_assert!(check_len >= 1);
                #[allow(clippy::arithmetic_side_effects)]
                iter.nth(check_len - 1); // Update iter. nth(0) == next().
                return Some(expansion.into());
            }
        }

        // Did not find a match.
        None
    }
}

/// Match a numeric entity like `&#x20;` or `&#32;`.
#[allow(clippy::from_str_radix_10)]
fn match_numeric_entity(
    iter: &mut slice::Iter<u8>,
) -> Option<Cow<'static, [u8]>> {
    assert_next_eq(iter, Some(b'&'), "match_numeric_entity() expected '&'");
    assert_next_eq(iter, Some(b'#'), "match_numeric_entity() expected '#'");

    let number = match peek(iter) {
        c @ Some(b'x' | b'X') => {
            // Hexadecimal entity
            assert_next_eq(iter, c, PEEK_MATCH_ERROR);

            let hex = slice_while(iter, u8::is_ascii_hexdigit);
            u32::from_str_radix(core::str::from_utf8(hex).unwrap(), 16)
        }
        Some(_) => {
            // Presumably a decimal entity
            let dec = slice_while(iter, u8::is_ascii_digit);
            u32::from_str_radix(core::str::from_utf8(dec).unwrap(), 10)
        }
        None => {
            // Iterator reached end; do not expand.
            return None;
        }
    };

    if Some(b';') == peek(iter) {
        assert_next_eq(iter, Some(b';'), PEEK_MATCH_ERROR);
    } else {
        // missing-semicolon-after-character-reference: ignore and continue.
        // https://html.spec.whatwg.org/multipage/parsing.html#parse-error-missing-semicolon-after-character-reference
    }

    match number {
        Ok(number) => {
            return Some(correct_numeric_entity(number));
        }
        Err(error) => match error.kind() {
            IntErrorKind::PosOverflow => {
                // Too large a number
                return Some(REPLACEMENT_CHAR_BYTES.into());
            }
            IntErrorKind::Empty => {
                // No number, e.g. &#; or &#x;. Fall through.
            }
            // Pretty sure this is impossible.
            _ => panic!("error parsing number in numeric entity: {error:?}"),
        },
    }

    // Do not expand.
    None
}

/// Unicode replacement character (U+FFFD, “�”).
///
/// According to the WHATWG HTML spec, this is used as an expansion for certain
/// invalid numeric entities.
///
/// According to Unicode 12, this is “used to replace an incoming character
/// whose value is unknown or unrepresentable in Unicode.” The latest chart for
/// the Specials block is [available as a PDF](https://www.unicode.org/charts/PDF/UFFF0.pdf).
pub const REPLACEMENT_CHAR_BYTES: &[u8] = "\u{fffd}".as_bytes();

/// Calculate the expansion for a numeric entity (after parsing it).
///
/// See <https://html.spec.whatwg.org/multipage/parsing.html#numeric-character-reference-end-state>
#[allow(clippy::match_same_arms)]
fn correct_numeric_entity(number: u32) -> Cow<'static, [u8]> {
    match number {
        // null-character-reference parse error:
        0x00 => REPLACEMENT_CHAR_BYTES.into(),

        // character-reference-outside-unicode-range parse error:
        0x11_0000.. => REPLACEMENT_CHAR_BYTES.into(),

        // https://infra.spec.whatwg.org/#surrogate
        // surrogate-character-reference parse error:
        0xD800..=0xDFFF => REPLACEMENT_CHAR_BYTES.into(),

        // control-character-reference parse error exceptions:
        0x80 => "\u{20AC}".as_bytes().into(), // EURO SIGN (€)
        0x82 => "\u{201A}".as_bytes().into(), // SINGLE LOW-9 QUOTATION MARK (‚)
        0x83 => "\u{0192}".as_bytes().into(), // LATIN SMALL LETTER F WITH HOOK (ƒ)
        0x84 => "\u{201E}".as_bytes().into(), // DOUBLE LOW-9 QUOTATION MARK („)
        0x85 => "\u{2026}".as_bytes().into(), // HORIZONTAL ELLIPSIS (…)
        0x86 => "\u{2020}".as_bytes().into(), // DAGGER (†)
        0x87 => "\u{2021}".as_bytes().into(), // DOUBLE DAGGER (‡)
        0x88 => "\u{02C6}".as_bytes().into(), // MODIFIER LETTER CIRCUMFLEX ACCENT (ˆ)
        0x89 => "\u{2030}".as_bytes().into(), // PER MILLE SIGN (‰)
        0x8A => "\u{0160}".as_bytes().into(), // LATIN CAPITAL LETTER S WITH CARON (Š)
        0x8B => "\u{2039}".as_bytes().into(), // SINGLE LEFT-POINTING ANGLE QUOTATION MARK (‹)
        0x8C => "\u{0152}".as_bytes().into(), // LATIN CAPITAL LIGATURE OE (Œ)
        0x8E => "\u{017D}".as_bytes().into(), // LATIN CAPITAL LETTER Z WITH CARON (Ž)
        0x91 => "\u{2018}".as_bytes().into(), // LEFT SINGLE QUOTATION MARK (‘)
        0x92 => "\u{2019}".as_bytes().into(), // RIGHT SINGLE QUOTATION MARK (’)
        0x93 => "\u{201C}".as_bytes().into(), // LEFT DOUBLE QUOTATION MARK (“)
        0x94 => "\u{201D}".as_bytes().into(), // RIGHT DOUBLE QUOTATION MARK (”)
        0x95 => "\u{2022}".as_bytes().into(), // BULLET (•)
        0x96 => "\u{2013}".as_bytes().into(), // EN DASH (–)
        0x97 => "\u{2014}".as_bytes().into(), // EM DASH (—)
        0x98 => "\u{02DC}".as_bytes().into(), // SMALL TILDE (˜)
        0x99 => "\u{2122}".as_bytes().into(), // TRADE MARK SIGN (™)
        0x9A => "\u{0161}".as_bytes().into(), // LATIN SMALL LETTER S WITH CARON (š)
        0x9B => "\u{203A}".as_bytes().into(), // SINGLE RIGHT-POINTING ANGLE QUOTATION MARK (›)
        0x9C => "\u{0153}".as_bytes().into(), // LATIN SMALL LIGATURE OE (œ)
        0x9E => "\u{017E}".as_bytes().into(), // LATIN SMALL LETTER Z WITH CARON (ž)
        0x9F => "\u{0178}".as_bytes().into(), // LATIN CAPITAL LETTER Y WITH DIAERESIS (Ÿ)

        // A few parse errors and other cases are handled by the catch-all.
        //
        //   * noncharacter-character-reference parse error
        //   * control-character-reference parse error
        //   * 0x0d (carriage return)
        //   * ASCII whitespace
        //   * ASCII control characters
        //
        // I found the spec a little confusing here, but a close reading and
        // some browser testing convinced me that all of these cases are handled
        // but just emitting the represented code point.

        // Everything else.
        c => char::from_u32(c)
            .map(|c| c.to_string().into_bytes().into())
            // Should never fall back since we handle all the cases above.
            .unwrap_or_else(|| REPLACEMENT_CHAR_BYTES.into()),
    }
}

/// Advance `iter` to consume longest possible candidate (alphanumeric only).
#[cfg(feature = "unescape")]
fn find_longest_candidate(iter: &mut slice::Iter<u8>) {
    use crate::ENTITY_MAX_LENGTH;
    assert_next_eq(iter, Some(b'&'), PEEK_MATCH_ERROR);

    // Start at 1 since we got the '&'.
    for _ in 1..ENTITY_MAX_LENGTH {
        if let Some(c) = peek(iter) {
            if c.is_ascii_alphanumeric() {
                iter.next();
                continue;
            }
        }

        break;
    }
}

/// Advance iterator while `predicate` matches (`next()` will return the first
/// byte that doesn’t match) and return a slice of the bytes that didn’t match.
fn slice_while<'a, P>(
    iter: &mut slice::Iter<'a, u8>,
    mut predicate: P,
) -> &'a [u8]
where
    P: FnMut(&u8) -> bool,
{
    slice_until(iter, move |c| !predicate(c))
}

/// Advance iterator until the byte _before_ `predicate` matches and return a
/// slice of the bytes matched.
fn slice_until<'a, P>(iter: &mut slice::Iter<'a, u8>, predicate: P) -> &'a [u8]
where
    P: FnMut(&u8) -> bool,
{
    let remainder = iter.as_slice();
    position_peek(iter, predicate)
        .map(|i| &remainder[..i])
        .unwrap_or(remainder)
}

/// Move to the next value in `iter` and assert that it equals `expected`.
fn assert_next_eq(iter: &mut slice::Iter<u8>, expected: Option<u8>, msg: &str) {
    assert_eq!(iter.next().copied(), expected, "{msg}");
}

/// Peek the next value in `iter` and assert that it equals `expected`.
fn assert_peek_eq(iter: &slice::Iter<u8>, expected: Option<u8>, msg: &str) {
    assert_eq!(peek(iter), expected, "{msg}");
}

/// Peek at the next value in `iter` without changing the `iter`.
fn peek(iter: &slice::Iter<u8>) -> Option<u8> {
    peek_n(iter, 0)
}

/// Peek at a future value in `iter` without changing the `iter`.
fn peek_n(iter: &slice::Iter<u8>, n: usize) -> Option<u8> {
    iter.as_slice().get(n).copied()
}

/// Like `position()`, but stops _before_ the found value.
fn position_peek<P>(
    iter: &mut slice::Iter<u8>,
    mut predicate: P,
) -> Option<usize>
where
    P: FnMut(&u8) -> bool,
{
    try_fold_peek(iter, 0, move |i, x| {
        if predicate(x) {
            Err(i)
        } else {
            // `i` counts items in a slice, so it can never be `usize::MAX`. If
            // there were `usize::MAX` items, then the last item would be
            // `usize::MAX - 1`, and this would return `Ok(usize::MAX)`.
            debug_assert!(i < usize::MAX);
            #[allow(clippy::arithmetic_side_effects)]
            Ok(i + 1)
        }
    })
    .err()
}

/// Like `try_fold()`, but stops _before_ the found value.
///
/// The passed function should return `Ok(_)` to continue to next value, or
/// `Err(_)` to stop with the iterator pointing at the previous value — i.e. the
/// next call to `iter.next()` will return the value currently being processed.
///
/// This uses `&u8` instead of `u8` so that you can pass `u8::is_ascii_digit` as
/// a predicate to `position_peek()` and friends.
fn try_fold_peek<T, F>(
    iter: &mut slice::Iter<u8>,
    initial: T,
    mut function: F,
) -> Result<T, T>
where
    F: FnMut(T, &u8) -> Result<T, T>,
{
    let mut accumulator = initial;
    for c in iter.as_slice() {
        accumulator = function(accumulator, c)?;
        iter.next();
    }
    Ok(accumulator)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{assert, check};
    use pastey::paste;

    // Test fast and slow versions of a function.
    macro_rules! test {
        ($name:ident, unescape ($($input:tt)+) == $expected:expr) => {
            paste! {
                #[cfg(feature = "unescape_fast")]
                #[test]
                fn [<fast_ $name>]() {
                    assert!(unescape_in((Matchgen, ContextGeneral), $($input)+) == $expected);
                }

                #[cfg(feature = "unescape")]
                #[test]
                fn [<slow_ $name>]() {
                    assert!(unescape_in((Phf, ContextGeneral), $($input)+) == $expected);
                }
            }
        };
        ($name:ident, unescape_attribute ($($input:tt)+) == $expected:expr) => {
            paste! {
                #[cfg(feature = "unescape_fast")]
                #[test]
                fn [<fast_ $name>]() {
                    assert!(unescape_in((Matchgen, ContextAttribute), $($input)+) == $expected);
                }

                #[cfg(feature = "unescape")]
                #[test]
                fn [<slow_ $name>]() {
                    assert!(unescape_in((Phf, ContextAttribute), $($input)+) == $expected);
                }
            }
        };
    }

    // Test fast and slow versions of unescape and unescape_attribute.
    macro_rules! test_both {
        ($name:ident, unescape ($input:expr) == $expected:expr) => {
            paste! {
                test!($name, unescape($input) == $expected);
                test!([<attribute_ $name>], unescape_attribute($input) == $expected);
            }
        };
    }

    test_both!(almost_entity, unescape("&time") == "&time");
    test_both!(exact_times, unescape("&times;") == "×");
    test_both!(exact_timesb, unescape("&timesb;") == "⊠");
    test_both!(bare_times_end, unescape("&times") == "×");
    test_both!(bare_times_bang, unescape("&times!") == "×!");

    test!(bare_entity_char, unescape("&timesa") == "×a");
    test!(other_entity, unescape("&timesbar;") == "⨱"); // To confirm next test
    test!(bare_entity_almost_other, unescape("&timesbar") == "×bar");
    test!(
        bare_entity_long_suffix,
        unescape("&timesbarrrrrr") == "×barrrrrr"
    );
    test!(bare_entity_equal, unescape("&times=") == "×=");
    test!(bare_entity_char_semicolon, unescape("&timesa;") == "×a;");
    test!(bare_entity_equal_semicolon, unescape("&times=;") == "×=;");
    test_both!(bare_entity_entity, unescape("&times&lt;") == "×<");
    test!(bare_entity_char_is_prefix, unescape("&timesb") == "×b");
    test!(
        bare_entity_char_is_prefix_entity,
        unescape("&timesb&lt;") == "×b<"
    );
    test!(
        attribute_bare_entity_char,
        unescape_attribute("&timesa") == "&timesa"
    );
    test!(
        attribute_bare_entity_equal,
        unescape_attribute("&times=") == "&times="
    );
    test!(
        attribute_bare_entity_char_semicolon,
        unescape_attribute("&timesa;") == "&timesa;"
    );
    test!(
        attribute_bare_entity_equal_semicolon,
        unescape_attribute("&times=;") == "&times=;"
    );
    test!(
        attribute_bare_entity_char_is_prefix,
        unescape_attribute("&timesb") == "&timesb"
    );
    test!(
        attribute_bare_entity_char_is_prefix_entity,
        unescape_attribute("&timesb&lt;") == "&timesb<"
    );

    test_both!(empty, unescape("") == "");
    test_both!(no_entities, unescape("none") == "none");
    test_both!(only_ampersand, unescape("&") == "&");
    test_both!(empty_entity, unescape("&;") == "&;");
    test_both!(invalid_entity, unescape("&time;") == "&time;");
    test_both!(middle_invalid_entity, unescape(" &time; ") == " &time; ");
    test_both!(
        mixed_valid_invalid_entities,
        unescape("&time; &amp; &time; &amp; &time;")
            == "&time; & &time; & &time;"
    );
    test_both!(middle_entity, unescape(" &amp; ") == " & ");
    test_both!(extra_ampersands, unescape("&&amp;&") == "&&&");
    test_both!(two_entities, unescape("AND &amp;&AMP; and") == "AND && and");
    test_both!(
        long_valid_entity,
        unescape("&CounterClockwiseContourIntegral;") == "∳"
    );
    test_both!(
        long_invalid_entity,
        unescape("&CounterClockwiseContourIntegralX;")
            == "&CounterClockwiseContourIntegralX;"
    );
    test_both!(
        very_long_invalid_entity,
        unescape("&aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;")
            == "&aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;"
    );

    test_both!(correct_hex_lowerx_lower, unescape("&#x7a;") == "z");
    test_both!(correct_hex_lowerx_upper, unescape("&#x7A;") == "z");
    test_both!(correct_hex_upperx_lower, unescape("&#X7a;") == "z");
    test_both!(correct_hex_upperx_upper, unescape("&#X7A;") == "z");
    test_both!(correct_hex_leading_zero, unescape("&#x07a;") == "z");
    test_both!(correct_hex_leading_zero_zero, unescape("&#x007a;") == "z");
    test_both!(correct_dec, unescape("&#122;") == "z");
    test_both!(correct_dec_leading_zero, unescape("&#0122;") == "z");
    test_both!(correct_dec_leading_zero_zero, unescape("&#00122;") == "z");
    test_both!(correct_hex_unicode, unescape("&#x21D2;") == "⇒");

    test_both!(bare_hex_char, unescape("&#x7Az") == "zz");
    test_both!(bare_hex_entity, unescape("&#x7A&lt;") == "z<");
    test_both!(bare_hex_end, unescape("&#x7A") == "z");
    test_both!(bare_dec_char, unescape("&#122z") == "zz");
    test_both!(bare_dec_entity, unescape("&#122&lt;") == "z<");
    test_both!(bare_dec_end, unescape("&#122") == "z");
    test_both!(bare_empty_numeric_char, unescape("&#z") == "&#z");
    test_both!(bare_empty_numeric_entity, unescape("&#&lt;") == "&#<");
    test_both!(bare_empty_numeric_end, unescape("&#") == "&#");

    test_both!(hex_instead_of_dec, unescape("&#a0;") == "&#a0;");
    test_both!(invalid_hex_lowerx, unescape("&#xZ;") == "&#xZ;");
    test_both!(invalid_hex_upperx, unescape("&#XZ;") == "&#XZ;");

    test_both!(hex_control_1, unescape("&#x1;") == "\u{1}");
    test_both!(dec_control_1, unescape("&#1;") == "\u{1}");
    test_both!(dec_cr, unescape("&#13;") == "\r");
    test_both!(hex_cr, unescape("&#xd;") == "\r");
    test_both!(hex_tab, unescape("&#9;") == "\t");
    test_both!(dec_tab, unescape("&#9;") == "\t");

    test_both!(hex_max_code_point, unescape("&#x10ffff;") == "\u{10ffff}");
    test_both!(
        hex_above_max_code_point,
        unescape("&#x110001;") == "\u{fffd}"
    );
    test_both!(hex_11_chars, unescape("&#x1100000000;") == "\u{fffd}");
    test_both!(
        bare_hex_11_chars_end,
        unescape("&#x1100000000") == "\u{fffd}"
    );

    test_both!(
        hex_40_chars,
        unescape("&#x110000000000000000000000000000000000000;") == "\u{fffd}"
    );
    test_both!(
        bare_hex_40_chars_end,
        unescape("&#x110000000000000000000000000000000000000") == "\u{fffd}"
    );

    test_both!(special_entity_null, unescape("&#0;") == "\u{fffd}");
    test_both!(special_entity_bullet, unescape("&#x95;") == "•");
    test_both!(
        special_entity_bullets,
        unescape("&#x95;&#149;&#x2022;•") == "••••"
    );
    test_both!(special_entity_space, unescape("&#x20") == " ");

    const ALL_SOURCE: &str =
        include_str!("../../tests/corpus/all-entities-source.txt");
    const ALL_EXPANDED: &str =
        include_str!("../../tests/corpus/all-entities-expanded.txt");
    test_both!(all_entities, unescape(ALL_SOURCE) == ALL_EXPANDED);

    #[cfg(feature = "unescape_fast")]
    #[test]
    fn fast_invalid_utf8() {
        assert!(
            unescape_bytes_in((Matchgen, ContextGeneral), &b"\xa1"[..])
                == &b"\xa1"[..]
        );
    }

    #[cfg(feature = "unescape")]
    #[test]
    fn slow_invalid_utf8() {
        assert!(
            unescape_bytes_in((Phf, ContextGeneral), &b"\xa1"[..])
                == &b"\xa1"[..]
        );
    }
    #[cfg(feature = "unescape_fast")]
    #[test]
    fn fast_attribute_invalid_utf8() {
        assert!(
            unescape_bytes_in((Matchgen, ContextAttribute), &b"\xa1"[..])
                == &b"\xa1"[..]
        );
    }

    #[cfg(feature = "unescape")]
    #[test]
    fn slow_attribute_invalid_utf8() {
        assert!(
            unescape_bytes_in((Phf, ContextAttribute), &b"\xa1"[..])
                == &b"\xa1"[..]
        );
    }

    #[test]
    fn correct_numeric_entity_euro() {
        match correct_numeric_entity(0x80) {
            Cow::Borrowed(s) => assert!(s == "\u{20AC}".as_bytes()),
            Cow::Owned(_) => panic!("expected borrowed"),
        }
    }

    #[test]
    fn correct_numeric_entity_null() {
        match correct_numeric_entity(0) {
            Cow::Borrowed(s) => assert!(s == "\u{fffd}".as_bytes()),
            Cow::Owned(_) => panic!("expected borrowed"),
        }
    }

    #[test]
    fn correct_numeric_entity_z() {
        match correct_numeric_entity(b'z'.into()) {
            Cow::Borrowed(_) => panic!("expected owned"),
            Cow::Owned(ref s) => assert!(s == b"z"),
        }
    }

    /// No bare entity may be a prefix for another bare entity. For example,
    /// `&times` is a prefix for `&timesbar;` and a few other entities, but
    /// never for another bare entity.
    ///
    /// Logic in `match_entity::<(Phf, ContextGeneral)>()` depends on this.
    #[test]
    fn bare_entity_prefix_rule() {
        let all_bare: Vec<_> = ALL_SOURCE
            .split_ascii_whitespace()
            .filter(|entity| entity.ends_with(';'))
            .collect();
        for bare in &all_bare {
            check!(
                all_bare
                    .iter()
                    .find(|other| other.starts_with(bare) && *other != bare)
                    == None,
                "No bare entity may be a prefix for another bare entity"
            );
        }
    }
}
