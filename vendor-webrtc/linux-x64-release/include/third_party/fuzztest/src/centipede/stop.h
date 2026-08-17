// Copyright 2023 The Centipede Authors.
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

#ifndef THIRD_PARTY_CENTIPEDE_STOP_H_
#define THIRD_PARTY_CENTIPEDE_STOP_H_

#include "absl/time/time.h"

namespace fuzztest::internal {

// Clears the request to stop early and sets the stop time.
//
// REQUIRES: Must be called before starting concurrent threads that may invoke
// the functions defined in this header. In particular, calling this function
// concurrently with `ShouldStop()` is not thread-safe.
void ClearEarlyStopRequestAndSetStopTime(absl::Time stop_time);

// Requests that Centipede soon stops whatever it is doing (fuzzing, minimizing
// reproducer, etc.), with `exit_code` indicating success (zero) or failure
// (non-zero).
//
// ENSURES: Thread-safe and safe to call from signal handlers.
void RequestEarlyStop(int exit_code);

// Returns whether `RequestEarlyStop()` was called or not since the most recent
// call to `ClearEarlyStopRequestAndSetStopTime()` (if any).
//
// ENSURES: Thread-safe.
bool EarlyStopRequested();

// Returns true iff it is time to stop, either because the stopping time has
// been reached or `RequestEarlyStop()` was called since the most recent call to
// `ClearEarlyStopRequestAndSetStopTime()` (if any).
//
// ENSURES: Thread-safe.
bool ShouldStop();

// Returns the value most recently passed to `RequestEarlyStop()` or 0 if
// `RequestEarlyStop()` was not called since the most recent call to
// `ClearEarlyStopRequestAndSetStopTime()` (if any).
//
// ENSURES: Thread-safe.
int ExitCode();

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_STOP_H_
