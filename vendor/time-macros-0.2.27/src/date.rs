use std::iter::Peekable;

use num_conv::Truncate;
use proc_macro::token_stream;
use time_core::util::{days_in_year, weeks_in_year};

use crate::Error;
use crate::helpers::{consume_number, consume_punct, days_in_year_month, ymd_to_yo, ywd_to_yo};
use crate::to_tokens::ToTokenStream;

#[cfg(feature = "large-dates")]
const MAX_YEAR: i32 = 999_999;
#[cfg(not(feature = "large-dates"))]
const MAX_YEAR: i32 = 9_999;

pub(crate) struct Date {
    pub(crate) year: i32,
    pub(crate) ordinal: u16,
}

pub(crate) fn parse(chars: &mut Peekable<token_stream::IntoIter>) -> Result<Date, Error> {
    let (year_sign_span, year_sign, explicit_sign) = if let Ok(span) = consume_punct('-', chars) {
        (Some(span), -1, true)
    } else if let Ok(span) = consume_punct('+', chars) {
        (Some(span), 1, true)
    } else {
        (None, 1, false)
    };
    let (year_span, mut year) = consume_number::<i32>("year", chars)?;
    year *= year_sign;
    if year.abs() > MAX_YEAR {
        return Err(Error::InvalidComponent {
            name: "year",
            value: year.to_string(),
            span_start: Some(year_sign_span.unwrap_or_else(|| year_span.start())),
            span_end: Some(year_span.end()),
        });
    }
    if !explicit_sign && year.abs() >= 10_000 {
        return Err(Error::Custom {
            message: "years with more than four digits must have an explicit sign".into(),
            span_start: Some(year_sign_span.unwrap_or_else(|| year_span.start())),
            span_end: Some(year_span.end()),
        });
    }

    consume_punct('-', chars)?;

    // year-week-day
    if let Some(proc_macro::TokenTree::Ident(ident)) = chars.peek()
        && let s = ident.to_string()
        && s.starts_with('W')
    {
        let w_span = ident.span();
        drop(chars.next()); // consume 'W' and possibly the week number

        let (week_span, week, day_span, day);

        if s.len() == 1 {
            (week_span, week) = consume_number::<u8>("week", chars)?;
            consume_punct('-', chars)?;
            (day_span, day) = consume_number::<u8>("day", chars)?;
        } else {
            let presumptive_week = &s[1..];
            if presumptive_week.bytes().all(|d| d.is_ascii_digit())
                && let Ok(week_number) = presumptive_week.replace('_', "").parse()
            {
                (week_span, week) = (w_span, week_number);
                consume_punct('-', chars)?;
                (day_span, day) = consume_number::<u8>("day", chars)?;
            } else {
                return Err(Error::InvalidComponent {
                    name: "week",
                    value: presumptive_week.to_string(),
                    span_start: Some(w_span.start()),
                    span_end: Some(w_span.end()),
                });
            }
        };

        if week > weeks_in_year(year) {
            return Err(Error::InvalidComponent {
                name: "week",
                value: week.to_string(),
                span_start: Some(w_span.start()),
                span_end: Some(week_span.end()),
            });
        }
        if day == 0 || day > 7 {
            return Err(Error::InvalidComponent {
                name: "day",
                value: day.to_string(),
                span_start: Some(day_span.start()),
                span_end: Some(day_span.end()),
            });
        }

        let (year, ordinal) = ywd_to_yo(year, week, day);

        return Ok(Date { year, ordinal });
    }

    // We don't yet know whether it's year-month-day or year-ordinal.
    let (month_or_ordinal_span, month_or_ordinal) =
        consume_number::<u16>("month or ordinal", chars)?;

    // year-month-day
    if consume_punct('-', chars).is_ok() {
        let (month_span, month) = (month_or_ordinal_span, month_or_ordinal);
        let (day_span, day) = consume_number::<u8>("day", chars)?;

        if month == 0 || month > 12 {
            return Err(Error::InvalidComponent {
                name: "month",
                value: month.to_string(),
                span_start: Some(month_span.start()),
                span_end: Some(month_span.end()),
            });
        }
        let month = month.truncate();
        if day == 0 || day > days_in_year_month(year, month) {
            return Err(Error::InvalidComponent {
                name: "day",
                value: day.to_string(),
                span_start: Some(day_span.start()),
                span_end: Some(day_span.end()),
            });
        }

        let (year, ordinal) = ymd_to_yo(year, month, day);

        Ok(Date { year, ordinal })
    }
    // year-ordinal
    else {
        let (ordinal_span, ordinal) = (month_or_ordinal_span, month_or_ordinal);

        if ordinal == 0 || ordinal > days_in_year(year) {
            return Err(Error::InvalidComponent {
                name: "ordinal",
                value: ordinal.to_string(),
                span_start: Some(ordinal_span.start()),
                span_end: Some(ordinal_span.end()),
            });
        }

        Ok(Date { year, ordinal })
    }
}

impl ToTokenStream for Date {
    fn append_to(self, ts: &mut proc_macro::TokenStream) {
        quote_append! { ts
            unsafe {
                ::time::Date::__from_ordinal_date_unchecked(
                    #(self.year),
                    #(self.ordinal),
                )
            }
        }
    }
}
