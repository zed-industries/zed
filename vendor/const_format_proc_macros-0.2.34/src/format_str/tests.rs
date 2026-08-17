use super::*;

use super::{ParseError as PE, ParseErrorKind as PEK};

use crate::formatting::{FormattingFlags as FF, IsAlternate, NumberFormatting};

use fastrand::Rng;

use std::ops::RangeInclusive;

fn err(s: &'static str) -> ParseError {
    FormatStr::parse(s, StrRawness::dummy()).unwrap_err()
}

fn ok(s: &'static str) -> FormatStr {
    FormatStr::parse(s, StrRawness::dummy()).unwrap()
}

const NOALT: IsAlternate = IsAlternate::No;
const NFDEC: NumberFormatting = NumberFormatting::Decimal;

#[test]
fn unclosed_arg() {
    assert_eq!(
        err("_{"),
        PE {
            pos: 1,
            kind: PEK::UnclosedArg
        }
    );
    assert_eq!(
        err("___{"),
        PE {
            pos: 3,
            kind: PEK::UnclosedArg
        }
    );
    assert_eq!(
        err("___{__"),
        PE {
            pos: 3,
            kind: PEK::UnclosedArg
        }
    );
}

#[test]
fn invalid_closed_arg() {
    assert_eq!(
        err("_}"),
        PE {
            pos: 1,
            kind: PEK::InvalidClosedArg
        }
    );
    assert_eq!(
        err("___}"),
        PE {
            pos: 3,
            kind: PEK::InvalidClosedArg
        }
    );
    assert_eq!(
        err("___}__"),
        PE {
            pos: 3,
            kind: PEK::InvalidClosedArg
        }
    );
}

#[test]
fn not_a_number() {
    assert_eq!(
        err("  {0a} "),
        PE {
            pos: 3,
            kind: PEK::not_a_number("0a")
        }
    );
    assert_eq!(
        err("   {4B:?} "),
        PE {
            pos: 4,
            kind: PEK::not_a_number("4B")
        }
    );
    assert_eq!(
        err("    {6_:} "),
        PE {
            pos: 5,
            kind: PEK::not_a_number("6_")
        }
    );
}

#[test]
fn not_an_ident() {
    assert_eq!(
        err("  {?} "),
        PE {
            pos: 3,
            kind: PEK::not_an_ident("?")
        }
    );
    assert_eq!(
        err("  {_} "),
        PE {
            pos: 3,
            kind: PEK::not_an_ident("_")
        }
    );
    assert_eq!(
        err("   {a?} "),
        PE {
            pos: 4,
            kind: PEK::not_an_ident("a?")
        }
    );
    assert_eq!(
        err("    {_?:} "),
        PE {
            pos: 5,
            kind: PEK::not_an_ident("_?")
        }
    );
}

#[test]
fn unknown_formatting() {
    assert_eq!(
        err("   {:!} "),
        PE {
            pos: 5,
            kind: PEK::unknown_formatting("!")
        }
    );
    assert_eq!(
        err("    {:????} "),
        PE {
            pos: 6,
            kind: PEK::unknown_formatting("????")
        }
    );
}

#[test]
fn ok_cases() {
    assert_eq!(
        ok("{{{100}}}{200:}{300:?}").list,
        vec![
            FmtStrComponent::str("{"),
            FmtStrComponent::arg(WhichArg::Positional(Some(100)), FF::display(NOALT)),
            FmtStrComponent::str("}"),
            FmtStrComponent::arg(WhichArg::Positional(Some(200)), FF::display(NOALT)),
            FmtStrComponent::arg(WhichArg::Positional(Some(300)), FF::debug(NFDEC, NOALT)),
        ]
    );

    assert_eq!(
        ok("{{{}}}{:}{:?}").list,
        vec![
            FmtStrComponent::str("{"),
            FmtStrComponent::arg(WhichArg::Positional(None), FF::display(NOALT)),
            FmtStrComponent::str("}"),
            FmtStrComponent::arg(WhichArg::Positional(None), FF::display(NOALT)),
            FmtStrComponent::arg(WhichArg::Positional(None), FF::debug(NFDEC, NOALT)),
        ]
    );

    assert_eq!(
        ok("{{{AA}}}{BB:}{CC:?}").list,
        vec![
            FmtStrComponent::str("{"),
            FmtStrComponent::arg(WhichArg::ident("AA"), FF::display(NOALT)),
            FmtStrComponent::str("}"),
            FmtStrComponent::arg(WhichArg::ident("BB"), FF::display(NOALT)),
            FmtStrComponent::arg(WhichArg::ident("CC"), FF::debug(NFDEC, NOALT)),
        ]
    );

    assert_eq!(
        ok("AA {BB} CC {__:}{_aA0ñÑóö:}{FF:?} EE").list,
        vec![
            FmtStrComponent::str("AA "),
            FmtStrComponent::arg(WhichArg::ident("BB"), FF::display(NOALT)),
            FmtStrComponent::str(" CC "),
            FmtStrComponent::arg(WhichArg::ident("__"), FF::display(NOALT)),
            FmtStrComponent::arg(WhichArg::ident("_aA0ñÑóö"), FF::display(NOALT)),
            FmtStrComponent::arg(WhichArg::ident("FF"), FF::debug(NFDEC, NOALT)),
            FmtStrComponent::str(" EE"),
        ]
    );
}

////////////////////////////////////////////////////////////////////////////////

trait RngExt {
    /// # Panic
    ///
    /// Panics if the input slice is empty
    fn pick<'a, T>(&self, slice: &'a [T]) -> &'a T;

    /// # Panic
    ///
    /// Panics if there are no `chars` in the `bounds`
    fn char_(&self, bounds: RangeInclusive<char>) -> char;
}

impl RngExt for Rng {
    fn pick<'a, T>(&self, slice: &'a [T]) -> &'a T {
        &slice[self.usize(0..slice.len())]
    }

    fn char_(&self, bounds: RangeInclusive<char>) -> char {
        if let None = bounds.clone().next() {
            panic!("There are no chars in the {:?} bounds", bounds);
        }

        let u32_bounds = u32::from(*bounds.start())..=u32::from(*bounds.end());

        loop {
            if let Some(x) = std::char::from_u32(self.u32(u32_bounds.clone())) {
                break x;
            }
        }
    }
}

fn generate_input() -> String {
    let rng = Rng::new();
    let len = rng.usize(0..40);
    let mut string = String::with_capacity(len * 2);

    for _ in 0..len {
        match rng.u8(0..100) {
            0..=10 => string.push(*rng.pick(&['{', '}'])),
            _ => {
                let range = rng
                    .pick(&[
                        'A'..='Z',
                        'a'..='z',
                        '0'..='9',
                        '\u{0000}'..='\u{007F}',
                        '\u{007F}'..='\u{07FF}',
                        '\u{07FF}'..='\u{FFFF}',
                        '\u{10000}'..='\u{10FFFF}',
                    ])
                    .clone();

                for _ in 0..rng.u32(0..4) {
                    string.push(rng.char_(range.clone()));
                }
            }
        };
    }

    string
}

#[test]
fn never_panics() {
    let iters = 200;
    loop {
        let mut ok_count = 0u64;
        for _ in 0..iters {
            let input = generate_input();
            // println!("input: {}\n\n", input);
            if input.parse::<FormatStr>().is_ok() {
                ok_count += 1;
            }
        }
        if ok_count * 100 / iters > 30 {
            break;
        } else {
            println!("retrying never_panics");
        }
    }
}
