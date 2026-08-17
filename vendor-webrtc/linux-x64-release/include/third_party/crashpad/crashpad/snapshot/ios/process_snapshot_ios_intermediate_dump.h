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

#ifndef CRASHPAD_SNAPSHOT_IOS_INTERMEDIATE_DUMP_PROCESS_SNAPSHOT_IOS_INTERMEDIATEDUMP_H_
#define CRASHPAD_SNAPSHOT_IOS_INTERMEDIATE_DUMP_PROCESS_SNAPSHOT_IOS_INTERMEDIATEDUMP_H_

#include <sys/sysctl.h>

#include <vector>

#include "base/files/file_path.h"
#include "snapshot/ios/exception_snapshot_ios_intermediate_dump.h"
#include "snapshot/ios/module_snapshot_ios_intermediate_dump.h"
#include "snapshot/ios/process_snapshot_ios_intermediate_dump.h"
#include "snapshot/ios/system_snapshot_ios_intermediate_dump.h"
#include "snapshot/ios/thread_snapshot_ios_intermediate_dump.h"
#include "snapshot/process_snapshot.h"
#include "snapshot/thread_snapshot.h"
#include "snapshot/unloaded_module_snapshot.h"
#include "util/ios/ios_intermediate_dump_reader.h"

namespace crashpad {
namespace internal {

//! \brief A ProcessSnapshot of a running (or crashed) process running on a
//!     iphoneOS system.
class ProcessSnapshotIOSIntermediateDump final : public ProcessSnapshot {
 public:
  ProcessSnapshotIOSIntermediateDump() = default;

  ProcessSnapshotIOSIntermediateDump(
      const ProcessSnapshotIOSIntermediateDump&) = delete;
  ProcessSnapshotIOSIntermediateDump& operator=(
      const ProcessSnapshotIOSIntermediateDump&) = delete;

  //! \brief Initializes the object.
  //!
  //! \param[in] dump_path The intermediate dump to read.
  //! \param[in] annotations Process annotations to set in each crash report.
  //!
  //! \return `true` if the snapshot could be created, `false` otherwise with
  //!     an appropriate message logged.
  bool InitializeWithFilePath(
      const base::FilePath& dump_path,
      const std::map<std::string, std::string>& annotations);

  //! \brief Initializes the object.
  //!
  //! \param[in] dump_interface An interface corresponding to an intermediate
  //!     dump file.
  //! \param[in] annotations Process annotations to set in each crash report.
  //!
  //! \return `true` if the snapshot could be created, `false` otherwise with
  //!     an appropriate message logged.
  bool InitializeWithFileInterface(
      const IOSIntermediateDumpInterface& dump_interface,
      const std::map<std::string, std::string>& annotations);

  //! On iOS, the client ID is under the control of the snapshot producer,
  //! which may call this method to set the client ID. If this is not done,
  //! ClientID() will return an identifier consisting entirely of zeroes.
  void SetClientID(const UUID& client_id);

  //! \brief Sets the value to be returned by ReportID().
  //!
  //! On iOS, the crash report ID is under the control of the snapshot
  //! producer, which may call this method to set the report ID. If this is not
  //! done, ReportID() will return an identifier consisting entirely of zeroes.
  void SetReportID(const UUID& report_id);

  // ProcessSnapshot:
  pid_t ProcessID() const override;
  pid_t ParentProcessID() const override;
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

  // Returns ModuleSnapshotIOSIntermediateDump::IntermediateDumpExtraMemory
  // stored in the intermediate dump. This is used by UserExtensionStreams for
  // processing the snapshot. This memory will not be written to a minidump
  std::vector<const MemorySnapshot*> IntermediateDumpExtraMemory() const;

 private:
  // Retain the reader for the lifetime of the ProcessSnapshot so large chunks
  // of data do not need to be copied around (such as MemorySnapshot
  // intermediate dumps).
  IOSIntermediateDumpReader reader_;
  pid_t p_pid_;
  pid_t e_ppid_;
  timeval p_starttime_;
  time_value_t basic_info_user_time_;
  time_value_t basic_info_system_time_;
  time_value_t thread_times_user_time_;
  time_value_t thread_times_system_time_;
  internal::SystemSnapshotIOSIntermediateDump system_;
  std::vector<std::unique_ptr<internal::ThreadSnapshotIOSIntermediateDump>>
      threads_;
  std::vector<std::unique_ptr<internal::ModuleSnapshotIOSIntermediateDump>>
      modules_;
  std::unique_ptr<internal::ExceptionSnapshotIOSIntermediateDump> exception_;
  UUID report_id_;
  UUID client_id_;
  std::map<std::string, std::string> annotations_simple_map_;
  timeval snapshot_time_;
  std::vector<std::unique_ptr<internal::MemorySnapshotIOSIntermediateDump>>
      extra_memory_;
  InitializationStateDcheck initialized_;
};

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_SNAPSHOT_IOS_INTERMEDIATE_DUMP_PROCESS_SNAPSHOT_IOS_INTERMEDIATEDUMP_H_
