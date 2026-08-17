// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_SEQUENCE_MANAGER_THREAD_CONTROLLER_POWER_MONITOR_H_
#define BASE_TASK_SEQUENCE_MANAGER_THREAD_CONTROLLER_POWER_MONITOR_H_

#include "base/base_export.h"
#include "base/power_monitor/power_observer.h"

namespace base {
namespace sequence_manager {
namespace internal {

// A helper class that keeps track of the power state and handles power
// notifications. The class register itself to the PowerMonitor and receives
// notifications on the bound thread (see BindToCurrentThread(...)).
class BASE_EXPORT ThreadControllerPowerMonitor : public PowerSuspendObserver {
 public:
  ThreadControllerPowerMonitor();
  ~ThreadControllerPowerMonitor() override;
  ThreadControllerPowerMonitor(const ThreadControllerPowerMonitor&) = delete;
  ThreadControllerPowerMonitor& operator=(const ThreadControllerPowerMonitor&) =
      delete;

  // Register this class to the power monitor to receive notifications on this
  // thread. It is safe to call this before PowerMonitor is initialized.
  void BindToCurrentThread();

  // Returns whether the process is between power suspend and resume
  // notifications.
  bool IsProcessInPowerSuspendState();

  // Initializes features for this class. See `base::features::Init()`.
  static void InitializeFeatures();

  static void OverrideUsePowerMonitorForTesting(bool use_power_monitor);
  static void ResetForTesting();

  // base::PowerSuspendObserver:
  void OnSuspend() override;
  void OnResume() override;

 private:
  // Power state based on notifications delivered to this observer.
  bool is_power_suspended_ = false;

  // Whether PowerMonitor observer is registered.
  bool is_observer_registered_ = false;
};

}  // namespace internal
}  // namespace sequence_manager
}  // namespace base

#endif  // BASE_TASK_SEQUENCE_MANAGER_THREAD_CONTROLLER_POWER_MONITOR_H_
