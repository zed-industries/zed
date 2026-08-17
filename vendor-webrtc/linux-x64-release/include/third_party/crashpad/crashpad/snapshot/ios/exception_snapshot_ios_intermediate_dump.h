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

#ifndef CRASHPAD_SNAPSHOT_IOS_INTERMEDIATE_DUMP_EXCEPTION_SNAPSHOT_IOS_INTERMEDIATEDUMP_H_
#define CRASHPAD_SNAPSHOT_IOS_INTERMEDIATE_DUMP_EXCEPTION_SNAPSHOT_IOS_INTERMEDIATEDUMP_H_

#include <mach/mach.h>
#include <stdint.h>

#include <vector>

#include "build/build_config.h"
#include "snapshot/cpu_context.h"
#include "snapshot/exception_snapshot.h"
#include "snapshot/ios/memory_snapshot_ios_intermediate_dump.h"
#include "util/ios/ios_intermediate_dump_map.h"
#include "util/misc/initialization_state_dcheck.h"

namespace crashpad {

namespace internal {

//! \brief An ExceptionSnapshot of an exception sustained by a running (or
//!     crashed) process on an iOS system.
class ExceptionSnapshotIOSIntermediateDump final : public ExceptionSnapshot {
 public:
  ExceptionSnapshotIOSIntermediateDump();

  ExceptionSnapshotIOSIntermediateDump(
      const ExceptionSnapshotIOSIntermediateDump&) = delete;
  ExceptionSnapshotIOSIntermediateDump& operator=(
      const ExceptionSnapshotIOSIntermediateDump&) = delete;

  ~ExceptionSnapshotIOSIntermediateDump() override;

  //! \brief Initialize the snapshot as a signal exception.
  //!
  //! \param[in] exception_data The intermediate dump map used to initialize
  //!     this object.
  //!
  //! \return `true` if the snapshot could be created, `false` otherwise with
  //!     an appropriate message logged.
  bool InitializeFromSignal(const IOSIntermediateDumpMap* exception_data);

  //! \brief Initialize the object as a Mach exception from an intermediate
  //!     dump.
  //!
  //! \param[in] exception_data The intermediate dump map used to initialize
  //!     this object.
  //! \param[in] thread_list The intermediate dump map containing list of
  //!     threads.
  //!
  //! \return `true` if the snapshot could be created, `false` otherwise with
  //!     an appropriate message logged.
  bool InitializeFromMachException(const IOSIntermediateDumpMap* exception_data,
                                   const IOSIntermediateDumpList* thread_list);

  //! \brief Initialize the object as an NSException from an intermediate dump.
  //!
  //! \param[in] exception_data The intermediate dump map used to initialize
  //!     this object.
  //! \param[in] thread_list The intermediate dump map containing list of
  //!     threads.
  //!
  //! \return `true` if the snapshot could be created, `false` otherwise with
  //!     an appropriate message logged.
  bool InitializeFromNSException(const IOSIntermediateDumpMap* exception_data,
                                 const IOSIntermediateDumpList* thread_list);

  // ExceptionSnapshot:
  const CPUContext* Context() const override;
  uint64_t ThreadID() const override;
  uint32_t Exception() const override;
  uint32_t ExceptionInfo() const override;
  uint64_t ExceptionAddress() const override;
  const std::vector<uint64_t>& Codes() const override;
  virtual std::vector<const MemorySnapshot*> ExtraMemory() const override;

 private:
  void LoadContextFromUncaughtNSExceptionFrames(
      const IOSIntermediateDumpData* data,
      const IOSIntermediateDumpMap* other_thread);
  void LoadContextFromThread(const IOSIntermediateDumpMap* exception_data,
                             const IOSIntermediateDumpMap* other_thread);
#if defined(ARCH_CPU_X86_64)
  CPUContextX86_64 context_x86_64_;
#elif defined(ARCH_CPU_ARM64)
  CPUContextARM64 context_arm64_;
#else
#error Port.
#endif  // ARCH_CPU_X86_64
  CPUContext context_;
  std::vector<uint64_t> codes_;
  uint64_t thread_id_;
  uintptr_t exception_address_;
  uint32_t exception_;
  uint32_t exception_info_;
  std::vector<std::unique_ptr<internal::MemorySnapshotIOSIntermediateDump>>
      extra_memory_;
  InitializationStateDcheck initialized_;
};

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_SNAPSHOT_IOS_INTERMEDIATE_DUMP_EXCEPTION_SNAPSHOT_IOS_INTERMEDIATEDUMP_H_
