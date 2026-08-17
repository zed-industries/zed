//! Experimental range support

use std::ops::{self, RangeFull};

use num_traits::Num;

use crate::parse;

/// **Experimental:** Parse ranges in many styles
pub fn parse_range<T: Num + Copy>(input: &str) -> Result<Range<T>, RangeParsingError<T>> {
    let input = input.trim();

    // the order is important, otherwise .. would match ... with the third dot split
    for (separator, inclusive) in [
        ("...", false),
        ("...=", true),
        ("..", false),
        ("..=", true),
        (":", false),
    ] {
        if input.contains(separator) {
            return parse_range_with_separator(input, separator, inclusive);
        }
    }

    Err(RangeParsingError::MissingRangeSeparator)
}

fn parse_range_with_separator<T: Num + Copy>(
    input: &str,
    delimiter: &str,
    inclusive: bool,
) -> Result<Range<T>, RangeParsingError<T>> {
    if let Some((front, back)) = input.split_once(delimiter) {
        let (front, back) = (front.trim(), back.trim());
        println!("{front} .. {back}");

        let front_empty = front.is_empty();
        let back_empty = back.is_empty();
        if front_empty && back_empty {
            return Ok(Range::RangeFull);
        }

        match (parse(front), parse(back)) {
            (Ok(front), Ok(back)) => Ok(if inclusive {
                (front..=back).into()
            } else {
                (front..back).into()
            }),
            (Ok(start), Err(_)) if back_empty => Ok(Range::RangeFrom { start }),
            (Err(_), Ok(end)) if front_empty => Ok(Range::RangeTo { end }),
            (Ok(_), Err(e)) => Err(RangeParsingError::InvalidEnd(e)),
            (Err(e), Ok(_)) => Err(RangeParsingError::InvalidStart(e)),

            (Err(e1), Err(e2)) => Err(RangeParsingError::InvalidStartEnd(e1, e2)),
        }
    } else {
        // unreachable
        Err(RangeParsingError::MissingRangeSeparator)
    }
}

/// **Experimental:** Range that includes all variants similar to [std::range]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Range<T: Num> {
    /// start..stop
    RangeExclusive {
        start: T,
        end: T,
    },
    /// start..=stop
    RangeInclusive {
        start: T,
        end: T,
    },
    RangeFrom {
        start: T,
    },
    RangeTo {
        end: T,
    },
    RangeFull,
}

impl<T: Num + Copy> From<ops::Range<T>> for Range<T> {
    fn from(value: ops::Range<T>) -> Self {
        Range::RangeInclusive {
            start: value.start,
            end: value.end,
        }
    }
}
impl<T: Num + Copy> From<ops::RangeInclusive<T>> for Range<T> {
    fn from(value: ops::RangeInclusive<T>) -> Self {
        Range::RangeInclusive {
            start: *value.start(),
            end: *value.end(),
        }
    }
}
impl<T: Num + Copy> From<ops::RangeFrom<T>> for Range<T> {
    fn from(value: ops::RangeFrom<T>) -> Self {
        Range::RangeFrom { start: value.start }
    }
}
impl<T: Num + Copy> From<ops::RangeTo<T>> for Range<T> {
    fn from(value: ops::RangeTo<T>) -> Self {
        Range::RangeTo { end: value.end }
    }
}
impl<T: Num> From<RangeFull> for Range<T> {
    fn from(_: RangeFull) -> Self {
        Range::RangeFull
    }
}

