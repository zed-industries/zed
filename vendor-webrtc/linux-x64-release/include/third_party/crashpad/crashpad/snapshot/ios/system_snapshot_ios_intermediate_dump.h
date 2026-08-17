// Copyright 2020 The Crashpad Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef CRASHPAD_SNAPSHOT_IOS_INTERMEDIATE_DUMP_SYSTEM_SNAPSHOT_IOS_INTERMEDIATEDUMP_H_
#define CRASHPAD_SNAPSHOT_IOS_INTERMEDIATE_DUMP_SYSTEM_SNAPSHOT_IOS_INTERMEDIATEDUMP_H_

#include <stdint.h>

#include <string>

#include "snapshot/system_snapshot.h"
#include "util/ios/ios_intermediate_dump_map.h"
#include "util/misc/initialization_state_dcheck.h"

namespace crashpad {

namespace internal {

//! \brief A SystemSnapshot of the running system, when the system runs iOS.
class SystemSnapshotIOSIntermediateDump final : public SystemSnapshot {
 public:
  SystemSnapshotIOSIntermediateDump();

  SystemSnapshotIOSIntermediateDump(const SystemSnapshotIOSIntermediateDump&) =
      delete;
  SystemSnapshotIOSIntermediateDump& operator=(
      const SystemSnapshotIOSIntermediateDump&) = delete;

  ~SystemSnapshotIOSIntermediateDump() override;

  //! \brief Initializes the object.
  //!
  //! \param[in] system_data An intermediate dump map containing various system
  //!     data points.
  void Initialize(const IOSIntermediateDumpMap* system_data);

  // SystemSnapshot:

  CPUArchitecture GetCPUArchitecture() const override;
  uint32_t CPURevision() const override;
  uint8_t CPUCount() const override;
  std::string CPUVendor() const override;
  void CPUFrequency(uint64_t* current_hz, uint64_t* max_hz) const override;
  uint32_t CPUX86Signature() const override;
  uint64_t CPUX86Features() const override;
  uint64_t CPUX86ExtendedFeatures() const override;
  uint32_t CPUX86Leaf7Features() const override;
  bool CPUX86SupportsDAZ() const override;
  OperatingSystem GetOperatingSystem() const override;
  bool OSServer() const override;
  void OSVersion(int* major,
                 int* minor,
                 int* bugfix,
                 std::string* build) const override;
  std::string OSVersionFull() const override;
  bool NXEnabled() const override;
  std::string MachineDescription() const override;
  void TimeZone(DaylightSavingTimeStatus* dst_status,
                int* standard_offset_seconds,
                int* daylight_offset_seconds,
                std::string* standard_name,
                std::string* daylight_name) const override;
  uint64_t AddressMask() const override;

  //! \brief Returns the number of nanoseconds between Crashpad initialization
  //!     and snapshot generation.
  uint64_t CrashpadUptime() const;

 private:
  std::string os_version_build_;
  std::string machine_description_;
  int os_version_major_;
  int os_version_minor_;
  int os_version_bugfix_;
  uint32_t active_;
  uint32_t inactive_;
  uint32_t wired_;
  uint32_t free_;
  int cpu_count_;
  std::string cpu_vendor_;
  DaylightSavingTimeStatus dst_status_;
  int standard_offset_seconds_;
  int daylight_offset_seconds_;
  std::string standard_name_;
  std::string daylight_name_;
  uint64_t address_mask_;
  uint64_t crashpad_uptime_ns_;
  InitializationStateDcheck initialized_;
};

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_SNAPSHOT_IOS_INTERMEDIATE_DUMP_SYSTEM_SNAPSHOT_IOS_INTERMEDIATEDUMP_H_
