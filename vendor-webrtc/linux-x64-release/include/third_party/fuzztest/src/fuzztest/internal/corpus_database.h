// Copyright 2023 Google LLC
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

#ifndef FUZZTEST_FUZZTEST_INTERNAL_CORPUS_DATABASE_H_
#define FUZZTEST_FUZZTEST_INTERNAL_CORPUS_DATABASE_H_

#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "./fuzztest/internal/configuration.h"

namespace fuzztest::internal {

// Encapsulates a file-system-based corpus database that contains the coverage,
// regression, and crashing inputs for each fuzz test in a test binary.
class CorpusDatabase {
 public:
  // Constructs a corpus database for `binary_identifier` located at
  // `database_path`. `database_path` can be absolute or relative to the path
  // given by the environment variable TEST_SRCDIR, and `binary_identifier` is
  // the test binary's relative path within `database_path`.
  //
  // The parameter `use_crashing_inputs` controls whether the database gives
  // access to or ignores the crashing inputs.
  explicit CorpusDatabase(absl::string_view database_path,
                          absl::string_view binary_identifier,
                          bool use_crashing_inputs);

  // Constructs a corpus database directly from `configuration`.
  explicit CorpusDatabase(const Configuration& configuration);

  // Returns set of all regression inputs from `corpus_database` for a fuzz
  // test.
  std::vector<std::string> GetRegressionInputs(
      absl::string_view test_name) const;

  // Returns set of all corpus inputs from `corpus_database` for a fuzz test.
  std::vector<std::string> GetCoverageInputsIfAny(
      absl::string_view test_name) const;

  // Returns set of all crashing inputs from `corpus_database` for a fuzz test.
  // Returns an empty set when `use_crashing_inputs_` is false.
  std::vector<std::string> GetCrashingInputsIfAny(
      absl::string_view test_name) const;

  bool use_crashing_inputs() const { return use_crashing_inputs_; }

 private:
  std::string corpus_path_for_test_binary_;
  bool use_crashing_inputs_ = false;
};

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_CORPUS_DATABASE_H_
