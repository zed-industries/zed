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

#ifndef THIRD_PARTY_CENTIPEDE_SHARD_READER_H_
#define THIRD_PARTY_CENTIPEDE_SHARD_READER_H_

#include <functional>
#include <string>
#include <string_view>

#include "absl/types/span.h"
#include "./centipede/feature.h"
#include "./common/defs.h"

namespace fuzztest::internal {

// `corpus_path` is a path to a BlobFile with corpus elements (inputs). If the
// path is empty or non-existent, no processing is done.
//
// `features_path` is a path to a BlobFile with {features/hash} pairs created by
// `PackFeaturesAndHash()`. If the path is empty or non-existent, an empty
// `FeatureVec` is passed to every call of `callback`.
//
// For every {features/hash} pair we need to find an input with this hash.
// This function reads `corpus_path` and `features_path` and calls `callback`
// on every pair {input, features}.
//
// If features are not found for a given input, callback's 2nd argument is {}.
//
// If features are found for a given input but are empty,
// then callback's 2nd argument is {feature_domains::kNoFeature}.
void ReadShard(std::string_view corpus_path, std::string_view features_path,
               const std::function<void(ByteArray, FeatureVec)> &callback);

// Unpacks the corpus from `sharded_file_paths` and writes each input to an
// individual file in `out_dir`. The file names are the inputs' hashes.
void ExportCorpus(absl::Span<const std::string> sharded_file_paths,
                  std::string_view out_dir);

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_SHARD_READER_H_
