// Copyright 2021 The Crashpad Authors
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

#ifndef CRASHPAD_SNAPSHOT_LINUX_CAPTURE_MEMORY_DELEGATE_LINUX_H_
#define CRASHPAD_SNAPSHOT_LINUX_CAPTURE_MEMORY_DELEGATE_LINUX_H_

#include "snapshot/capture_memory.h"

#include <stdint.h>

#include <memory>
#include <vector>

#include "snapshot/linux/process_reader_linux.h"
#include "util/numeric/checked_range.h"

namespace crashpad {
namespace internal {

class MemorySnapshotGeneric;

class CaptureMemoryDelegateLinux : public CaptureMemory::Delegate {
 public:
  //! \brief A MemoryCaptureDelegate for Linux.
  //!
  //! \param[in] process_reader A ProcessReaderLinux for the target process.
  //! \param[in] thread_opt The thread being inspected. Memory ranges
  //!     overlapping this thread's stack will be ignored on the assumption
  //!     that they're already captured elsewhere. May be nullptr.
  //! \param[in] snapshots A vector of MemorySnapshotGeneric to which the
  //!     captured memory will be added.
  //! \param[in] budget_remaining If non-null, a pointer to the remaining number
  //!     of bytes to capture. If this is `0`, no further memory will be
  //!     captured.
  CaptureMemoryDelegateLinux(
      ProcessReaderLinux* process_reader,
      const ProcessReaderLinux::Thread* thread_opt,
      std::vector<std::unique_ptr<MemorySnapshotGeneric>>* snapshots,
      uint32_t* budget_remaining);

  // MemoryCaptureDelegate:
  bool Is64Bit() const override;
  bool ReadMemory(uint64_t at, uint64_t num_bytes, void* into) const override;
  std::vector<CheckedRange<uint64_t>> GetReadableRanges(
      const CheckedRange<uint64_t, uint64_t>& range) const override;
  void AddNewMemorySnapshot(
      const CheckedRange<uint64_t, uint64_t>& range) override;

 private:
  CheckedRange<uint64_t, uint64_t> stack_;
  ProcessReaderLinux* process_reader_;  // weak
  std::vector<std::unique_ptr<MemorySnapshotGeneric>>* snapshots_;  // weak
  uint32_t* budget_remaining_;
};

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_SNAPSHOT_LINUX_CAPTURE_MEMORY_DELEGATE_LINUX_H_
