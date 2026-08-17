// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_WEBRTC_OVERRIDES_TIMER_BASED_TICK_PROVIDER_H_
#define THIRD_PARTY_BLINK_WEBRTC_OVERRIDES_TIMER_BASED_TICK_PROVIDER_H_

#include "base/functional/callback_forward.h"
#include "base/time/time.h"
#include "third_party/webrtc/rtc_base/system/rtc_export.h"
#include "third_party/webrtc_overrides/metronome_source.h"

namespace blink {

// Tick provider that generates ticks on a fixed specified cadence based on
// PostDelayedTaskAt.
class RTC_EXPORT TimerBasedTickProvider : public MetronomeSource::TickProvider {
 public:
  // The default metronome tick period.
  static constexpr base::TimeDelta kDefaultPeriod = base::Hertz(64);

  // Returns the time of the next default tick given a target `time` and
  // `period`.
  static base::TimeTicks TimeSnappedToNextTick(base::TimeTicks time,
                                               base::TimeDelta period);

  // Create with tick period `tick_period`.
  explicit TimerBasedTickProvider(base::TimeDelta tick_period);

  // TickProvider overrides.
  void RequestCallOnNextTick(base::OnceClosure callback) override;
  base::TimeDelta TickPeriod() override;

 private:
  const base::TimeDelta tick_period_;
};

}

#endif  // THIRD_PARTY_BLINK_WEBRTC_OVERRIDES_TIMER_BASED_TICK_PROVIDER_H_
