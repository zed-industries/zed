// Copyright 2022 The Centipede Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef THIRD_PARTY_CENTIPEDE_RUNNER_DL_INFO_H_
#define THIRD_PARTY_CENTIPEDE_RUNNER_DL_INFO_H_

#include <cstdint>
#include <cstring>

#include "absl/base/nullability.h"

namespace fuzztest::internal {

// Basic information about one dynamic library (or executable).
// No CTOR - these objects may need to be linker-initialized.
struct DlInfo {
  uintptr_t start_address;  // Address in memory where the object is loaded.
  uintptr_t size;           // Number of bytes in the object.
  intptr_t link_offset;  // Difference between runtime addresses and link-time
                         // addresses.
  char path[4096];          // Pathname from which the object was loaded.

  void Clear() { memset(this, 0, sizeof(*this)); }

  // Returns true if this object has been set.
  bool IsSet() const {
    // start_address can be zero for a non-PIE binary, but size can't be zero.
    return size != 0;
  }
  // Returns true if `addr` is in [start_address, start_address + size).
  bool InBounds(uintptr_t addr) const {
    return addr >= start_address && addr < start_address + size;
  }
};

// Returns DlInfo for the dynamic library who's exact path is `dl_path_suffix`.
// If `dl_path_suffix` is `nullptr`, returns DlInfo for the main binary.
// If the required library is not found, returns empty DlInfo (`!IsSet()`).
DlInfo GetDlInfo(const char* absl_nullable dl_path_suffix);

// Returns DlInfo for the dynamic library that contains `pc`.
DlInfo GetDlInfo(uintptr_t pc);

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_RUNNER_DL_INFO_H_
