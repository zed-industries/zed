// Copyright 2017 The Crashpad Authors
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

#ifndef CRASHPAD_UTIL_PROCESS_PROCESS_MEMORY_LINUX_H_
#define CRASHPAD_UTIL_PROCESS_PROCESS_MEMORY_LINUX_H_

#include <sys/types.h>

#include <functional>
#include <string>

#include "base/files/scoped_file.h"
#include "util/misc/address_types.h"
#include "util/process/process_memory.h"

namespace crashpad {

class PtraceConnection;

//! \brief Accesses the memory of another Linux process.
class ProcessMemoryLinux final : public ProcessMemory {
 public:
  explicit ProcessMemoryLinux(PtraceConnection* connection);

  ProcessMemoryLinux(const ProcessMemoryLinux&) = delete;
  ProcessMemoryLinux& operator=(const ProcessMemoryLinux&) = delete;

  ~ProcessMemoryLinux();

  //! \brief Returns the input pointer with any non-addressing bits, such as
  //!     tags removed.
  VMAddress PointerToAddress(VMAddress address) const;

 private:
  ssize_t ReadUpTo(VMAddress address, size_t size, void* buffer) const override;

  std::function<ssize_t(VMAddress, size_t, void*)> read_up_to_;
  base::ScopedFD mem_fd_;
  bool ignore_top_byte_;
};

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_PROCESS_PROCESS_MEMORY_LINUX_H_
