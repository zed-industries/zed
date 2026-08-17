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

#ifndef THIRD_PARTY_CENTIPEDE_SEED_CORPUS_MAKER_LIB_H_
#define THIRD_PARTY_CENTIPEDE_SEED_CORPUS_MAKER_LIB_H_

#include <cstdint>
#include <iostream>
#include <string>
#include <string_view>
#include <utility>
#include <variant>
#include <vector>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "./centipede/feature.h"
#include "./common/defs.h"

namespace fuzztest::internal {

// Native struct used by the seed corpus library for seed corpus source.
//
// TODO(b/362576261): Currently this is mirroring the `proto::SeedCorpusSource`
// proto. But in the future it may change with the core seeding API - any
// difference is commented below.
struct SeedCorpusSource {
  std::string dir_glob;
  uint32_t num_recent_dirs;
  std::string shard_rel_glob;
  // If non-empty, will be used to glob the individual input files (with one
  // input in each file) in the source dirs. Any files matching `shard_rel_glob`
  // will be skipped.
  std::string individual_input_rel_glob;
  std::variant<float, uint32_t> sampled_fraction_or_count;
};

// Native struct used by the seed corpus library for seed corpus destination.
//
// TODO(b/362576261): Currently this is mirroring the
// `proto::SeedCorpusDestination` proto. But in the future it may change with
// the core seeding API.
struct SeedCorpusDestination {
  std::string dir_path;
  std::string shard_rel_glob;
  uint32_t shard_index_digits;
  uint32_t num_shards;
};

// Native struct used by the seed corpus library for seed corpus configuration.
//
// TODO(b/362576261): Currently this is mirroring the `proto::SeedCorpusConfig`
// proto. But in the future it may change with the core seeding API.
struct SeedCorpusConfig {
  std::vector<SeedCorpusSource> sources;
  SeedCorpusDestination destination;
};

using InputAndFeatures = std::pair<ByteArray, FeatureVec>;
using InputAndFeaturesVec = std::vector<InputAndFeatures>;

// Extracts a sample of corpus elements from `source` and appends the results to
// `elements`. `source` defines the locations of the corpus shards and the size
// of the sample.
//
// `coverage_binary_name` should be the basename of the coverage binary for
// which the seed corpus is to be created, and the `coverage_binary_hash` should
// be the hash of that binary. If a corpus shard file found in the source
// directory contains a matching features shard file in the
// <coverage_binary_name>-<coverage_binary_hash> subdir, the matching features
// will be copied over to `elements`; otherwise, and empty `FeatureVec` will be
// used instead.
absl::Status SampleSeedCorpusElementsFromSource(  //
    const SeedCorpusSource& source,               //
    std::string_view coverage_binary_name,        //
    std::string_view coverage_binary_hash,        //
    InputAndFeaturesVec& elements);

// Writes seed corpus `elements` to `destination`. Any previously existing
// corpus shard files matching `destination.shard_glob()` will be deleted
// before writing (even if writing subsequently fails).
//
// `coverage_binary_name` should be the basename of the coverage binary for
// which the seed corpus is to be created, and the `coverage_binary_hash` should
// be the hash of that binary. The features in each `FeatureVec` of the
// `elements` will be saved to a features shard file under
// <coverage_binary_name>-<coverage_binary_hash> subdir of the destination.
absl::Status WriteSeedCorpusElementsToDestination(  //
    const InputAndFeaturesVec& elements,            //
    std::string_view coverage_binary_name,          //
    std::string_view coverage_binary_hash,          //
    const SeedCorpusDestination& destination);

// Reads and samples seed corpus elements from all the sources and writes the
// results to the destination, as defined in `config`. The paths and globs in
// `config` can be relative paths: in that case, they are resolved to absolute
// using as the base dir.
//
// `coverage_binary_name` should be the basename of the coverage binary for
// which the seed corpus is to be created, and the `coverage_binary_hash` should
// be the hash of that binary. The features matching each sampled source corpus
// element will be copied over from the
// <coverage_binary_name>-<coverage_binary_hash> subdir of the source to the
// same subdir of the destination.
absl::Status GenerateSeedCorpusFromConfig(  //
    const SeedCorpusConfig& config,         //
    std::string_view coverage_binary_name,  //
    std::string_view coverage_binary_hash);

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_SEED_CORPUS_MAKER_LIB_H_
