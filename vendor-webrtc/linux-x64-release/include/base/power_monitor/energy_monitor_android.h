// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_POWER_MONITOR_ENERGY_MONITOR_ANDROID_H_
#define BASE_POWER_MONITOR_ENERGY_MONITOR_ANDROID_H_

#include <cstdint>

#include "base/base_export.h"
#include "build/build_config.h"

namespace base {
namespace android {

// Read and return the current remaining battery capacity (microampere-hours).
// Only supported with a device power source (i.e. not in child processes in
// Chrome) and on devices with Android >= Lollipop as well as a power supply
// that supports this counter. Returns 0 if unsupported.
int BASE_EXPORT GetRemainingBatteryCapacity();
// Read and return the total energy consumed since boot in microwatt-seconds.
// Only supported on specific devices with Android >= Vanilla Ice Cream.
// Returns 0 if unsupported.
// This should be called only after we know the battery status from
// PowerMonitor::AddPowerStateObserverAndReturnBatteryPowerStatus. Otherwise the
// monitor might be not initialized, and this function may return 0.
int64_t BASE_EXPORT GetTotalEnergyConsumed();

}  // namespace android
}  // namespace base

#endif  // BASE_POWER_MONITOR_ENERGY_MONITOR_ANDROID_H_
