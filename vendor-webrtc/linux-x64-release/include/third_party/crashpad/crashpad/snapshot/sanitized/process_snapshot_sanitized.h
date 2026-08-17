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

#ifndef CRASHPAD_SNAPSHOT_SANITIZED_PROCESS_SNAPSHOT_SANITIZED_H_
#define CRASHPAD_SNAPSHOT_SANITIZED_PROCESS_SNAPSHOT_SANITIZED_H_

#include <memory>
#include <string>
#include <vector>

#include "snapshot/exception_snapshot.h"
#include "snapshot/process_snapshot.h"
#include "snapshot/sanitized/module_snapshot_sanitized.h"
#include "snapshot/sanitized/thread_snapshot_sanitized.h"
#include "snapshot/thread_snapshot.h"
#include "snapshot/unloaded_module_snapshot.h"
#include "util/misc/address_types.h"
#include "util/misc/initialization_state_dcheck.h"
#include "util/misc/range_set.h"
#include "util/process/process_id.h"
#include "util/process/process_memory_sanitized.h"

namespace crashpad {

//! \brief A ProcessSnapshot which wraps and filters sensitive information from
//!     another ProcessSnapshot.
class ProcessSnapshotSanitized final : public ProcessSnapshot {
 public:
  ProcessSnapshotSanitized();

  ProcessSnapshotSanitized(const ProcessSnapshotSanitized&) = delete;
  ProcessSnapshotSanitized& operator=(const ProcessSnapshotSanitized&) = delete;

  ~ProcessSnapshotSanitized() override;

  //! \brief Initializes this object.
  //!
  //! This method must be successfully called before calling any other method on
  //! this object.
  //!
  //! \param[in] snapshot The ProcessSnapshot to sanitize.
  //! \param[in] allowed_annotations A list of annotations names to allow to
  //!     be returned by AnnotationsSimpleMap() or from this object's module
  //!     snapshots. If `nullptr`, all annotations will be returned.
  //      These annotation names support pattern matching, eg: "switch-*"
  //! \param[in] allowed_memory_ranges A list of memory ranges to allow to be
  //!     accessible via Memory(), or `nullptr` to allow all ranges.
  //! \param[in] target_module_address An address in the target process'
  //!     address space within the bounds of a module to target. If the
  //!     crashing thread's context and stack do not contain any pointers into
  //!     this module's address range, this method will return `false`. If this
  //!     value is 0, this method will not check the context or stack for
  //!     references to any particular module.
  //! \param[in] sanitize_stacks If `true`, the MemorySnapshots for each
  //!     thread's stack will be filtered using an
  //!     internal::StackSnapshotSanitized.
  //! \return `false` if \a snapshot does not meet sanitization requirements and
  //!     should be filtered entirely. Otherwise `true`.
  bool Initialize(
      const ProcessSnapshot* snapshot,
      std::unique_ptr<const std::vector<std::string>> allowed_annotations,
      std::unique_ptr<const std::vector<std::pair<VMAddress, VMAddress>>>
          allowed_memory_ranges,
      VMAddress target_module_address,
      bool sanitize_stacks);

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

 private:
  // Only used when allowed_annotations_ != nullptr.
  std::vector<std::unique_ptr<internal::ModuleSnapshotSanitized>> modules_;

  // Only used when sanitize_stacks_ == true.
  std::vector<std::unique_ptr<internal::ThreadSnapshotSanitized>> threads_;

  RangeSet address_ranges_;
  const ProcessSnapshot* snapshot_;
  ProcessMemorySanitized process_memory_;
  std::unique_ptr<const std::vector<std::string>> allowed_annotations_;
  bool sanitize_stacks_;
  InitializationStateDcheck initialized_;
};

}  // namespace crashpad

#endif  // CRASHPAD_SNAPSHOT_SANITIZED_PROCESS_SNAPSHOT_SANITIZED_H_
