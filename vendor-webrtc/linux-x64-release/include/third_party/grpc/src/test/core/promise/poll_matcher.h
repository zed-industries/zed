// Copyright 2024 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef GRPC_TEST_CORE_PROMISE_POLL_MATCHER_H
#define GRPC_TEST_CORE_PROMISE_POLL_MATCHER_H

#include "gmock/gmock.h"

// Various gmock matchers for Poll

namespace grpc_core {

// Expect that a promise is still pending:
// EXPECT_THAT(some_promise(), IsPending());
MATCHER(IsPending, "") {
  if (arg.ready()) {
    *result_listener << "is ready";
    return false;
  }
  return true;
}

// Expect that a promise is ready:
// EXPECT_THAT(some_promise(), IsReady());
MATCHER(IsReady, "") {
  if (arg.pending()) {
    *result_listener << "is pending";
    return false;
  }
  return true;
}

// Expect that a promise is ready with a specific value:
// EXPECT_THAT(some_promise(), IsReady(value));
MATCHER_P(IsReady, value, "") {
  if (arg.pending()) {
    *result_listener << "is pending";
    return false;
  }
  if (arg.value() != value) {
    *result_listener << "is " << ::testing::PrintToString(arg.value());
    return false;
  }
  return true;
}

}  // namespace grpc_core

#endif  // GRPC_TEST_CORE_PROMISE_POLL_MATCHER_H
