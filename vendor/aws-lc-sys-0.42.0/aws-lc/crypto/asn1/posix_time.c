// Copyright (c) 2022, Google Inc.
// SPDX-License-Identifier: ISC

// Time conversion to/from POSIX time_t and struct tm, with no support
// for time zones other than UTC

#include <assert.h>
#include <inttypes.h>
#include <limits.h>
#include <string.h>
#include <time.h>

#include "internal.h"

#define SECS_PER_HOUR (60 * 60)
#define SECS_PER_DAY (INT64_C(24) * SECS_PER_HOUR)


// Is a year/month/day combination valid, in the range from year 0000
// to 9999?
static int is_valid_date(int64_t year, int64_t month, int64_t day) {
  if (day < 1 || month < 1 || year < 0 || year > 9999) {
    return 0;
  }
  switch (month) {
    case 1:
    case 3:
    case 5:
    case 7:
    case 8:
    case 10:
    case 12:
      return day > 0 && day <= 31;
    case 4:
    case 6:
    case 9:
    case 11:
      return day > 0 && day <= 30;
    case 2:
      if ((year % 4 == 0 && year % 100 != 0) || year % 400 == 0) {
        return day > 0 && day <= 29;
      } else {
        return day > 0 && day <= 28;
      }
    default:
      return 0;
  }
}

// Is a time valid? Leap seconds of 60 are not considered valid, as
// the POSIX time in seconds does not include them.
static int is_valid_time(int64_t hours, int64_t minutes, int64_t seconds) {
  if (hours < 0 || minutes < 0 || seconds < 0 || hours > 23 || minutes > 59 ||
      seconds > 59) {
    return 0;
  }
  return 1;
}

// 0000-01-01 00:00:00 UTC
#define MIN_POSIX_TIME INT64_C(-62167219200)
// 9999-12-31 23:59:59 UTC
#define MAX_POSIX_TIME INT64_C(253402300799)

// Is an int64 time within our expected range?
static int is_valid_posix_time(int64_t time) {
  return MIN_POSIX_TIME <= time && time <= MAX_POSIX_TIME;
}

// Inspired by algorithms presented in
// https://howardhinnant.github.io/date_algorithms.html
// (Public Domain)
static int posix_time_from_utc(int64_t year, int64_t month, int64_t day,
                               int64_t hours, int64_t minutes, int64_t seconds,
                               int64_t *out_time) {
  if (!is_valid_date(year, month, day) ||
      !is_valid_time(hours, minutes, seconds)) {
    return 0;
  }
  if (month <= 2) {
    year--;  // Start years on Mar 1, so leap days always finish a year.
  }
  // At this point year will be in the range -1 and 9999.
  assert(-1 <= year && year <= 9999);
  int64_t era = (year >= 0 ? year : year - 399) / 400;
  int64_t year_of_era = year - era * 400;
  int64_t day_of_year =
      (153 * (month > 2 ? month - 3 : month + 9) + 2) / 5 + day - 1;
  int64_t day_of_era =
      year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
  int64_t posix_days = era * 146097 + day_of_era - 719468;
  *out_time = posix_days * SECS_PER_DAY + hours * SECS_PER_HOUR + minutes * 60 +
              seconds;
  return 1;
}

// Inspired by algorithms presented in
// https://howardhinnant.github.io/date_algorithms.html
// (Public Domain)
static int utc_from_posix_time(int64_t time, int *out_year, int *out_month,
                               int *out_day, int *out_hours, int *out_minutes,
                               int *out_seconds) {
  if (!is_valid_posix_time(time)) {
    return 0;
  }
  int64_t days = time / SECS_PER_DAY;
  int64_t leftover_seconds = time % SECS_PER_DAY;
  if (leftover_seconds < 0) {
    days--;
    leftover_seconds += SECS_PER_DAY;
  }
  days += 719468;  // Shift to starting epoch of Mar 1 0000.
  // At this point, days will be in the range -61 and 3652364.
  assert(-61 <= days && days <= 3652364);
  int64_t era = (days > 0 ? days : days - 146096) / 146097;
  int64_t day_of_era = days - era * 146097;
  int64_t year_of_era = (day_of_era - day_of_era / 1460 + day_of_era / 36524 -
                         day_of_era / 146096) /
                        365;
  *out_year = (int)(year_of_era + era * 400);  // Year starting on Mar 1.
  int64_t day_of_year =
      day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
  int64_t month_of_year = (5 * day_of_year + 2) / 153;
  *out_month =
      (int)(month_of_year < 10 ? month_of_year + 3 : month_of_year - 9);
  if (*out_month <= 2) {
    (*out_year)++;  // Adjust year back to Jan 1 start of year.
  }
  *out_day = (int)(day_of_year - (153 * month_of_year + 2) / 5 + 1);
  *out_hours = (int)(leftover_seconds / SECS_PER_HOUR);
  leftover_seconds %= SECS_PER_HOUR;
  *out_minutes = (int)(leftover_seconds / 60);
  *out_seconds = (int)(leftover_seconds % 60);
  return 1;
}

