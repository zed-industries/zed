//! A trait that can be used to parse an item from an input.

use core::num::NonZero;
use core::ops::Deref;

use num_conv::prelude::*;

use crate::error::ParseFromDescription::{InvalidComponent, InvalidLiteral};
use crate::error::TryFromParsed;
#[cfg(feature = "alloc")]
use crate::format_description::OwnedFormatItem;
use crate::format_description::well_known::iso8601::EncodedConfig;
use crate::format_description::well_known::{Iso8601, Rfc2822, Rfc3339};
use crate::format_description::{BorrowedFormatItem, modifier};
use crate::internal_macros::{bug, try_likely_ok};
use crate::parsing::combinator::{
    ExactlyNDigits, Sign, any_digit, ascii_char, ascii_char_ignore_case, one_or_two_digits, opt,
    sign,
};
use crate::parsing::{Parsed, ParsedItem, component};
use crate::{Date, Month, OffsetDateTime, Time, UtcOffset, error};

/// A type that can be parsed.
#[cfg_attr(docsrs, doc(notable_trait))]
#[doc(alias = "Parseable")]
pub trait Parsable: sealed::Sealed {}
impl Parsable for BorrowedFormatItem<'_> {}
impl Parsable for [BorrowedFormatItem<'_>] {}
#[cfg(feature = "alloc")]
impl Parsable for OwnedFormatItem {}
#[cfg(feature = "alloc")]
impl Parsable for [OwnedFormatItem] {}
impl Parsable for Rfc2822 {}
impl Parsable for Rfc3339 {}
impl<const CONFIG: EncodedConfig> Parsable for Iso8601<CONFIG> {}
impl<T> Parsable for T where T: Deref<Target: Parsable> {}

/// Seal the trait to prevent downstream users from implementing it, while still allowing it to
/// exist in generic bounds.
mod sealed {
    use super::*;
    use crate::{PrimitiveDateTime, UtcDateTime};

    /// Parse the item using a format description and an input.
    pub trait Sealed {
        /// Parse the item into the provided [`Parsed`] struct.
        ///
        /// This method can be used to parse a single component without parsing the full value.
        fn parse_into<'a>(
            &self,
            input: &'a [u8],
            parsed: &mut Parsed,
        ) -> Result<&'a [u8], error::Parse>;

        /// Parse the item into a new [`Parsed`] struct.
        ///
        /// This method can only be used to parse a complete value of a type. If any characters
        /// remain after parsing, an error will be returned.
        #[inline]
        fn parse(&self, input: &[u8]) -> Result<Parsed, error::Parse> {
            let mut parsed = Parsed::new();
            if self.parse_into(input, &mut parsed)?.is_empty() {
                Ok(parsed)
            } else {
                Err(error::Parse::ParseFromDescription(
                    error::ParseFromDescription::UnexpectedTrailingCharacters,
                ))
            }
        }

        /// Parse a [`Date`] from the format description.
        #[inline]
        fn parse_date(&self, input: &[u8]) -> Result<Date, error::Parse> {
            Ok(self.parse(input)?.try_into()?)
        }

        /// Parse a [`Time`] from the format description.
        #[inline]
        fn parse_time(&self, input: &[u8]) -> Result<Time, error::Parse> {
            Ok(self.parse(input)?.try_into()?)
        }

        /// Parse a [`UtcOffset`] from the format description.
        #[inline]
        fn parse_offset(&self, input: &[u8]) -> Result<UtcOffset, error::Parse> {
            Ok(self.parse(input)?.try_into()?)
        }

        /// Parse a [`PrimitiveDateTime`] from the format description.
        #[inline]
        fn parse_primitive_date_time(
            &self,
            input: &[u8],
        ) -> Result<PrimitiveDateTime, error::Parse> {
            Ok(self.parse(input)?.try_into()?)
        }

        /// Parse a [`UtcDateTime`] from the format description.
        #[inline]
        fn parse_utc_date_time(&self, input: &[u8]) -> Result<UtcDateTime, error::Parse> {
            Ok(self.parse(input)?.try_into()?)
        }

        /// Parse a [`OffsetDateTime`] from the format description.
        #[inline]
        fn parse_offset_date_time(&self, input: &[u8]) -> Result<OffsetDateTime, error::Parse> {
            Ok(self.parse(input)?.try_into()?)
        }
    }
}

