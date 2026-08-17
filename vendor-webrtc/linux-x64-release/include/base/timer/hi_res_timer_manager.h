// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TIMER_HI_RES_TIMER_MANAGER_H_
#define BASE_TIMER_HI_RES_TIMER_MANAGER_H_

#include "base/base_export.h"
#include "base/power_monitor/power_observer.h"
#include "base/timer/timer.h"
#include "build/build_config.h"

namespace base {

// Ensures that the Windows high resolution timer is only used
// when not running on battery power.
class BASE_EXPORT HighResolutionTimerManager
    : public base::PowerSuspendObserver,
      public base::PowerStateObserver {
 public:
  HighResolutionTimerManager();

  HighResolutionTimerManager(const HighResolutionTimerManager&) = delete;
  HighResolutionTimerManager& operator=(const HighResolutionTimerManager&) =
      delete;

  ~HighResolutionTimerManager() override;

  // base::PowerStateObserver methods.
  void OnBatteryPowerStatusChange(
      PowerStateObserver::BatteryPowerStatus battery_power_status) override;
  // base::PowerSuspendObserver methods.
  void OnSuspend() override;
  void OnResume() override;

  // Returns true if the hi resolution clock could be used right now.
  bool hi_res_clock_available() const { return hi_res_clock_available_; }

 private:
  // Enable or disable the faster multimedia timer.
  void UseHiResClock(bool use);

  bool hi_res_clock_available_;

#if BUILDFLAG(IS_WIN)
  // Timer for polling the high resolution timer usage.
  base::RepeatingTimer timer_;
#endif
};

}  // namespace base

#endif  // BASE_TIMER_HI_RES_TIMER_MANAGER_H_
