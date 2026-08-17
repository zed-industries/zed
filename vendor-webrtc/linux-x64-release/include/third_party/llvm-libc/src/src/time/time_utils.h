//===-- Collection of utils for mktime and friends --------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_TIME_TIME_UTILS_H
#define LLVM_LIBC_SRC_TIME_TIME_UTILS_H

#include "hdr/types/size_t.h"
#include "hdr/types/struct_tm.h"
#include "hdr/types/time_t.h"
#include "src/__support/CPP/optional.h"
#include "src/__support/CPP/string_view.h"
#include "src/__support/common.h"
#include "src/__support/macros/config.h"
#include "src/errno/libc_errno.h"
#include "time_constants.h"

#include <stdint.h>

namespace LIBC_NAMESPACE_DECL {
namespace time_utils {

// calculates the seconds from the epoch for tm_in. Does not update the struct,
// you must call update_from_seconds for that.
cpp::optional<time_t> mktime_internal(const tm *tm_out);

// Update the "tm" structure's year, month, etc. members from seconds.
// "total_seconds" is the number of seconds since January 1st, 1970.
int64_t update_from_seconds(time_t total_seconds, tm *tm);

// TODO(michaelrj): move these functions to use ErrorOr instead of setting
// errno. They always accompany a specific return value so we only need the one
// variable.

// POSIX.1-2017 requires this.
LIBC_INLINE time_t out_of_range() {
#ifdef EOVERFLOW
  // For non-POSIX uses of the standard C time functions, where EOVERFLOW is
  // not defined, it's OK not to set errno at all. The plain C standard doesn't
  // require it.
  libc_errno = EOVERFLOW;
#endif
  return time_constants::OUT_OF_RANGE_RETURN_VALUE;
}

LIBC_INLINE void invalid_value() { libc_errno = EINVAL; }

LIBC_INLINE char *asctime(const tm *timeptr, char *buffer,
                          size_t bufferLength) {
  if (timeptr == nullptr || buffer == nullptr) {
    invalid_value();
    return nullptr;
  }
  if (timeptr->tm_wday < 0 ||
      timeptr->tm_wday > (time_constants::DAYS_PER_WEEK - 1)) {
    invalid_value();
    return nullptr;
  }
  if (timeptr->tm_mon < 0 ||
      timeptr->tm_mon > (time_constants::MONTHS_PER_YEAR - 1)) {
    invalid_value();
    return nullptr;
  }

  // TODO(michaelr): move this to use the strftime machinery
  // equivalent to strftime(buffer, bufferLength, "%a %b %T %Y\n", timeptr)
  int written_size = __builtin_snprintf(
      buffer, bufferLength, "%.3s %.3s%3d %.2d:%.2d:%.2d %d\n",
      time_constants::WEEK_DAY_NAMES[timeptr->tm_wday].data(),
      time_constants::MONTH_NAMES[timeptr->tm_mon].data(), timeptr->tm_mday,
      timeptr->tm_hour, timeptr->tm_min, timeptr->tm_sec,
      time_constants::TIME_YEAR_BASE + timeptr->tm_year);
  if (written_size < 0)
    return nullptr;
  if (static_cast<size_t>(written_size) >= bufferLength) {
    out_of_range();
    return nullptr;
  }
  return buffer;
}

LIBC_INLINE tm *gmtime_internal(const time_t *timer, tm *result) {
  time_t seconds = *timer;
  // Update the tm structure's year, month, day, etc. from seconds.
  if (update_from_seconds(seconds, result) < 0) {
    out_of_range();
    return nullptr;
  }

  return result;
}

// TODO: localtime is not yet implemented and a temporary solution is to
//       use gmtime, https://github.com/llvm/llvm-project/issues/107597
LIBC_INLINE tm *localtime(const time_t *t_ptr) {
  static tm result;
  return time_utils::gmtime_internal(t_ptr, &result);
}

// Returns number of years from (1, year).
LIBC_INLINE constexpr int64_t get_num_of_leap_years_before(int64_t year) {
  return (year / 4) - (year / 100) + (year / 400);
}

// Returns True if year is a leap year.
LIBC_INLINE constexpr bool is_leap_year(const int64_t year) {
  return (((year) % 4) == 0 && (((year) % 100) != 0 || ((year) % 400) == 0));
}

LIBC_INLINE constexpr int get_days_in_year(const int year) {
  return is_leap_year(year) ? time_constants::DAYS_PER_LEAP_YEAR
                            : time_constants::DAYS_PER_NON_LEAP_YEAR;
}

// This is a helper class that takes a struct tm and lets you inspect its
// values. Where relevant, results are bounds checked and returned as optionals.
// This class does not, however, do data normalization except where necessary.
// It will faithfully return a date of 9999-99-99, even though that makes no
// sense.
class TMReader final {
  const tm *timeptr;