impl sealed::Sealed for BorrowedFormatItem<'_> {
    #[inline]
    fn parse_into<'a>(
        &self,
        input: &'a [u8],
        parsed: &mut Parsed,
    ) -> Result<&'a [u8], error::Parse> {
        Ok(parsed.parse_item(input, self)?)
    }
}

impl sealed::Sealed for [BorrowedFormatItem<'_>] {
    #[inline]
    fn parse_into<'a>(
        &self,
        input: &'a [u8],
        parsed: &mut Parsed,
    ) -> Result<&'a [u8], error::Parse> {
        Ok(parsed.parse_items(input, self)?)
    }
}

#[cfg(feature = "alloc")]
impl sealed::Sealed for OwnedFormatItem {
    #[inline]
    fn parse_into<'a>(
        &self,
        input: &'a [u8],
        parsed: &mut Parsed,
    ) -> Result<&'a [u8], error::Parse> {
        Ok(parsed.parse_item(input, self)?)
    }
}

#[cfg(feature = "alloc")]
impl sealed::Sealed for [OwnedFormatItem] {
    #[inline]
    fn parse_into<'a>(
        &self,
        input: &'a [u8],
        parsed: &mut Parsed,
    ) -> Result<&'a [u8], error::Parse> {
        Ok(parsed.parse_items(input, self)?)
    }
}

impl<T> sealed::Sealed for T
where
    T: Deref<Target: sealed::Sealed>,
{
    #[inline]
    fn parse_into<'a>(
        &self,
        input: &'a [u8],
        parsed: &mut Parsed,
    ) -> Result<&'a [u8], error::Parse> {
        self.deref().parse_into(input, parsed)
    }
}

