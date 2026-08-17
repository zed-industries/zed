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

#ifndef FUZZTEST_CENTIPEDE_TEST_COVERAGE_UTIL_H_
#define FUZZTEST_CENTIPEDE_TEST_COVERAGE_UTIL_H_

#include <cstddef>
#include <cstdlib>
#include <string>
#include <string_view>
#include <vector>

#include "absl/log/check.h"
#include "./centipede/centipede_callbacks.h"
#include "./centipede/corpus.h"
#include "./centipede/environment.h"
#include "./centipede/feature.h"
#include "./centipede/mutation_input.h"
#include "./centipede/runner_result.h"
#include "./common/defs.h"
namespace fuzztest::internal {
// Runs all `inputs`, returns FeatureVec for every input.
// `env` defines what target is executed and with what flags.
std::vector<CorpusRecord> RunInputsAndCollectCorpusRecords(
    const Environment &env, const std::vector<std::string> &inputs);

// Runs all `inputs`, returns a CorpusRecord for every input.
// `env` defines what target is executed and with what flags.
std::vector<FeatureVec> RunInputsAndCollectCoverage(
    const Environment &env, const std::vector<std::string> &inputs);

// A simple CentipedeCallbacks derivative.
class TestCallbacks : public CentipedeCallbacks {
 public:
  explicit TestCallbacks(const Environment &env) : CentipedeCallbacks(env) {}
  bool Execute(std::string_view binary, const std::vector<ByteArray> &inputs,
               BatchResult &batch_result) override {
    int result =
        ExecuteCentipedeSancovBinaryWithShmem(binary, inputs, batch_result);
    CHECK_EQ(EXIT_SUCCESS, result);
    return true;
  }
  std::vector<ByteArray> Mutate(const std::vector<MutationInputRef> &inputs,
                                size_t num_mutants) override {
    return {};
  }
};
}  // namespace fuzztest::internal

#endif  // FUZZTEST_CENTIPEDE_TEST_COVERAGE_UTIL_H_
