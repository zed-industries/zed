/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_DATE_COMPONENTS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_DATE_COMPONENTS_H_

#include <limits>
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

// A DateComponents instance represents one of the following date and time
// combinations:
// * kMonth type: year-month
// * kDate type: year-month-day
// * kWeek type: year-week
// * kTime type: hour-minute-second-millisecond
// * kDateTime or kDateTimeLocal type:
//   year-month-day hour-minute-second-millisecond
class PLATFORM_EXPORT DateComponents {
  DISALLOW_NEW();

 public:
  DateComponents()
      : millisecond_(0),
        second_(0),
        minute_(0),
        hour_(0),
        month_day_(0),
        month_(0),
        year_(0),
        week_(0),
        type_(kInvalid) {}

  enum Type {
    kInvalid,
    kDate,
    kDateTimeLocal,
    kMonth,
    kTime,
    kWeek,
  };

  int Millisecond() const { return millisecond_; }
  int Second() const { return second_; }
  int Minute() const { return minute_; }
  int Hour() const { return hour_; }
  int MonthDay() const { return month_day_; }
  int WeekDay() const;
  int Month() const { return month_; }
  int FullYear() const { return year_; }
  int Week() const { return week_; }
  Type GetType() const { return type_; }

  enum class SecondFormat {
    kNone,  // Suppress the second part and the millisecond part if they are 0.
    kSecond,  // Always show the second part, and suppress the millisecond part
              // if it is 0.
    kMillisecond  // Always show the second part and the millisecond part.
  };

  // Returns an ISO 8601 representation for this instance.
  // The format argument is valid for kDateTime, kDateTimeLocal, and
  // kTime types.
  WTF::String ToString(SecondFormat format = SecondFormat::kNone) const;

  // Parse*() and SetMillisecondsSince*() functions are initializers for an
  // DateComponents instance. If these functions return false, the instance
  // might be invalid.

  // The following five functions parse the input 'src', and updates some
  // fields of this instance. The parsing starts at src[start].
  // The functions return true if the parsing succeeds, and set 'end' to the
  // next index after the last consumed. Extra leading characters cause parse
  // failures, and the trailing extra characters don't cause parse failures.

  // Sets FullYear and Month.
  bool ParseMonth(const WTF::String& src, unsigned start, unsigned& end);
  // Sets FullYear, Month and MonthDay.
  bool ParseDate(const WTF::String& src, unsigned start, unsigned& end);
  // Sets FullYear and Week.
  bool ParseWeek(const WTF::String& src, unsigned start, unsigned& end);
  // Sets Hour, Minute, Second and Millisecond.
  bool ParseTime(const WTF::String& src, unsigned start, unsigned& end);
  // Sets FullYear, Month, MonthDay, Hour, Minute, Second and Millisecond.
  bool ParseDateTimeLocal(const WTF::String&, unsigned start, unsigned& end);

  // The following SetMillisecondsSinceEpochFor*() functions take
  // the number of milliseconds since 1970-01-01 00:00:00.000 UTC as
  // the argument, and update all fields for the corresponding
  // DateComponents type. The functions return true if it succeeds, and
  // false if they fail.

  // For kDate type. Updates FullYear, Month and MonthDay.
  bool SetMillisecondsSinceEpochForDate(double ms);
  // For kDateTimeLocal type. Updates FullYear, Month, MonthDay, Hour,
  // Minute, Second and Millisecond.
  bool SetMillisecondsSinceEpochForDateTimeLocal(double ms);
  // For kMonth type. Updates FullYear and Month.
  bool SetMillisecondsSinceEpochForMonth(double ms);
  // For kWeek type. Updates FullYear and Week.
  bool SetMillisecondsSinceEpochForWeek(double ms);

  // For kTime type. Updates Hour, Minute, Second and Millisecond.
  bool SetMillisecondsSinceMidnight(double ms);

  // Another initializer for kMonth type. Updates FullYear and Month.
  bool SetMonthsSinceEpoch(double months);
  // Another initializer for kWeek type. Updates FullYear and Week.
  bool SetWeek(int year, int week_number);

  // Returns the number of milliseconds from 1970-01-01 00:00:00 UTC.
  // For a DateComponents initialized with ParseDateTimeLocal(),
  // MillisecondsSinceEpoch() returns a value for UTC timezone.
  double MillisecondsSinceEpoch() const;
  // Returns the number of months from 1970-01.
  // Do not call this for types other than kMonth.
  double MonthsSinceEpoch() const;

  static inline double InvalidMilliseconds() {
    return std::numeric_limits<double>::quiet_NaN();
  }

  // Minimum and maxmimum limits for SetMillisecondsSince*(),
  // SetMonthsSinceEpoch(), MillisecondsSinceEpoch(), and MonthsSinceEpoch().

  // 0001-01-01T00:00Z
  static inline double MinimumDate() { return -62135596800000.0; }
  // Ditto.
  static inline double MinimumDateTime() { return -62135596800000.0; }
  // 0001-01
  static inline double MinimumMonth() { return (1 - 1970) * 12.0 + 1 - 1; }
  // 00:00:00.000
  static inline double MinimumTime() { return 0; }
  // 0001-01-01, the first Monday of 0001.
  static inline double MinimumWeek() { return -62135596800000.0; }
  // 275760-09-13T00:00Z
  static inline double MaximumDate() { return 8640000000000000.0; }
  // Ditto.
  static inline double MaximumDateTime() { return 8640000000000000.0; }
  // 275760-09
  static inline double MaximumMonth() { return (275760 - 1970) * 12.0 + 9 - 1; }
  // 23:59:59.999
  static inline double MaximumTime() { return 86399999; }
  // 275760-09-08, the Monday of the week including 275760-09-13.
  static inline double MaximumWeek() { return 8639999568000000.0; }

  // HTML5 uses ISO-8601 format with year >= 1. Gregorian calendar started in
  // 1582. However, we need to support 0001-01-01 in Gregorian calendar rule.
  static inline int MinimumYear() { return 1; }
  // Date in ECMAScript can't represent dates later than 275760-09-13T00:00Z.
  // So, we have the same upper limit in HTML5 date/time types.
  static inline int MaximumYear() { return 275760; }
  static const int kMinimumWeekNumber;
  static const int kMaximumWeekNumber;

 private:
  // Returns the maximum week number in this DateComponents's year.
  // The result is either of 52 and 53.
  int MaxWeekNumberInYear() const;
  bool ParseYear(const WTF::String&, unsigned start, unsigned& end);
  // Helper for MillisecondsSinceEpoch().
  double MillisecondsSinceEpochForTime() const;
  // Helpers for SetMillisecondsSinceEpochFor*().
  bool SetMillisecondsSinceEpochForDateInternal(double ms);
  void SetMillisecondsSinceMidnightInternal(double ms);
  // Helper for ToString().
  WTF::String ToStringForTime(SecondFormat) const;

  // m_weekDay values
  enum {
    kSunday = 0,
    kMonday,
    kTuesday,
    kWednesday,
    kThursday,
    kFriday,
    kSaturday,
  };

  int millisecond_;  // 0 - 999
  int second_;
  int minute_;
  int hour_;
  int month_day_;  // 1 - 31
  int month_;      // 0:January - 11:December
  int year_;       // 1 - 275760
  int week_;       // 1 - 53

  Type type_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_DATE_COMPONENTS_H_
