// Copyright 2019 The Crashpad Authors
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

#ifndef CRASHPAD_SNAPSHOT_MINIDUMP_EXCEPTION_SNAPSHOT_MINIDUMP_H_
#define CRASHPAD_SNAPSHOT_MINIDUMP_EXCEPTION_SNAPSHOT_MINIDUMP_H_

#include <windows.h>
#include <dbghelp.h>

#include "build/build_config.h"
#include "snapshot/cpu_context.h"
#include "snapshot/exception_snapshot.h"
#include "snapshot/minidump/minidump_context_converter.h"
#include "util/file/file_reader.h"
#include "util/misc/initialization_state.h"

namespace crashpad {
namespace internal {

//! \brief An ExceptionSnapshot based on a minidump file.
class ExceptionSnapshotMinidump final : public ExceptionSnapshot {
 public:
  ExceptionSnapshotMinidump();

  ExceptionSnapshotMinidump(const ExceptionSnapshotMinidump&) = delete;
  ExceptionSnapshotMinidump& operator=(const ExceptionSnapshotMinidump&) =
      delete;

  ~ExceptionSnapshotMinidump() override;

  //! \brief Initializes the object.
  //!
  //! \param[in] file_reader A file reader corresponding to a minidump file.
  //!     The file reader must support seeking.
  //! \param[in] arch The CPU architecture of this snapshot.
  //! \param[in] minidump_exception_stream_rva The file offset in \a file_reader
  //!     at which the MINIDUMP_EXCEPTION_STREAM structure is located.
  //!
  //! \return `true` if the snapshot could be created, `false` otherwise with
  //!     an appropriate message logged.
  bool Initialize(FileReaderInterface* file_reader,
                  CPUArchitecture arch,
                  RVA minidump_exception_stream_rva);

  // ExceptionSnapshot:
  const CPUContext* Context() const override;
  uint64_t ThreadID() const override;
  uint32_t Exception() const override;
  uint32_t ExceptionInfo() const override;
  uint64_t ExceptionAddress() const override;
  const std::vector<uint64_t>& Codes() const override;
  std::vector<const MemorySnapshot*> ExtraMemory() const override;

  // Allow callers to explicitly check whether this exception snapshot has been
  // initialized.
  bool IsValid() const { return initialized_.is_valid(); }

 private:
  MINIDUMP_EXCEPTION_STREAM minidump_exception_stream_;
  MinidumpContextConverter context_;
  std::vector<uint64_t> exception_information_;
  InitializationState initialized_;
};

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_SNAPSHOT_MINIDUMP_EXCEPTION_SNAPSHOT_MINIDUMP_H_
