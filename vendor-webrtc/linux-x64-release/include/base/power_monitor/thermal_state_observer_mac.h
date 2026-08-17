// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_POWER_MONITOR_THERMAL_STATE_OBSERVER_MAC_H_
#define BASE_POWER_MONITOR_THERMAL_STATE_OBSERVER_MAC_H_

#include <IOKit/pwr_mgt/IOPMLib.h>
#include <dispatch/dispatch.h>

#include <memory>

#include "base/base_export.h"
#include "base/functional/callback.h"
#include "base/gtest_prod_util.h"
#include "base/power_monitor/power_observer.h"

namespace base {

// This class is used to listen for the thermal state change notification
// NSProcessInfoThermalStateDidChangeNotification, routing it to
// PowerMonitorSource.
class BASE_EXPORT ThermalStateObserverMac {
 public:
  using StateUpdateCallback =
      base::RepeatingCallback<void(PowerThermalObserver::DeviceThermalState)>;
  using SpeedLimitUpdateCallback = base::RepeatingCallback<void(int)>;

  ThermalStateObserverMac(
      StateUpdateCallback state_update_callback,
      SpeedLimitUpdateCallback speed_limit_update_callback,
      // This optional argument is overridden from tests because Apple software
      // doesn't seem to permit injecting notifications in their domains.
      // NOTE: this must be a statically allocated string as the pointer value
      // is stored internally.
      const char* power_notification_key = kIOPMCPUPowerNotificationKey);
  ~ThermalStateObserverMac();

  PowerThermalObserver::DeviceThermalState GetCurrentThermalState();
  int GetCurrentSpeedLimit() const;

 private:
  FRIEND_TEST_ALL_PREFIXES(ThermalStateObserverMacTest, StateChange);
  PowerThermalObserver::DeviceThermalState state_for_testing_ =
      PowerThermalObserver::DeviceThermalState::kUnknown;

  const char* const power_notification_key_;
  int speed_limit_notification_token_ = 0;

  struct ObjCStorage;
  std::unique_ptr<ObjCStorage> objc_storage_;
};

}  // namespace base

#endif  // BASE_POWER_MONITOR_THERMAL_STATE_OBSERVER_MAC_H_
