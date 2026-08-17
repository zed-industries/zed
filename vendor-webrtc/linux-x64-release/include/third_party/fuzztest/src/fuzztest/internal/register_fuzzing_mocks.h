// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef FUZZTEST_FUZZTEST_INTERNAL_REGISTER_FUZZING_MOCKS_H_
#define FUZZTEST_FUZZTEST_INTERNAL_REGISTER_FUZZING_MOCKS_H_

#include <cstdint>

#include "absl/base/fast_type_id.h"
#include "absl/functional/function_ref.h"
#include "absl/types/span.h"

namespace fuzztest::internal {

// TypeErasedFuzzFunctionT(datastream, args_tuple, result) is a type erased
// function pointer for use with absl::MockingBitGen and fuzztest mocking.
using TypeErasedFuzzFunctionT = void (*)(absl::Span<const uint8_t>&, void*,
                                         void*);

// Registers the fuzzing functions for Abseil distributions.
void RegisterAbslRandomFuzzingMocks(
    absl::FunctionRef<void(absl::FastTypeIdType, TypeErasedFuzzFunctionT)>
        register_fn);

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_REGISTER_FUZZING_MOCKS_H_
