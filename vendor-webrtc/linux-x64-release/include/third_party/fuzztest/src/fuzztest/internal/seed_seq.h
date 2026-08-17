// Copyright 2022 Google LLC
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

#ifndef FUZZTEST_FUZZTEST_INTERNAL_SEED_SEQ_H_
#define FUZZTEST_FUZZTEST_INTERNAL_SEED_SEQ_H_

#include <cstdint>
#include <optional>
#include <ostream>
#include <random>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "absl/types/span.h"

namespace fuzztest::internal {

// Returns the seed sequence decoded from the value associated with the
// environment variable `env_var`. If the environment variable `env_var` is not
// set in the environment, returns a new random seed sequence. In either case,
// outputs the encoding of the returned seed sequence to `out`.
//
// Note: The return type is `std::seed_seq`, and not `absl::SeedSeq`, since
// Abseil doesn't guarantee seed stability
// (https://abseil.io/docs/cpp/guides/random#seed-stability).
std::seed_seq GetFromEnvOrMakeSeedSeq(
    std::ostream& out, absl::string_view env_var = "FUZZTEST_PRNG_SEED");

// Returns an encoding of `seed_material` as a displayable string.
//
// Note: This function is exported so that it can be used in unit tests. Client
// code should use `GetFromEnvOrMakeSeedSeq()` instead.
std::string EncodeSeedMaterial(absl::Span<const uint32_t> seed_material);

// Returns a sequence of integers decoded from `seed_material`.  If
// `seed_material` is not an encoding of a sequence of integers obtained from
// `EncodeSeedMaterial()`, returns `std::nullopt`.
//
// Note: This function is exported so that it can be used in unit tests. Client
// code should use `GetFromEnvOrMakeSeedSeq()` instead.
std::optional<std::vector<uint32_t>> DecodeSeedMaterial(
    absl::string_view seed_material);

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_SEED_SEQ_H_
