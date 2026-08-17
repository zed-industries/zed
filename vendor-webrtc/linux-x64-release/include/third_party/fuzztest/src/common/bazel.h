// Copyright 2024 The Centipede Authors.
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

#ifndef FUZZTEST_COMMON_BAZEL_H_
#define FUZZTEST_COMMON_BAZEL_H_

#include "absl/status/status.h"
#include "absl/time/time.h"

namespace fuzztest::internal {

struct TestShard {
  int index = 0;
  int total_shards = 1;
};

// Get Bazel sharding information based on
// https://bazel.build/reference/test-encyclopedia#initial-conditions
TestShard GetBazelTestShard();

// Returns Ok if there is enough time left to run a single test for
// `test_time_limit` given `target_start_time` and timeout and sharding
// information from Bazel, or returns an error status otherwise. It uses
// `executed_tests_in_shard` and `fuzz_test_count` to include suggestions into
// the error status message.
absl::Status VerifyBazelHasEnoughTimeToRunTest(absl::Time target_start_time,
                                               absl::Duration test_time_limit,
                                               int executed_tests_in_shard,
                                               int fuzz_test_count);

}  // namespace fuzztest::internal

#endif  // FUZZTEST_COMMON_BAZEL_H_
