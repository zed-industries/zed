//! Various modifiers for components.

use core::num::NonZero;

/// Generate the provided code if and only if `pub` is present.
macro_rules! if_pub {
    (pub $(#[$attr:meta])*; $($x:tt)*) => {
        $(#[$attr])*
        ///
        /// This function exists since [`Default::default()`] cannot be used in a `const` context.
        /// It may be removed once that becomes possible. As the [`Default`] trait is in the
        /// prelude, removing this function in the future will not cause any resolution failures for
        /// the overwhelming majority of users; only users who use `#![no_implicit_prelude]` will be
        /// affected. As such it will not be considered a breaking change.
        $($x)*
    };
    ($($_:tt)*) => {};
}

/// Implement `Default` for the given type. This also generates an inherent implementation of a
/// `default` method that is `const fn`, permitting the default value to be used in const contexts.
// Every modifier should use this macro rather than a derived `Default`.
macro_rules! impl_const_default {
    ($($(#[$doc:meta])* $(@$pub:ident)? $type:ty => $default:expr;)*) => {$(
        impl $type {
            if_pub! {
                $($pub)?
                $(#[$doc])*;
                #[inline]
                pub const fn default() -> Self {
                    $default
                }
            }
        }

        $(#[$doc])*
        impl Default for $type {
            #[inline]
            fn default() -> Self {
                $default
            }
        }
    )*};
}

// Keep this first so that it's shown at the top of documentation.
impl_const_default! {
    /// Creates a modifier that indicates the value is [padded with zeroes](Padding::Zero).
    @pub Day => Self { padding: Padding::Zero };
    /// Creates a modifier that indicates the value uses the
    /// [`Numerical`](Self::Numerical) representation.
    MonthRepr => Self::Numerical;
    /// Creates an instance of this type that indicates the value uses the
    /// [`Numerical`](MonthRepr::Numerical) representation, is [padded with zeroes](Padding::Zero),
    /// and is case-sensitive when parsing.
    @pub Month => Self {
        padding: Padding::Zero,
        repr: MonthRepr::Numerical,
        case_sensitive: true,
    };
    /// Creates a modifier that indicates the value is [padded with zeroes](Padding::Zero).
    @pub Ordinal => Self { padding: Padding::Zero };
    /// Creates a modifier that indicates the value uses the [`Long`](Self::Long) representation.
    WeekdayRepr => Self::Long;
    /// Creates a modifier that indicates the value uses the [`Long`](WeekdayRepr::Long)
    /// representation and is case-sensitive when parsing. If the representation is changed to a
    /// numerical one, the instance defaults to one-based indexing.
    @pub Weekday => Self {
        repr: WeekdayRepr::Long,
        one_indexed: true,
        case_sensitive: true,
    };
    /// Creates a modifier that indicates that the value uses the [`Iso`](Self::Iso) representation.
    WeekNumberRepr => Self::Iso;
    /// Creates a modifier that indicates that the value is [padded with zeroes](Padding::Zero)
            /// and uses the [`Iso`](WeekNumberRepr::Iso) representation.
    @pub WeekNumber => Self {
        padding: Padding::Zero,
        repr: WeekNumberRepr::Iso,
    };
    /// Creates a modifier that indicates the value uses the [`Full`](Self::Full) representation.
    YearRepr => Self::Full;
    /// Creates a modifier that indicates the value uses the [`Extended`](Self::Extended) range.
    YearRange => Self::Extended;
    /// Creates a modifier that indicates the value uses the [`Full`](YearRepr::Full)
    /// representation, is [padded with zeroes](Padding::Zero), uses the Gregorian calendar as its
    /// base, and only includes the year's sign if necessary.
    @pub Year => Self {
        padding: Padding::Zero,
        repr: YearRepr::Full,
        range: YearRange::Extended,
        iso_week_based: false,
        sign_is_mandatory: false,
    };
    /// Creates a modifier that indicates the value is [padded with zeroes](Padding::Zero) and
    /// has the 24-hour representation.
    @pub Hour => Self {
        padding: Padding::Zero,
        is_12_hour_clock: false,
    };
    /// Creates a modifier that indicates the value is [padded with zeroes](Padding::Zero).
    @pub Minute => Self { padding: Padding::Zero };
    /// Creates a modifier that indicates the value uses the upper-case representation and is
    /// case-sensitive when parsing.
    @pub Period => Self {
        is_uppercase: true,
        case_sensitive: true,
    };
    /// Creates a modifier that indicates the value is [padded with zeroes](Padding::Zero).
    @pub Second => Self { padding: Padding::Zero };
    /// Creates a modifier that indicates the stringified value contains [one or more
    /// digits](Self::OneOrMore).
    SubsecondDigits => Self::OneOrMore;
    /// Creates a modifier that indicates the stringified value contains [one or more
    /// digits](SubsecondDigits::OneOrMore).
    @pub Subsecond => Self { digits: SubsecondDigits::OneOrMore };
    /// Creates a modifier that indicates the value only uses a sign for negative values and is
    /// [padded with zeroes](Padding::Zero).
    @pub OffsetHour => Self {
        sign_is_mandatory: false,
        padding: Padding::Zero,
    };
    /// Creates a modifier that indicates the value is [padded with zeroes](Padding::Zero).
    @pub OffsetMinute => Self { padding: Padding::Zero };
    /// Creates a modifier that indicates the value is [padded with zeroes](Padding::Zero).
    @pub OffsetSecond => Self { padding: Padding::Zero };
    /// Creates a modifier that indicates the value is [padded with zeroes](Self::Zero).
    Padding => Self::Zero;
    /// Creates a modifier that indicates the value represents the [number of seconds](Self::Second)
    /// since the Unix epoch.
    UnixTimestampPrecision => Self::Second;
    /// Creates a modifier that indicates the value represents the [number of
    /// seconds](UnixTimestampPrecision::Second) since the Unix epoch. The sign is not mandatory.
    @pub UnixTimestamp => Self {
        precision: UnixTimestampPrecision::Second,
        sign_is_mandatory: false,
    };
    /// Indicate that any trailing characters after the end of input are prohibited and will cause
    /// an error when used with `parse`.
    TrailingInput => Self::Prohibit;
    /// Creates a modifier used to represent the end of input, not allowing any trailing input (i.e.
    /// the input must be fully consumed).
    @pub End => Self { trailing_input: TrailingInput::Prohibit };
}

/// Day of the month.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Day {
    /// The padding to obtain the minimum width.
    pub padding: Padding,
}

impl Day {
    /// Set the padding type.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_padding(self, padding: Padding) -> Self {
        Self { padding }
    }
}

/// The representation of a month.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonthRepr {
    /// The number of the month (January is 1, December is 12).
    Numerical,
    /// The long form of the month name (e.g. "January").
    Long,
    /// The short form of the month name (e.g. "Jan").
    Short,
}

/// Month of the year.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Month {
    /// The padding to obtain the minimum width.
    pub padding: Padding,
    /// What form of representation should be used?
    pub repr: MonthRepr,
    /// Is the value case sensitive when parsing?
    pub case_sensitive: bool,
}

impl Month {
    /// Set the padding type.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_padding(self, padding: Padding) -> Self {
        Self { padding, ..self }
    }

    /// Set the manner in which the month is represented.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_repr(self, repr: MonthRepr) -> Self {
        Self { repr, ..self }
    }

    /// Set whether the value is case sensitive when parsing.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_case_sensitive(self, case_sensitive: bool) -> Self {
        Self {
            case_sensitive,
            ..self
        }
    }
}

