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

#ifndef CRASHPAD_SNAPSHOT_IOS_INTERMEDIATE_DUMP_MEMORY_SNAPSHOT_IOS_INTERMEDIATEDUMP_H_
#define CRASHPAD_SNAPSHOT_IOS_INTERMEDIATE_DUMP_MEMORY_SNAPSHOT_IOS_INTERMEDIATEDUMP_H_

#include <mach/mach.h>

#include <vector>

#include "snapshot/memory_snapshot.h"
#include "util/misc/initialization_state_dcheck.h"

namespace crashpad {
namespace internal {

//! \brief A MemorySnapshot of a memory region.
class MemorySnapshotIOSIntermediateDump final : public MemorySnapshot {
 public:
  MemorySnapshotIOSIntermediateDump() = default;

  MemorySnapshotIOSIntermediateDump(const MemorySnapshotIOSIntermediateDump&) =
      delete;
  MemorySnapshotIOSIntermediateDump& operator=(
      const MemorySnapshotIOSIntermediateDump&) = delete;

  ~MemorySnapshotIOSIntermediateDump() = default;

  //! \brief Initializes the object.
  //!
  //! \param[in] address The base address of the memory region to snapshot.
  //! \param[in] data The destination address where the snapshot will be stored.
  //! \param[in] size The size of the memory region to snapshot.
  void Initialize(vm_address_t address, vm_address_t data, vm_size_t size);

  // MemorySnapshot:
  uint64_t Address() const override;
  size_t Size() const override;
  bool Read(Delegate* delegate) const override;
  const MemorySnapshot* MergeWithOtherSnapshot(
      const MemorySnapshot* other) const override;

 private:
  template <class T>
  friend const MemorySnapshot* MergeWithOtherSnapshotImpl(
      const T* self,
      const MemorySnapshot* other);

  vm_address_t address_;
  vm_address_t data_;

  // Because the iOS snapshot memory region is owned by the intermediate dump,
  // it's necessary to copy the merged data into a vector owned by the memory
  // snapshot itself.
  std::vector<uint8_t> merged_data_;

  vm_size_t size_;
  InitializationStateDcheck initialized_;
};

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_SNAPSHOT_IOS_INTERMEDIATE_DUMP_MEMORY_SNAPSHOT_IOS_INTERMEDIATEDUMP_H_
