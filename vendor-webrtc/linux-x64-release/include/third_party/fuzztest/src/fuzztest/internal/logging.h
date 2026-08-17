// Copyright 2022 Google LLC
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

#ifndef FUZZTEST_FUZZTEST_INTERNAL_LOGGING_H_
#define FUZZTEST_FUZZTEST_INTERNAL_LOGGING_H_

#include <cstdio>
#include <string>

#include "absl/strings/str_cat.h"

namespace fuzztest::internal {

// Returns the current FILE pointing to the original stderr.
FILE* GetStderr();

// Silences all output sent to stdout and stderr, except the fuzzer's own log.
// Fuzzer should log to GetStderr().
void SilenceTargetStdoutAndStderr();

// Revive the silenced stdout and stderr of target.
// Also redirect GetStderr() to stderr.
void RestoreTargetStdoutAndStderr();

// Check if FUZZTEST_SILENCE_TARGET env set.
bool IsSilenceTargetEnabled();

[[noreturn]] void Abort(const char* file, int line, const std::string& message);

#define FUZZTEST_INTERNAL_CHECK_PRECONDITION(P, ...) \
  ((P) ? (void)0                                     \
       : ::fuzztest::internal::Abort(                \
             __FILE__, __LINE__,                     \
             absl::StrCat("Failed precondition (", #P, "): ", __VA_ARGS__)))

#define FUZZTEST_INTERNAL_CHECK(cond, ...)                     \
  ((cond) ? (void)0                                            \
          : ::fuzztest::internal::Abort(                       \
                __FILE__, __LINE__,                            \
                absl::StrCat("Internal error! Check (", #cond, \
                             ") failed: ", __VA_ARGS__)))

// This Abort function will inject `message` into the signal handler's output
// along with the file, line and test name of the currently running test.
// Meant for failures related to test setup and not the code under test itself.
[[noreturn]] void AbortInTest(const std::string& message);
extern const std::string* volatile test_abort_message;

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_LOGGING_H_
