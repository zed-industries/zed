// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_POWER_MONITOR_TIMER_SAMPLING_EVENT_SOURCE_H_
#define BASE_POWER_MONITOR_TIMER_SAMPLING_EVENT_SOURCE_H_

#include "base/base_export.h"
#include "base/power_monitor/sampling_event_source.h"
#include "base/time/time.h"
#include "base/timer/timer.h"

namespace base {

// Generates a sampling event at regular time intervals.
class BASE_EXPORT TimerSamplingEventSource : public SamplingEventSource {
 public:
  // |interval| is the time interval between sampling events.
  explicit TimerSamplingEventSource(TimeDelta interval);

  ~TimerSamplingEventSource() override;

  // SamplingEventSource:
  bool Start(SamplingEventCallback callback) override;
  TimeDelta GetSampleInterval() override;

 private:
  const TimeDelta interval_;
  RepeatingTimer timer_;
};

}  // namespace base

#endif  // BASE_POWER_MONITOR_TIMER_SAMPLING_EVENT_SOURCE_H_
