// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_COMPUTE_PRESSURE_CHANGE_RATE_MONITOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_COMPUTE_PRESSURE_CHANGE_RATE_MONITOR_H_

#include <array>

#include "base/timer/timer.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_pressure_source.h"
#include "third_party/blink/renderer/modules/compute_pressure/pressure_record.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"

namespace blink {

// ChangeRateMonitor is a class encompassing the implementation of
// parameters involved the rate obfuscation mitigation.
// https://w3c.github.io/compute-pressure/#ref-for-dfn-rate-obfuscation-1
class ChangeRateMonitor final {
 public:
  ChangeRateMonitor();
  ~ChangeRateMonitor();

  // Reset counters and internal data if more time has passed than the
  // current observation window's length.
  void ResetIfNeeded();

  void ResetChangeCount(V8PressureSource::Enum);

  void IncreaseChangeCount(V8PressureSource::Enum);

  // Check if the amount of state changes is exceeding the limit of changes
  // allowed during the observation window's length.
  bool ChangeCountExceedsLimit(V8PressureSource::Enum) const;

  void set_change_count_threshold_for_testing(uint64_t count) {
    change_count_threshold_ = count;
  }

  void set_penalty_duration_for_testing(base::TimeDelta time) {
    penalty_duration_ = time;
  }

  // Return the penalty duration applied during the observation window's
  // duration;
  base::TimeDelta penalty_duration() const { return penalty_duration_; }

 private:
  // Reset all private members.
  void Reset();

  // Timestamp when the observation window is created.
  base::TimeTicks start_time_;

  // Duration of the current observation window.
  // A randomized number.
  base::TimeDelta observation_window_time_;

  // Maximum number of state changes during observation window length.
  // A randomized number.
  uint64_t change_count_threshold_;

  // Penalty duration when |change_count_| reaches |change_count_threshold_|.
  // A randomized number.
  base::TimeDelta penalty_duration_;

  // Map to keep the state changes count per source.
  std::array<uint64_t, V8PressureSource::kEnumSize> change_count_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_COMPUTE_PRESSURE_CHANGE_RATE_MONITOR_H_
