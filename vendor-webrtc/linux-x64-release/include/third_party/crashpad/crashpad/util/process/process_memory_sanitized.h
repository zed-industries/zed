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

#ifndef CRASHPAD_UTIL_PROCESS_PROCESS_MEMORY_SANITIZED_H_
#define CRASHPAD_UTIL_PROCESS_PROCESS_MEMORY_SANITIZED_H_

#include <sys/types.h>

#include <utility>
#include <vector>

#include "util/misc/address_types.h"
#include "util/misc/initialization_state_dcheck.h"
#include "util/process/process_memory.h"

namespace crashpad {

//! \brief Sanitized access to the memory of another process.
class ProcessMemorySanitized final : public ProcessMemory {
 public:
  ProcessMemorySanitized();

  ProcessMemorySanitized(const ProcessMemorySanitized&) = delete;
  ProcessMemorySanitized& operator=(const ProcessMemorySanitized&) = delete;

  ~ProcessMemorySanitized();

  //! \brief Initializes this object to read memory from the underlying
  //!     \a memory object if the memory range is in \a allowed_ranges.
  //!
  //! This method must be called successfully prior to calling any other method
  //! in this class.
  //!
  //! \param[in] memory The memory object to read memory from.
  //! \param[in] allowed_ranges A list of allowed memory ranges.
  //!
  //! \return `true` on success, `false` on failure with a message logged.
  bool Initialize(
      const ProcessMemory* memory,
      const std::vector<std::pair<VMAddress, VMAddress>>* allowed_ranges);

 private:
  ssize_t ReadUpTo(VMAddress address, size_t size, void* buffer) const override;

  const ProcessMemory* memory_;
  InitializationStateDcheck initialized_;
  std::vector<std::pair<VMAddress, VMAddress>> allowed_ranges_;
};

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_PROCESS_PROCESS_MEMORY_SANITIZED_H_