  template <size_t N>
  LIBC_INLINE constexpr cpp::optional<cpp::string_view>
  bounds_check(const cpp::array<cpp::string_view, N> &arr, int index) const {
    if (index >= 0 && index < static_cast<int>(arr.size()))
      return arr[index];
    return cpp::nullopt;
  }

public:
  LIBC_INLINE constexpr explicit TMReader(const tm *tmptr) : timeptr(tmptr) {}

  // Strings
  LIBC_INLINE constexpr cpp::optional<cpp::string_view>
  get_weekday_short_name() const {
    return bounds_check(time_constants::WEEK_DAY_NAMES, timeptr->tm_wday);
  }

  LIBC_INLINE constexpr cpp::optional<cpp::string_view>
  get_weekday_full_name() const {
    return bounds_check(time_constants::WEEK_DAY_FULL_NAMES, timeptr->tm_wday);
  }

  LIBC_INLINE constexpr cpp::optional<cpp::string_view>
  get_month_short_name() const {
    return bounds_check(time_constants::MONTH_NAMES, timeptr->tm_mon);
  }

  LIBC_INLINE constexpr cpp::optional<cpp::string_view>
  get_month_full_name() const {
    return bounds_check(time_constants::MONTH_FULL_NAMES, timeptr->tm_mon);
  }

  LIBC_INLINE constexpr cpp::string_view get_am_pm() const {
    if (timeptr->tm_hour < 12)
      return "AM";
    return "PM";
  }

  LIBC_INLINE constexpr cpp::string_view get_timezone_name() const {
    // TODO: timezone support
    return "UTC";
  }

  // Numbers
  LIBC_INLINE constexpr int get_sec() const { return timeptr->tm_sec; }
  LIBC_INLINE constexpr int get_min() const { return timeptr->tm_min; }
  LIBC_INLINE constexpr int get_hour() const { return timeptr->tm_hour; }
  LIBC_INLINE constexpr int get_mday() const { return timeptr->tm_mday; }
  LIBC_INLINE constexpr int get_mon() const { return timeptr->tm_mon; }
  LIBC_INLINE constexpr int get_yday() const { return timeptr->tm_yday; }
  LIBC_INLINE constexpr int get_wday() const { return timeptr->tm_wday; }
  LIBC_INLINE constexpr int get_isdst() const { return timeptr->tm_isdst; }

  // returns the year, counting from 1900
  LIBC_INLINE constexpr int get_year_raw() const { return timeptr->tm_year; }
  // returns the year, counting from 0
  LIBC_INLINE constexpr int get_year() const {
    return timeptr->tm_year + time_constants::TIME_YEAR_BASE;
  }

  LIBC_INLINE constexpr int is_leap_year() const {
    return time_utils::is_leap_year(get_year());
  }

  LIBC_INLINE constexpr int get_iso_wday() const {
    using time_constants::DAYS_PER_WEEK;
    using time_constants::MONDAY;
    // ISO uses a week that starts on Monday, but struct tm starts its week on
    // Sunday. This function normalizes the weekday so that it always returns a
    // value 0-6
    const int NORMALIZED_WDAY = timeptr->tm_wday % DAYS_PER_WEEK;
    return (NORMALIZED_WDAY + (DAYS_PER_WEEK - MONDAY)) % DAYS_PER_WEEK;
  }

  // returns the week of the current year, with weeks starting on start_day.
  LIBC_INLINE constexpr int get_week(time_constants::WeekDay start_day) const {
    using time_constants::DAYS_PER_WEEK;
    // The most recent start_day. The rest of the days into the current week
    // don't count, so ignore them.
    // Also add 7 to handle start_day > tm_wday
    const int start_of_cur_week =
        timeptr->tm_yday -
        ((timeptr->tm_wday + DAYS_PER_WEEK - start_day) % DAYS_PER_WEEK);

    // The original formula is ceil((start_of_cur_week + 1) / DAYS_PER_WEEK)
    // That becomes (start_of_cur_week + 1 + DAYS_PER_WEEK - 1) / DAYS_PER_WEEK)
    // Which simplifies to (start_of_cur_week + DAYS_PER_WEEK) / DAYS_PER_WEEK
    const int ceil_weeks_since_start =
        (start_of_cur_week + DAYS_PER_WEEK) / DAYS_PER_WEEK;

    return ceil_weeks_since_start;
  }

