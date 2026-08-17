// Copyright 2023 The Centipede Authors.
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

#ifndef THIRD_PARTY_CENTIPEDE_DISTILL_H_
#define THIRD_PARTY_CENTIPEDE_DISTILL_H_

#include <cstddef>
#include <cstdint>
#include <vector>

#include "./centipede/environment.h"

namespace fuzztest::internal {

// Options for `Distill()`.
struct DistillOptions {
  // From each feature-equivalent set of inputs, select up to this many winners.
  uint8_t feature_frequency_threshold = 1;
};

// Reads `env.total_shards` input shards from `WorkDir{env}.CorpusFiles()` and
// `WorkDir{env}.FeaturesFiles()`, distills them, and writes out the winning
// inputs to `env.num_threads` output shards.
//
// All reads and writes are parallelized for higher throughput. A side effect of
// that is that the results are generally non-deterministic (for a given
// feature-equivalent set of inputs, any one can win and make it to the output).
//
// Returns EXIT_SUCCESS.
int Distill(const Environment &env, const DistillOptions &opts = {});

// Same as `Distill()`, but runs distillation without I/O parallelization and
// reads shards in the order specified by `shard_indices` for deterministic
// results.
void DistillForTests(const Environment &env,
                     const std::vector<size_t> &shard_indices);

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_DISTILL_H_
