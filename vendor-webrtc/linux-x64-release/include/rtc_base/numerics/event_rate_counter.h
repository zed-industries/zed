/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef RTC_BASE_NUMERICS_EVENT_RATE_COUNTER_H_
#define RTC_BASE_NUMERICS_EVENT_RATE_COUNTER_H_

#include <cstdint>

#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "rtc_base/numerics/sample_stats.h"

namespace webrtc {

// Calculates statistics based on events. For example for computing frame rates.
// Note that it doesn't provide any running statistics or reset funcitonality,
// so it's mostly useful for end of call statistics.
class EventRateCounter {
 public:
  // Adds an event based on it's `event_time` for correct updates of the
  // interval statistics, each event must be added past the previous events.
  void AddEvent(Timestamp event_time);
  // Adds the events from `other`. Note that the interval stats won't be
  // recalculated, only merged, so this is not equivalent to if the events would
  // have been added to the same counter from the start.
  void AddEvents(EventRateCounter other);
  bool IsEmpty() const;
  // Average number of events per second. Defaults to 0 for no events and NAN
  // for one event.
  double Rate() const;
  SampleStats<TimeDelta>& interval() { return interval_; }
  TimeDelta TotalDuration() const;
  int Count() const { return event_count_; }

 private:
  Timestamp first_time_ = Timestamp::PlusInfinity();
  Timestamp last_time_ = Timestamp::MinusInfinity();
  int64_t event_count_ = 0;
  SampleStats<TimeDelta> interval_;
};
}  // namespace webrtc
#endif  // RTC_BASE_NUMERICS_EVENT_RATE_COUNTER_H_
