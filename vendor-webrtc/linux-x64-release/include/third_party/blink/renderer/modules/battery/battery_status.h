// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BATTERY_BATTERY_STATUS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BATTERY_BATTERY_STATUS_H_

#include "base/time/time.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

#include <cmath>
#include <limits>

namespace blink {

// Simple struct to hold the battery status.  This class is copyable.
class MODULES_EXPORT BatteryStatus final {
  DISALLOW_NEW();

 public:
  BatteryStatus()
      : charging_(true), discharging_time_(base::TimeDelta::Max()), level_(1) {}
  BatteryStatus(bool charging,
                base::TimeDelta charging_time,
                base::TimeDelta discharging_time,
                double level)
      : charging_(charging),
        charging_time_(charging_time),
        discharging_time_(discharging_time),
        level_(EnsureTwoSignificantDigits(level)) {}
  BatteryStatus(const BatteryStatus&) = default;
  BatteryStatus& operator=(const BatteryStatus&) = default;

  bool Charging() const { return charging_; }
  base::TimeDelta charging_time() const { return charging_time_; }
  base::TimeDelta discharging_time() const { return discharging_time_; }
  double Level() const { return level_; }

 private:
  double EnsureTwoSignificantDigits(double level) {
    // Convert battery level value which should be in [0, 1] to a value in
    // [0, 1] with 2 digits of precision. This is to provide a consistent
    // experience across platforms (e.g. on Mac and Android the battery changes
    // are generally reported with 1% granularity). It also serves the purpose
    // of reducing the possibility of fingerprinting and triggers less level
    // change events on platforms where the granularity is high.
    DCHECK(level >= 0 && level <= 1);
    return std::round(level * 100) / 100.f;
  }

  bool charging_;
  base::TimeDelta charging_time_;
  base::TimeDelta discharging_time_;
  double level_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BATTERY_BATTERY_STATUS_H_
