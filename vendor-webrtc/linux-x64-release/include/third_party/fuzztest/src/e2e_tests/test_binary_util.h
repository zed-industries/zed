// Copyright 2024 Google LLC
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

#ifndef FUZZTEST_E2E_TESTS_TEST_BINARY_UTIL_H_
#define FUZZTEST_E2E_TESTS_TEST_BINARY_UTIL_H_

#include <string>

#include "absl/container/flat_hash_map.h"
#include "absl/strings/string_view.h"
#include "absl/time/time.h"
#include "./fuzztest/internal/subprocess.h"

namespace fuzztest::internal {

// Returns a string of the form "--flag_name=flag_value" (or "--flag_name" if
// `flag_value` is empty), but with the flag name transformed to take into
// account any internal flag prefix.
std::string CreateFuzzTestFlag(absl::string_view flag_name,
                               absl::string_view flag_value);

// Returns the full path to the test binary based on the path relative to the
// e2e_tests directory. The test binary's name must end with the suffix
// ".stripped", but the function doesn't require the suffix in `relative_path`.
// If the suffix is missing, it will be added.
std::string BinaryPath(absl::string_view relative_path);

// Returns the full path to the Centipede binary.
std::string CentipedePath();

struct RunOptions {
  // General flags to pass to the binary. Useful when passing flags to
  // binaries like Centipede.
  absl::flat_hash_map<std::string, std::string> flags;
  // Flags to pass to a FuzzTest binary. These should be given without the
  // internal flag prefix, e.g., just ("fuzz_for", "1s").
  absl::flat_hash_map<std::string, std::string> fuzztest_flags;
  // Environment variables to pass to the binary.
  absl::flat_hash_map<std::string, std::string> env;
  // Duration after which the binary will be terminated.
  absl::Duration timeout = absl::Minutes(10);
};

// Runs the binary given by its full `binary_path` in a subprocess and returns
// the results. The `options` parameter can be used to pass flags and
// environment variables to the binary.
RunResults RunBinary(absl::string_view binary_path,
                     const RunOptions& options = {});

}  // namespace fuzztest::internal

#endif  // FUZZTEST_E2E_TESTS_TEST_BINARY_UTIL_H_