/// Ordinal day of the year.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ordinal {
    /// The padding to obtain the minimum width.
    pub padding: Padding,
}

impl Ordinal {
    /// Set the padding type.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_padding(self, padding: Padding) -> Self {
        Self { padding }
    }
}

/// The representation used for the day of the week.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeekdayRepr {
    /// The short form of the weekday (e.g. "Mon").
    Short,
    /// The long form of the weekday (e.g. "Monday").
    Long,
    /// A numerical representation using Sunday as the first day of the week.
    ///
    /// Sunday is either 0 or 1, depending on the other modifier's value.
    Sunday,
    /// A numerical representation using Monday as the first day of the week.
    ///
    /// Monday is either 0 or 1, depending on the other modifier's value.
    Monday,
}

/// Day of the week.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Weekday {
    /// What form of representation should be used?
    pub repr: WeekdayRepr,
    /// When using a numerical representation, should it be zero or one-indexed?
    pub one_indexed: bool,
    /// Is the value case sensitive when parsing?
    pub case_sensitive: bool,
}

impl Weekday {
    /// Set the manner in which the weekday is represented.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_repr(self, repr: WeekdayRepr) -> Self {
        Self { repr, ..self }
    }

    /// Set whether the value is one-indexed when using a numerical representation.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_one_indexed(self, one_indexed: bool) -> Self {
        Self {
            one_indexed,
            ..self
        }
    }

    /// Set whether the value is case sensitive when parsing.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_case_sensitive(self, case_sensitive: bool) -> Self {
        Self {
            case_sensitive,
            ..self
        }
    }
}

