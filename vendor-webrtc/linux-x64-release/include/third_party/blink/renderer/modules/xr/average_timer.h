// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_AVERAGE_TIMER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_AVERAGE_TIMER_H_

#include "base/time/time.h"

namespace blink {
class AverageTimer {
 public:
  void StartTimer();
  void StopTimer();
  base::TimeDelta TakeAverageMicroseconds();

 private:
  // The time when the timer was started.
  base::TimeTicks start_;
  // The total time spent in the timer.
  base::TimeDelta total_time_;
  // The number of times the timer was stopped or collected.
  int size_ = 0;
};
}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_AVERAGE_TIMER_H_
