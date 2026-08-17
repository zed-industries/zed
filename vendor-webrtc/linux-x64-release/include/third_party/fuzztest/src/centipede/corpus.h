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

#ifndef THIRD_PARTY_CENTIPEDE_CORPUS_H_
#define THIRD_PARTY_CENTIPEDE_CORPUS_H_

#include <cstddef>
#include <cstdint>
#include <ostream>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

#include "absl/log/check.h"
#include "./centipede/binary_info.h"
#include "./centipede/execution_metadata.h"
#include "./centipede/feature.h"
#include "./centipede/feature_set.h"
#include "./centipede/util.h"
#include "./common/defs.h"

namespace fuzztest::internal {

// WeightedDistribution maintains an array of integer weights.
// It allows to compute a random number in range [0,size()) such that
// the probability of each number is proportional to its weight.
class WeightedDistribution {
 public:
  // Adds one more weight.
  void AddWeight(uint64_t weight);
  // Removes the last weight and returns it.
  // Precondition: size() > 0.
  uint64_t PopBack();
  // Changes the existing idx-th weight to new_weight.
  void ChangeWeight(size_t idx, uint64_t new_weight);
  // Returns a random number in [0,size()), using a random number `random`.
  // For proper randomness, `random` should come from a 64-bit RNG.
  // RandomIndex() must not be called after ChangeWeight() without first
  // calling RecomputeInternalState().
  size_t RandomIndex(size_t random) const;
  // Returns the number of weights.
  size_t size() const { return weights_.size(); }
  // Removes all weights.
  void clear() {
    weights_.clear();
    cumulative_weights_.clear();
  }
  // Fixes the internal state that could become stale after call(s) to
  // ChangeWeight().
  void RecomputeInternalState();

  // Computes a random weighted subset of elements to remove.
  // Removes this subset from `this`.
  // Returns the subset as a sorted array of indices.
  std::vector<size_t> RemoveRandomWeightedSubset(size_t target_size, Rng &rng) {
    auto subset_to_remove = RandomWeightedSubset(weights_, target_size, rng);
    RemoveSubset(subset_to_remove, weights_);
    RemoveSubset(subset_to_remove, cumulative_weights_);
    return subset_to_remove;
  }

 private:
  // The array of weights. The probability of choosing the index Idx
  // is weights_[Idx] / SumOfAllWeights.
  std::vector<uint64_t> weights_;
  // i-th element is the sum of the first i elements of weights_.
  std::vector<uint64_t> cumulative_weights_;
  // If false, cumulative_weights_ needs to be recomputed.
  bool cumulative_weights_valid_ = true;
};

class CoverageFrontier;  // Forward decl, used in Corpus.

// Input data and metadata.
struct CorpusRecord {
  ByteArray data;
  FeatureVec features;
  ExecutionMetadata metadata;
};

// Maintains the corpus of inputs.
// Allows to prune (forget) inputs that become uninteresting.
class Corpus {
 public:
  Corpus() = default;

  Corpus(const Corpus &) = default;
  Corpus(Corpus &&) noexcept = default;
  Corpus &operator=(const Corpus &) = default;
  Corpus &operator=(Corpus &&) noexcept = default;

  // Mutators.

  // Adds a corpus element, consisting of 'data' (the input bytes, non-empty),
  // 'fv' (the features associated with this input), and execution `metadata`.
  // `fs` is used to compute weights of `fv`.
  void Add(const ByteArray &data, const FeatureVec &fv,
           const ExecutionMetadata &metadata, const FeatureSet &fs,
           const CoverageFrontier &coverage_frontier);
  // Removes elements that contain only frequent features, according to 'fs'.
  // Also, randomly removes elements to reduce the size to <= `max_corpus_size`.
  // `max_corpus_size` should be positive.
  // Returns the number of removed elements.
  size_t Prune(const FeatureSet &fs, const CoverageFrontier &coverage_frontier,
               size_t max_corpus_size, Rng &rng);

  // Accessors.

  // Returns the inputs.
  const std::vector<CorpusRecord> &Records() const { return records_; }
  // Returns the total number of inputs added.
  size_t NumTotal() const { return num_pruned_ + NumActive(); }
  // Return the number of currently active inputs, i.e. inputs that we want to
  // keep mutating.
  size_t NumActive() const { return records_.size(); }
  // Returns the max and avg sizes of the inputs.
  std::pair<size_t, size_t> MaxAndAvgSize() const;
  // Returns a random active corpus record using weighted distribution.
  // See WeightedDistribution.
  const CorpusRecord &WeightedRandom(size_t random) const;
  // Returns a random active corpus record using uniform distribution.
  const CorpusRecord &UniformRandom(size_t random) const;
  // Returns the element with index 'idx', where `idx` < NumActive().
  const ByteArray &Get(size_t idx) const { return records_[idx].data; }
  // Returns the execution metadata for the element `idx`, `idx` < NumActive().
  const ExecutionMetadata &GetMetadata(size_t idx) const {
    return records_[idx].metadata;
  }

  // Logging.

  // Saves the corpus stats in JSON format to the `filepath` file, using `fs`
  // for feature frequencies.
  void DumpStatsToFile(const FeatureSet &fs, std::string_view filepath,
                       std::string_view description);
  // Returns a string used for logging the corpus memory usage.
  std::string MemoryUsageString() const;

 private:
  std::vector<CorpusRecord> records_;
  // Maintains weights for elements of records_.
  WeightedDistribution weighted_distribution_;
  size_t num_pruned_ = 0;
};

// Coverage frontier is a set of PCs that are themselves covered, but some of
// adjacent PCs in the same function are not.
// This class identifies precise frontiers. Each frontier is assigned a weight.
// Frontier weight is a representation of how much code is behind the
// frontier. Therefore, it should be used to prioritize which frontier to focus
// first.
class CoverageFrontier {
 public:
  explicit CoverageFrontier(const BinaryInfo &binary_info)
      : binary_info_(binary_info),
        frontier_(binary_info.pc_table.size()),
        frontier_weight_(binary_info.pc_table.size()) {}

  // Computes the coverage frontier of `corpus`.
  // Returns the number of functions in the frontier.
  size_t Compute(const Corpus &corpus);

  // Same as above.
  size_t Compute(const std::vector<CorpusRecord> &corpus_records);

  // Returns the number of functions in the frontier.
  size_t NumFunctionsInFrontier() const { return num_functions_in_frontier_; }

  // Returns true iff `idx` belongs to the frontier.
  bool PcIndexIsFrontier(size_t idx) const {
    CHECK_LT(idx, MaxPcIndex());
    return frontier_[idx];
  }

  // Returns the size of the pc_table used to create `this`.
  size_t MaxPcIndex() const { return binary_info_.pc_table.size(); }

  // Returns the frontier weight of pc at `idx`, weight of a non-frontier is 0.
  uint64_t FrontierWeight(size_t idx) const {
    CHECK_LT(idx, MaxPcIndex());
    return frontier_weight_[idx];
  }

 private:
  const BinaryInfo &binary_info_;

  // frontier_[idx] is true iff pc_table_[i] is part of the coverage frontier.
  std::vector<bool> frontier_;
  // Stores the weight associated with frontier_[idx].
  std::vector<uint64_t> frontier_weight_;

  // The number of functions in the frontier.
  size_t num_functions_in_frontier_ = 0;
};

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_CORPUS_H_
