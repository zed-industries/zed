// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_POWER_MONITOR_POWER_MONITOR_SOURCE_H_
#define BASE_POWER_MONITOR_POWER_MONITOR_SOURCE_H_

#include "base/base_export.h"
#include "base/power_monitor/power_observer.h"
#include "base/synchronization/lock.h"
#include "build/build_config.h"

namespace base {

// Communicates power state changes to the power monitor.
class BASE_EXPORT PowerMonitorSource {
 public:
  PowerMonitorSource();

  PowerMonitorSource(const PowerMonitorSource&) = delete;
  PowerMonitorSource& operator=(const PowerMonitorSource&) = delete;

  virtual ~PowerMonitorSource();

  // Normalized list of power events.
  enum PowerEvent {
    POWER_STATE_EVENT,  // The Power status of the system has changed.
    SUSPEND_EVENT,      // The system is being suspended.
    RESUME_EVENT        // The system is being resumed.
  };

  // Reads the current DeviceThermalState, if available on the platform.
  // Otherwise, returns kUnknown.
  virtual PowerThermalObserver::DeviceThermalState GetCurrentThermalState()
      const;

  // Reads the initial operating system CPU speed limit, if available on the
  // platform. Otherwise returns PowerThermalObserver::kSpeedLimitMax.
  // Only called on the main thread in PowerMonitor::Initialize().
  // The actual speed limit value will be updated asynchronously via the
  // ProcessSpeedLimitEvent() if/when the value changes.
  virtual int GetInitialSpeedLimit() const;

  // Update the result of thermal state.
  virtual void SetCurrentThermalState(
      PowerThermalObserver::DeviceThermalState state);

  // Platform-specific method to determine the battery power status.
  virtual PowerStateObserver::BatteryPowerStatus GetBatteryPowerStatus()
      const = 0;

  static const char* DeviceThermalStateToString(
      PowerThermalObserver::DeviceThermalState state);

 protected:
  friend class PowerMonitorTest;

  // Friend function that is allowed to access the protected ProcessPowerEvent.
  friend void ProcessPowerEventHelper(PowerEvent);
  friend void ProcessThermalEventHelper(
      PowerThermalObserver::DeviceThermalState);

  // Process*Event should only be called from a single thread, most likely
  // the UI thread or, in child processes, the IO thread.
  static void ProcessPowerEvent(PowerEvent event_id);
  static void ProcessThermalEvent(
      PowerThermalObserver::DeviceThermalState new_thermal_state);
  static void ProcessSpeedLimitEvent(int speed_limit);
};

}  // namespace base

#endif  // BASE_POWER_MONITOR_POWER_MONITOR_SOURCE_H_
