// Copyright 2022 The gRPC Authors
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

#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_POSIX_ENGINE_CLOSURE_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_POSIX_ENGINE_CLOSURE_H
#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>

#include <utility>

#include "absl/functional/any_invocable.h"
#include "absl/status/status.h"

namespace grpc_event_engine::experimental {

// The callbacks for Endpoint read and write take an absl::Status as
// argument - this is important for the tcp code to function correctly. We need
// a custom closure type because the default EventEngine::Closure type doesn't
// provide a way to pass a status when the callback is run.
class PosixEngineClosure final
    : public grpc_event_engine::experimental::EventEngine::Closure {
 public:
  PosixEngineClosure() = default;
  PosixEngineClosure(absl::AnyInvocable<void(absl::Status)> cb,
                     bool is_permanent)
      : cb_(std::move(cb)),
        is_permanent_(is_permanent),
        status_(absl::OkStatus()) {}
  ~PosixEngineClosure() final = default;
  void SetStatus(absl::Status status) { status_ = status; }
  void Run() override {
    // We need to read the is_permanent_ variable before executing the
    // enclosed callback. This is because a permanent closure may delete this
    // object within the callback itself and thus reading this variable after
    // the callback execution is not safe.
    if (!is_permanent_) {
      cb_(std::exchange(status_, absl::OkStatus()));
      delete this;
    } else {
      cb_(std::exchange(status_, absl::OkStatus()));
    }
  }

  // This closure clean doesn't itself up after execution by default. The caller
  // should take care if its lifetime.
  static PosixEngineClosure* ToPermanentClosure(
      absl::AnyInvocable<void(absl::Status)> cb) {
    return new PosixEngineClosure(std::move(cb), true);
  }

  // This closure clean's itself up after execution. It is expected to be
  // used only in tests.
  static PosixEngineClosure* TestOnlyToClosure(
      absl::AnyInvocable<void(absl::Status)> cb) {
    return new PosixEngineClosure(std::move(cb), false);
  }

 private:
  absl::AnyInvocable<void(absl::Status)> cb_;
  bool is_permanent_ = false;
  absl::Status status_;
};

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_POSIX_ENGINE_CLOSURE_H