impl sealed::Sealed for Rfc2822 {
    fn parse_into<'a>(
        &self,
        input: &'a [u8],
        parsed: &mut Parsed,
    ) -> Result<&'a [u8], error::Parse> {
        use crate::parsing::combinator::rfc::rfc2822::{cfws, fws, zone_literal};

        let colon = ascii_char::<b':'>;
        let comma = ascii_char::<b','>;

        let input = opt(cfws)(input).into_inner();
        let weekday = component::parse_weekday(
            input,
            modifier::Weekday {
                repr: modifier::WeekdayRepr::Short,
                one_indexed: false,
                case_sensitive: false,
            },
        );
        let input = if let Some(item) = weekday {
            let input = try_likely_ok!(
                item.consume_value(|value| parsed.set_weekday(value))
                    .ok_or(InvalidComponent("weekday"))
            );
            let input = try_likely_ok!(comma(input).ok_or(InvalidLiteral)).into_inner();
            opt(cfws)(input).into_inner()
        } else {
            input
        };
        let input = try_likely_ok!(
            one_or_two_digits(input)
                .and_then(|item| item.consume_value(|value| parsed.set_day(NonZero::new(value)?)))
                .ok_or(InvalidComponent("day"))
        );
        let input = try_likely_ok!(cfws(input).ok_or(InvalidLiteral)).into_inner();
        let input = try_likely_ok!(
            component::parse_month(
                input,
                modifier::Month {
                    padding: modifier::Padding::None,
                    repr: modifier::MonthRepr::Short,
                    case_sensitive: false,
                },
            )
            .and_then(|item| item.consume_value(|value| parsed.set_month(value)))
            .ok_or(InvalidComponent("month"))
        );
        let input = try_likely_ok!(cfws(input).ok_or(InvalidLiteral)).into_inner();
        let input = match ExactlyNDigits::<4>::parse(input) {
            Some(item) => {
                let input = try_likely_ok!(
                    item.flat_map(|year| if year >= 1900 { Some(year) } else { None })
                        .and_then(|item| {
                            item.consume_value(|value| {
                                parsed.set_year(value.cast_signed().extend())
                            })
                        })
                        .ok_or(InvalidComponent("year"))
                );
                try_likely_ok!(fws(input).ok_or(InvalidLiteral)).into_inner()
            }
            None => {
                let input = try_likely_ok!(
                    ExactlyNDigits::<2>::parse(input)
                        .and_then(|item| {
                            item.map(|year| year.extend::<u32>())
                                .map(|year| if year < 50 { year + 2000 } else { year + 1900 })
                                .map(|year| year.cast_signed())
                                .consume_value(|value| parsed.set_year(value))
                        })
                        .ok_or(InvalidComponent("year"))
                );
                try_likely_ok!(cfws(input).ok_or(InvalidLiteral)).into_inner()
            }
        };

        let input = try_likely_ok!(
            ExactlyNDigits::<2>::parse(input)
                .and_then(|item| item.consume_value(|value| parsed.set_hour_24(value)))
                .ok_or(InvalidComponent("hour"))
        );
        let input = opt(cfws)(input).into_inner();
        let input = try_likely_ok!(colon(input).ok_or(InvalidLiteral)).into_inner();
        let input = opt(cfws)(input).into_inner();
        let input = try_likely_ok!(
            ExactlyNDigits::<2>::parse(input)
                .and_then(|item| item.consume_value(|value| parsed.set_minute(value)))
                .ok_or(InvalidComponent("minute"))
        );

        let input = if let Some(input) = colon(opt(cfws)(input).into_inner()) {
            let input = input.into_inner(); // discard the colon
            let input = opt(cfws)(input).into_inner();
            let input = try_likely_ok!(
                ExactlyNDigits::<2>::parse(input)
                    .and_then(|item| item.consume_value(|value| parsed.set_second(value)))
                    .ok_or(InvalidComponent("second"))
            );
            try_likely_ok!(cfws(input).ok_or(InvalidLiteral)).into_inner()
        } else {
            try_likely_ok!(cfws(input).ok_or(InvalidLiteral)).into_inner()
        };

        // The RFC explicitly allows leap seconds.
        parsed.leap_second_allowed = true;

        if let Some(zone_literal) = zone_literal(input) {
            let input = try_likely_ok!(
                zone_literal
                    .consume_value(|value| parsed.set_offset_hour(value))
                    .ok_or(InvalidComponent("offset hour"))
            );
            try_likely_ok!(
                parsed
                    .set_offset_minute_signed(0)
                    .ok_or(InvalidComponent("offset minute"))
            );
            try_likely_ok!(
                parsed
                    .set_offset_second_signed(0)
                    .ok_or(InvalidComponent("offset second"))
            );
            return Ok(input);
        }

        let ParsedItem(input, offset_sign) =
            try_likely_ok!(sign(input).ok_or(InvalidComponent("offset hour")));
        let input = try_likely_ok!(
            ExactlyNDigits::<2>::parse(input)
                .and_then(|item| {
                    item.map(|offset_hour| match offset_sign {
                        Sign::Negative => -offset_hour.cast_signed(),
                        Sign::Positive => offset_hour.cast_signed(),
                    })
                    .consume_value(|value| parsed.set_offset_hour(value))
                })
                .ok_or(InvalidComponent("offset hour"))
        );
        let input = try_likely_ok!(
            ExactlyNDigits::<2>::parse(input)
                .and_then(|item| {
                    item.consume_value(|value| parsed.set_offset_minute_signed(value.cast_signed()))
                })
                .ok_or(InvalidComponent("offset minute"))
        );

        let input = opt(cfws)(input).into_inner();

        Ok(input)
    }

    fn parse_offset_date_time(&self, input: &[u8]) -> Result<OffsetDateTime, error::Parse> {
        use crate::parsing::combinator::rfc::rfc2822::{cfws, fws, zone_literal};

        let colon = ascii_char::<b':'>;
        let comma = ascii_char::<b','>;

        let input = opt(cfws)(input).into_inner();
        let weekday = component::parse_weekday(
            input,
            modifier::Weekday {
                repr: modifier::WeekdayRepr::Short,
                one_indexed: false,
                case_sensitive: false,
            },
        );
        let input = if let Some(item) = weekday {
            let input = item.discard_value();
            let input = try_likely_ok!(comma(input).ok_or(InvalidLiteral)).into_inner();
            opt(cfws)(input).into_inner()
        } else {
            input
        };
        let ParsedItem(input, day) =
            try_likely_ok!(one_or_two_digits(input).ok_or(InvalidComponent("day")));
        let input = try_likely_ok!(cfws(input).ok_or(InvalidLiteral)).into_inner();
        let ParsedItem(input, month) = try_likely_ok!(
            component::parse_month(
                input,
                modifier::Month {
                    padding: modifier::Padding::None,
                    repr: modifier::MonthRepr::Short,
                    case_sensitive: false,
                },
            )
            .ok_or(InvalidComponent("month"))
        );
        let input = try_likely_ok!(cfws(input).ok_or(InvalidLiteral)).into_inner();
        let (input, year) = match ExactlyNDigits::<4>::parse(input) {
            Some(item) => {
                let ParsedItem(input, year) = try_likely_ok!(
                    item.flat_map(|year| if year >= 1900 { Some(year) } else { None })
                        .ok_or(InvalidComponent("year"))
                );
                let input = try_likely_ok!(fws(input).ok_or(InvalidLiteral)).into_inner();
                (input, year)
            }
            None => {
                let ParsedItem(input, year) = try_likely_ok!(
                    ExactlyNDigits::<2>::parse(input)
                        .map(|item| {
                            item.map(|year| year.extend::<u16>())
                                .map(|year| if year < 50 { year + 2000 } else { year + 1900 })
                        })
                        .ok_or(InvalidComponent("year"))
                );
                let input = try_likely_ok!(cfws(input).ok_or(InvalidLiteral)).into_inner();
                (input, year)
            }
        };

        let ParsedItem(input, hour) =
            try_likely_ok!(ExactlyNDigits::<2>::parse(input).ok_or(InvalidComponent("hour")));
        let input = opt(cfws)(input).into_inner();
        let input = try_likely_ok!(colon(input).ok_or(InvalidLiteral)).into_inner();
        let input = opt(cfws)(input).into_inner();
        let ParsedItem(input, minute) =
            try_likely_ok!(ExactlyNDigits::<2>::parse(input).ok_or(InvalidComponent("minute")));

        let (input, mut second) = if let Some(input) = colon(opt(cfws)(input).into_inner()) {
            let input = input.into_inner(); // discard the colon
            let input = opt(cfws)(input).into_inner();
            let ParsedItem(input, second) =
                try_likely_ok!(ExactlyNDigits::<2>::parse(input).ok_or(InvalidComponent("second")));
            let input = try_likely_ok!(cfws(input).ok_or(InvalidLiteral)).into_inner();
            (input, second)
        } else {
            (
                try_likely_ok!(cfws(input).ok_or(InvalidLiteral)).into_inner(),
                0,
            )
        };

        let (input, offset_hour, offset_minute) = if let Some(zone_literal) = zone_literal(input) {
            let ParsedItem(input, offset_hour) = zone_literal;
            (input, offset_hour, 0)
        } else {
            let ParsedItem(input, offset_sign) =
                try_likely_ok!(sign(input).ok_or(InvalidComponent("offset hour")));
            let ParsedItem(input, offset_hour) = try_likely_ok!(
                ExactlyNDigits::<2>::parse(input)
                    .map(|item| {
                        item.map(|offset_hour| match offset_sign {
                            Sign::Negative => -offset_hour.cast_signed(),
                            Sign::Positive => offset_hour.cast_signed(),
                        })
                    })
                    .ok_or(InvalidComponent("offset hour"))
            );
            let ParsedItem(input, offset_minute) = try_likely_ok!(
                ExactlyNDigits::<2>::parse(input).ok_or(InvalidComponent("offset minute"))
            );
            (input, offset_hour, offset_minute.cast_signed())
        };

        let input = opt(cfws)(input).into_inner();

        if !input.is_empty() {
            return Err(error::Parse::ParseFromDescription(
                error::ParseFromDescription::UnexpectedTrailingCharacters,
            ));
        }

        let mut nanosecond = 0;
        let leap_second_input = if second == 60 {
            second = 59;
            nanosecond = 999_999_999;
            true
        } else {
            false
        };

        let dt = try_likely_ok!(
            (|| {
                let date = try_likely_ok!(Date::from_calendar_date(
                    year.cast_signed().extend(),
                    month,
                    day
                ));
                let time = try_likely_ok!(Time::from_hms_nano(hour, minute, second, nanosecond));
                let offset = try_likely_ok!(UtcOffset::from_hms(offset_hour, offset_minute, 0));
                Ok(OffsetDateTime::new_in_offset(date, time, offset))
            })()
            .map_err(TryFromParsed::ComponentRange)
        );

        if leap_second_input && !dt.is_valid_leap_second_stand_in() {
            return Err(error::Parse::TryFromParsed(TryFromParsed::ComponentRange(
                error::ComponentRange::conditional("second"),
            )));
        }

        Ok(dt)
    }
}

