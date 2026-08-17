// Prefer runtime checks if possible, as otherwise tests can't be run at all if something is
// changed.

use std::borrow::Borrow;
use std::error::Error as StdError;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::time::{Duration as StdDuration, Instant as StdInstant, SystemTime};

use quickcheck::Arbitrary;
use rand08::distributions::{Distribution as DistributionRand08, Standard as StandardRand08};
use rand09::distr::{Distribution as DistributionRand09, StandardUniform as StandardUniformRand09};
use serde::{Deserialize, Serialize};
#[expect(deprecated)]
use time::Instant;
use time::format_description::well_known::iso8601;
use time::format_description::{BorrowedFormatItem, Component, modifier, well_known};
use time::formatting::Formattable;
use time::parsing::{Parsable, Parsed};
use time::{
    Date, Duration, Error, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcDateTime, UtcOffset,
    Weekday, error, ext,
};

#[expect(clippy::cognitive_complexity, reason = "all test the same thing")]
#[test]
fn alignment() {
    macro_rules! assert_alignment {
        ($t:ty, $alignment:expr) => {
            let alignment = $alignment;
            assert_eq!(
                align_of::<$t>(),
                alignment,
                "alignment of `{}` was {}",
                stringify!($t),
                alignment,
            );
        };
    }

    assert_alignment!(Date, 4);
    assert_alignment!(Duration, 8);
    assert_alignment!(OffsetDateTime, 4);
    assert_alignment!(PrimitiveDateTime, 4);
    assert_alignment!(UtcDateTime, 4);
    assert_alignment!(Time, 4);
    assert_alignment!(UtcOffset, 1);
    assert_alignment!(error::ComponentRange, 8);
    assert_alignment!(error::ConversionRange, 1);
    assert_alignment!(error::DifferentVariant, 1);
    assert_alignment!(error::IndeterminateOffset, 1);
    assert_alignment!(modifier::Day, 1);
    assert_alignment!(modifier::Hour, 1);
    assert_alignment!(modifier::Minute, 1);
    assert_alignment!(modifier::Month, 1);
    assert_alignment!(modifier::OffsetHour, 1);
    assert_alignment!(modifier::OffsetMinute, 1);
    assert_alignment!(modifier::OffsetSecond, 1);
    assert_alignment!(modifier::Ordinal, 1);
    assert_alignment!(modifier::Period, 1);
    assert_alignment!(modifier::Second, 1);
    assert_alignment!(modifier::Subsecond, 1);
    assert_alignment!(modifier::WeekNumber, 1);
    assert_alignment!(modifier::Weekday, 1);
    assert_alignment!(modifier::Year, 1);
    assert_alignment!(well_known::Rfc2822, 1);
    assert_alignment!(well_known::Rfc3339, 1);
    assert_alignment!(
        well_known::Iso8601<{ iso8601::Config::DEFAULT.encode() }>,
        1
    );
    assert_alignment!(iso8601::Config, 1);
    assert_alignment!(iso8601::DateKind, 1);
    assert_alignment!(iso8601::FormattedComponents, 1);
    assert_alignment!(iso8601::OffsetPrecision, 1);
    assert_alignment!(iso8601::TimePrecision, 1);
    assert_alignment!(Parsed, align_of::<u128>());
    assert_alignment!(Month, 1);
    assert_alignment!(Weekday, 1);
    assert_alignment!(Error, 8);
    assert_alignment!(error::Format, 8);
    assert_alignment!(error::InvalidFormatDescription, 8);
    assert_alignment!(error::Parse, 8);
    assert_alignment!(error::ParseFromDescription, 8);
    assert_alignment!(error::TryFromParsed, 8);
    assert_alignment!(Component, 2);
    assert_alignment!(BorrowedFormatItem<'_>, 8);
    assert_alignment!(modifier::MonthRepr, 1);
    assert_alignment!(modifier::Padding, 1);
    assert_alignment!(modifier::SubsecondDigits, 1);
    assert_alignment!(modifier::WeekNumberRepr, 1);
    assert_alignment!(modifier::WeekdayRepr, 1);
    assert_alignment!(modifier::YearRepr, 1);
}

#[expect(clippy::cognitive_complexity, reason = "all test the same thing")]
#[test]
fn size() {
    macro_rules! assert_size {
        ($t:ty, $size:literal, $opt_size:literal) => {
            assert!(
                size_of::<$t>() <= $size,
                concat!("size of `{}` used to be ", $size, ", but is now {}"),
                stringify!($t),
                size_of::<$t>(),
            );
            assert!(
                size_of::<Option<$t>>() <= $opt_size,
                concat!(
                    "size of `Option<{}>` used to be ",
                    $opt_size,
                    ", but is now {}"
                ),
                stringify!($t),
                size_of::<Option<$t>>(),
            );
        };
    }

    assert_size!(Date, 4, 4);
    assert_size!(Duration, 16, 16);
    assert_size!(OffsetDateTime, 16, 16);
    assert_size!(PrimitiveDateTime, 12, 12);
    assert_size!(UtcDateTime, 12, 12);
    assert_size!(Time, 8, 8);
    assert_size!(UtcOffset, 3, 4);
    assert_size!(error::ComponentRange, 24, 24);
    assert_size!(error::ConversionRange, 0, 1);
    assert_size!(error::DifferentVariant, 0, 1);
    assert_size!(error::IndeterminateOffset, 0, 1);
    assert_size!(modifier::Day, 1, 1);
    assert_size!(modifier::Hour, 2, 2);
    assert_size!(modifier::Minute, 1, 1);
    assert_size!(modifier::Month, 3, 3);
    assert_size!(modifier::OffsetHour, 2, 2);
    assert_size!(modifier::OffsetMinute, 1, 1);
    assert_size!(modifier::OffsetSecond, 1, 1);
    assert_size!(modifier::Ordinal, 1, 1);
    assert_size!(modifier::Period, 2, 2);
    assert_size!(modifier::Second, 1, 1);
    assert_size!(modifier::Subsecond, 1, 1);
    assert_size!(modifier::WeekNumber, 2, 2);
    assert_size!(modifier::Weekday, 3, 3);
    assert_size!(modifier::Year, 5, 5);
    assert_size!(well_known::Rfc2822, 0, 1);
    assert_size!(well_known::Rfc3339, 0, 1);
    assert_size!(
        well_known::Iso8601<{ iso8601::Config::DEFAULT.encode() }>,
        0,
        1
    );
    assert_size!(iso8601::Config, 7, 7);
    assert_size!(iso8601::DateKind, 1, 1);
    assert_size!(iso8601::FormattedComponents, 1, 1);
    assert_size!(iso8601::OffsetPrecision, 1, 1);
    assert_size!(iso8601::TimePrecision, 2, 2);
    assert_size!(Parsed, 64, 64);
    assert_size!(Month, 1, 1);
    assert_size!(Weekday, 1, 1);
    assert_size!(Error, 48, 48);
    assert_size!(error::Format, 24, 24);
    assert_size!(error::InvalidFormatDescription, 48, 48);
    assert_size!(error::Parse, 32, 32);
    assert_size!(error::ParseFromDescription, 24, 24);
    assert_size!(error::TryFromParsed, 24, 24);
    assert_size!(Component, 6, 6);
    assert_size!(BorrowedFormatItem<'_>, 24, 24);
    assert_size!(modifier::MonthRepr, 1, 1);
    assert_size!(modifier::Padding, 1, 1);
    assert_size!(modifier::SubsecondDigits, 1, 1);
    assert_size!(modifier::WeekNumberRepr, 1, 1);
    assert_size!(modifier::WeekdayRepr, 1, 1);
    assert_size!(modifier::YearRepr, 1, 1);
}

macro_rules! assert_obj_safe {
    ($($xs:path),+ $(,)?) => {
        $(const _: Option<&dyn $xs> = None;)+
    };
}

assert_obj_safe!(ext::NumericalDuration);
assert_obj_safe!(ext::NumericalStdDuration);
// `Parsable` is not object safe.
// `Formattable` is not object safe.

macro_rules! assert_impl {
    ($(#[$meta:meta])* $($(@$lifetimes:lifetime),+ ;)? $type:ty: $($trait:path),+ $(,)?) => {
        $(#[$meta])*
        const _: fn() = || {
            fn assert_impl_all<$($($lifetimes,)+)? T: ?Sized $(+ $trait)+>() {}
            assert_impl_all::<$type>();
        };
    };
}

assert_impl! { @'a; Date:
    Add<Duration, Output = Date>,
    Add<StdDuration, Output = Date>,
    AddAssign<Duration>,
    AddAssign<StdDuration>,
    Arbitrary,
    Clone,
    Debug,
    Deserialize<'a>,
    Display,
    Hash,
    Ord,
    PartialEq<Date>,
    PartialOrd<Date>,
    Serialize,
    Sub<Date, Output = Duration>,
    Sub<Duration, Output = Date>,
    Sub<StdDuration, Output = Date>,
    SubAssign<Duration>,
    SubAssign<StdDuration>,
    TryFrom<Parsed, Error = error::TryFromParsed>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { @'a; Duration:
    Add<Duration, Output = Duration>,
    Add<StdDuration, Output = Duration>,
    AddAssign<Duration>,
    AddAssign<StdDuration>,
    Arbitrary,
    Clone,
    Debug,
    Default,
    Deserialize<'a>,
    Div<Duration, Output = f64>,
    Div<StdDuration, Output = f64>,
    Div<f32, Output = Duration>,
    Div<f64, Output = Duration>,
    Div<i16, Output = Duration>,
    Div<i32, Output = Duration>,
    Div<i8, Output = Duration>,
    Div<u16, Output = Duration>,
    Div<u32, Output = Duration>,
    Div<u8, Output = Duration>,
    DivAssign<f32>,
    DivAssign<f64>,
    DivAssign<i16>,
    DivAssign<i32>,
    DivAssign<i8>,
    DivAssign<u16>,
    DivAssign<u32>,
    DivAssign<u8>,
    Hash,
    Mul<f32, Output = Duration>,
    Mul<f64, Output = Duration>,
    Mul<i16, Output = Duration>,
    Mul<i32, Output = Duration>,
    Mul<i8, Output = Duration>,
    Mul<u16, Output = Duration>,
    Mul<u32, Output = Duration>,
    Mul<u8, Output = Duration>,
    MulAssign<f32>,
    MulAssign<f64>,
    MulAssign<i16>,
    MulAssign<i32>,
    MulAssign<i8>,
    MulAssign<u16>,
    MulAssign<u32>,
    MulAssign<u8>,
    Neg<Output = Duration>,
    Ord,
    PartialEq<Duration>,
    PartialEq<StdDuration>,
    PartialOrd<Duration>,
    PartialOrd<StdDuration>,
    Serialize,
    Sub<Duration, Output = Duration>,
    Sub<StdDuration, Output = Duration>,
    SubAssign<Duration>,
    SubAssign<StdDuration>,
    Sum<&'a Duration>,
    Sum<Duration>,
    TryFrom<StdDuration, Error = error::ConversionRange>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { #[expect(deprecated)] Instant:
    Add<Duration, Output = Instant>,
    Add<StdDuration, Output = Instant>,
    AddAssign<Duration>,
    AddAssign<StdDuration>,
    AsRef<StdInstant>,
    Borrow<StdInstant>,
    Clone,
    Debug,
    From<StdInstant>,
    Hash,
    Ord,
    PartialEq<Instant>,
    PartialEq<StdInstant>,
    PartialOrd<Instant>,
    PartialOrd<StdInstant>,
    Sub<Duration, Output = Instant>,
    Sub<StdDuration, Output = Instant>,
    Sub<Instant, Output = Duration>,
    Sub<StdInstant, Output = Duration>,
    SubAssign<Duration>,
    SubAssign<StdDuration>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { @'a; OffsetDateTime:
    Add<Duration, Output = OffsetDateTime>,
    Add<StdDuration, Output = OffsetDateTime>,
    AddAssign<Duration>,
    AddAssign<StdDuration>,
    Arbitrary,
    Clone,
    Debug,
    Deserialize<'a>,
    Display,
    From<SystemTime>,
    Hash,
    Ord,
    PartialEq<OffsetDateTime>,
    PartialEq<SystemTime>,
    PartialOrd<OffsetDateTime>,
    PartialOrd<SystemTime>,
    Serialize,
    Sub<OffsetDateTime, Output = Duration>,
    Sub<SystemTime, Output = Duration>,
    Sub<Duration, Output = OffsetDateTime>,
    Sub<StdDuration, Output = OffsetDateTime>,
    SubAssign<Duration>,
    SubAssign<StdDuration>,
    TryFrom<Parsed, Error = error::TryFromParsed>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { @'a; PrimitiveDateTime:
    Add<Duration, Output = PrimitiveDateTime>,
    Add<StdDuration, Output = PrimitiveDateTime>,
    AddAssign<Duration>,
    AddAssign<StdDuration>,
    Arbitrary,
    Clone,
    Debug,
    Deserialize<'a>,
    Display,
    Hash,
    Ord,
    PartialEq<PrimitiveDateTime>,
    PartialOrd<PrimitiveDateTime>,
    Serialize,
    Sub<Duration, Output = PrimitiveDateTime>,
    Sub<StdDuration, Output = PrimitiveDateTime>,
    Sub<PrimitiveDateTime>,
    SubAssign<Duration>,
    SubAssign<StdDuration>,
    TryFrom<Parsed, Error = error::TryFromParsed>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { @'a; UtcDateTime:
    Add<Duration, Output = UtcDateTime>,
    Add<StdDuration, Output = UtcDateTime>,
    AddAssign<Duration>,
    AddAssign<StdDuration>,
    Arbitrary,
    Clone,
    Debug,
    Deserialize<'a>,
    Display,
    Hash,
    Ord,
    PartialEq<UtcDateTime>,
    PartialEq<OffsetDateTime>,
    PartialEq<SystemTime>,
    PartialOrd<UtcDateTime>,
    PartialOrd<OffsetDateTime>,
    PartialOrd<SystemTime>,
    Serialize,
    Sub<Duration, Output = UtcDateTime>,
    Sub<StdDuration, Output = UtcDateTime>,
    Sub<UtcDateTime>,
    Sub<OffsetDateTime>,
    SubAssign<Duration>,
    SubAssign<StdDuration>,
    TryFrom<Parsed, Error = error::TryFromParsed>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { @'a; Time:
    Add<Duration, Output = Time>,
    Add<StdDuration, Output = Time>,
    AddAssign<Duration>,
    AddAssign<StdDuration>,
    Arbitrary,
    Clone,
    Debug,
    Deserialize<'a>,
    Display,
    Hash,
    Ord,
    PartialEq<Time>,
    PartialOrd<Time>,
    Serialize,
    Sub<Duration, Output = Time>,
    Sub<StdDuration, Output = Time>,
    Sub<Time, Output = Duration>,
    SubAssign<Duration>,
    SubAssign<StdDuration>,
    TryFrom<Parsed, Error = error::TryFromParsed>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { @'a; UtcOffset:
    Arbitrary,
    Clone,
    Debug,
    Deserialize<'a>,
    Display,
    Hash,
    Neg,
    Ord,
    PartialEq<UtcOffset>,
    PartialOrd<UtcOffset>,
    Serialize,
    TryFrom<Parsed, Error = error::TryFromParsed>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { error::ComponentRange:
    Clone,
    Debug,
    Display,
    StdError,
    serde::de::Expected,
    Hash,
    PartialEq<error::ComponentRange>,
    TryFrom<Error, Error = error::DifferentVariant>,
    TryFrom<error::TryFromParsed, Error = error::DifferentVariant>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { error::ConversionRange:
    Clone,
    Debug,
    Display,
    StdError,
    PartialEq<error::ConversionRange>,
    TryFrom<Error, Error = error::DifferentVariant>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { error::DifferentVariant:
    Clone,
    Debug,
    Display,
    StdError,
    PartialEq<error::DifferentVariant>,
    TryFrom<Error, Error = error::DifferentVariant>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { error::IndeterminateOffset:
    Clone,
    Debug,
    Display,
    StdError,
    PartialEq<error::IndeterminateOffset>,
    TryFrom<Error, Error = error::DifferentVariant>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { modifier::Day:
    Clone,
    Debug,
    Default,
    PartialEq<modifier::Day>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { modifier::Hour:
    Clone,
    Debug,
    Default,
    PartialEq<modifier::Hour>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { modifier::Minute:
    Clone,
    Debug,
    Default,
    PartialEq<modifier::Minute>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { modifier::Month:
    Clone,
    Debug,
    Default,
    PartialEq<modifier::Month>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { modifier::OffsetHour:
    Clone,
    Debug,
    Default,
    PartialEq<modifier::OffsetHour>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { modifier::OffsetMinute:
    Clone,
    Debug,
    Default,
    PartialEq<modifier::OffsetMinute>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { modifier::OffsetSecond:
    Clone,
    Debug,
    Default,
    PartialEq<modifier::OffsetSecond>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { modifier::Ordinal:
    Clone,
    Debug,
    Default,
    PartialEq<modifier::Ordinal>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { modifier::Period:
    Clone,
    Debug,
    Default,
    PartialEq<modifier::Period>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { modifier::Second:
    Clone,
    Debug,
    Default,
    PartialEq<modifier::Second>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { modifier::Subsecond:
    Clone,
    Debug,
    Default,
    PartialEq<modifier::Subsecond>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { modifier::WeekNumber:
    Clone,
    Debug,
    Default,
    PartialEq<modifier::WeekNumber>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { modifier::Weekday:
    Clone,
    Debug,
    Default,
    PartialEq<modifier::Weekday>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { modifier::Year:
    Clone,
    Debug,
    Default,
    PartialEq<modifier::Year>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { well_known::Rfc2822:
    Clone,
    Debug,
    PartialEq<well_known::Rfc2822>,
    Copy,
    Eq,
    Formattable,
    Parsable,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { well_known::Rfc3339:
    Clone,
    Debug,
    PartialEq<well_known::Rfc3339>,
    Copy,
    Eq,
    Formattable,
    Parsable,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { well_known::Iso8601::<{ iso8601::Config::DEFAULT.encode() }>:
    Clone,
    Debug,
    PartialEq<well_known::Iso8601<{ iso8601::Config::DEFAULT.encode() }>>,
    Copy,
    Eq,
    Formattable,
    Parsable,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { iso8601::Config:
    Debug,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { iso8601::DateKind:
    Clone,
    Debug,
    PartialEq<iso8601::DateKind>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { iso8601::FormattedComponents:
    Clone,
    Debug,
    PartialEq<iso8601::FormattedComponents>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { iso8601::OffsetPrecision:
    Clone,
    Debug,
    PartialEq<iso8601::OffsetPrecision>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { iso8601::TimePrecision:
    Clone,
    Debug,
    PartialEq<iso8601::TimePrecision>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { Parsed:
    Clone,
    Debug,
    Copy,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { @'a; Month:
    Arbitrary,
    Clone,
    Debug,
    Deserialize<'a>,
    Display,
    Hash,
    PartialEq<Month>,
    TryFrom<u8, Error = error::ComponentRange>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { @'a; Weekday:
    Arbitrary,
    Clone,
    Debug,
    Deserialize<'a>,
    Display,
    Hash,
    PartialEq<Weekday>,
    Serialize,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { Error:
    Debug,
    Display,
    StdError,
    From<error::ComponentRange>,
    From<error::ConversionRange>,
    From<error::DifferentVariant>,
    From<error::Format>,
    From<error::IndeterminateOffset>,
    From<error::InvalidFormatDescription>,
    From<error::Parse>,
    From<error::ParseFromDescription>,
    From<error::TryFromParsed>,
    Send,
    Sync,
    Unpin,
}
assert_impl! { error::Format:
    Debug,
    Display,
    StdError,
    From<std::io::Error>,
    TryFrom<Error, Error = error::DifferentVariant>,
    Send,
    Sync,
    Unpin,
}
assert_impl! { error::InvalidFormatDescription:
    Clone,
    Debug,
    Display,
    StdError,
    PartialEq<error::InvalidFormatDescription>,
    TryFrom<Error, Error = error::DifferentVariant>,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { error::Parse:
    Clone,
    Debug,
    Display,
    StdError,
    From<error::ParseFromDescription>,
    From<error::TryFromParsed>,
    PartialEq<error::Parse>,
    TryFrom<Error, Error = error::DifferentVariant>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { error::ParseFromDescription:
    Clone,
    Debug,
    Display,
    StdError,
    PartialEq<error::ParseFromDescription>,
    TryFrom<Error, Error = error::DifferentVariant>,
    TryFrom<error::Parse, Error = error::DifferentVariant>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { error::TryFromParsed:
    Clone,
    Debug,
    Display,
    StdError,
    From<error::ComponentRange>,
    PartialEq<error::TryFromParsed>,
    TryFrom<Error, Error = error::DifferentVariant>,
    TryFrom<error::Parse, Error = error::DifferentVariant>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { @'a; Component:
    Clone,
    Debug,
    PartialEq<Component>,
    PartialEq<BorrowedFormatItem<'a>>,
    TryFrom<BorrowedFormatItem<'a>, Error = error::DifferentVariant>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { @'a; BorrowedFormatItem<'_>:
    Clone,
    Debug,
    From<&'a [BorrowedFormatItem<'a>]>,
    From<Component>,
    PartialEq<&'a [BorrowedFormatItem<'a>]>,
    PartialEq<Component>,
    PartialEq<BorrowedFormatItem<'a>>,
    Eq,
    Formattable,
    Parsable,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { @'a; &[BorrowedFormatItem<'_>]:
    PartialEq<BorrowedFormatItem<'a>>,
    TryFrom<BorrowedFormatItem<'a>, Error = error::DifferentVariant>,
}
assert_impl! { modifier::MonthRepr:
    Clone,
    Debug,
    Default,
    PartialEq<modifier::MonthRepr>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { modifier::Padding:
    Clone,
    Debug,
    Default,
    PartialEq<modifier::Padding>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { modifier::SubsecondDigits:
    Clone,
    Debug,
    Default,
    PartialEq<modifier::SubsecondDigits>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { modifier::WeekNumberRepr:
    Clone,
    Debug,
    Default,
    PartialEq<modifier::WeekNumberRepr>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { modifier::WeekdayRepr:
    Clone,
    Debug,
    Default,
    PartialEq<modifier::WeekdayRepr>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { modifier::YearRepr:
    Clone,
    Debug,
    Default,
    PartialEq<modifier::YearRepr>,
    Copy,
    Eq,
    RefUnwindSafe,
    Send,
    Sync,
    Unpin,
    UnwindSafe,
}
assert_impl! { StandardRand08:
    DistributionRand08<Date>,
    DistributionRand08<Duration>,
    DistributionRand08<OffsetDateTime>,
    DistributionRand08<UtcDateTime>,
    DistributionRand08<PrimitiveDateTime>,
    DistributionRand08<Time>,
    DistributionRand08<UtcOffset>,
    DistributionRand08<Month>,
    DistributionRand08<Weekday>,
}
assert_impl! { StandardUniformRand09:
    DistributionRand09<Date>,
    DistributionRand09<Duration>,
    DistributionRand09<OffsetDateTime>,
    DistributionRand09<UtcDateTime>,
    DistributionRand09<PrimitiveDateTime>,
    DistributionRand09<Time>,
    DistributionRand09<UtcOffset>,
    DistributionRand09<Month>,
    DistributionRand09<Weekday>,
}
assert_impl! { StdDuration:
    Add<Duration, Output = Duration>,
    AddAssign<Duration>,
    Div<Duration, Output = f64>,
    PartialEq<Duration>,
    PartialOrd<Duration>,
    Sub<Duration, Output = Duration>,
    SubAssign<Duration>,
    TryFrom<Duration>,
}
assert_impl! { #[expect(deprecated)] StdInstant:
    Add<Duration, Output = StdInstant>,
    AddAssign<Duration>,
    Sub<Duration, Output = StdInstant>,
    SubAssign<Duration>,
    PartialEq<Instant>,
    PartialOrd<Instant>,
    From<Instant>,
    Sub<Instant>,
}
assert_impl! { SystemTime:
    Add<Duration, Output = SystemTime>,
    AddAssign<Duration>,
    Sub<Duration, Output = SystemTime>,
    SubAssign<Duration>,
    From<OffsetDateTime>,
    PartialEq<OffsetDateTime>,
    PartialOrd<OffsetDateTime>,
    Sub<OffsetDateTime>,
}
assert_impl! { i8:
    Mul<Duration>,
}
assert_impl! { i16:
    Mul<Duration>,
}
assert_impl! { i32:
    Mul<Duration>,
}
assert_impl! { u8:
    Mul<Duration>,
    From<Month>,
}
assert_impl! { u16:
    Mul<Duration>,
}
assert_impl! { u32:
    Mul<Duration>,
}
assert_impl! { f32:
    Mul<Duration>,
}
assert_impl! { f64:
    Mul<Duration>,
}