impl<T: Num + Clone> Range<T> {
    pub fn as_range_exclusive(&self) -> Option<ops::Range<T>> {
        match self {
            Range::RangeExclusive { start, end } => Some(start.clone()..end.clone()),
            _ => None,
        }
    }
    pub fn as_range_inclusive(&self) -> Option<ops::RangeInclusive<T>> {
        match self {
            Range::RangeInclusive { start, end } => Some(start.clone()..=end.clone()),
            _ => None,
        }
    }
    pub fn as_range_from(&self) -> Option<ops::RangeFrom<T>> {
        match self {
            Range::RangeFrom { start } => Some(start.clone()..),
            _ => None,
        }
    }
    pub fn as_range_to(&self) -> Option<ops::RangeTo<T>> {
        match self {
            Range::RangeTo { end } => Some(..end.clone()),
            _ => None,
        }
    }
    pub fn as_full_range(&self) -> Option<ops::RangeFull> {
        match self {
            Range::RangeFull => Some(ops::RangeFull),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RangeParsingError<T: Num> {
    MissingRangeSeparator,
    InvalidStart(T::FromStrRadixErr),
    InvalidEnd(T::FromStrRadixErr),
    InvalidStartEnd(T::FromStrRadixErr, T::FromStrRadixErr),
}

#[cfg(test)]
mod test {
    use std::ops::RangeFull;

    use super::*;

    #[test]
    fn simple_cases() {
        assert_eq!(
            Err(RangeParsingError::MissingRangeSeparator),
            parse_range::<u8>("invalid")
        );

        assert_eq!(Ok((32u8..42).into()), parse_range::<u8>("32..42"));
        assert_eq!(Ok((32u8..42).into()), parse_range::<u8>("32:42"));
        assert_eq!(Ok((32u8..42).into()), parse_range::<u8>("32...42"));

        assert_eq!(
            Err(RangeParsingError::InvalidStart(
                parse::<u8>("_").unwrap_err()
            )),
            parse_range::<u8>("invalid..42")
        );
        assert_eq!(
            Err(RangeParsingError::InvalidEnd(parse::<u8>("_").unwrap_err())),
            parse_range::<u8>("42...invalid")
        );
        assert_eq!(
            Err(RangeParsingError::InvalidStartEnd(
                parse::<u8>("_").unwrap_err(),
                parse::<u8>("_").unwrap_err(),
            )),
            parse_range::<u8>("invalid:invalid")
        );
    }

    #[test]
    fn full_range() {
        assert_eq!(Ok(RangeFull.into()), parse_range::<u8>(".."));
        assert_eq!(Ok(RangeFull.into()), parse_range::<i8>(".."));

        let f32_fr = parse_range::<f32>("..").expect("weird float error type");
        assert_eq!(Some(RangeFull), f32_fr.as_full_range());
        assert_eq!(None, f32_fr.as_range_exclusive());
        assert_eq!(None, f32_fr.as_range_inclusive());
        assert_eq!(None, f32_fr.as_range_to());
        assert_eq!(None, f32_fr.as_range_from());
    }

    #[test]
    fn full_to() {
        assert_eq!(Ok((..42).into()), parse_range::<u8>("..42"));
        assert_eq!(Ok((..-42).into()), parse_range::<i8>("..-42"));

        let f32_r = parse_range::<f32>("..-42.23").expect("weird float error type");
        assert_eq!(None, f32_r.as_full_range());
        assert_eq!(None, f32_r.as_range_exclusive());
        assert_eq!(None, f32_r.as_range_inclusive());
        assert_eq!(Some((..-42.23).into()), f32_r.as_range_to());
        assert_eq!(None, f32_r.as_range_from());
    }

    #[test]
    fn full_from() {
        assert_eq!(Ok((42..).into()), parse_range::<u8>("42.."));
        assert_eq!(Ok((-42..).into()), parse_range::<i8>("-42.."));

        let f32_r = parse_range::<f32>("-42.23..").expect("weird float error type");
        assert_eq!(None, f32_r.as_full_range());
        assert_eq!(None, f32_r.as_range_exclusive());
        assert_eq!(None, f32_r.as_range_inclusive());
        assert_eq!(None, f32_r.as_range_to());
        assert_eq!(Some((-42.23..).into()), f32_r.as_range_from());
    }
}