impl sealed::Sealed for Rfc3339 {
    fn parse_into<'a>(
        &self,
        input: &'a [u8],
        parsed: &mut Parsed,
    ) -> Result<&'a [u8], error::Parse> {
        let dash = ascii_char::<b'-'>;
        let colon = ascii_char::<b':'>;

        let input = try_likely_ok!(
            ExactlyNDigits::<4>::parse(input)
                .and_then(|item| {
                    item.consume_value(|value| parsed.set_year(value.cast_signed().extend()))
                })
                .ok_or(InvalidComponent("year"))
        );
        let input = try_likely_ok!(dash(input).ok_or(InvalidLiteral)).into_inner();
        let input = try_likely_ok!(
            ExactlyNDigits::<2>::parse(input)
                .and_then(
                    |item| item.flat_map(|value| Month::from_number(NonZero::new(value)?).ok())
                )
                .and_then(|item| item.consume_value(|value| parsed.set_month(value)))
                .ok_or(InvalidComponent("month"))
        );
        let input = try_likely_ok!(dash(input).ok_or(InvalidLiteral)).into_inner();
        let input = try_likely_ok!(
            ExactlyNDigits::<2>::parse(input)
                .and_then(|item| item.consume_value(|value| parsed.set_day(NonZero::new(value)?)))
                .ok_or(InvalidComponent("day"))
        );

        // RFC3339 allows any separator, not just `T`, not just `space`.
        // cf. Section 5.6: Internet Date/Time Format:
        //   NOTE: ISO 8601 defines date and time separated by "T".
        //   Applications using this syntax may choose, for the sake of
        //   readability, to specify a full-date and full-time separated by
        //   (say) a space character.
        // Specifically, rusqlite uses space separators.
        let input = try_likely_ok!(input.get(1..).ok_or(InvalidComponent("separator")));

        let input = try_likely_ok!(
            ExactlyNDigits::<2>::parse(input)
                .and_then(|item| item.consume_value(|value| parsed.set_hour_24(value)))
                .ok_or(InvalidComponent("hour"))
        );
        let input = try_likely_ok!(colon(input).ok_or(InvalidLiteral)).into_inner();
        let input = try_likely_ok!(
            ExactlyNDigits::<2>::parse(input)
                .and_then(|item| item.consume_value(|value| parsed.set_minute(value)))
                .ok_or(InvalidComponent("minute"))
        );
        let input = try_likely_ok!(colon(input).ok_or(InvalidLiteral)).into_inner();
        let input = try_likely_ok!(
            ExactlyNDigits::<2>::parse(input)
                .and_then(|item| item.consume_value(|value| parsed.set_second(value)))
                .ok_or(InvalidComponent("second"))
        );
        let input = if let Some(ParsedItem(input, ())) = ascii_char::<b'.'>(input) {
            let ParsedItem(mut input, mut value) =
                try_likely_ok!(any_digit(input).ok_or(InvalidComponent("subsecond")))
                    .map(|v| (v - b'0').extend::<u32>() * 100_000_000);

            let mut multiplier = 10_000_000;
            while let Some(ParsedItem(new_input, digit)) = any_digit(input) {
                value += (digit - b'0').extend::<u32>() * multiplier;
                input = new_input;
                multiplier /= 10;
            }

            try_likely_ok!(
                parsed
                    .set_subsecond(value)
                    .ok_or(InvalidComponent("subsecond"))
            );
            input
        } else {
            input
        };

        // The RFC explicitly allows leap seconds.
        parsed.leap_second_allowed = true;

        if let Some(ParsedItem(input, ())) = ascii_char_ignore_case::<b'Z'>(input) {
            try_likely_ok!(
                parsed
                    .set_offset_hour(0)
                    .ok_or(InvalidComponent("offset hour"))
            );
            try_likely_ok!(
                parsed
                    .set_offset_minute_signed(0)
                    .ok_or(InvalidComponent("offset minute"))
            );
            try_likely_ok!(
                parsed
                    .set_offset_second_signed(0)
                    .ok_or(InvalidComponent("offset second"))
            );
            return Ok(input);
        }

        let ParsedItem(input, offset_sign) =
            try_likely_ok!(sign(input).ok_or(InvalidComponent("offset hour")));
        let input = try_likely_ok!(
            ExactlyNDigits::<2>::parse(input)
                .and_then(|item| {
                    item.filter(|&offset_hour| offset_hour <= 23)?
                        .map(|offset_hour| match offset_sign {
                            Sign::Negative => -offset_hour.cast_signed(),
                            Sign::Positive => offset_hour.cast_signed(),
                        })
                        .consume_value(|value| parsed.set_offset_hour(value))
                })
                .ok_or(InvalidComponent("offset hour"))
        );
        let input = try_likely_ok!(colon(input).ok_or(InvalidLiteral)).into_inner();
        let input = try_likely_ok!(
            ExactlyNDigits::<2>::parse(input)
                .and_then(|item| {
                    item.map(|offset_minute| match offset_sign {
                        Sign::Negative => -offset_minute.cast_signed(),
                        Sign::Positive => offset_minute.cast_signed(),
                    })
                    .consume_value(|value| parsed.set_offset_minute_signed(value))
                })
                .ok_or(InvalidComponent("offset minute"))
        );

        Ok(input)
    }

    fn parse_offset_date_time(&self, input: &[u8]) -> Result<OffsetDateTime, error::Parse> {
        let dash = ascii_char::<b'-'>;
        let colon = ascii_char::<b':'>;

        let ParsedItem(input, year) =
            try_likely_ok!(ExactlyNDigits::<4>::parse(input).ok_or(InvalidComponent("year")));
        let input = try_likely_ok!(dash(input).ok_or(InvalidLiteral)).into_inner();
        let ParsedItem(input, month) = try_likely_ok!(
            ExactlyNDigits::<2>::parse(input)
                .and_then(|parsed| parsed.flat_map(NonZero::new))
                .ok_or(InvalidComponent("month"))
        );
        let input = try_likely_ok!(dash(input).ok_or(InvalidLiteral)).into_inner();
        let ParsedItem(input, day) =
            try_likely_ok!(ExactlyNDigits::<2>::parse(input).ok_or(InvalidComponent("day")));

        // RFC3339 allows any separator, not just `T`, not just `space`.
        // cf. Section 5.6: Internet Date/Time Format:
        //   NOTE: ISO 8601 defines date and time separated by "T".
        //   Applications using this syntax may choose, for the sake of
        //   readability, to specify a full-date and full-time separated by
        //   (say) a space character.
        // Specifically, rusqlite uses space separators.
        let input = try_likely_ok!(input.get(1..).ok_or(InvalidComponent("separator")));

        let ParsedItem(input, hour) =
            try_likely_ok!(ExactlyNDigits::<2>::parse(input).ok_or(InvalidComponent("hour")));
        let input = try_likely_ok!(colon(input).ok_or(InvalidLiteral)).into_inner();
        let ParsedItem(input, minute) =
            try_likely_ok!(ExactlyNDigits::<2>::parse(input).ok_or(InvalidComponent("minute")));
        let input = try_likely_ok!(colon(input).ok_or(InvalidLiteral)).into_inner();
        let ParsedItem(input, mut second) =
            try_likely_ok!(ExactlyNDigits::<2>::parse(input).ok_or(InvalidComponent("second")));
        let ParsedItem(input, mut nanosecond) =
            if let Some(ParsedItem(input, ())) = ascii_char::<b'.'>(input) {
                let ParsedItem(mut input, mut value) =
                    try_likely_ok!(any_digit(input).ok_or(InvalidComponent("subsecond")))
                        .map(|v| (v - b'0').extend::<u32>() * 100_000_000);

                let mut multiplier = 10_000_000;
                while let Some(ParsedItem(new_input, digit)) = any_digit(input) {
                    value += (digit - b'0').extend::<u32>() * multiplier;
                    input = new_input;
                    multiplier /= 10;
                }

                ParsedItem(input, value)
            } else {
                ParsedItem(input, 0)
            };
        let ParsedItem(input, offset) = {
            if let Some(ParsedItem(input, ())) = ascii_char_ignore_case::<b'Z'>(input) {
                ParsedItem(input, UtcOffset::UTC)
            } else {
                let ParsedItem(input, offset_sign) =
                    try_likely_ok!(sign(input).ok_or(InvalidComponent("offset hour")));
                let ParsedItem(input, offset_hour) = try_likely_ok!(
                    ExactlyNDigits::<2>::parse(input)
                        .and_then(|parsed| parsed.filter(|&offset_hour| offset_hour <= 23))
                        .ok_or(InvalidComponent("offset hour"))
                );
                let input = try_likely_ok!(colon(input).ok_or(InvalidLiteral)).into_inner();
                let ParsedItem(input, offset_minute) = try_likely_ok!(
                    ExactlyNDigits::<2>::parse(input).ok_or(InvalidComponent("offset minute"))
                );
                try_likely_ok!(
                    match offset_sign {
                        Sign::Negative => UtcOffset::from_hms(
                            -offset_hour.cast_signed(),
                            -offset_minute.cast_signed(),
                            0,
                        ),
                        Sign::Positive => UtcOffset::from_hms(
                            offset_hour.cast_signed(),
                            offset_minute.cast_signed(),
                            0,
                        ),
                    }
                    .map(|offset| ParsedItem(input, offset))
                    .map_err(TryFromParsed::ComponentRange)
                )
            }
        };

        if !input.is_empty() {
            return Err(error::Parse::ParseFromDescription(
                error::ParseFromDescription::UnexpectedTrailingCharacters,
            ));
        }

        // The RFC explicitly permits leap seconds. We don't currently support them, so treat it as
        // the preceding nanosecond. However, leap seconds can only occur as the last second of the
        // month UTC.
        let leap_second_input = if second == 60 {
            second = 59;
            nanosecond = 999_999_999;
            true
        } else {
            false
        };

        let date = try_likely_ok!(
            Month::from_number(month)
                .and_then(|month| Date::from_calendar_date(year.cast_signed().extend(), month, day))
                .map_err(TryFromParsed::ComponentRange)
        );
        let time = try_likely_ok!(
            Time::from_hms_nano(hour, minute, second, nanosecond)
                .map_err(TryFromParsed::ComponentRange)
        );
        let dt = OffsetDateTime::new_in_offset(date, time, offset);

        if leap_second_input && !dt.is_valid_leap_second_stand_in() {
            return Err(error::Parse::TryFromParsed(TryFromParsed::ComponentRange(
                error::ComponentRange::conditional("second"),
            )));
        }

        Ok(dt)
    }
}

