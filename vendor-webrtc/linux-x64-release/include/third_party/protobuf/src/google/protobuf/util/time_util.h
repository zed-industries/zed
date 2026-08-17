// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Defines utilities for the Timestamp and Duration well known types.

#ifndef GOOGLE_PROTOBUF_UTIL_TIME_UTIL_H__
#define GOOGLE_PROTOBUF_UTIL_TIME_UTIL_H__

#include <cstdint>
#include <ctime>
#include <ostream>
#include <string>
#ifdef _MSC_VER
#ifdef _XBOX_ONE
struct timeval {
  int64_t tv_sec;  /* seconds */
  int64_t tv_usec; /* and microseconds */
};
#else
#include <winsock2.h>
#endif  // _XBOX_ONE
#else
#include <sys/time.h>
#endif

#include "google/protobuf/duration.pb.h"
#include "google/protobuf/timestamp.pb.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace util {

// Utility functions for Timestamp and Duration.
class PROTOBUF_EXPORT TimeUtil {
  typedef google::protobuf::Timestamp Timestamp;
  typedef google::protobuf::Duration Duration;

 public:
  // The min/max Timestamp/Duration values we support.
  //
  // For "0001-01-01T00:00:00Z".
  static constexpr int64_t kTimestampMinSeconds = -62135596800LL;
  // For "9999-12-31T23:59:59.999999999Z".
  static constexpr int64_t kTimestampMaxSeconds = 253402300799LL;
  static constexpr int32_t kTimestampMinNanoseconds = 0;
  static constexpr int32_t kTimestampMaxNanoseconds = 999999999;
  static constexpr int64_t kDurationMinSeconds = -315576000000LL;
  static constexpr int64_t kDurationMaxSeconds = 315576000000LL;
  static constexpr int32_t kDurationMinNanoseconds = -999999999;
  static constexpr int32_t kDurationMaxNanoseconds = 999999999;

  static bool IsTimestampValid(const Timestamp& timestamp) {
    return timestamp.seconds() <= kTimestampMaxSeconds &&
           timestamp.seconds() >= kTimestampMinSeconds &&
           timestamp.nanos() <= kTimestampMaxNanoseconds &&
           timestamp.nanos() >= kTimestampMinNanoseconds;
  }

  static bool IsDurationValid(const Duration& duration) {
    return duration.seconds() <= kDurationMaxSeconds &&
           duration.seconds() >= kDurationMinSeconds &&
           duration.nanos() <= kDurationMaxNanoseconds &&
           duration.nanos() >= kDurationMinNanoseconds &&
           !(duration.seconds() >= 1 && duration.nanos() < 0) &&
           !(duration.seconds() <= -1 && duration.nanos() > 0);
  }

  // Converts Timestamp to/from RFC 3339 date string format.
  // Generated output will always be Z-normalized and uses 3, 6 or 9
  // fractional digits as required to represent the exact time. When
  // parsing, any fractional digits (or none) and any offset are
  // accepted as long as they fit into nano-seconds precision.
  // Note that Timestamp can only represent time from
  // 0001-01-01T00:00:00Z to 9999-12-31T23:59:59.999999999Z. Converting
  // a Timestamp outside of this range is undefined behavior.
  // See https://www.ietf.org/rfc/rfc3339.txt
  //
  // Example of generated format:
  //   "1972-01-01T10:00:20.021Z"
  //
  // Example of accepted format:
  //   "1972-01-01T10:00:20.021-05:00"
  static std::string ToString(const Timestamp& timestamp);
  static bool FromString(absl::string_view value, Timestamp* timestamp);

  // Converts Duration to/from string format. The string format will contains
  // 3, 6, or 9 fractional digits depending on the precision required to
  // represent the exact Duration value. For example:
  //   "1s", "1.010s", "1.000000100s", "-3.100s"
  // The range that can be represented by Duration is from -315,576,000,000
  // to +315,576,000,000 inclusive (in seconds).
  static std::string ToString(const Duration& duration);
  static bool FromString(absl::string_view value, Duration* duration);

