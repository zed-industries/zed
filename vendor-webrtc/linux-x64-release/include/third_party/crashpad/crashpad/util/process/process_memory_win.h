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

#ifndef CRASHPAD_UTIL_PROCESS_PROCESS_MEMORY_WIN_H_
#define CRASHPAD_UTIL_PROCESS_PROCESS_MEMORY_WIN_H_

#include <windows.h>

#include "util/misc/address_types.h"
#include "util/misc/initialization_state_dcheck.h"
#include "util/process/process_memory.h"
#include "util/win/process_info.h"

namespace crashpad {

//! \brief Accesses the memory of another Windows process.
class ProcessMemoryWin final : public ProcessMemory {
 public:
  ProcessMemoryWin();

  ProcessMemoryWin(const ProcessMemoryWin&) = delete;
  ProcessMemoryWin& operator=(const ProcessMemoryWin&) = delete;

  ~ProcessMemoryWin();

  //! \brief Initializes this object to read the memory of a process with the
  //!     provided handle.
  //!
  //! This method must be called successfully prior to calling any other method
  //! in this class.
  //!
  //! \param[in] handle The HANDLE of a target process.
  //!
  //! \return `true` on success, `false` on failure with a message logged.
  bool Initialize(HANDLE handle);

  //! \brief Attempts to read \a size bytes from the target process starting at
  //!     address \a address into \a buffer. If some of the specified range is
  //!     not accessible, reads up to the first inaccessible byte.
  //!
  //! \return The actual number of bytes read.
  size_t ReadAvailableMemory(VMAddress address,
                             size_t num_bytes,
                             void* buffer) const;

 private:
  ssize_t ReadUpTo(VMAddress address, size_t size, void* buffer) const override;

  HANDLE handle_;
  ProcessInfo process_info_;
  InitializationStateDcheck initialized_;
};

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_PROCESS_PROCESS_MEMORY_WIN_H_
