// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_POWER_MONITOR_SPEED_LIMIT_OBSERVER_WIN_H_
#define BASE_POWER_MONITOR_SPEED_LIMIT_OBSERVER_WIN_H_

#include "base/base_export.h"
#include "base/functional/callback.h"
#include "base/moving_window.h"
#include "base/power_monitor/power_observer.h"
#include "base/time/time.h"
#include "base/timer/timer.h"

namespace base {

// This class is used to listen for speed-limit changes and route new values to
// PowerMonitorSource when they are changed. The speed-limit value represents
// how well the CPU is running, where 100 means that it is running at normal
// speed (not throttled) and 0 means that it is so severely throttled (thermal
// throttling, power throttling, or other) that it is not running at all.
// A value under 70 indicates noticeable throttling, and a value under 40
// indicates severe throttling. Well designed systems with sufficient power
// and cooling should be able to run with no throttling, but some systems
// (laptops in particular) may be throttled, especially in hot environments or
// when running on battery. On a well designed computer this metric should stay
// at 100, only going lower if there is insufficient cooling or power.
class BASE_EXPORT SpeedLimitObserverWin final {
 public:
  typedef base::RepeatingCallback<void(int)> SpeedLimitUpdateCallback;

  explicit SpeedLimitObserverWin(
      SpeedLimitUpdateCallback speed_limit_update_callback);
  ~SpeedLimitObserverWin();

 private:
  int GetCurrentSpeedLimit() const;
  void OnTimerTick();
  float EstimateThrottlingLevel() const;

  size_t num_cpus() const { return num_cpus_; }

  const SpeedLimitUpdateCallback callback_;

  // Periodically calls OnTimerTick() where a new speed-limit metric is
  // calculated. The timer is cancelled once this object is destroyed.
  base::RepeatingTimer timer_;
  // Number of logical cores in the existing physical processor.
  // Example: a processor with 6 cores which supports hyperthreading has 12
  // logical cores, hence `num_cpus_` equals 12 in this case.
  const size_t num_cpus_;
  // A simple MA filter of size 10 is used to smooth out the speed-limit
  // value and to remove noise from short spikes in CPU load. The existing
  // sample rate is one sample per seconds but the existing choice is rather
  // ad-hoc and not based on any deeper analysis into exact frequency
  // characteristics of the underlying process.
  mutable MovingAverage<int, int64_t> moving_average_;
  // Max speed-limit value is 100 (%) and it is also used in cases where the
  // native Windows API(s) fail.
  int speed_limit_ = PowerThermalObserver::kSpeedLimitMax;
};

}  // namespace base

#endif  // BASE_POWER_MONITOR_SPEED_LIMIT_OBSERVER_WIN_H_