/// The representation used for the week number.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeekNumberRepr {
    /// Week 1 is the week that contains January 4.
    Iso,
    /// Week 1 begins on the first Sunday of the calendar year.
    Sunday,
    /// Week 1 begins on the first Monday of the calendar year.
    Monday,
}

/// Week within the year.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeekNumber {
    /// The padding to obtain the minimum width.
    pub padding: Padding,
    /// What kind of representation should be used?
    pub repr: WeekNumberRepr,
}

impl WeekNumber {
    /// Set the padding type.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_padding(self, padding: Padding) -> Self {
        Self { padding, ..self }
    }

    /// Set the manner in which the week number is represented.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_repr(self, repr: WeekNumberRepr) -> Self {
        Self { repr, ..self }
    }
}

/// The representation used for a year value.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YearRepr {
    /// The full value of the year.
    Full,
    /// All digits except the last two. Includes the sign, if any.
    Century,
    /// Only the last two digits of the year.
    LastTwo,
}

/// The range of years that are supported.
///
/// This modifier has no effect when the year repr is [`LastTwo`](YearRepr::LastTwo).
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YearRange {
    /// Years between -9999 and 9999 are supported.
    Standard,
    /// Years between -999_999 and 999_999 are supported, with the sign being required if the year
    /// contains more than four digits.
    ///
    /// If the `large-dates` feature is not enabled, this variant is equivalent to `Standard`.
    Extended,
}

/// Year of the date.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Year {
    /// The padding to obtain the minimum width.
    pub padding: Padding,
    /// What kind of representation should be used?
    pub repr: YearRepr,
    /// What range of years is supported?
    pub range: YearRange,
    /// Whether the value is based on the ISO week number or the Gregorian calendar.
    pub iso_week_based: bool,
    /// Whether the `+` sign is present when a positive year contains fewer than five digits.
    pub sign_is_mandatory: bool,
}

impl Year {
    /// Set the padding type.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_padding(self, padding: Padding) -> Self {
        Self { padding, ..self }
    }

    /// Set the manner in which the year is represented.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_repr(self, repr: YearRepr) -> Self {
        Self { repr, ..self }
    }

    /// Set the range of years that are supported.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_range(self, range: YearRange) -> Self {
        Self { range, ..self }
    }

    /// Set whether the year is based on the ISO week number.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_iso_week_based(self, iso_week_based: bool) -> Self {
        Self {
            iso_week_based,
            ..self
        }
    }

    /// Set whether the `+` sign is mandatory for positive years with fewer than five digits.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_sign_is_mandatory(self, sign_is_mandatory: bool) -> Self {
        Self {
            sign_is_mandatory,
            ..self
        }
    }
}

/// Hour of the day.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hour {
    /// The padding to obtain the minimum width.
    pub padding: Padding,
    /// Is the hour displayed using a 12 or 24-hour clock?
    pub is_12_hour_clock: bool,
}

impl Hour {
    /// Set the padding type.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_padding(self, padding: Padding) -> Self {
        Self { padding, ..self }
    }

    /// Set whether the hour uses a 12-hour clock.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_is_12_hour_clock(self, is_12_hour_clock: bool) -> Self {
        Self {
            is_12_hour_clock,
            ..self
        }
    }
}

/// Minute within the hour.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Minute {
    /// The padding to obtain the minimum width.
    pub padding: Padding,
}

impl Minute {
    /// Set the padding type.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_padding(self, padding: Padding) -> Self {
        Self { padding }
    }
}

/// AM/PM part of the time.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Period {
    /// Is the period uppercase or lowercase?
    pub is_uppercase: bool,
    /// Is the value case sensitive when parsing?
    ///
    /// Note that when `false`, the `is_uppercase` field has no effect on parsing behavior.
    pub case_sensitive: bool,
}

impl Period {
    /// Set whether the period is uppercase.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_is_uppercase(self, is_uppercase: bool) -> Self {
        Self {
            is_uppercase,
            ..self
        }
    }

    /// Set whether the value is case sensitive when parsing.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_case_sensitive(self, case_sensitive: bool) -> Self {
        Self {
            case_sensitive,
            ..self
        }
    }
}

/// Second within the minute.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Second {
    /// The padding to obtain the minimum width.
    pub padding: Padding,
}

impl Second {
    /// Set the padding type.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_padding(self, padding: Padding) -> Self {
        Self { padding }
    }
}