  // Gets the current UTC time.
  static Timestamp GetCurrentTime();
  // Returns the Time representing "1970-01-01 00:00:00".
  static Timestamp GetEpoch();

  // Converts between Duration and integer types. The behavior is undefined if
  // the input value is not in the valid range of Duration.
  static Duration NanosecondsToDuration(int64_t nanos);
  static Duration MicrosecondsToDuration(int64_t micros);
  static Duration MillisecondsToDuration(int64_t millis);
  static Duration SecondsToDuration(int64_t seconds);
  static Duration MinutesToDuration(int64_t minutes);
  static Duration HoursToDuration(int64_t hours);
  // Result will be truncated towards zero. For example, "-1.5s" will be
  // truncated to "-1s", and "1.5s" to "1s" when converting to seconds.
  // It's undefined behavior if the input duration is not valid or the result
  // exceeds the range of int64. A duration is not valid if it's not in the
  // valid range of Duration, or have an invalid nanos value (i.e., larger
  // than 999999999, less than -999999999, or have a different sign from the
  // seconds part).
  static int64_t DurationToNanoseconds(const Duration& duration);
  static int64_t DurationToMicroseconds(const Duration& duration);
  static int64_t DurationToMilliseconds(const Duration& duration);
  static int64_t DurationToSeconds(const Duration& duration);
  static int64_t DurationToMinutes(const Duration& duration);
  static int64_t DurationToHours(const Duration& duration);
  // Creates Timestamp from integer types. The integer value indicates the
  // time elapsed from Epoch time. The behavior is undefined if the input
  // value is not in the valid range of Timestamp.
  static Timestamp NanosecondsToTimestamp(int64_t nanos);
  static Timestamp MicrosecondsToTimestamp(int64_t micros);
  static Timestamp MillisecondsToTimestamp(int64_t millis);
  static Timestamp SecondsToTimestamp(int64_t seconds);
  // Result will be truncated down to the nearest integer value. For example,
  // with "1969-12-31T23:59:59.9Z", TimestampToMilliseconds() returns -100
  // and TimestampToSeconds() returns -1. It's undefined behavior if the input
  // Timestamp is not valid (i.e., its seconds part or nanos part does not fall
  // in the valid range) or the return value doesn't fit into int64.
  static int64_t TimestampToNanoseconds(const Timestamp& timestamp);
  static int64_t TimestampToMicroseconds(const Timestamp& timestamp);
  static int64_t TimestampToMilliseconds(const Timestamp& timestamp);
  static int64_t TimestampToSeconds(const Timestamp& timestamp);

  // Conversion to/from other time/date types. Note that these types may
  // have a different precision and time range from Timestamp/Duration.
  // When converting to a lower precision type, the value will be truncated
  // to the nearest value that can be represented. If the value is
  // out of the range of the result type, the return value is undefined.
  //
  // Conversion to/from time_t
  static Timestamp TimeTToTimestamp(time_t value);
  static time_t TimestampToTimeT(const Timestamp& value);

  // Conversion to/from timeval
  static Timestamp TimevalToTimestamp(const timeval& value);
  static timeval TimestampToTimeval(const Timestamp& value);
  static Duration TimevalToDuration(const timeval& value);
  static timeval DurationToTimeval(const Duration& value);
};

}  // namespace util
}  // namespace protobuf
}  // namespace google

