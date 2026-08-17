/*
 * Copyright (C) 2021 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACED_PROBES_POWER_LINUX_POWER_SYSFS_DATA_SOURCE_H_
#define SRC_TRACED_PROBES_POWER_LINUX_POWER_SYSFS_DATA_SOURCE_H_

#include <optional>

#include "perfetto/ext/base/weak_ptr.h"
#include "perfetto/tracing/core/data_source_config.h"
#include "src/traced/probes/probes_data_source.h"

namespace perfetto {
class BatteryInfo;
class TraceWriter;

namespace base {
class TaskRunner;
}

class LinuxPowerSysfsDataSource : public ProbesDataSource {
 public:
  class BatteryInfo {
   public:
    explicit BatteryInfo(
        const char* power_supply_dir_path = "/sys/class/power_supply");
    ~BatteryInfo();

    // The current coloumb counter value in µAh.
    std::optional<int64_t> GetChargeCounterUah(size_t battery_idx);

    // The current energy counter in µWh.
    std::optional<int64_t> GetEnergyCounterUah(size_t battery_idx);

    // The voltage in µV.
    std::optional<int64_t> GetVoltageUv(size_t battery_idx);

    // The battery capacity in percent.
    std::optional<int64_t> GetCapacityPercent(size_t battery_idx);

    // The current reading of the battery in µA.
    std::optional<int64_t> GetCurrentNowUa(size_t battery_idx);

    // The smoothed current reading of the battery in µA.
    std::optional<int64_t> GetAverageCurrentUa(size_t battery_idx);

    // Name of the battery.
    std::string GetBatteryName(size_t battery_idx);

    size_t num_batteries() const;

   private:
    std::string power_supply_dir_path_;
    // The subdirectories that contain info of a battery power supply, e.g.
    // BAT0.
    std::vector<std::string> sysfs_battery_subdirs_;
  };
  static const ProbesDataSource::Descriptor descriptor;

  LinuxPowerSysfsDataSource(DataSourceConfig,
                            base::TaskRunner*,
                            TracingSessionID,
                            std::unique_ptr<TraceWriter> writer);

  ~LinuxPowerSysfsDataSource() override;

  base::WeakPtr<LinuxPowerSysfsDataSource> GetWeakPtr() const;

  // ProbesDataSource implementation.
  void Start() override;
  void Flush(FlushRequestID, std::function<void()> callback) override;
  // Use the default ClearIncrementalState() implementation: this data source
  // doesn't have any incremental state.

 private:
  void Tick();
  void WriteBatteryCounters();

  uint32_t poll_interval_ms_ = 0;

  base::TaskRunner* const task_runner_;
  std::unique_ptr<TraceWriter> writer_;
  std::unique_ptr<BatteryInfo> battery_info_;
  base::WeakPtrFactory<LinuxPowerSysfsDataSource> weak_factory_;  // Keep last.
};

}  // namespace perfetto

#endif  // SRC_TRACED_PROBES_POWER_LINUX_POWER_SYSFS_DATA_SOURCE_H_
