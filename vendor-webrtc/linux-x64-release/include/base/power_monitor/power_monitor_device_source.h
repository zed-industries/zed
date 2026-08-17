// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_POWER_MONITOR_POWER_MONITOR_DEVICE_SOURCE_H_
#define BASE_POWER_MONITOR_POWER_MONITOR_DEVICE_SOURCE_H_

#include <memory>
#include <vector>

#include "base/base_export.h"
#include "base/power_monitor/power_monitor_source.h"
#include "base/power_monitor/power_observer.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_WIN)
#include <windows.h>

#include "base/power_monitor/speed_limit_observer_win.h"
#include "base/threading/sequence_bound.h"
#endif  // BUILDFLAG(IS_WIN)

#if BUILDFLAG(IS_MAC)
#include <IOKit/IOTypes.h>

#include "base/apple/scoped_cftyperef.h"
#include "base/mac/scoped_ionotificationportref.h"
#include "base/power_monitor/battery_level_provider.h"
#include "base/power_monitor/iopm_power_source_sampling_event_source.h"
#include "base/power_monitor/thermal_state_observer_mac.h"
#endif  // BUILDFLAG(IS_MAC)

#if BUILDFLAG(IS_IOS)
#include <objc/runtime.h>
#endif  // BUILDFLAG(IS_IOS)

namespace base {

// A class used to monitor the power state change and notify the observers about
// the change event.
class BASE_EXPORT PowerMonitorDeviceSource : public PowerMonitorSource {
 public:
  PowerMonitorDeviceSource();

  PowerMonitorDeviceSource(const PowerMonitorDeviceSource&) = delete;
  PowerMonitorDeviceSource& operator=(const PowerMonitorDeviceSource&) = delete;

  ~PowerMonitorDeviceSource() override;

#if BUILDFLAG(IS_CHROMEOS)
  // On Chrome OS, Chrome receives power-related events from powerd, the system
  // power daemon, via D-Bus signals received on the UI thread. base can't
  // directly depend on that code, so this class instead exposes static methods
  // so that events can be passed in.
  static void SetPowerSource(
      PowerStateObserver::BatteryPowerStatus battery_power_status);
  static void HandleSystemSuspending();
  static void HandleSystemResumed();
  static void ThermalEventReceived(
      PowerThermalObserver::DeviceThermalState state);

  // These two methods is used for handling thermal state update requests, such
  // as asking for initial state when starting lisitening to thermal change.
  PowerThermalObserver::DeviceThermalState GetCurrentThermalState()
      const override;
  void SetCurrentThermalState(
      PowerThermalObserver::DeviceThermalState state) override;
#endif

 private:
  friend class PowerMonitorDeviceSourceTest;

#if BUILDFLAG(IS_WIN)
  // Represents a message-only window for power message handling on Windows.
  // Only allow PowerMonitor to create it.
  class PowerMessageWindow {
   public:
    PowerMessageWindow();
    ~PowerMessageWindow();

   private:
    static LRESULT CALLBACK WndProcThunk(HWND hwnd,
                                         UINT message,
                                         WPARAM wparam,
                                         LPARAM lparam);

    // Instance of the module containing the window procedure.
    HMODULE instance_ = nullptr;
    // A hidden message-only window.
    HWND message_hwnd_ = nullptr;
    // A handle, returned when we register for power setting notification
    HPOWERNOTIFY power_notify_handle_ = nullptr;
  };
#endif  // BUILDFLAG(IS_WIN)

#if (BUILDFLAG(IS_APPLE) && !BUILDFLAG(IS_IOS_TVOS)) || BUILDFLAG(IS_WIN)
  void PlatformInit();
  void PlatformDestroy();
#endif  // BUILDFLAG(IS_APPLE) || BUILDFLAG(IS_WIN)

#if BUILDFLAG(IS_MAC)
  // Callback from IORegisterForSystemPower(). |refcon| is the |this| pointer.
  static void SystemPowerEventCallback(void* refcon,
                                       io_service_t service,
                                       natural_t message_type,
                                       void* message_argument);
#endif  // BUILDFLAG(IS_MAC)

  // Platform-specific method to check whether the system is currently
  // running on battery power. Returns kBatteryPower if running on battery,
  // kExternalPower if running on external power or kUnknown if the power
  // state is unknown (for example, during early process lifetime when the
  // state hasn't been obtained yet).
  PowerStateObserver::BatteryPowerStatus GetBatteryPowerStatus() const override;

#if BUILDFLAG(IS_ANDROID)
  PowerThermalObserver::DeviceThermalState GetCurrentThermalState()
      const override;
#endif  // BUILDFLAG(IS_ANDROID)

#if BUILDFLAG(IS_WIN)
  // PowerMonitorSource:
  int GetInitialSpeedLimit() const override;
#endif  // BUILDFLAG(IS_WIN)

#if BUILDFLAG(IS_MAC)
  // PowerMonitorSource:
  PowerThermalObserver::DeviceThermalState GetCurrentThermalState()
      const override;
  int GetInitialSpeedLimit() const override;

  // Retrieves the current battery state to update `is_on_battery_`.
  void GetBatteryState();
  void OnBatteryStateReceived(
      const std::optional<BatteryLevelProvider::BatteryState>& battery_state);

  // Reference to the system IOPMrootDomain port.
  io_connect_t power_manager_port_ = IO_OBJECT_NULL;

  // Notification port that delivers power (sleep/wake) notifications.
  mac::ScopedIONotificationPortRef notification_port_;

  // Notifier reference for the |notification_port_|.
  io_object_t notifier_ = IO_OBJECT_NULL;

  // Generates power-source-change events.
  IOPMPowerSourceSamplingEventSource power_source_event_source_;

  std::unique_ptr<BatteryLevelProvider> battery_level_provider_;

  // Observer of thermal state events: critical temperature etc.
  std::unique_ptr<ThermalStateObserverMac> thermal_state_observer_;

  PowerStateObserver::BatteryPowerStatus battery_power_status_ =
      PowerStateObserver::BatteryPowerStatus::kUnknown;
#endif

#if BUILDFLAG(IS_IOS)
  // Holds pointers to system event notification observers.
  std::vector<id> notification_observers_;
#endif

#if BUILDFLAG(IS_WIN)
  PowerMessageWindow power_message_window_;
  // |speed_limit_observer_| is owned by the main/UI thread but the
  // SpeedLimitObserverWin is bound to a different sequence.
  std::unique_ptr<base::SequenceBound<SpeedLimitObserverWin>>
      speed_limit_observer_;
#endif

#if BUILDFLAG(IS_CHROMEOS)
  PowerThermalObserver::DeviceThermalState current_thermal_state_ =
      PowerThermalObserver::DeviceThermalState::kUnknown;
#endif
};

}  // namespace base

#endif  // BASE_POWER_MONITOR_POWER_MONITOR_DEVICE_SOURCE_H_
