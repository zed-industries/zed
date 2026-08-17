/*
 *  Copyright 2005 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_TIME_UTILS_H_
#define RTC_BASE_TIME_UTILS_H_

#include <stdint.h>
#include <time.h>

#include "rtc_base/checks.h"
#include "rtc_base/system/rtc_export.h"
#include "rtc_base/system_time.h"

namespace webrtc {

static const int64_t kNumMillisecsPerSec = INT64_C(1000);
static const int64_t kNumMicrosecsPerSec = INT64_C(1000000);
static const int64_t kNumNanosecsPerSec = INT64_C(1000000000);

static const int64_t kNumMicrosecsPerMillisec =
    kNumMicrosecsPerSec / kNumMillisecsPerSec;
static const int64_t kNumNanosecsPerMillisec =
    kNumNanosecsPerSec / kNumMillisecsPerSec;
static const int64_t kNumNanosecsPerMicrosec =
    kNumNanosecsPerSec / kNumMicrosecsPerSec;

// Elapsed milliseconds between NTP base, 1900 January 1 00:00 GMT
// (see https://tools.ietf.org/html/rfc868), and January 1 00:00 GMT 1970
// epoch. This is useful when converting between the NTP time base and the
// time base used in RTCP reports.
constexpr int64_t kNtpJan1970Millisecs = 2'208'988'800 * kNumMillisecsPerSec;

// TODO(honghaiz): Define a type for the time value specifically.

class ClockInterface {
 public:
  virtual ~ClockInterface() {}
  virtual int64_t TimeNanos() const = 0;
};

// Sets the global source of time. This is useful mainly for unit tests.
//
// Returns the previously set ClockInterface, or nullptr if none is set.
//
// Does not transfer ownership of the clock. SetClockForTesting(nullptr)
// should be called before the ClockInterface is deleted.
//
// This method is not thread-safe; it should only be used when no other thread
// is running (for example, at the start/end of a unit test, or start/end of
// main()).
//
// TODO(deadbeef): Instead of having functions that access this global
// ClockInterface, we may want to pass the ClockInterface into everything
// that uses it, eliminating the need for a global variable and this function.
RTC_EXPORT ClockInterface* SetClockForTesting(ClockInterface* clock);

// Returns previously set clock, or nullptr if no custom clock is being used.
RTC_EXPORT ClockInterface* GetClockForTesting();

#if defined(WINUWP)
// Synchronizes the current clock based upon an NTP server's epoch in
// milliseconds.
void SyncWithNtp(int64_t time_from_ntp_server_ms);

// Returns the current time in nanoseconds. The clock is synchonized with the
// system wall clock time upon instatiation. It may also be synchronized using
// the SyncWithNtp() function above. Please note that the clock will most likely
// drift away from the system wall clock time as time goes by.
int64_t WinUwpSystemTimeNanos();
#endif  // defined(WINUWP)

// Returns the actual system time, even if a clock is set for testing.
// Useful for timeouts while using a test clock, or for logging.
int64_t SystemTimeMillis();

// Returns the current time in milliseconds in 32 bits.
uint32_t Time32();

// Returns the current time in milliseconds in 64 bits.
RTC_EXPORT int64_t TimeMillis();
// Deprecated. Do not use this in any new code.
inline int64_t Time() {
  return TimeMillis();
}

// Returns the current time in microseconds.
RTC_EXPORT int64_t TimeMicros();

// Returns the current time in nanoseconds.
RTC_EXPORT int64_t TimeNanos();

// Returns a future timestamp, 'elapsed' milliseconds from now.
int64_t TimeAfter(int64_t elapsed);

// Number of milliseconds that would elapse between 'earlier' and 'later'
// timestamps.  The value is negative if 'later' occurs before 'earlier'.
int64_t TimeDiff(int64_t later, int64_t earlier);
int32_t TimeDiff32(uint32_t later, uint32_t earlier);

// The number of milliseconds that have elapsed since 'earlier'.
inline int64_t TimeSince(int64_t earlier) {
  return TimeMillis() - earlier;
}

// The number of milliseconds that will elapse between now and 'later'.
inline int64_t TimeUntil(int64_t later) {
  return later - TimeMillis();
}

// Convert from tm, which is relative to 1900-01-01 00:00 to number of
// seconds from 1970-01-01 00:00 ("epoch"). Don't return time_t since that
// is still 32 bits on many systems.
int64_t TmToSeconds(const tm& tm);

// Return the number of microseconds since January 1, 1970, UTC.
// Useful mainly when producing logs to be correlated with other
// devices, and when the devices in question all have properly
// synchronized clocks.
//
// Note that this function obeys the system's idea about what the time
// is. It is not guaranteed to be monotonic; it will jump in case the
// system time is changed, e.g., by some other process calling
// settimeofday. Always use webrtc::TimeMicros(), not this function, for
// measuring time intervals and timeouts.
RTC_EXPORT int64_t TimeUTCMicros();

// Return the number of milliseconds since January 1, 1970, UTC.
// See above.
RTC_EXPORT int64_t TimeUTCMillis();

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::ClockInterface;
using ::webrtc::GetClockForTesting;
using ::webrtc::kNtpJan1970Millisecs;
using ::webrtc::kNumMicrosecsPerMillisec;
using ::webrtc::kNumMicrosecsPerSec;
using ::webrtc::kNumMillisecsPerSec;
using ::webrtc::kNumNanosecsPerMicrosec;
using ::webrtc::kNumNanosecsPerMillisec;
using ::webrtc::kNumNanosecsPerSec;
using ::webrtc::SetClockForTesting;
using ::webrtc::SystemTimeMillis;
using ::webrtc::Time;
using ::webrtc::Time32;
using ::webrtc::TimeAfter;
using ::webrtc::TimeDiff;
using ::webrtc::TimeDiff32;
using ::webrtc::TimeMicros;
using ::webrtc::TimeMillis;
using ::webrtc::TimeNanos;
using ::webrtc::TimeSince;
using ::webrtc::TimeUntil;
using ::webrtc::TimeUTCMicros;
using ::webrtc::TimeUTCMillis;
using ::webrtc::TmToSeconds;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_TIME_UTILS_H_
