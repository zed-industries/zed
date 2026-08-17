//===-- Collection of constants for time functions --------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_TIME_TIME_CONSTANTS_H
#define LLVM_LIBC_SRC_TIME_TIME_CONSTANTS_H

#include "hdr/types/time_t.h"
#include "src/__support/CPP/array.h"
#include "src/__support/CPP/string_view.h"
#include <stdint.h>

namespace LIBC_NAMESPACE_DECL {
namespace time_constants {

enum Month : int {
  JANUARY = 0,
  FEBRUARY,
  MARCH,
  APRIL,
  MAY,
  JUNE,
  JULY,
  AUGUST,
  SEPTEMBER,
  OCTOBER,
  NOVEMBER,
  DECEMBER
};

enum WeekDay : int {
  SUNDAY = 0,
  MONDAY,
  TUESDAY,
  WEDNESDAY,
  THURSDAY,
  FRIDAY,
  SATURDAY
};

constexpr int SECONDS_PER_MIN = 60;
constexpr int MINUTES_PER_HOUR = 60;
constexpr int HOURS_PER_DAY = 24;
constexpr int DAYS_PER_WEEK = 7;
constexpr int WEEKS_PER_YEAR = 52;
constexpr int MONTHS_PER_YEAR = 12;
constexpr int MAX_DAYS_PER_MONTH = 31;
constexpr int DAYS_PER_NON_LEAP_YEAR = 365;
constexpr int DAYS_PER_LEAP_YEAR = 366;

constexpr int LAST_DAY_OF_NON_LEAP_YEAR = DAYS_PER_NON_LEAP_YEAR - 1;
constexpr int LAST_DAY_OF_LEAP_YEAR = DAYS_PER_LEAP_YEAR - 1;

constexpr int SECONDS_PER_HOUR = SECONDS_PER_MIN * MINUTES_PER_HOUR;
constexpr int SECONDS_PER_DAY = SECONDS_PER_HOUR * HOURS_PER_DAY;
constexpr int NUMBER_OF_SECONDS_IN_LEAP_YEAR =
    DAYS_PER_LEAP_YEAR * SECONDS_PER_DAY;

constexpr int TIME_YEAR_BASE = 1900;
constexpr int EPOCH_YEAR = 1970;
constexpr int EPOCH_WEEK_DAY = 4;

constexpr int ISO_FIRST_DAY_OF_YEAR = 3; // the 4th day of the year, 0-indexed.

// For asctime the behavior is undefined if struct tm's tm_wday or tm_mon are
// not within the normal ranges as defined in <time.h>, or if struct tm's
// tm_year exceeds {INT_MAX}-1990, or if the below asctime_internal algorithm
// would attempt to generate more than 26 bytes of output (including the
// terminating null).
constexpr int ASCTIME_BUFFER_SIZE = 256;
constexpr int ASCTIME_MAX_BYTES = 26;

/* 2000-03-01 (mod 400 year, immediately after feb29 */
constexpr int64_t SECONDS_UNTIL2000_MARCH_FIRST =
    (946684800LL + SECONDS_PER_DAY * (31 + 29));
constexpr int WEEK_DAY_OF2000_MARCH_FIRST = 3;

constexpr int DAYS_PER400_YEARS =
    (DAYS_PER_NON_LEAP_YEAR * 400) + (400 / 4) - 3;
constexpr int DAYS_PER100_YEARS =
    (DAYS_PER_NON_LEAP_YEAR * 100) + (100 / 4) - 1;
constexpr int DAYS_PER4_YEARS = (DAYS_PER_NON_LEAP_YEAR * 4) + 1;

// The latest time that can be represented in this form is 03:14:07 UTC on
// Tuesday, 19 January 2038 (corresponding to 2,147,483,647 seconds since the
// start of the epoch). This means that systems using a 32-bit time_t type are
// susceptible to the Year 2038 problem.
constexpr int END_OF32_BIT_EPOCH_YEAR = 2038;

constexpr time_t OUT_OF_RANGE_RETURN_VALUE = -1;

constexpr cpp::array<cpp::string_view, DAYS_PER_WEEK> WEEK_DAY_NAMES = {
    "Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"};

constexpr cpp::array<cpp::string_view, DAYS_PER_WEEK> WEEK_DAY_FULL_NAMES = {
    "Sunday",   "Monday", "Tuesday", "Wednesday",
    "Thursday", "Friday", "Saturday"};

constexpr cpp::array<cpp::string_view, MONTHS_PER_YEAR> MONTH_NAMES = {
    "Jan", "Feb", "Mar", "Apr", "May", "Jun",
    "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"};

constexpr cpp::array<cpp::string_view, MONTHS_PER_YEAR> MONTH_FULL_NAMES = {
    "January", "February", "March",     "April",   "May",      "June",
    "July",    "August",   "September", "October", "November", "December"};

constexpr int NON_LEAP_YEAR_DAYS_IN_MONTH[] = {31, 28, 31, 30, 31, 30,
                                               31, 31, 30, 31, 30, 31};

} // namespace time_constants
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_TIME_TIME_CONSTANTS_H
