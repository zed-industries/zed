// Copyright 2014 The Crashpad Authors
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

#ifndef CRASHPAD_SNAPSHOT_MEMORY_SNAPSHOT_GENERIC_H_
#define CRASHPAD_SNAPSHOT_MEMORY_SNAPSHOT_GENERIC_H_

#include <stdint.h>
#include <sys/types.h>

#include "base/containers/heap_array.h"
#include "base/logging.h"
#include "base/numerics/safe_math.h"
#include "snapshot/memory_snapshot.h"
#include "util/misc/address_types.h"
#include "util/misc/initialization_state_dcheck.h"
#include "util/process/process_memory.h"

namespace crashpad {
namespace internal {

//! \brief A MemorySnapshot of a memory region in a process on the running
//!     system. Works on multiple platforms by using a platform-specific
//!     ProcessMemory object.
class MemorySnapshotGeneric final : public MemorySnapshot {
 public:
  MemorySnapshotGeneric() = default;

  MemorySnapshotGeneric(const MemorySnapshotGeneric&) = delete;
  MemorySnapshotGeneric& operator=(const MemorySnapshotGeneric&) = delete;

  ~MemorySnapshotGeneric() = default;

  //! \brief Initializes the object.
  //!
  //! Memory is read lazily. No attempt is made to read the memory snapshot data
  //! until Read() is called, and the memory snapshot data is discared when
  //! Read() returns.
  //!
  //! \param[in] process_memory A reader for the process being snapshotted.
  //! \param[in] address The base address of the memory region to snapshot, in
  //!     the snapshot process’ address space.
  //! \param[in] size The size of the memory region to snapshot.
  void Initialize(const ProcessMemory* process_memory,
                  VMAddress address,
                  VMSize size) {
    INITIALIZATION_STATE_SET_INITIALIZING(initialized_);
    process_memory_ = process_memory;
    address_ = address;
    size_ = base::checked_cast<size_t>(size);
    INITIALIZATION_STATE_SET_VALID(initialized_);
  }

  // MemorySnapshot:

  uint64_t Address() const override {
    INITIALIZATION_STATE_DCHECK_VALID(initialized_);
    return address_;
  }

  size_t Size() const override {
    INITIALIZATION_STATE_DCHECK_VALID(initialized_);
    return size_;
  }

  bool Read(Delegate* delegate) const override {
    INITIALIZATION_STATE_DCHECK_VALID(initialized_);

    if (size_ == 0) {
      return delegate->MemorySnapshotDelegateRead(nullptr, size_);
    }

    auto buffer = base::HeapArray<uint8_t>::Uninit(size_);
    if (!process_memory_->Read(address_, buffer.size(), buffer.data())) {
      return false;
    }
    return delegate->MemorySnapshotDelegateRead(buffer.data(), buffer.size());
  }

  const MemorySnapshot* MergeWithOtherSnapshot(
      const MemorySnapshot* other) const override {
    const MemorySnapshotGeneric* other_as_memory_snapshot_concrete =
        reinterpret_cast<const MemorySnapshotGeneric*>(other);
    if (process_memory_ != other_as_memory_snapshot_concrete->process_memory_) {
      LOG(ERROR) << "different process_memory_ for snapshots";
      return nullptr;
    }
    CheckedRange<uint64_t, size_t> merged(0, 0);
    if (!LoggingDetermineMergedRange(this, other, &merged))
      return nullptr;

    auto result = std::make_unique<MemorySnapshotGeneric>();
    result->Initialize(process_memory_, merged.base(), merged.size());
    return result.release();
  }

 private:
  template <class T>
  friend const MemorySnapshot* MergeWithOtherSnapshotImpl(
      const T* self,
      const MemorySnapshot* other);

  const ProcessMemory* process_memory_;  // weak
  VMAddress address_;
  size_t size_;
  InitializationStateDcheck initialized_;
};

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_SNAPSHOT_GENERIC_MEMORY_SNAPSHOT_GENERIC_H_
