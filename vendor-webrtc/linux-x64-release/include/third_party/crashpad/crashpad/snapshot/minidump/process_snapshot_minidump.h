// Copyright 2015 The Crashpad Authors
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

#ifndef CRASHPAD_SNAPSHOT_MINIDUMP_PROCESS_SNAPSHOT_MINIDUMP_H_
#define CRASHPAD_SNAPSHOT_MINIDUMP_PROCESS_SNAPSHOT_MINIDUMP_H_

#include <windows.h>
#include <dbghelp.h>
#include <stdint.h>
#include <sys/time.h>

#include <map>
#include <memory>
#include <string>
#include <vector>

#include "minidump/minidump_extensions.h"
#include "snapshot/exception_snapshot.h"
#include "snapshot/memory_snapshot.h"
#include "snapshot/minidump/exception_snapshot_minidump.h"
#include "snapshot/minidump/minidump_stream.h"
#include "snapshot/minidump/module_snapshot_minidump.h"
#include "snapshot/minidump/system_snapshot_minidump.h"
#include "snapshot/minidump/thread_snapshot_minidump.h"
#include "snapshot/module_snapshot.h"
#include "snapshot/process_snapshot.h"
#include "snapshot/system_snapshot.h"
#include "snapshot/thread_snapshot.h"
#include "snapshot/unloaded_module_snapshot.h"
#include "util/file/file_reader.h"
#include "util/misc/initialization_state_dcheck.h"
#include "util/misc/uuid.h"
#include "util/process/process_id.h"

namespace crashpad {

namespace internal {
class MemoryMapRegionSnapshotMinidump;
}  // namespace internal

//! \brief A ProcessSnapshot based on a minidump file.
class ProcessSnapshotMinidump final : public ProcessSnapshot {
 public:
  ProcessSnapshotMinidump();

  ProcessSnapshotMinidump(const ProcessSnapshotMinidump&) = delete;
  ProcessSnapshotMinidump& operator=(const ProcessSnapshotMinidump&) = delete;

  ~ProcessSnapshotMinidump() override;

  //! \brief Initializes the object.
  //!
  //! \param[in] file_reader A file reader corresponding to a minidump file.
  //!     The file reader must support seeking.
  //!
  //! \return `true` if the snapshot could be created, `false` otherwise with
  //!     an appropriate message logged.
  bool Initialize(FileReaderInterface* file_reader);

  // ProcessSnapshot:

  crashpad::ProcessID ProcessID() const override;
  crashpad::ProcessID ParentProcessID() const override;
  void SnapshotTime(timeval* snapshot_time) const override;
  void ProcessStartTime(timeval* start_time) const override;
  void ProcessCPUTimes(timeval* user_time, timeval* system_time) const override;
  void ReportID(UUID* report_id) const override;
  void ClientID(UUID* client_id) const override;
  const std::map<std::string, std::string>& AnnotationsSimpleMap()
      const override;
  const SystemSnapshot* System() const override;
  std::vector<const ThreadSnapshot*> Threads() const override;
  std::vector<const ModuleSnapshot*> Modules() const override;
  std::vector<UnloadedModuleSnapshot> UnloadedModules() const override;
  const ExceptionSnapshot* Exception() const override;
  std::vector<const MemoryMapRegionSnapshot*> MemoryMap() const override;
  std::vector<HandleSnapshot> Handles() const override;
  std::vector<const MemorySnapshot*> ExtraMemory() const override;
  const ProcessMemory* Memory() const override;

  //! \brief Returns a list of custom minidump streams. This routine is the
  //!     equivalent of ModuleSnapshot::CustomMinidumpStreams(), except that in
  //!     a minidump it is impossible to associate a custom stream to a specific
  //!     module.
  //!
  //! \return The caller does not take ownership of the returned objects, they
  //!     are scoped to the lifetime of the ProcessSnapshotMinidump object that
  //!     they were obtained from.
  std::vector<const MinidumpStream*> CustomMinidumpStreams() const;

 private:
  // Initializes data carried in a MinidumpCrashpadInfo stream on behalf of
  // Initialize().
  bool InitializeCrashpadInfo();

  // Initializes data carried in a MINIDUMP_MODULE_LIST stream on behalf of
  // Initialize().
  bool InitializeModules();

  // Initializes data carried in a MINIDUMP_THREAD_LIST stream on behalf of
  // Initialize().
  bool InitializeThreads();

  // Initializes data carried in a MINIDUMP_THREAD_NAME_LIST stream on behalf of
  // Initialize().
  bool InitializeThreadNames();

  // Initializes data carried in a MINIDUMP_MEMORY_INFO_LIST stream on behalf of
  // Initialize().
  bool InitializeMemoryInfo();

  // Initializes data carried in a MINIDUMP_MEMORY_LIST stream on behalf of
  // Initialize().
  bool InitializeExtraMemory();

  // Initializes data carried in a MINIDUMP_SYSTEM_INFO stream on behalf of
  // Initialize().
  bool InitializeSystemSnapshot();

  // Initializes data carried in a MinidumpModuleCrashpadInfoList structure on
  // behalf of InitializeModules(). This makes use of MinidumpCrashpadInfo as
  // well, so it must be called after InitializeCrashpadInfo().
  bool InitializeModulesCrashpadInfo(
      std::map<uint32_t, MINIDUMP_LOCATION_DESCRIPTOR>*
          module_crashpad_info_links);

  // Initializes data carried in a MINIDUMP_MISC_INFO structure on behalf of
  // Initialize().
  bool InitializeMiscInfo();

  // Initializes custom minidump streams.
  bool InitializeCustomMinidumpStreams();

  // Initializes data carried in a MINIDUMP_EXCEPTION_STREAM stream on behalf of
  // Initialize().
  bool InitializeExceptionSnapshot();

  MINIDUMP_HEADER header_;
  std::vector<MINIDUMP_DIRECTORY> stream_directory_;
  std::map<MinidumpStreamType, const MINIDUMP_LOCATION_DESCRIPTOR*> stream_map_;
  std::vector<std::unique_ptr<internal::ModuleSnapshotMinidump>> modules_;
  std::vector<std::unique_ptr<internal::ThreadSnapshotMinidump>> threads_;
  std::map<uint32_t, std::string> thread_names_;
  std::vector<UnloadedModuleSnapshot> unloaded_modules_;
  std::vector<std::unique_ptr<internal::MemoryMapRegionSnapshotMinidump>>
      mem_regions_;
  std::vector<const MemoryMapRegionSnapshot*> mem_regions_exposed_;
  std::vector<std::unique_ptr<internal::MemorySnapshotMinidump>> extra_memory_;
  std::vector<std::unique_ptr<MinidumpStream>> custom_streams_;
  MinidumpCrashpadInfo crashpad_info_;
  internal::SystemSnapshotMinidump system_snapshot_;
  internal::ExceptionSnapshotMinidump exception_snapshot_;
  CPUArchitecture arch_;
  std::map<std::string, std::string> annotations_simple_map_;
  std::string full_version_;
  FileReaderInterface* file_reader_;  // weak
  crashpad::ProcessID process_id_;
  uint32_t create_time_;
  uint32_t user_time_;
  uint32_t kernel_time_;
  InitializationStateDcheck initialized_;
};

}  // namespace crashpad

#endif  // CRASHPAD_SNAPSHOT_MINIDUMP_PROCESS_SNAPSHOT_MINIDUMP_H_
