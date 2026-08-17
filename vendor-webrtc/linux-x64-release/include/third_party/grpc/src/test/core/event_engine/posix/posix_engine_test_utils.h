// Copyright 2022 gRPC Authors
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

#ifndef GRPC_TEST_CORE_EVENT_ENGINE_POSIX_POSIX_ENGINE_TEST_UTILS_H
#define GRPC_TEST_CORE_EVENT_ENGINE_POSIX_POSIX_ENGINE_TEST_UTILS_H

#include <grpc/event_engine/event_engine.h>

#include <utility>

#include "absl/functional/any_invocable.h"
#include "src/core/lib/event_engine/posix_engine/event_poller.h"

namespace grpc_event_engine {
namespace experimental {

class TestScheduler : public Scheduler {
 public:
  explicit TestScheduler(grpc_event_engine::experimental::EventEngine* engine)
      : engine_(engine) {}
  TestScheduler() : engine_(nullptr) {};
  void ChangeCurrentEventEngine(
      grpc_event_engine::experimental::EventEngine* engine) {
    engine_ = engine;
  }
  void Run(experimental::EventEngine::Closure* closure) override {
    if (engine_ != nullptr) {
      engine_->Run(closure);
    } else {
      closure->Run();
    }
  }

  void Run(absl::AnyInvocable<void()> cb) override {
    if (engine_ != nullptr) {
      engine_->Run(std::move(cb));
    } else {
      cb();
    }
  }

 private:
  grpc_event_engine::experimental::EventEngine* engine_;
};

// Creates a client socket and blocks until it connects to the specified
// server address. The function abort fails upon encountering errors.
int ConnectToServerOrDie(
    const grpc_event_engine::experimental::EventEngine::ResolvedAddress&
        server_address);

}  // namespace experimental
}  // namespace grpc_event_engine

#endif  // GRPC_TEST_CORE_EVENT_ENGINE_POSIX_POSIX_ENGINE_TEST_UTILS_H
