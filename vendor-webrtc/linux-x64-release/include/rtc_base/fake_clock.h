/*
 *  Copyright 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_FAKE_CLOCK_H_
#define RTC_BASE_FAKE_CLOCK_H_

#include <stdint.h>

#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"
#include "rtc_base/time_utils.h"

namespace webrtc {

// Fake clock for use with unit tests, which does not tick on its own.
// Starts at time 0.
//
// TODO(deadbeef): Unify with webrtc::SimulatedClock.
class FakeClock : public ClockInterface {
 public:
  FakeClock() = default;
  FakeClock(const FakeClock&) = delete;
  FakeClock& operator=(const FakeClock&) = delete;
  ~FakeClock() override = default;

  // ClockInterface implementation.
  int64_t TimeNanos() const override;

  // Methods that can be used by the test to control the time.

  // Should only be used to set a time in the future.
  void SetTime(Timestamp new_time);

  void AdvanceTime(TimeDelta delta);

 private:
  mutable Mutex lock_;
  int64_t time_ns_ RTC_GUARDED_BY(lock_) = 0;
};

class ThreadProcessingFakeClock : public ClockInterface {
 public:
  int64_t TimeNanos() const override { return clock_.TimeNanos(); }
  void SetTime(Timestamp time);
  void AdvanceTime(TimeDelta delta);

 private:
  FakeClock clock_;
};

// Helper class that sets itself as the global clock in its constructor and
// unsets it in its destructor.
class ScopedBaseFakeClock : public FakeClock {
 public:
  ScopedBaseFakeClock();
  ~ScopedBaseFakeClock() override;

 private:
  ClockInterface* prev_clock_;
};

// TODO(srte): Rename this to reflect that it also does thread processing.
class ScopedFakeClock : public ThreadProcessingFakeClock {
 public:
  ScopedFakeClock();
  ~ScopedFakeClock() override;

 private:
  ClockInterface* prev_clock_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::FakeClock;
using ::webrtc::ScopedBaseFakeClock;
using ::webrtc::ScopedFakeClock;
using ::webrtc::ThreadProcessingFakeClock;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_FAKE_CLOCK_H_