namespace google {
namespace protobuf {
// Overloaded operators for Duration.
//
// Assignment operators.
PROTOBUF_EXPORT Duration& operator+=(Duration& d1,
                                     const Duration& d2);  // NOLINT
PROTOBUF_EXPORT Duration& operator-=(Duration& d1,
                                     const Duration& d2);     // NOLINT
PROTOBUF_EXPORT Duration& operator*=(Duration& d, int64_t r);  // NOLINT
PROTOBUF_EXPORT Duration& operator*=(Duration& d, double r);  // NOLINT
PROTOBUF_EXPORT Duration& operator/=(Duration& d, int64_t r);  // NOLINT
PROTOBUF_EXPORT Duration& operator/=(Duration& d, double r);  // NOLINT
// Overload for other integer types.
template <typename T>
Duration& operator*=(Duration& d, T r) {  // NOLINT
  int64_t x = r;
  return d *= x;
}
template <typename T>
Duration& operator/=(Duration& d, T r) {  // NOLINT
  int64_t x = r;
  return d /= x;
}
PROTOBUF_EXPORT Duration& operator%=(Duration& d1,
                                     const Duration& d2);  // NOLINT
// Relational operators.
inline bool operator<(const Duration& d1, const Duration& d2) {
  if (d1.seconds() == d2.seconds()) {
    return d1.nanos() < d2.nanos();
  }
  return d1.seconds() < d2.seconds();
}
inline bool operator>(const Duration& d1, const Duration& d2) {
  return d2 < d1;
}
inline bool operator>=(const Duration& d1, const Duration& d2) {
  return !(d1 < d2);
}
inline bool operator<=(const Duration& d1, const Duration& d2) {
  return !(d2 < d1);
}
inline bool operator==(const Duration& d1, const Duration& d2) {
  return d1.seconds() == d2.seconds() && d1.nanos() == d2.nanos();
}
inline bool operator!=(const Duration& d1, const Duration& d2) {
  return !(d1 == d2);
}
// Additive operators
inline Duration operator-(const Duration& d) {
  Duration result;
  result.set_seconds(-d.seconds());
  result.set_nanos(-d.nanos());
  return result;
}
inline Duration operator+(const Duration& d1, const Duration& d2) {
  Duration result = d1;
  return result += d2;
}
inline Duration operator-(const Duration& d1, const Duration& d2) {
  Duration result = d1;
  return result -= d2;
}
// Multiplicative operators
template <typename T>
inline Duration operator*(Duration d, T r) {
  return d *= r;
}
template <typename T>
inline Duration operator*(T r, Duration d) {
  return d *= r;
}
template <typename T>
inline Duration operator/(Duration d, T r) {
  return d /= r;
}
PROTOBUF_EXPORT int64_t operator/(const Duration& d1, const Duration& d2);

inline Duration operator%(const Duration& d1, const Duration& d2) {
  Duration result = d1;
  return result %= d2;
}

inline std::ostream& operator<<(std::ostream& out, const Duration& d) {
  out << google::protobuf::util::TimeUtil::ToString(d);
  return out;
}

// Overloaded operators for Timestamp
//
// Assignment operators.
PROTOBUF_EXPORT Timestamp& operator+=(Timestamp& t,
                                      const Duration& d);  // NOLINT
PROTOBUF_EXPORT Timestamp& operator-=(Timestamp& t,
                                      const Duration& d);  // NOLINT
// Relational operators.
inline bool operator<(const Timestamp& t1, const Timestamp& t2) {
  if (t1.seconds() == t2.seconds()) {
    return t1.nanos() < t2.nanos();
  }
  return t1.seconds() < t2.seconds();
}
inline bool operator>(const Timestamp& t1, const Timestamp& t2) {
  return t2 < t1;
}
inline bool operator>=(const Timestamp& t1, const Timestamp& t2) {
  return !(t1 < t2);
}
inline bool operator<=(const Timestamp& t1, const Timestamp& t2) {
  return !(t2 < t1);
}
inline bool operator==(const Timestamp& t1, const Timestamp& t2) {
  return t1.seconds() == t2.seconds() && t1.nanos() == t2.nanos();
}
inline bool operator!=(const Timestamp& t1, const Timestamp& t2) {
  return !(t1 == t2);
}
// Additive operators.
inline Timestamp operator+(const Timestamp& t, const Duration& d) {
  Timestamp result = t;
  return result += d;
}
inline Timestamp operator+(const Duration& d, const Timestamp& t) {
  Timestamp result = t;
  return result += d;
}
inline Timestamp operator-(const Timestamp& t, const Duration& d) {
  Timestamp result = t;
  return result -= d;
}
PROTOBUF_EXPORT Duration operator-(const Timestamp& t1, const Timestamp& t2);

inline std::ostream& operator<<(std::ostream& out, const Timestamp& t) {
  out << google::protobuf::util::TimeUtil::ToString(t);
  return out;
}

}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_UTIL_TIME_UTIL_H__
