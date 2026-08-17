/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_NTP_TIME_UTIL_H_
#define MODULES_RTP_RTCP_SOURCE_NTP_TIME_UTIL_H_

#include <stdint.h>

#include "api/units/time_delta.h"
#include "rtc_base/numerics/safe_conversions.h"
#include "system_wrappers/include/ntp_time.h"

namespace webrtc {

// Helper function for compact ntp representation:
// RFC 3550, Section 4. Time Format.
// Wallclock time is represented using the timestamp format of
// the Network Time Protocol (NTP).
// ...
// In some fields where a more compact representation is
// appropriate, only the middle 32 bits are used; that is, the low 16
// bits of the integer part and the high 16 bits of the fractional part.
inline uint32_t CompactNtp(NtpTime ntp) {
  return (ntp.seconds() << 16) | (ntp.fractions() >> 16);
}

// Converts interval to compact ntp (1/2^16 seconds) resolution.
// Negative values converted to 0, Overlarge values converted to max uint32_t.
uint32_t SaturatedToCompactNtp(TimeDelta delta);

// Convert interval to the NTP time resolution (1/2^32 seconds ~= 0.2 ns).
// For deltas with absolute value larger than 35 minutes result is unspecified.
inline constexpr int64_t ToNtpUnits(TimeDelta delta) {
  // For better precision `delta` is taken with best TimeDelta precision (us),
  // then multiplaction and conversion to seconds are swapped to avoid float
  // arithmetic.
  // 2^31 us ~= 35.8 minutes.
  return (saturated_cast<int32_t>(delta.us()) * (int64_t{1} << 32)) / 1'000'000;
}

// Converts interval from compact ntp (1/2^16 seconds) resolution to TimeDelta.
// This interval can be up to ~9.1 hours (2^15 seconds).
// Values close to 2^16 seconds are considered negative.
TimeDelta CompactNtpIntervalToTimeDelta(uint32_t compact_ntp_interval);

// Converts interval from compact ntp (1/2^16 seconds) resolution to TimeDelta.
// This interval can be up to ~9.1 hours (2^15 seconds).
// Values close to 2^16 seconds are considered negative and are converted to
// minimum value of 1ms.
TimeDelta CompactNtpRttToTimeDelta(uint32_t compact_ntp_interval);

}  // namespace webrtc
#endif  // MODULES_RTP_RTCP_SOURCE_NTP_TIME_UTIL_H_
