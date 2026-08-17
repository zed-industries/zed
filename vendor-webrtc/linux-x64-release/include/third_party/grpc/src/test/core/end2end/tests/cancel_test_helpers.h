//
//
// Copyright 2015 gRPC authors.
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
//
//

#ifndef GRPC_TEST_CORE_END2END_TESTS_CANCEL_TEST_HELPERS_H
#define GRPC_TEST_CORE_END2END_TESTS_CANCEL_TEST_HELPERS_H

#include <grpc/status.h>

#include "test/core/end2end/end2end_tests.h"

namespace grpc_core {
class CancellationMode {
 public:
  virtual void Apply(CoreEnd2endTest::Call& call) = 0;
  virtual grpc_status_code ExpectedStatus() = 0;
  virtual ~CancellationMode() = default;
};

class CancelCancellationMode : public CancellationMode {
 public:
  void Apply(CoreEnd2endTest::Call& call) override { call.Cancel(); }
  grpc_status_code ExpectedStatus() override { return GRPC_STATUS_CANCELLED; }
};

class DeadlineCancellationMode : public CancellationMode {
 public:
  void Apply(CoreEnd2endTest::Call&) override {}
  grpc_status_code ExpectedStatus() override {
    return GRPC_STATUS_DEADLINE_EXCEEDED;
  }
};
}  // namespace grpc_core

#endif  // GRPC_TEST_CORE_END2END_TESTS_CANCEL_TEST_HELPERS_H
