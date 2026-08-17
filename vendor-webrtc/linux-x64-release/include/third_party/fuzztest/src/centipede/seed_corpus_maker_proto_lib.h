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

#ifndef THIRD_PARTY_CENTIPEDE_SEED_CORPUS_MAKER_PROTO_LIB_H_
#define THIRD_PARTY_CENTIPEDE_SEED_CORPUS_MAKER_PROTO_LIB_H_

#include <string_view>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "./centipede/seed_corpus_maker_lib.h"

namespace fuzztest::internal {

// Generates seed corpus similarly to `GenerateSeedCorpusFromConfig`, but using
// `config_spec` based on its value: If a file with `config_spec` path exists,
// tries to parse it as a `proto::SeedCorpusConfig` textproto. Otherwise, tries
// to parse `config_spec` as a verbatim `proto::SeedCorpusConfig` textproto.
// Resolves any relative paths and globs in the config fields to absolute ones,
// using as the base dir either the file's parent dir (if `config_spec` is a
// file) or the current dir (otherwise). If `override_out_dir` is non-empty, it
// overrides `destination.dir_path` in the resolved config.
absl::Status GenerateSeedCorpusFromConfigProto(  //
    std::string_view config_spec,                //
    std::string_view coverage_binary_name,       //
    std::string_view coverage_binary_hash,       //
    std::string_view override_out_dir);

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_SEED_CORPUS_MAKER_PROTO_LIB_H_
