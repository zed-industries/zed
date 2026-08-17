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

#ifndef FUZZTEST_FUZZTEST_INTERNAL_CONFIGURATION_H_
#define FUZZTEST_FUZZTEST_INTERNAL_CONFIGURATION_H_

#include <cstddef>
#include <functional>
#include <optional>
#include <string>
#include <vector>

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "absl/time/time.h"

namespace fuzztest::internal {

enum class TimeBudgetType { kPerTest, kTotal };

// To be used by ABSL_FLAG for parsing TimeBudgetType.
bool AbslParseFlag(absl::string_view text, TimeBudgetType* mode,
                   std::string* error);

// To be used by ABSL_FLAG for stringifying TimeBudgetType.
std::string AbslUnparseFlag(TimeBudgetType mode);

// Returns the TimeBudgetType from flag string values. Returns an error if the
// string is not representing a valid TimeBudgetType.
absl::StatusOr<TimeBudgetType> ParseTimeBudgetType(absl::string_view text);

// The configuration of a fuzz test.
struct Configuration {
  // The location of the database that contains coverage, regression, and
  // crashing inputs for each test binary and fuzz test in the project (eg.,
  // ~/.cache/fuzztest).
  std::string corpus_database;
  // The directory path to export stats with a layout similar to
  // `corpus_database`.
  std::string stats_root;
  // The root directory for Centipede workdirs, with a layout similar to
  // `corpus_database`.
  std::string workdir_root;
  // The identifier of the test binary in the corpus database (eg.,
  // relative/path/to/binary).
  std::string binary_identifier;
  // The fuzz tests in the test binary.
  std::vector<std::string> fuzz_tests;
  // The fuzz tests in the current shard.
  std::vector<std::string> fuzz_tests_in_current_shard;
  // Generate separate TESTs that replay crashing inputs for the selected fuzz
  // tests.
  bool reproduce_findings_as_separate_tests = false;
  // When working on a corpus database, a few steps can be performed for each
  // test:
  //   1. Replaying inputs from the database, by default the regression inputs
  //      will be used.
  //   2. Fuzzing with generated inputs, some of which trigger crashes.
  //   3. Updating the database with the coverage-increasing and crashing
  //      inputs.
  //
  // If set, coverage inputs are included for replaying.
  bool replay_coverage_inputs = false;
  // If set, further steps are skipped after replaying.
  bool only_replay = false;
  // If set, replay without spawning subprocesses.
  bool replay_in_single_process = false;
  // If set, will be used when working on a corpus database to resume
  // the progress in case the execution got interrupted.
  std::optional<std::string> execution_id;
  // If set, print log from subprocesses spawned by FuzzTest.
  bool print_subprocess_log = false;

  // Stack limit in bytes.
  size_t stack_limit = 128 * 1024;
  // RSS limit in bytes. Zero indicates no limit.
  size_t rss_limit = 0;
  // Time limit per test input.
  absl::Duration time_limit_per_input = absl::InfiniteDuration();
  // Fuzzing or corpus replay time limit.
  absl::Duration time_limit = absl::InfiniteDuration();
  // Whether the time limit is for each test or for all tests in the binary.
  TimeBudgetType time_budget_type = TimeBudgetType::kPerTest;
  // The number of fuzzing jobs to run in parallel. Zero indicates that the
  // number of jobs is unspecified by the test binary.
  size_t jobs = 0;

  // When set, `FuzzTestFuzzer` replays only one input (no fuzzing is done).
  std::optional<std::string> crashing_input_to_reproduce;

  // A command template that could be used to replay a crashing input.
  // The reproduction command template must have the following place holders:
  // - $TEST_FILTER: for replaying only a subset of the tests in a binary.
  std::optional<std::string> reproduction_command_template;

  // Preprocessing step for reproducing crashing input.
  // Note: This field is not serialized and deserialized.
  // TODO(b/329709054): Consider eliminating the field.
  std::function<void()> preprocess_crash_reproducing = [] {};

  std::string Serialize() const;

  absl::Duration GetTimeLimitPerTest() const;

  static absl::StatusOr<Configuration> Deserialize(
      absl::string_view serialized);
};

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_CONFIGURATION_H_