  LIBC_INLINE constexpr int get_iso_week() const {
    using time_constants::DAYS_PER_WEEK;
    using time_constants::ISO_FIRST_DAY_OF_YEAR;
    using time_constants::MONDAY;
    using time_constants::WeekDay;
    using time_constants::WEEKS_PER_YEAR;

    constexpr WeekDay START_DAY = MONDAY;

    // The most recent start_day. The rest of the days into the current week
    // don't count, so ignore them.
    // Also add 7 to handle start_day > tm_wday
    const int start_of_cur_week =
        timeptr->tm_yday -
        ((timeptr->tm_wday + DAYS_PER_WEEK - START_DAY) % DAYS_PER_WEEK);

    // if the week starts in the previous year, and also if the 4th of this year
    // is not in this week.
    if (start_of_cur_week < -3) {
      const int days_into_prev_year =
          get_days_in_year(get_year() - 1) + start_of_cur_week;
      // Each year has at least 52 weeks, but a year's last week will be 53 if
      // its first week starts in the previous year and its last week ends
      // in the next year. We know get_year() - 1 must extend into get_year(),
      // so here we check if it also extended into get_year() - 2 and add 1 week
      // if it does.
      return WEEKS_PER_YEAR +
             ((days_into_prev_year % DAYS_PER_WEEK) > ISO_FIRST_DAY_OF_YEAR);
    }

    // subtract 1 to account for yday being 0 indexed
    const int days_until_end_of_year =
        get_days_in_year(get_year()) - start_of_cur_week - 1;

    // if there are less than 3 days from the start of this week to the end of
    // the year, then there must be 4 days in this week in the next year, which
    // means that this week is the first week of that year.
    if (days_until_end_of_year < 3)
      return 1;

    // else just calculate the current week like normal.
    const int ceil_weeks_since_start =
        (start_of_cur_week + DAYS_PER_WEEK) / DAYS_PER_WEEK;

    // add 1 if this year's first week starts in the previous year.
    const int WEEK_STARTS_IN_PREV_YEAR =
        ((start_of_cur_week + time_constants::DAYS_PER_WEEK) %
         time_constants::DAYS_PER_WEEK) > time_constants::ISO_FIRST_DAY_OF_YEAR;
    return ceil_weeks_since_start + WEEK_STARTS_IN_PREV_YEAR;
  }

  LIBC_INLINE constexpr int get_iso_year() const {
    const int BASE_YEAR = get_year();
    // The ISO year is the same as a standard year for all dates after the start
    // of the first week and before the last week. Since the first ISO week of a
    // year starts on the 4th, anything after that is in this year.
    if (timeptr->tm_yday >= time_constants::ISO_FIRST_DAY_OF_YEAR &&
        timeptr->tm_yday < time_constants::DAYS_PER_NON_LEAP_YEAR -
                               time_constants::DAYS_PER_WEEK)
      return BASE_YEAR;

    const int ISO_WDAY = get_iso_wday();
    // The first week of the ISO year is defined as the week containing the
    // 4th day of January.

    // first week
    if (timeptr->tm_yday < time_constants::ISO_FIRST_DAY_OF_YEAR) {
      /*
      If jan 4 is in this week, then we're in BASE_YEAR, else we're in the
      previous year. The formula's been rearranged so here's the derivation:

              +--------+-- days until jan 4
              |        |
       wday + (4 - yday) < 7
       |               |
       +---------------+-- weekday of jan 4

       rearranged to get all the constants on one side:

       wday - yday < 7 - 4
      */
      const int IS_CUR_YEAR = (ISO_WDAY - timeptr->tm_yday <
                               time_constants::DAYS_PER_WEEK -
                                   time_constants::ISO_FIRST_DAY_OF_YEAR);
      return BASE_YEAR - !IS_CUR_YEAR;
    }

    // last week
    const int DAYS_LEFT_IN_YEAR =
        get_days_in_year(get_year()) - timeptr->tm_yday;
    /*
    Similar to above, we're checking if jan 4 (of next year) is in this week. If
    it is, this is in the next year. Note that this also handles the case of
    yday > days in year gracefully.

           +------------------+-- days until jan 4 (of next year)
           |                  |
    wday + (4 + remaining days) < 7
    |                         |
    +-------------------------+-- weekday of jan 4

    rearranging we get:

    wday + remaining days < 7 - 4
    */
    const int IS_NEXT_YEAR =
        (ISO_WDAY + DAYS_LEFT_IN_YEAR <
         time_constants::DAYS_PER_WEEK - time_constants::ISO_FIRST_DAY_OF_YEAR);
    return BASE_YEAR + IS_NEXT_YEAR;
  }

  LIBC_INLINE time_t get_epoch() const {
    auto seconds = mktime_internal(timeptr);
    return seconds ? *seconds : time_utils::out_of_range();
  }

  // returns the timezone offset in microwave time:
  // return (hours * 100) + minutes;
  // This means that a shift of -4:30 is returned as -430, simplifying
  // conversion.
  LIBC_INLINE constexpr int get_timezone_offset() const {
    // TODO: timezone support
    return 0;
  }
};

} // namespace time_utils
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_TIME_TIME_UTILS_H
