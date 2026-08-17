// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_POWER_MONITOR_POWER_MONITOR_H_
#define BASE_POWER_MONITOR_POWER_MONITOR_H_

#include <memory>

#include "base/base_export.h"
#include "base/no_destructor.h"
#include "base/observer_list_threadsafe.h"
#include "base/power_monitor/power_observer.h"
#include "base/time/time.h"
#include "base/trace_event/base_tracing.h"
#include "build/build_config.h"

namespace base {

class PowerMonitorSource;

// A class used to monitor the power state change and notify the observers about
// the change event. The threading model of this class is as follows:
// Once initialized, it is threadsafe. However, the client must ensure that
// initialization happens before any other methods are invoked, including
// IsInitialized(). IsInitialized() exists only as a convenience for detection
// of test contexts where the PowerMonitor global is never created.
class BASE_EXPORT PowerMonitor {
 public:
  static PowerMonitor* GetInstance();

  // Initializes global PowerMonitor state. Takes ownership of |source|, which
  // will be leaked on process teardown. May only be called once. Not threadsafe
  // - no other PowerMonitor methods may be called on any thread while calling
  // Initialize(). |source| must not be nullptr.
  void Initialize(std::unique_ptr<PowerMonitorSource> source);

  PowerMonitor(const PowerMonitor&) = delete;
  PowerMonitor& operator=(const PowerMonitor&) = delete;

  // Returns true if Initialize() has been called. Safe to call on any thread,
  // but must not be called while Initialize() or ShutdownForTesting() is being
  // invoked.
  bool IsInitialized() const;

  // Add and remove an observer.
  // Can be called from any thread. |observer| is notified on the sequence
  // from which it was registered.
  // Must not be called from within a notification callback.
  //
  // It is safe to add observers before the PowerMonitor is initialized. It is
  // safe to remove an observer even if it was not added as an observer.
  void AddPowerSuspendObserver(PowerSuspendObserver* observer);
  void RemovePowerSuspendObserver(PowerSuspendObserver* observer);
  void AddPowerStateObserver(PowerStateObserver* observer);
  void RemovePowerStateObserver(PowerStateObserver* observer);
  void AddPowerThermalObserver(PowerThermalObserver* observer);
  void RemovePowerThermalObserver(PowerThermalObserver* observer);

  // Atomically add a PowerSuspendObserver and read the current power suspended
  // state. This variant must be used to avoid race between adding an observer
  // and reading the power state. The following code would be racy:
  //    AddOPowerSuspendbserver(...);
  //    if (PowerMonitor::IsSystemSuspended()) { ... }
  //
  // Returns true if the system is currently suspended.
  bool AddPowerSuspendObserverAndReturnSuspendedState(
      PowerSuspendObserver* observer);
  // Returns the battery power status (battery power, external power, unknown).
  PowerStateObserver::BatteryPowerStatus
  AddPowerStateObserverAndReturnBatteryPowerStatus(
      PowerStateObserver* observer);
  // Returns the power thermal state.
  PowerThermalObserver::DeviceThermalState
  AddPowerStateObserverAndReturnPowerThermalState(
      PowerThermalObserver* observer);

  // Is the computer currently on battery power. May only be called if the
  // PowerMonitor has been initialized.
  bool IsOnBatteryPower() const;

  // Returns the current state of the battery power, that can be unknown if the
  // value isn't initialized yet. May only be called if the PowerMonitor has
  // been initialized.
  PowerStateObserver::BatteryPowerStatus GetBatteryPowerStatus() const;

  // Returns the time of the last system resume. If no system suspend/resume was
  // observed, returns an empty time. If the system is currently suspended,
  // returns TimeTicks::Max().
  TimeTicks GetLastSystemResumeTime() const;

  // Read the current DeviceThermalState if known. Can be called on any thread.
  // May only be called if the PowerMonitor has been initialized.
  PowerThermalObserver::DeviceThermalState GetCurrentThermalState() const;

  // Update the result of thermal state.
  void SetCurrentThermalState(PowerThermalObserver::DeviceThermalState state);

  // Uninitializes the PowerMonitor. Should be called at the end of any unit
  // test that mocks out the PowerMonitor, to avoid affecting subsequent tests.
  // There must be no live observers when invoked. Safe to call even if the
  // PowerMonitor hasn't been initialized.
  void ShutdownForTesting();

 private:
  friend class PowerMonitorSource;
  friend class base::NoDestructor<PowerMonitor>;

  PowerMonitor();
  ~PowerMonitor();

  const PowerMonitorSource* Source() const;

  void NotifyPowerStateChange(bool on_battery_power);
  void NotifyPowerStateChange(
      PowerStateObserver::BatteryPowerStatus battery_power_status);
  void NotifySuspend();
  void NotifyResume();
  void NotifyThermalStateChange(
      PowerThermalObserver::DeviceThermalState new_state);
  void NotifySpeedLimitChange(int speed_limit);

  bool is_system_suspended_ GUARDED_BY(is_system_suspended_lock_) = false;
  mutable Lock is_system_suspended_lock_;
  TimeTicks last_system_resume_time_ GUARDED_BY(is_system_suspended_lock_);

  PowerStateObserver::BatteryPowerStatus battery_power_status_
      GUARDED_BY(battery_power_status_lock_) =
          PowerStateObserver::BatteryPowerStatus::kUnknown;

  mutable Lock battery_power_status_lock_;

  perfetto::NamedTrack process_track_;
  PowerThermalObserver::DeviceThermalState power_thermal_state_
      GUARDED_BY(power_thermal_state_lock_) =
          PowerThermalObserver::DeviceThermalState::kUnknown;
  int speed_limit_ GUARDED_BY(power_thermal_state_lock_) =
      PowerThermalObserver::kSpeedLimitMax;
  Lock power_thermal_state_lock_;

  scoped_refptr<ObserverListThreadSafe<PowerStateObserver>>
      power_state_observers_;
  scoped_refptr<ObserverListThreadSafe<PowerSuspendObserver>>
      power_suspend_observers_;
  scoped_refptr<ObserverListThreadSafe<PowerThermalObserver>>
      thermal_state_observers_;
  std::unique_ptr<PowerMonitorSource> source_;
};

}  // namespace base

#endif  // BASE_POWER_MONITOR_POWER_MONITOR_H_
