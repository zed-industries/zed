/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef SYSTEM_WRAPPERS_INCLUDE_NTP_TIME_H_
#define SYSTEM_WRAPPERS_INCLUDE_NTP_TIME_H_

#include <cmath>
#include <cstdint>
#include <limits>

#include "rtc_base/numerics/safe_conversions.h"

namespace webrtc {

class NtpTime {
 public:
  static constexpr uint64_t kFractionsPerSecond = 0x100000000;
  NtpTime() : value_(0) {}
  explicit NtpTime(uint64_t value) : value_(value) {}
  NtpTime(uint32_t seconds, uint32_t fractions)
      : value_(seconds * kFractionsPerSecond + fractions) {}

  NtpTime(const NtpTime&) = default;
  NtpTime& operator=(const NtpTime&) = default;
  explicit operator uint64_t() const { return value_; }

  void Set(uint32_t seconds, uint32_t fractions) {
    value_ = seconds * kFractionsPerSecond + fractions;
  }
  void Reset() { value_ = 0; }

  int64_t ToMs() const {
    static constexpr double kNtpFracPerMs = 4.294967296E6;  // 2^32 / 1000.
    const double frac_ms = static_cast<double>(fractions()) / kNtpFracPerMs;
    return 1000 * static_cast<int64_t>(seconds()) +
           static_cast<int64_t>(frac_ms + 0.5);
  }
  // NTP standard (RFC1305, section 3.1) explicitly state value 0 is invalid.
  bool Valid() const { return value_ != 0; }

  uint32_t seconds() const {
    return dchecked_cast<uint32_t>(value_ / kFractionsPerSecond);
  }
  uint32_t fractions() const {
    return dchecked_cast<uint32_t>(value_ % kFractionsPerSecond);
  }

 private:
  uint64_t value_;
};

inline bool operator==(const NtpTime& n1, const NtpTime& n2) {
  return static_cast<uint64_t>(n1) == static_cast<uint64_t>(n2);
}
inline bool operator!=(const NtpTime& n1, const NtpTime& n2) {
  return !(n1 == n2);
}

// Converts `int64_t` milliseconds to Q32.32-formatted fixed-point seconds.
// Performs clamping if the result overflows or underflows.
inline int64_t Int64MsToQ32x32(int64_t milliseconds) {
  // TODO(bugs.webrtc.org/10893): Change to use `webrtc::saturated_cast` once
  // the bug has been fixed.
  double result =
      std::round(milliseconds * (NtpTime::kFractionsPerSecond / 1000.0));

  // Explicitly cast values to double to avoid implicit conversion warnings
  // The conversion of the std::numeric_limits<int64_t>::max() triggers
  // -Wimplicit-int-float-conversion warning in clang 10.0.0 without explicit
  // cast
  if (result <= static_cast<double>(std::numeric_limits<int64_t>::min())) {
    return std::numeric_limits<int64_t>::min();
  }

  if (result >= static_cast<double>(std::numeric_limits<int64_t>::max())) {
    return std::numeric_limits<int64_t>::max();
  }

  return dchecked_cast<int64_t>(result);
}

// Converts `int64_t` milliseconds to UQ32.32-formatted fixed-point seconds.
// Performs clamping if the result overflows or underflows.
inline uint64_t Int64MsToUQ32x32(int64_t milliseconds) {
  // TODO(bugs.webrtc.org/10893): Change to use `webrtc::saturated_cast` once
  // the bug has been fixed.
  double result =
      std::round(milliseconds * (NtpTime::kFractionsPerSecond / 1000.0));

  // Explicitly cast values to double to avoid implicit conversion warnings
  // The conversion of the std::numeric_limits<int64_t>::max() triggers
  // -Wimplicit-int-float-conversion warning in clang 10.0.0 without explicit
  // cast
  if (result <= static_cast<double>(std::numeric_limits<uint64_t>::min())) {
    return std::numeric_limits<uint64_t>::min();
  }

  if (result >= static_cast<double>(std::numeric_limits<uint64_t>::max())) {
    return std::numeric_limits<uint64_t>::max();
  }

  return dchecked_cast<uint64_t>(result);
}

// Converts Q32.32-formatted fixed-point seconds to `int64_t` milliseconds.
inline int64_t Q32x32ToInt64Ms(int64_t q32x32) {
  return dchecked_cast<int64_t>(
      std::round(q32x32 * (1000.0 / NtpTime::kFractionsPerSecond)));
}

// Converts UQ32.32-formatted fixed-point seconds to `int64_t` milliseconds.
inline int64_t UQ32x32ToInt64Ms(uint64_t q32x32) {
  return dchecked_cast<int64_t>(
      std::round(q32x32 * (1000.0 / NtpTime::kFractionsPerSecond)));
}

// Converts UQ32.32-formatted fixed-point seconds to `int64_t` microseconds.
inline int64_t UQ32x32ToInt64Us(uint64_t q32x32) {
  return dchecked_cast<int64_t>(
      std::round(q32x32 * (1'000'000.0 / NtpTime::kFractionsPerSecond)));
}

// Converts Q32.32-formatted fixed-point seconds to `int64_t` microseconds.
inline int64_t Q32x32ToInt64Us(int64_t q32x32) {
  return dchecked_cast<int64_t>(
      std::round(q32x32 * (1'000'000.0 / NtpTime::kFractionsPerSecond)));
}

}  // namespace webrtc
#endif  // SYSTEM_WRAPPERS_INCLUDE_NTP_TIME_H_
