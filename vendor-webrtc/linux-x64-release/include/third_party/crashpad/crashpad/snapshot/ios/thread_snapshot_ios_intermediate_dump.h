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

#ifndef CRASHPAD_SNAPSHOT_IOS_INTERMEDIATE_DUMP_THREAD_SNAPSHOT_IOS_INTERMEDIATEDUMP_H_
#define CRASHPAD_SNAPSHOT_IOS_INTERMEDIATE_DUMP_THREAD_SNAPSHOT_IOS_INTERMEDIATEDUMP_H_

#include <string>

#include "build/build_config.h"
#include "snapshot/cpu_context.h"
#include "snapshot/ios/memory_snapshot_ios_intermediate_dump.h"
#include "snapshot/thread_snapshot.h"
#include "util/ios/ios_intermediate_dump_map.h"
#include "util/misc/initialization_state_dcheck.h"

namespace crashpad {
namespace internal {

//! \brief A ThreadSnapshot of a thread on an iOS system.
class ThreadSnapshotIOSIntermediateDump final : public ThreadSnapshot {
 public:
  ThreadSnapshotIOSIntermediateDump();

  ThreadSnapshotIOSIntermediateDump(const ThreadSnapshotIOSIntermediateDump&) =
      delete;
  ThreadSnapshotIOSIntermediateDump& operator=(
      const ThreadSnapshotIOSIntermediateDump&) = delete;

  ~ThreadSnapshotIOSIntermediateDump() override;

  //! \brief Initializes the object.
  //!
  //! \brief thread_data The intermediate dump map used to initialize this
  //!     object.
  //!
  //! \return `true` if the snapshot could be created.
  bool Initialize(const IOSIntermediateDumpMap* thread_data);

  // ThreadSnapshot:
  const CPUContext* Context() const override;
  const MemorySnapshot* Stack() const override;
  uint64_t ThreadID() const override;
  std::string ThreadName() const override;
  int SuspendCount() const override;
  int Priority() const override;
  uint64_t ThreadSpecificDataAddress() const override;
  std::vector<const MemorySnapshot*> ExtraMemory() const override;

 private:
#if defined(ARCH_CPU_X86_64)
  CPUContextX86_64 context_x86_64_;
#elif defined(ARCH_CPU_ARM64)
  CPUContextARM64 context_arm64_;
#else
#error Port.
#endif  // ARCH_CPU_X86_64
  CPUContext context_;
  std::vector<uint8_t> exception_stack_memory_;
  MemorySnapshotIOSIntermediateDump stack_;
  std::string thread_name_;
  uint64_t thread_id_;
  uint64_t thread_specific_data_address_;
  int suspend_count_;
  int priority_;
  std::vector<std::unique_ptr<internal::MemorySnapshotIOSIntermediateDump>>
      extra_memory_;
  InitializationStateDcheck initialized_;
};

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_SNAPSHOT_IOS_INTERMEDIATE_DUMP_THREAD_SNAPSHOT_IOS_INTERMEDIATEDUMP_H_
