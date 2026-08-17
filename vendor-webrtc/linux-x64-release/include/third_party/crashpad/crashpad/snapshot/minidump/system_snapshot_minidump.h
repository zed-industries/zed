// Copyright 2018 The Crashpad Authors
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

#ifndef CRASHPAD_SNAPSHOT_MINIDUMP_SYSTEM_SNAPSHOT_MINIDUMP_H_
#define CRASHPAD_SNAPSHOT_MINIDUMP_SYSTEM_SNAPSHOT_MINIDUMP_H_

#include <windows.h>

#include "minidump/minidump_extensions.h"
#include "snapshot/system_snapshot.h"
#include "util/file/file_reader.h"
#include "util/misc/initialization_state_dcheck.h"

namespace crashpad {
namespace internal {

//! \brief A SystemSnapshot based on a minidump file.
class SystemSnapshotMinidump : public SystemSnapshot {
 public:
  SystemSnapshotMinidump();

  SystemSnapshotMinidump(const SystemSnapshotMinidump&) = delete;
  SystemSnapshotMinidump& operator=(const SystemSnapshotMinidump&) = delete;

  ~SystemSnapshotMinidump() override;

  //! \brief Initializes the object.
  //!
  //! \param[in] file_reader A file reader corresponding to a minidump file.
  //!     The file reader must support seeking.
  //! \param[in] minidump_system_info_rva The file offset in \a file_reader at
  //!     which the thread’s MINIDUMP_SYSTEM_INFO structure is located.
  //! \param[in] version The OS version taken from the build string in
  //!     MINIDUMP_MISC_INFO_4.
  //!
  //! \return `true` if the snapshot could be created, `false` otherwise with
  //!     an appropriate message logged.
  bool Initialize(FileReaderInterface* file_reader,
                  RVA minidump_system_info_rva,
                  const std::string& version);

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
  std::string MachineDescription() const override;
  bool NXEnabled() const override;
  void TimeZone(DaylightSavingTimeStatus* dst_status,
                int* standard_offset_seconds,
                int* daylight_offset_seconds,
                std::string* standard_name,
                std::string* daylight_name) const override;
  uint64_t AddressMask() const override;

 private:
  MINIDUMP_SYSTEM_INFO minidump_system_info_;
  std::string minidump_build_name_;
  std::string full_version_;
  InitializationStateDcheck initialized_;
};

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_SNAPSHOT_MINIDUMP_SYSTEM_SNAPSHOT_MINIDUMP_H_
