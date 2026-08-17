// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_POWER_MONITOR_TEST_H_
#define BASE_TEST_POWER_MONITOR_TEST_H_

#include "base/memory/raw_ptr.h"
#include "base/power_monitor/power_monitor.h"
#include "base/power_monitor/power_monitor_source.h"
#include "base/power_monitor/power_observer.h"

namespace base {

namespace test {

// Use PowerMonitorTestSource via ScopedPowerMonitorTestSource wrapper when you
// need to simulate power events (suspend and resume).
class PowerMonitorTestSource;

// ScopedPowerMonitorTestSource initializes the PowerMonitor with a Mock
// PowerMonitorSource. Mock power notifications can be simulated through this
// helper class.
//
// Example:
//   base::test::ScopedPowerMonitorTestSource power_monitor_source;
//   power_monitor_source.Suspend();
//   [...]
//   power_monitor_source.Resume();
class ScopedPowerMonitorTestSource {
 public:
  ScopedPowerMonitorTestSource();
  ~ScopedPowerMonitorTestSource();

  ScopedPowerMonitorTestSource(const ScopedPowerMonitorTestSource&) = delete;
  ScopedPowerMonitorTestSource& operator=(const ScopedPowerMonitorTestSource&) =
      delete;

  // Retrieve current states.
  PowerThermalObserver::DeviceThermalState GetCurrentThermalState() const;
  PowerStateObserver::BatteryPowerStatus GetBatteryPowerStatus() const;

  // Sends asynchronous notifications to registered observers.
  void Suspend();
  void Resume();
  void SetBatteryPowerStatus(
      PowerStateObserver::BatteryPowerStatus battery_power_status);

  void GenerateSuspendEvent();
  void GenerateResumeEvent();
  void GeneratePowerStateEvent(
      PowerStateObserver::BatteryPowerStatus battery_power_status);
  void GenerateThermalThrottlingEvent(
      PowerThermalObserver::DeviceThermalState new_thermal_state);
  void GenerateSpeedLimitEvent(int speed_limit);

 private:
  // Owned by PowerMonitor.
  raw_ptr<PowerMonitorTestSource, DanglingUntriaged>
      power_monitor_test_source_ = nullptr;
};

class PowerMonitorTestObserver : public PowerSuspendObserver,
                                 public PowerThermalObserver,
                                 public PowerStateObserver {
 public:
  PowerMonitorTestObserver();
  ~PowerMonitorTestObserver() override;

  // PowerStateObserver overrides.
  void OnBatteryPowerStatusChange(
      PowerStateObserver::BatteryPowerStatus battery_power_status) override;
  // PowerSuspendObserver overrides.
  void OnSuspend() override;
  void OnResume() override;
  // PowerThermalObserver overrides.
  void OnThermalStateChange(
      PowerThermalObserver::DeviceThermalState new_state) override;
  void OnSpeedLimitChange(int speed_limit) override;

  // Test status counts.
  int power_state_changes() const { return power_state_changes_; }
  int suspends() const { return suspends_; }
  int resumes() const { return resumes_; }
  int thermal_state_changes() const { return thermal_state_changes_; }
  int speed_limit_changes() const { return speed_limit_changes_; }

  PowerStateObserver::BatteryPowerStatus last_power_status() const {
    return last_power_status_;
  }
  PowerThermalObserver::DeviceThermalState last_thermal_state() const {
    return last_thermal_state_;
  }
  int last_speed_limit() const { return last_speed_limit_; }

 private:
  // Count of OnPowerStateChange notifications.
  int power_state_changes_ = 0;
  // Count of OnSuspend notifications.
  int suspends_ = 0;
  // Count of OnResume notifications.
  int resumes_ = 0;
  // Count of OnThermalStateChange notifications.
  int thermal_state_changes_ = 0;
  // Count of OnSpeedLimitChange notifications.
  int speed_limit_changes_ = 0;

  // Last power state we were notified of.
  PowerStateObserver::BatteryPowerStatus last_power_status_ =
      PowerStateObserver::BatteryPowerStatus::kUnknown;
  // Last power thermal we were notified of.
  PowerThermalObserver::DeviceThermalState last_thermal_state_ =
      PowerThermalObserver::DeviceThermalState::kUnknown;
  // Last speed limit we were notified of.
  int last_speed_limit_ = PowerThermalObserver::kSpeedLimitMax;
};

}  // namespace test
}  // namespace base

#endif  // BASE_TEST_POWER_MONITOR_TEST_H_
