/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_DRIFTING_CLOCK_H_
#define TEST_DRIFTING_CLOCK_H_

#include <stdint.h>

#include "system_wrappers/include/clock.h"
#include "system_wrappers/include/ntp_time.h"

namespace webrtc {
namespace test {
class DriftingClock : public Clock {
 public:
  static constexpr float kNoDrift = 1.0f;

  DriftingClock(Clock* clock, float speed);

  static constexpr float PercentsFaster(float percent) {
    return 1.0f + percent / 100.0f;
  }
  static constexpr float PercentsSlower(float percent) {
    return 1.0f - percent / 100.0f;
  }

  Timestamp CurrentTime() override { return Drift(clock_->CurrentTime()); }
  NtpTime ConvertTimestampToNtpTime(Timestamp timestamp) override {
    return Drift(clock_->ConvertTimestampToNtpTime(timestamp));
  }

 private:
  TimeDelta Drift() const;
  Timestamp Drift(Timestamp timestamp) const;
  NtpTime Drift(NtpTime ntp_time) const;

  Clock* const clock_;
  const float drift_;
  const Timestamp start_time_;
};
}  // namespace test
}  // namespace webrtc

#endif  // TEST_DRIFTING_CLOCK_H_
