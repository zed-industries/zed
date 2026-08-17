//! Rules defined in [RFC 2822].
//!
//! [RFC 2822]: https://datatracker.ietf.org/doc/html/rfc2822

use num_conv::prelude::*;

use crate::parsing::ParsedItem;
use crate::parsing::combinator::rfc::rfc2234::wsp;
use crate::parsing::combinator::{ascii_char, one_or_more, zero_or_more};

const DEPTH_LIMIT: u8 = 32;

/// Consume the `fws` rule.
// The full rule is equivalent to /\r\n[ \t]+|[ \t]+(?:\r\n[ \t]+)*/
#[inline]
pub(crate) fn fws(mut input: &[u8]) -> Option<ParsedItem<'_, ()>> {
    if let [b'\r', b'\n', rest @ ..] = input {
        one_or_more(wsp)(rest)
    } else {
        input = one_or_more(wsp)(input)?.into_inner();
        while let [b'\r', b'\n', rest @ ..] = input {
            input = one_or_more(wsp)(rest)?.into_inner();
        }
        Some(ParsedItem(input, ()))
    }
}

/// Consume the `cfws` rule.
// The full rule is equivalent to any combination of `fws` and `comment` so long as it is not empty.
#[inline]
pub(crate) fn cfws(input: &[u8]) -> Option<ParsedItem<'_, ()>> {
    one_or_more(|input| fws(input).or_else(|| comment(input, 1)))(input)
}

/// Consume the `comment` rule.
#[inline]
fn comment(mut input: &[u8], depth: u8) -> Option<ParsedItem<'_, ()>> {
    // Avoid stack exhaustion DoS by limiting recursion depth. This will cause highly-nested
    // comments to fail parsing, but comments *at all* are incredibly rare in practice.
    //
    // The error from this will not be descriptive, but the rarity and near-certain maliciousness of
    // such inputs makes this an acceptable trade-off.
    if depth == DEPTH_LIMIT {
        return None;
    }

    input = ascii_char::<b'('>(input)?.into_inner();
    input = zero_or_more(fws)(input).into_inner();
    while let Some(rest) = ccontent(input, depth + 1) {
        input = rest.into_inner();
        input = zero_or_more(fws)(input).into_inner();
    }
    input = ascii_char::<b')'>(input)?.into_inner();

    Some(ParsedItem(input, ()))
}

/// Consume the `ccontent` rule.
#[inline]
fn ccontent(input: &[u8], depth: u8) -> Option<ParsedItem<'_, ()>> {
    ctext(input)
        .or_else(|| quoted_pair(input))
        .or_else(|| comment(input, depth))
}

/// Consume the `ctext` rule.
#[expect(
    clippy::unnecessary_lazy_evaluations,
    reason = "rust-lang/rust-clippy#8522"
)]
#[inline]
fn ctext(input: &[u8]) -> Option<ParsedItem<'_, ()>> {
    no_ws_ctl(input).or_else(|| match input {
        [33..=39 | 42..=91 | 93..=126, rest @ ..] => Some(ParsedItem(rest, ())),
        _ => None,
    })
}

/// Consume the `quoted_pair` rule.
#[inline]
fn quoted_pair(mut input: &[u8]) -> Option<ParsedItem<'_, ()>> {
    input = ascii_char::<b'\\'>(input)?.into_inner();
    input = text(input).into_inner();

    // If nothing is parsed by `text`, this means by hit the `obs-text` rule and nothing matched.
    // This is technically a success, and we used to check the `obs-qp` rule to ensure everything
    // possible was consumed. After further analysis, it was determined that this check was
    // unnecessary due to `obs-text` wholly subsuming `obs-qp` in this context. For this reason, if
    // `text` fails to parse anything, we consider it a success without further consideration.

    Some(ParsedItem(input, ()))
}

/// Consume the `no_ws_ctl` rule.
#[inline]
const fn no_ws_ctl(input: &[u8]) -> Option<ParsedItem<'_, ()>> {
    match input {
        [1..=8 | 11..=12 | 14..=31 | 127, rest @ ..] => Some(ParsedItem(rest, ())),
        _ => None,
    }
}