impl<const CONFIG: EncodedConfig> sealed::Sealed for Iso8601<CONFIG> {
    #[inline]
    fn parse_into<'a>(
        &self,
        mut input: &'a [u8],
        parsed: &mut Parsed,
    ) -> Result<&'a [u8], error::Parse> {
        use crate::parsing::combinator::rfc::iso8601::ExtendedKind;

        let mut extended_kind = ExtendedKind::Unknown;
        let mut date_is_present = false;
        let mut time_is_present = false;
        let mut offset_is_present = false;
        let mut first_error = None;

        parsed.leap_second_allowed = true;

        match Self::parse_date(parsed, &mut extended_kind)(input) {
            Ok(new_input) => {
                input = new_input;
                date_is_present = true;
            }
            Err(err) => {
                first_error.get_or_insert(err);
            }
        }

        match Self::parse_time(parsed, &mut extended_kind, date_is_present)(input) {
            Ok(new_input) => {
                input = new_input;
                time_is_present = true;
            }
            Err(err) => {
                first_error.get_or_insert(err);
            }
        }

        // If a date and offset are present, a time must be as well.
        if !date_is_present || time_is_present {
            match Self::parse_offset(parsed, &mut extended_kind)(input) {
                Ok(new_input) => {
                    input = new_input;
                    offset_is_present = true;
                }
                Err(err) => {
                    first_error.get_or_insert(err);
                }
            }
        }

        if !date_is_present && !time_is_present && !offset_is_present {
            match first_error {
                Some(err) => return Err(err),
                None => bug!("an error should be present if no components were parsed"),
            }
        }

        Ok(input)
    }
}
