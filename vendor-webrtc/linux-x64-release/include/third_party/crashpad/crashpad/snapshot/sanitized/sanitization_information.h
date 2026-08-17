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

#ifndef CRASHPAD_SNAPSHOT_SANITIZED_SANITIZATION_INFORMATION_H_
#define CRASHPAD_SNAPSHOT_SANITIZED_SANITIZATION_INFORMATION_H_

#include <stdint.h>

#include <string>
#include <utility>
#include <vector>

#include "util/misc/address_types.h"
#include "util/process/process_memory_range.h"

namespace crashpad {

#pragma pack(push, 1)

//! \brief Struture containing information about how snapshots should be
//!     sanitized.
//!
//! \see ProcessSnapshotSanitized
struct SanitizationInformation {
  //! \brief The address in the client process' address space of a nullptr
  //!     terminated array of NUL-terminated strings. The string values are the
  //!     names of allowed annotations. This value is 0 if all annotations are
  //!     allowed.
  VMAddress allowed_annotations_address;

  //! \brief An address in the client process' address space within a module to
  //!     target. When a target module is used, crash dumps are discarded unless
  //!     the crashing thread's program counter or pointer-aligned values on the
  //!     crashing thread's stack point into the target module. This value is 0
  //!     if there is no target module.
  VMAddress target_module_address;

  //! \brief The address in the client process' address space of a
  //!     \a SanitizationAllowedMemoryRanges, a list of address ranges allowed
  //!     to be accessed by ProcessMemorySanitized. This value is 0 if no memory
  //!     is allowed to be read using ProcessMemorySanitized.
  VMAddress allowed_memory_ranges_address;

  //! \brief Non-zero if stacks should be sanitized for possible PII.
  uint8_t sanitize_stacks;
};

//! \brief Describes a list of allowed memory ranges.
struct SanitizationAllowedMemoryRanges {
  //! \brief Describes a range of memory.
  struct Range {
    VMAddress base;
    VMSize length;
  };

  //! \brief Address of an array of |size| elements of type Range.
  VMAddress entries;
  VMSize size;
};

#pragma pack(pop)

//! \brief Reads a list of allowed annotations from another process.
//!
//! \param[in] memory A memory reader for the target process.
//! \param[in] list_address The address in the target process' address space of
//!     a nullptr terminated array of NUL-terminated strings.
//! \param[out] allowed_annotations The list read, valid only if this function
//!     returns `true`.
//! \return `true` on success, `false` on failure with a message logged.
bool ReadAllowedAnnotations(const ProcessMemoryRange& memory,
                            VMAddress list_address,
                            std::vector<std::string>* allowed_annotations);

//! \brief Reads a list of allowed memory ranges from another process.
//!
//! \param[in] memory A memory reader for the target process.
//! \param[in] list_address The address in the target process' address space of
//!     a nullptr terminated array of NUL-terminated strings.
//! \param[out] allowed_memory_ranges A list of allowed memory regions, valid
//!     only if this function returns `true`.
//! \return `true` on success, `false` on failure with a message logged.
bool ReadAllowedMemoryRanges(
    const ProcessMemoryRange& memory,
    VMAddress list_address,
    std::vector<std::pair<VMAddress, VMAddress>>* allowed_memory_ranges);

}  // namespace crashpad

#endif  // CRASHPAD_SNAPSHOT_SANITIZED_SANITIZATION_INFORMATION_H_