int OPENSSL_tm_to_posix(const struct tm *tm, int64_t *out) {
  return posix_time_from_utc(tm->tm_year + INT64_C(1900),
                             tm->tm_mon + INT64_C(1), tm->tm_mday, tm->tm_hour,
                             tm->tm_min, tm->tm_sec, out);
}

int OPENSSL_posix_to_tm(int64_t time, struct tm *out_tm) {
  if (out_tm == NULL) {
    OPENSSL_PUT_ERROR(ASN1, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }
  struct tm tmp_tm;
  OPENSSL_memset(&tmp_tm, 0, sizeof(tmp_tm));
  if (!utc_from_posix_time(time, &tmp_tm.tm_year, &tmp_tm.tm_mon,
                           &tmp_tm.tm_mday, &tmp_tm.tm_hour, &tmp_tm.tm_min,
                           &tmp_tm.tm_sec)) {
    return 0;
  }
  tmp_tm.tm_year -= 1900;
  tmp_tm.tm_mon -= 1;
  *out_tm = tmp_tm;

  return 1;
}

int OPENSSL_timegm(const struct tm *tm, time_t *out) {
  OPENSSL_STATIC_ASSERT(
      sizeof(time_t) == sizeof(int32_t) || sizeof(time_t) == sizeof(int64_t),
      time_t_is_broken);
  int64_t posix_time;
  if (!OPENSSL_tm_to_posix(tm, &posix_time)) {
    return 0;
  }
  if (sizeof(time_t) == sizeof(int32_t) &&
      (posix_time > INT32_MAX || posix_time < INT32_MIN)) {
    return 0;
  }
  *out = (time_t)posix_time;
  return 1;
}

struct tm *OPENSSL_gmtime(const time_t *time, struct tm *out_tm) {
  OPENSSL_STATIC_ASSERT(
      sizeof(time_t) == sizeof(int32_t) || sizeof(time_t) == sizeof(int64_t),
      time_t_is_broken);
  int64_t posix_time = *time;
  if (!OPENSSL_posix_to_tm(posix_time, out_tm)) {
    return NULL;
  }
  return out_tm;
}

int OPENSSL_gmtime_adj(struct tm *tm, int offset_day, int64_t offset_sec) {
  int64_t posix_time;
  if (!OPENSSL_tm_to_posix(tm, &posix_time)) {
    return 0;
  }
  OPENSSL_STATIC_ASSERT(INT_MAX <= INT64_MAX / SECS_PER_DAY,
                day_offset_in_seconds_cannot_overflow)
  OPENSSL_STATIC_ASSERT(MAX_POSIX_TIME <= INT64_MAX - INT_MAX * SECS_PER_DAY,
                addition_cannot_overflow)
  OPENSSL_STATIC_ASSERT(MIN_POSIX_TIME >= INT64_MIN - INT_MIN * SECS_PER_DAY,
                addition_cannot_underflow)
  posix_time += offset_day * SECS_PER_DAY;
  if (posix_time > 0 && offset_sec > INT64_MAX - posix_time) {
    return 0;
  }
  if (posix_time < 0 && offset_sec < INT64_MIN - posix_time) {
    return 0;
  }
  posix_time += offset_sec;

  if (!OPENSSL_posix_to_tm(posix_time, tm)) {
    return 0;
  }

  return 1;
}

int OPENSSL_gmtime_diff(int *out_days, int *out_secs, const struct tm *from,
                        const struct tm *to) {
  int64_t time_to, time_from;
  if (!OPENSSL_tm_to_posix(to, &time_to) ||
      !OPENSSL_tm_to_posix(from, &time_from)) {
    return 0;
  }
  // Times are in range, so these calculations can not overflow.
  OPENSSL_STATIC_ASSERT(SECS_PER_DAY <= INT_MAX, seconds_per_day_does_not_fit_in_int)
  OPENSSL_STATIC_ASSERT((MAX_POSIX_TIME - MIN_POSIX_TIME) / SECS_PER_DAY <= INT_MAX,
                range_of_valid_POSIX_times_in_days_does_not_fit_in_int)
  int64_t timediff = time_to - time_from;
  int64_t daydiff = timediff / SECS_PER_DAY;
  timediff %= SECS_PER_DAY;
  *out_secs = (int)timediff;
  *out_days = (int)daydiff;
  return 1;
}