/// Consume the `text` rule.
#[inline]
fn text<'a>(input: &'a [u8]) -> ParsedItem<'a, ()> {
    let new_text = |input: &'a [u8]| match input {
        [1..=9 | 11..=12 | 14..=127, rest @ ..] => Some(ParsedItem(rest, ())),
        _ => None,
    };

    let obs_char = |input: &'a [u8]| match input {
        // This is technically allowed, but consuming this would mean the rest of the string is
        // eagerly consumed without consideration for where the comment actually ends.
        [b')', ..] => None,
        [0..=9 | 11..=12 | 14..=127, rest @ ..] => Some(rest),
        _ => None,
    };

    let obs_text = |mut input| {
        input = zero_or_more(ascii_char::<b'\n'>)(input).into_inner();
        input = zero_or_more(ascii_char::<b'\r'>)(input).into_inner();
        while let Some(rest) = obs_char(input) {
            input = rest;
            input = zero_or_more(ascii_char::<b'\n'>)(input).into_inner();
            input = zero_or_more(ascii_char::<b'\r'>)(input).into_inner();
        }

        ParsedItem(input, ())
    };

    new_text(input).unwrap_or_else(|| obs_text(input))
}

/// Consume an old zone literal, returning the offset in hours.
#[inline]
pub(crate) fn zone_literal(input: &[u8]) -> Option<ParsedItem<'_, i8>> {
    let [first, second, third, rest @ ..] = input else {
        const UT_VARIANTS: [u16; 4] = [
            u16::from_ne_bytes([b'u', b't']),
            u16::from_ne_bytes([b'u', b'T']),
            u16::from_ne_bytes([b'U', b't']),
            u16::from_ne_bytes([b'U', b'T']),
        ];

        let [first, rest @ ..] = input else {
            return None;
        };
        if let [second, rest @ ..] = rest
            && UT_VARIANTS.contains(&u16::from_ne_bytes([*first, *second]))
        {
            return Some(ParsedItem(rest, 0));
        }
        return (*first != b'j' && *first != b'J' && first.is_ascii_alphabetic())
            .then_some(ParsedItem(rest, 0));
    };
    let byte = u32::from_ne_bytes([
        0,
        first.to_ascii_lowercase(),
        second.to_ascii_lowercase(),
        third.to_ascii_lowercase(),
    ]);
    const ZONES: [u32; 8] = [
        u32::from_ne_bytes([0, b'e', b's', b't']),
        u32::from_ne_bytes([0, b'e', b'd', b't']),
        u32::from_ne_bytes([0, b'c', b's', b't']),
        u32::from_ne_bytes([0, b'c', b'd', b't']),
        u32::from_ne_bytes([0, b'm', b's', b't']),
        u32::from_ne_bytes([0, b'm', b'd', b't']),
        u32::from_ne_bytes([0, b'p', b's', b't']),
        u32::from_ne_bytes([0, b'p', b'd', b't']),
    ];

    let eq = [
        if ZONES[0] == byte { i32::MAX } else { 0 },
        if ZONES[1] == byte { i32::MAX } else { 0 },
        if ZONES[2] == byte { i32::MAX } else { 0 },
        if ZONES[3] == byte { i32::MAX } else { 0 },
        if ZONES[4] == byte { i32::MAX } else { 0 },
        if ZONES[5] == byte { i32::MAX } else { 0 },
        if ZONES[6] == byte { i32::MAX } else { 0 },
        if ZONES[7] == byte { i32::MAX } else { 0 },
    ];
    if eq == [0; 8] && byte != const { u32::from_ne_bytes([0, b'g', b'm', b't']) } {
        return None;
    }

    let nonzero_zones = [
        eq[0] & -5,
        eq[1] & -4,
        eq[2] & -6,
        eq[3] & -5,
        eq[4] & -7,
        eq[5] & -6,
        eq[6] & -8,
        eq[7] & -7,
    ];
    let zone = nonzero_zones.iter().sum::<i32>().truncate();
    Some(ParsedItem(rest, zone))
}
