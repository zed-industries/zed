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

#ifndef THIRD_PARTY_CENTIPEDE_FUZZTEST_MUTATOR_H_
#define THIRD_PARTY_CENTIPEDE_FUZZTEST_MUTATOR_H_

#include <cstddef>
#include <cstdint>
#include <memory>
#include <vector>

#include "./centipede/execution_metadata.h"
#include "./centipede/knobs.h"
#include "./centipede/mutation_input.h"
#include "./common/defs.h"

namespace fuzztest::internal {

// Mutator based on the FuzzTest std::vector domain.  It always
// generates non-empty results, with a default limit on the mutant
// size unless changed by `set_max_len`.
//
// This class is thread-compatible.
class FuzzTestMutator {
 public:
  // Initialize the mutator with the given `knobs` and RNG `seed`.
  explicit FuzzTestMutator(const Knobs &knobs, uint64_t seed);
  ~FuzzTestMutator();

  // Takes non-empty `inputs` and produces `num_mutants` mutants.
  std::vector<ByteArray> MutateMany(const std::vector<MutationInputRef> &inputs,
                                    size_t num_mutants);

  // Adds `dict_entries` to the internal mutation dictionary.
  void AddToDictionary(const std::vector<ByteArray>& dict_entries);

  // Sets max length in bytes for mutants with modified sizes.
  //
  // Returns false on invalid `max_len`, true otherwise.
  bool set_max_len(size_t max_len);

  // TODO(xinhaoyuan): Support set_alignment().

 private:
  struct MutationMetadata;
  class MutatorDomain;

  // Propagates the execution `metadata` to the internal mutation dictionary.
  void SetMetadata(const ExecutionMetadata& metadata);

  // The crossover algorithm based on the legacy ByteArrayMutator.
  // TODO(ussuri): Implement and use the domain level crossover.
  void CrossOverInsert(ByteArray &data, const ByteArray &other);
  void CrossOverOverwrite(ByteArray &data, const ByteArray &other);
  void CrossOver(ByteArray &data, const ByteArray &other);

  // Size limits on the cmp entries to be used in mutation.
  static constexpr uint8_t kMaxCmpEntrySize = 15;
  static constexpr uint8_t kMinCmpEntrySize = 2;

  const Knobs &knobs_;
  Rng prng_;
  size_t max_len_ = 1000;

  std::unique_ptr<MutationMetadata> mutation_metadata_;
  std::unique_ptr<MutatorDomain> domain_;
};

}  // namespace fuzztest::internal

#endif
