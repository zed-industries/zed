// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_POWER_MONITOR_TEST_UTILS_H_
#define BASE_TEST_POWER_MONITOR_TEST_UTILS_H_

#include <optional>

#include "base/functional/callback.h"
#include "base/power_monitor/battery_level_provider.h"
#include "base/power_monitor/sampling_event_source.h"

namespace base::test {

class TestSamplingEventSource : public SamplingEventSource {
 public:
  TestSamplingEventSource();
  ~TestSamplingEventSource() override;

  bool Start(SamplingEventCallback callback) override;
  TimeDelta GetSampleInterval() override;

  void SimulateEvent();

 private:
  SamplingEventCallback sampling_event_callback_;
};

class TestBatteryLevelProvider : public base::BatteryLevelProvider {
 public:
  TestBatteryLevelProvider();
  void GetBatteryState(
      base::OnceCallback<
          void(const std::optional<base::BatteryLevelProvider::BatteryState>&)>
          callback) override;

  void SetBatteryState(
      std::optional<base::BatteryLevelProvider::BatteryState> battery_state);

  static base::BatteryLevelProvider::BatteryState CreateBatteryState(
      int battery_count = 1,
      bool is_external_power_connected = false,
      int charge_percent = 100);

 private:
  std::optional<base::BatteryLevelProvider::BatteryState> battery_state_;
};

}  // namespace base::test

#endif  // BASE_TEST_POWER_MONITOR_TEST_UTILS_H_
