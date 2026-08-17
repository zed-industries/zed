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

#ifndef CRASHPAD_SNAPSHOT_MINIDUMP_MEMORY_SNAPSHOT_MINIDUMP_H_
#define CRASHPAD_SNAPSHOT_MINIDUMP_MEMORY_SNAPSHOT_MINIDUMP_H_

#include <windows.h>
#include <dbghelp.h>

#include <vector>

#include "snapshot/memory_snapshot.h"
#include "util/file/file_reader.h"
#include "util/misc/initialization_state_dcheck.h"

namespace crashpad {
namespace internal {
class MemorySnapshotMinidump : public MemorySnapshot {
 public:
  MemorySnapshotMinidump();

  MemorySnapshotMinidump(const MemorySnapshotMinidump&) = delete;
  MemorySnapshotMinidump& operator=(const MemorySnapshotMinidump&) = delete;

  ~MemorySnapshotMinidump() override;

  //! \brief Initializes the object.
  //!
  //! \param[in] file_reader A file reader corresponding to a minidump file.
  //!     The file reader must support seeking.
  //! \param[in] location The location within the file where we will find a
  //!     MINIDUMP_MEMORY_DESCRIPTOR from which to initialize this object.
  //!
  //! \return `true` if the snapshot could be created, `false` otherwise with
  //!     an appropriate message logged.
  bool Initialize(FileReaderInterface* file_reader, RVA location);

  uint64_t Address() const override;
  size_t Size() const override;
  bool Read(Delegate* delegate) const override;
  const MemorySnapshot* MergeWithOtherSnapshot(
      const MemorySnapshot* other) const override;

 private:
  uint64_t address_;
  std::vector<uint8_t> data_;
  InitializationStateDcheck initialized_;
};

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_SNAPSHOT_MINIDUMP_MEMORY_SNAPSHOT_MINIDUMP_H_
