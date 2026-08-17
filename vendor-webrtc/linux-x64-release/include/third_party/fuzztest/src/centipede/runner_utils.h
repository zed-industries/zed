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

#ifndef THIRD_PARTY_CENTIPEDE_RUNNER_UTILS_H_
#define THIRD_PARTY_CENTIPEDE_RUNNER_UTILS_H_

#include <cstdint>

#include "absl/base/nullability.h"

namespace fuzztest::internal {

// If `condition` prints `error` and calls exit(1).
// TODO(kcc): change all uses of PrintErrorAndExitIf() to RunnerCheck()
// as it is a more common pattern.
void PrintErrorAndExitIf(bool condition, const char* absl_nonnull error);

// A rough equivalent of "CHECK(condition) << error;".
inline void RunnerCheck(bool condition, const char* absl_nonnull error) {
  PrintErrorAndExitIf(!condition, error);
}

// Returns the lower bound of the stack region for the current thread. 0 will be
// returned on failures.
uintptr_t GetCurrentThreadStackRegionLow();

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_RUNNER_UTILS_H_
