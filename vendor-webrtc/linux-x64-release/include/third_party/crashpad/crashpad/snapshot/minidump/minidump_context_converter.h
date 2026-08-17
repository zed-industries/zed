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

#ifndef CRASHPAD_SNAPSHOT_MINIDUMP_MINIDUMP_CONTEXT_CONVERTER_H_
#define CRASHPAD_SNAPSHOT_MINIDUMP_MINIDUMP_CONTEXT_CONVERTER_H_

#include <vector>

#include "snapshot/cpu_context.h"
#include "util/misc/initialization_state.h"
#include "util/misc/initialization_state_dcheck.h"

namespace crashpad {
namespace internal {

class MinidumpContextConverter {
 public:
  MinidumpContextConverter();

  bool Initialize(CPUArchitecture arch,
                  const std::vector<unsigned char>& minidump_context);
  const CPUContext* Get() const {
    INITIALIZATION_STATE_DCHECK_VALID(initialized_);
    return &context_;
  }

 private:
  CPUContext context_;
  std::vector<unsigned char> context_memory_;
  InitializationStateDcheck initialized_;
};

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_SNAPSHOT_MINIDUMP_MINIDUMP_CONTEXT_CONVERTER_H_
