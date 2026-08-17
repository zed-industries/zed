// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Basic time formatting methods.  Most methods format based on the current
// locale. *TimeFormatWithPattern() are special; see comments there.

#ifndef BASE_I18N_TIME_FORMATTING_H_
#define BASE_I18N_TIME_FORMATTING_H_

#include <string>
#include <string_view>

#include "base/i18n/base_i18n_export.h"
#include "build/build_config.h"
#include "third_party/icu/source/common/unicode/uversion.h"

U_NAMESPACE_BEGIN
class TimeZone;
U_NAMESPACE_END

namespace base {

class Time;
class TimeDelta;

// Argument type used to specify the hour clock type.
enum HourClockType {
  k12HourClock,  // Uses 1-12. e.g., "3:07 PM"
  k24HourClock,  // Uses 0-23. e.g., "15:07"
};

// Argument type used to specify whether or not to include AM/PM sign.
enum AmPmClockType {
  kDropAmPm,  // Drops AM/PM sign. e.g., "3:07"
  kKeepAmPm,  // Keeps AM/PM sign. e.g., "3:07 PM"
};

// Should match UMeasureFormatWidth in measfmt.h; replicated here to avoid
// requiring third_party/icu dependencies with this file.
enum DurationFormatWidth {
  DURATION_WIDTH_WIDE,    // "3 hours, 7 minutes"
  DURATION_WIDTH_SHORT,   // "3 hr, 7 min"
  DURATION_WIDTH_NARROW,  // "3h 7m"
  DURATION_WIDTH_NUMERIC  // "3:07"
};

// Date formats from third_party/icu/source/i18n/unicode/udat.h. Add more as
// necessary.
enum DateFormat {
  // November 2007
  DATE_FORMAT_YEAR_MONTH,
  // Tuesday, 7 November
  DATE_FORMAT_MONTH_WEEKDAY_DAY,
};

// Returns the time of day, e.g., "3:07 PM".
BASE_I18N_EXPORT std::u16string TimeFormatTimeOfDay(const Time& time);

// Returns the time of day in 24-hour clock format with millisecond accuracy,
// e.g., "15:07:30.568"
BASE_I18N_EXPORT std::u16string TimeFormatTimeOfDayWithMilliseconds(
    const Time& time);

// Returns the time of day in the specified hour clock type. e.g.
// "3:07 PM" (type == k12HourClock, ampm == kKeepAmPm).
// "3:07"    (type == k12HourClock, ampm == kDropAmPm).
// "15:07"   (type == k24HourClock).
BASE_I18N_EXPORT std::u16string TimeFormatTimeOfDayWithHourClockType(
    const Time& time,
    HourClockType type,
    AmPmClockType ampm);

// Returns a shortened date, e.g. "Nov 7, 2007"
BASE_I18N_EXPORT std::u16string TimeFormatShortDate(const Time& time);

// Returns a numeric date such as 12/13/52.
BASE_I18N_EXPORT std::u16string TimeFormatShortDateNumeric(const Time& time);

// Returns a numeric date and time such as "12/13/52 2:44:30 PM".
BASE_I18N_EXPORT std::u16string TimeFormatShortDateAndTime(const Time& time);

#if BUILDFLAG(IS_CHROMEOS)
// Returns a month and year, e.g. "November 2007" for the specified time zone.
BASE_I18N_EXPORT std::u16string TimeFormatMonthAndYearForTimeZone(
    const Time& time,
    const icu::TimeZone* time_zone);
#endif  // BUILDFLAG(IS_CHROMEOS)

// Returns a month and year, e.g. "November 2007"
BASE_I18N_EXPORT std::u16string TimeFormatMonthAndYear(const Time& time);

// Returns a numeric date and time with time zone such as
// "12/13/52 2:44:30 PM PST".
BASE_I18N_EXPORT std::u16string TimeFormatShortDateAndTimeWithTimeZone(
    const Time& time);

// Formats a time in a friendly sentence format, e.g.
// "Monday, March 6, 2008 2:44:30 PM".
BASE_I18N_EXPORT std::u16string TimeFormatFriendlyDateAndTime(const Time& time);

// Formats a time in a friendly sentence format, e.g.
// "Monday, March 6, 2008".
BASE_I18N_EXPORT std::u16string TimeFormatFriendlyDate(const Time& time);

// Formats a time using a pattern to produce output for different locales when
// an unusual time format is needed, e.g. "Feb. 2, 18:00". See
// https://unicode-org.github.io/icu/userguide/format_parse/datetime/#datetime-format-syntax
// for pattern details.
//
// Use this version when you want to display the resulting string to the user.
//
// This localizes more than you might expect: it does not simply translate days
// of the week, etc., and then feed them into the provided pattern. The pattern
// will also be run through a pattern localizer that will add spacing,
// delimiters, etc. appropriate for the current locale. If you don't want this,
// look at `UnlocalizedTimeFormatWithPattern()` below. If you want translation
// but don't want to adjust the pattern as well, talk to base/ OWNERS about your
// use case.
BASE_I18N_EXPORT std::u16string LocalizedTimeFormatWithPattern(
    const Time& time,
    std::string_view pattern);

// Formats a time using a pattern to produce en-US-like output, e.g. "Feb. 2,
// 18:00". See
// https://unicode-org.github.io/icu/userguide/format_parse/datetime/#datetime-format-syntax
// for pattern details. NOTE: While ICU only supports millisecond precision
// (fractional second patterns "SSS..." will be filled with zeroes after the
// third 'S'), this supports microsecond precision (up to six 'S's may become
// non-zero values), since some callers need that.
//
// `time_zone` can be set to a desired time zone (e.g.
// icu::TimeZone::getGMT()); if left as null, the local time zone will be used.
//
// Use this version when you want to control the output format precisely, e.g.
// for logging or to format a string for consumption by some server.
//
// This always outputs in US English and does not change the provided pattern at
// all before formatting. It returns a `std::string` instead of a
// `std::u16string` under the assumption that it will not be used in UI.
BASE_I18N_EXPORT std::string UnlocalizedTimeFormatWithPattern(
    const Time& time,
    std::string_view pattern,
    const icu::TimeZone* time_zone = nullptr);

// Formats a time compliant to ISO 8601 in UTC, e.g. "2020-12-31T23:59:59.999Z".
BASE_I18N_EXPORT std::string TimeFormatAsIso8601(const Time& time);

// Formats a time in the IMF-fixdate format defined by RFC 7231 (satisfying its
// HTTP-date format), e.g. "Sun, 06 Nov 1994 08:49:37 GMT".
BASE_I18N_EXPORT std::string TimeFormatHTTP(const Time& time);

// Formats a time duration of hours and minutes into various formats, e.g.,
// "3:07" or "3 hours, 7 minutes", and returns true on success. See
// DurationFormatWidth for details.
[[nodiscard]] BASE_I18N_EXPORT bool TimeDurationFormat(
    TimeDelta time,
    DurationFormatWidth width,
    std::u16string* out);

// Formats a time duration of hours, minutes and seconds into various formats,
// e.g., "3:07:30" or "3 hours, 7 minutes, 30 seconds", and returns true on
// success. See DurationFormatWidth for details.
[[nodiscard]] BASE_I18N_EXPORT bool TimeDurationFormatWithSeconds(
    TimeDelta time,
    DurationFormatWidth width,
    std::u16string* out);

// Formats a time duration of hours, minutes and seconds into various formats,
// without the leading 0 time measurement units. e.g., "7m 30s" or
// "30 seconds", and returns true on success.
// Since the numeric format of time duration with the leading 0 omitted
// can produces ambiguous outputs such as "7:30", the "hh:mm:ss" format
// will always be used.
// See DurationFormatWidth for details.
[[nodiscard]] BASE_I18N_EXPORT bool TimeDurationCompactFormatWithSeconds(
    TimeDelta time,
    DurationFormatWidth width,
    std::u16string* out);

// Formats a date interval into various formats, e.g. "2 December - 4 December"
// or "March 2016 - December 2016". See DateFormat for details.
BASE_I18N_EXPORT std::u16string DateIntervalFormat(const Time& begin_time,
                                                   const Time& end_time,
                                                   DateFormat format);

// Gets the hour clock type of the current locale. e.g.
// k12HourClock (en-US).
// k24HourClock (en-GB).
BASE_I18N_EXPORT HourClockType GetHourClockType();

}  // namespace base

#endif  // BASE_I18N_TIME_FORMATTING_H_