/// The number of digits present in a subsecond representation.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubsecondDigits {
    /// Exactly one digit.
    One,
    /// Exactly two digits.
    Two,
    /// Exactly three digits.
    Three,
    /// Exactly four digits.
    Four,
    /// Exactly five digits.
    Five,
    /// Exactly six digits.
    Six,
    /// Exactly seven digits.
    Seven,
    /// Exactly eight digits.
    Eight,
    /// Exactly nine digits.
    Nine,
    /// Any number of digits (up to nine) that is at least one. When formatting, the minimum digits
    /// necessary will be used.
    OneOrMore,
}

/// Subsecond within the second.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Subsecond {
    /// How many digits are present in the component?
    pub digits: SubsecondDigits,
}

impl Subsecond {
    /// Set the number of digits present in the subsecond representation.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_digits(self, digits: SubsecondDigits) -> Self {
        Self { digits }
    }
}

/// Hour of the UTC offset.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OffsetHour {
    /// Whether the `+` sign is present on positive values.
    pub sign_is_mandatory: bool,
    /// The padding to obtain the minimum width.
    pub padding: Padding,
}

impl OffsetHour {
    /// Set whether the `+` sign is mandatory for positive values.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_sign_is_mandatory(self, sign_is_mandatory: bool) -> Self {
        Self {
            sign_is_mandatory,
            ..self
        }
    }

    /// Set the padding type.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_padding(self, padding: Padding) -> Self {
        Self { padding, ..self }
    }
}

/// Minute within the hour of the UTC offset.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OffsetMinute {
    /// The padding to obtain the minimum width.
    pub padding: Padding,
}

impl OffsetMinute {
    /// Set the padding type.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_padding(self, padding: Padding) -> Self {
        Self { padding }
    }
}

/// Second within the minute of the UTC offset.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OffsetSecond {
    /// The padding to obtain the minimum width.
    pub padding: Padding,
}

impl OffsetSecond {
    /// Set the padding type.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_padding(self, padding: Padding) -> Self {
        Self { padding }
    }
}

/// Type of padding to ensure a minimum width.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Padding {
    /// A space character (` `) should be used as padding.
    Space,
    /// A zero character (`0`) should be used as padding.
    Zero,
    /// There is no padding. This can result in a width below the otherwise minimum number of
    /// characters.
    None,
}

/// Ignore some number of bytes.
///
/// This has no effect when formatting.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ignore {
    /// The number of bytes to ignore.
    pub count: NonZero<u16>,
}

// Needed as `Default` is deliberately not implemented for `Ignore`. The number of bytes to ignore
// must be explicitly provided.
impl Ignore {
    /// Create an instance of `Ignore` with the provided number of bytes to ignore.
    #[inline]
    pub const fn count(count: NonZero<u16>) -> Self {
        Self { count }
    }

    /// Set the number of bytes to ignore.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_count(self, count: NonZero<u16>) -> Self {
        Self { count }
    }
}

/// The precision of a Unix timestamp.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnixTimestampPrecision {
    /// Seconds since the Unix epoch.
    Second,
    /// Milliseconds since the Unix epoch.
    Millisecond,
    /// Microseconds since the Unix epoch.
    Microsecond,
    /// Nanoseconds since the Unix epoch.
    Nanosecond,
}

/// A Unix timestamp.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnixTimestamp {
    /// The precision of the timestamp.
    pub precision: UnixTimestampPrecision,
    /// Whether the `+` sign must be present for a non-negative timestamp.
    pub sign_is_mandatory: bool,
}

impl UnixTimestamp {
    /// Set the precision of the timestamp.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_precision(self, precision: UnixTimestampPrecision) -> Self {
        Self { precision, ..self }
    }

    /// Set whether the `+` sign is mandatory for non-negative timestamps.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_sign_is_mandatory(self, sign_is_mandatory: bool) -> Self {
        Self {
            sign_is_mandatory,
            ..self
        }
    }
}

/// Whether trailing input after the declared end is permitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrailingInput {
    /// Trailing input is not permitted and will cause an error.
    Prohibit,
    /// Trailing input is permitted but discarded.
    Discard,
}

/// The end of input.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct End {
    /// How to handle any input after this component.
    pub(crate) trailing_input: TrailingInput,
}

impl End {
    /// Set how to handle any input after this component.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn with_trailing_input(self, trailing_input: TrailingInput) -> Self {
        Self {
            trailing_input,
            ..self
        }
    }
}
